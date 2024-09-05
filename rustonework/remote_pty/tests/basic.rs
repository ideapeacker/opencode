use remote_pty::self_cert;

#[test]
fn test_self_signed_cert() {
    use remote_pty::self_cert;

    if let Ok(s) = self_cert::get_self_signed_cert() {
        //println!("{:?}", s);
    }
}
