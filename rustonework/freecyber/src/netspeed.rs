use flate2::write::GzEncoder;
use flate2::Compression;
use pretty_bytes::converter::convert;
use reqwest::blocking::Body;
use reqwest::blocking::{Client, ClientBuilder};
use reqwest::Proxy;
use std::io::{Cursor, Read};

/// "socks5://192.168.1.1:9000"
///
/// "http://127.0.0.1:7890"
pub fn download_speed(proxy: &str, url: &str) -> Result<String, reqwest::Error> {
    // let download_url = "http://speedtest.tele2.net/10MB.zip"; // 测试下载速度的URL
    let proxy = Proxy::http(proxy)?;
    let client = ClientBuilder::new().proxy(proxy).build().unwrap();
    let response = client.get(url).send()?;
    let content_length = response.content_length().unwrap_or(0);
    println!("conent_length:{}", content_length);

    let body = response.bytes()?;
    println!("body.len:{}", body.len());

    let speed = content_length as f64 / body.len() as f64;
    let speed = convert(speed);
    Ok(format!("{}/s", speed))
}

pub fn upload_speed(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let upload_url = "http://httpbin.org/post"; // 测试上传速度的URL

    let data_size = 10 * 1024 * 1024; // 10MB
    let data = Cursor::new(vec![0u8; data_size]);
    let mut encoder = GzEncoder::new(data, Compression::default());
    let mut buffer = Vec::new();
    encoder.read_to_end(&mut buffer)?;

    let client = Client::new();
    let body = Body::from(buffer);
    let response = client.post(url).body(body).send()?;
    let content_length = response.content_length().unwrap_or(0);
    let speed = content_length as f64 / data_size as f64;
    let speed = convert(speed);
    Ok(format!("{}/s", speed))
}
