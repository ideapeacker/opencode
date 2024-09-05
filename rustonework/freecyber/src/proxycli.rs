use tokio::net::{TcpListener, TcpStream};

pub async fn forward(
    local: &str,
    port: u16,
    dest: &str,
    dst_port: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    let proxy = format!("{}:{}", local, port);
    println!("Real proxy server is {}:{}", dest, dst_port);
    let listener = TcpListener::bind(proxy).await?;

    while let Ok((mut stream, addr)) = listener.accept().await {
        let dst = dest.to_string();
        tokio::spawn(async move {
            match TcpStream::connect(format!("{}:{}", dst, dst_port)).await {
                Ok(mut cli) => {
                    if let Err(e) = tokio::io::copy_bidirectional(&mut stream, &mut cli).await {
                        eprintln!("copy_bidirectional : {:?}", e);
                    }
                }
                Err(e) => {
                    eprintln!("{:?} failed to connect proxy server. {:?}", &addr, e);
                }
            }
        });
    }
    Ok(())
}
