use std::io::{self, Read};

fn main() {
    let mut input = [0u8; 1];

    loop {
        match io::stdin().read(&mut input) {
            Ok(size) => {
                if size > 0 {
                    println!(
                        "{}=>{:?}:{:?}",
                        size,
                        &input,
                        String::from_utf8_lossy(&input[0..size])
                    );
                }
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
}
