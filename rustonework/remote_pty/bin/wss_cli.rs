use futures_util::{future, pin_mut, StreamExt};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() {
    let connect_addr = "wss://127.0.0.1:4443";

    // TLS 客户端
    let config = ignore_cert_verify::configure_client();
    let config = Arc::new(config);
    let connector = tokio_tungstenite::Connector::Rustls(config);

    match tokio_tungstenite::connect_async_tls_with_config(
        connect_addr,
        None,
        false,
        Some(connector),
    )
    .await
    {
        Ok((ws_stream, resp)) => {
            println!("Status:{:?}", resp.status());

            let (write, read) = ws_stream.split();

            let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
            tokio::spawn(read_stdin(stdin_tx));

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
        Err(e) => {
            eprintln!("WSS ERR: #{e}");
        }
    }
}
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

pub mod ignore_cert_verify {
    use std::fmt::{Debug, Formatter};
    use tokio_rustls::rustls::client::danger::{
        HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier,
    };
    use tokio_rustls::rustls::pki_types::{CertificateDer, ServerName, UnixTime};
    use tokio_rustls::rustls::{ClientConfig, DigitallySignedStruct, Error, SignatureScheme};

    // 创建 ClientConfig 实例
    // 使用自定义证书验证器
    pub fn configure_client() -> ClientConfig {
        // let crypto = rustls::ClientConfig::builder()
        //     .with_safe_defaults()
        //     .with_custom_certificate_verifier(SkipServerVerification::new())
        //     .with_no_client_auth();

        let crypto = ClientConfig::builder()
            .dangerous()
            .with_custom_certificate_verifier(SkipServerVerification::new())
            .with_no_client_auth();
        crypto
    }

    // Implementation of `ServerCertVerifier` that verifies everything as trustworthy.
    struct SkipServerVerification;

    impl SkipServerVerification {
        fn new() -> std::sync::Arc<Self> {
            std::sync::Arc::new(Self)
        }
    }

    impl Debug for SkipServerVerification {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "SkipServerVerification []")
        }
    }

    impl ServerCertVerifier for SkipServerVerification {
        fn verify_server_cert(
            &self,
            _end_entity: &CertificateDer<'_>,
            _intermediates: &[CertificateDer<'_>],
            _server_name: &ServerName<'_>,
            _ocsp_response: &[u8],
            _now: UnixTime,
        ) -> Result<ServerCertVerified, Error> {
            Ok(ServerCertVerified::assertion())
        }

        fn verify_tls12_signature(
            &self,
            _message: &[u8],
            _cert: &CertificateDer<'_>,
            _dss: &DigitallySignedStruct,
        ) -> Result<HandshakeSignatureValid, Error> {
            Ok(HandshakeSignatureValid::assertion())
        }

        fn verify_tls13_signature(
            &self,
            _message: &[u8],
            _cert: &CertificateDer<'_>,
            _dss: &DigitallySignedStruct,
        ) -> Result<HandshakeSignatureValid, Error> {
            Ok(HandshakeSignatureValid::assertion())
        }

        fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
            vec![
                SignatureScheme::RSA_PKCS1_SHA1,
                SignatureScheme::ECDSA_SHA1_Legacy,
                SignatureScheme::RSA_PKCS1_SHA256,
                SignatureScheme::ECDSA_NISTP256_SHA256,
                SignatureScheme::ED25519,
            ]
        }
    }
}
