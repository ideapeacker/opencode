use std::io::{BufRead, BufReader};
use std::time::Instant;

#[test]
fn test() {
    let file = std::fs::File::open("abc.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();

        let sock_proxy = format!("socks5://{}", &line.trim());
        println!("{}", sock_proxy);
        let cur = Instant::now();

        let speed = freecyber::netspeed::download_speed(&sock_proxy, "https://www.google.com");
        let now = cur.elapsed();

        println!("==>>{}s.{}ms", now.as_secs(), now.subsec_millis());
        match speed {
            Ok(speed) => {
                println!("{}=>{}", line, speed);
            }
            Err(e) => {
                println!("{}=>{}", line, e);
            }
        }
    }
}

#[ignore]
#[test]
fn test2() {
    let s = freecyber::netspeed::download_speed("", "").unwrap();
    println!("=>{}", s);
}
