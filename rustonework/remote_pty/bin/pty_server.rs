use chrono::prelude::*;
use std::io;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let (local, port) = remote_pty::args::get_config_2();
    let local = format!("{}:{}", local, port);
    println!("{:?}: Listen on {}", Local::now(), &local);
    let lister = TcpListener::bind(local).await?;
    if let Ok((c, _)) = lister.accept().await {
        let mut stdin = tokio::io::stdin();
        let (mut reader, mut writer) = c.into_split();
        tokio::spawn(async move {
            let _ = tokio::io::copy(&mut stdin, &mut writer).await;
        });
        let _ = tokio::io::copy(&mut reader, &mut tokio::io::stdout()).await;
    }

    Ok(())
}
