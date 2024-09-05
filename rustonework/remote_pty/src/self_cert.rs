use rcgen::{generate_simple_self_signed, CertifiedKey};
use tokio_rustls::rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};

pub fn get_self_signed_cert(
) -> Result<(CertificateDer<'static>, PrivateKeyDer<'static>), Box<dyn std::error::Error>> {
    // Generate a certificate that's valid for "localhost" and "hello.world.example"
    let subject_alt_names = vec!["hello.world.example".to_string(), "localhost".to_string()];

    let CertifiedKey { cert, key_pair } = generate_simple_self_signed(subject_alt_names).unwrap();

    // The certificate is now valid for localhost and the domain "hello.world.example"
    // cargo test --test basic -- --nocapture
    // let key = rustls::PrivateKey(cert.serialize_private_key_der());
    let key = key_pair.serialized_der();
    let k = Vec::from(key);
    Ok((
        CertificateDer::from(cert),
        PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(k)),
    ))
}
