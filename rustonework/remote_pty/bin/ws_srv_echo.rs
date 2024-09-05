use futures_util::{future, StreamExt, TryStreamExt};
use std::io::Error;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let listener = TcpListener::bind("127.0.0.1:9000").await?;

    while let Ok((s, _)) = listener.accept().await {
        tokio::spawn(accept_ws_connection(s));
    }
    Ok(())
}

async fn accept_ws_connection(stream: TcpStream) {
    let peer_addr = stream.peer_addr().unwrap();
    println!("accept a tcp connect: {:?}", peer_addr);

    match tokio_tungstenite::accept_async(stream).await {
        Ok(ws_stream) => {
            println!("New WebSocket connection: {}", peer_addr);
            let (write, read) = ws_stream.split();
            // We should not forward messages other than text or binary.
            read.try_filter(|msg| future::ready(msg.is_text() || msg.is_binary()))
                .forward(write)
                .await
                .expect("Failed to forward messages")
        }
        Err(e) => {
            eprintln!("Error during the websocket handshake occurred. #{e}");
        }
    }
}
