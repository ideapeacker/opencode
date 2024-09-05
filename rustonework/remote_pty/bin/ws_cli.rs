use futures_util::{future, pin_mut, StreamExt};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::TcpStream;

use tokio_tungstenite::tungstenite::error::Error as WsError;
use tokio_tungstenite::tungstenite::handshake::client::{generate_key, Request, Response};
use tokio_tungstenite::tungstenite::http::{
    request::Builder as HttpReqBuilder, response::Builder as HttpRespBuilder, Response as HttpResp,
    Uri, Version,
};
use tokio_tungstenite::{
    client_async, connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};

async fn cli_async() -> Result<(WebSocketStream<TcpStream>, Response), WsError> {
    let req = HttpReqBuilder::new();
    let uri = "ws://127.0.0.1:9000/";
    let host = "127.0.0.1:9000";
    let stream = tokio::net::TcpStream::connect(host).await.unwrap();

    match client_async(
        req.method("GET")
            .header("Host", host)
            .header("Connection", "Upgrade")
            .header("Upgrade", "websocket")
            .header("Sec-WebSocket-Version", "13")
            .header("Sec-WebSocket-Key", generate_key())
            .header("Key", generate_key())
            .header("MCookie", "&xjwshenHZJLSDH")
            .uri(uri)
            .body(())
            .unwrap(),
        stream,
    )
    .await
    {
        Ok((ws_stream, resp)) => Ok((ws_stream, resp)),
        Err(e) => Err(WsError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            e.to_string(),
        ))),
    }
}

async fn url_async() -> Result<(WebSocketStream<MaybeTlsStream<TcpStream>>, Response), WsError> {
    let url = url::Url::parse("ws://127.0.0.1:9000/").unwrap();

    match connect_async(url).await {
        Ok((ws_stream, resp)) => Ok((ws_stream, resp)),
        Err(e) => Err(WsError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            e.to_string(),
        ))),
    }
}

#[tokio::main]
async fn main() {
    let (ws_stream, resp) = url_async().await.unwrap();

    println!("WebSocket handshake has been successfully completed");

    println!("STATUS:{:?}", resp.status());
    if Version::HTTP_09 == resp.version() {
        println!("Version: HTTP_09");
    } else if Version::HTTP_10 == resp.version() {
        println!("Version: HTTP_10");
    } else if Version::HTTP_11 == resp.version() {
        println!("Version: HTTP_11");
    }

    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
    tokio::spawn(read_stdin(stdin_tx));

    let (write, read) = ws_stream.split();

    let stdin_to_ws = stdin_rx.map(Ok).forward(write);
    let ws_to_stdout = {
        read.for_each(|message| async {
            let data = message.unwrap().into_data();
            tokio::io::stdout().write_all(&data).await.unwrap();
        })
    };

    pin_mut!(stdin_to_ws, ws_to_stdout);
    future::select(stdin_to_ws, ws_to_stdout).await;
}

// Our helper method which will read data from stdin and send it along the
// sender provided.
async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>) {
    let mut stdin = tokio::io::stdin();
    loop {
        let mut buf = vec![0; 1024];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };
        buf.truncate(n);
        tx.unbounded_send(Message::binary(buf)).unwrap();
    }
}
