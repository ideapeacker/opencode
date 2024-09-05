use async_ssh2_tokio::client::{AuthMethod, Client, ServerCheckMethod};

#[tokio::main]
async fn main() -> Result<(), async_ssh2_tokio::Error> {
    // if you want to use key auth, then use following:
    // AuthMethod::with_key_file("key_file_name", Some("passphrase"));
    // or
    // AuthMethod::with_key_file("key_file_name", None);
    // or
    // AuthMethod::with_key(key: &str, passphrase: Option<&str>)
    let auth_method = AuthMethod::with_password("kali");
    let client = Client::connect(
        ("127.0.0.1", 22),
        "kali",
        auth_method,
        ServerCheckMethod::NoCheck,
    )
    .await?;

    let result = client.execute("echo Hello SSH").await?;
    println!("{:?}", result.stdout);
    assert_eq!(result.exit_status, 0);

    let result = client
        .execute("cat /home/kali/workspace/rustonework/target/release/ssh2_cli")
        .await?;

    println!("{}=>{:?}", &result.stdout.len(), result);

    Ok(())
}
