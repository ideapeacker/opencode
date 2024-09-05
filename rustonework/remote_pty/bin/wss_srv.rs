use std::error::Error as StdError;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_rustls::{rustls, TlsAcceptor};

use remote_pty::self_cert;

use futures_util::StreamExt;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    let (fullchain, privkey) = self_cert::get_self_signed_cert()?;

    let config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(vec![fullchain], privkey)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidInput, err))?;

    let listener = TcpListener::bind(format!("[::]:{}", 4443)).await?;

    let acceptor = TlsAcceptor::from(Arc::new(config));
    let acceptor = Arc::new(acceptor);

    while let Ok((mut stream, addr)) = listener.accept().await {
        let acceptor = Arc::clone(&acceptor);
        tokio::spawn(async move {
            match acceptor.accept(&mut stream).await {
                Ok(stream) => {
                    println!("Err accept a ssl connection. {:?}", &addr);
                    match tokio_tungstenite::accept_async(stream).await {
                        Ok(ws_stream) => {
                            println!("New WSS Streamer: {:?}", &addr);
                            let _ = ws_stream
                                .for_each(|message| async {
                                    let data = message.unwrap().into_data();
                                    tokio::io::stdout().write_all(&data).await.unwrap();
                                })
                                .await;
                        }
                        Err(e) => eprintln!("Error: TCP to WS Transform | {}", e),
                    }
                }
                Err(e) => {
                    eprintln!("Err accept a ssl connection. #{e}");
                }
            }
        });
    }
    Ok(())
}
