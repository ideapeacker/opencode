use freecyber::netspeed::download_speed;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let speed = download_speed("", "").unwrap();

    let esp = now.elapsed();

    println!("==>>{:?}", esp);
}
