use chrono::prelude::*;

#[tokio::main]
async fn main() {
    println!("{:?}", Local::now());
}
