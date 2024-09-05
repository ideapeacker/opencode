use chrono::prelude::*;
use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpListener;
use std::process;
use std::time::Duration;
fn main() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:9000")?;
    if let Ok((mut stream, addr)) = listener.accept() {
        println!("accept:{:?}", addr.to_string());

        let _ = stream.set_nodelay(true);
        let _ = stream.set_nonblocking(true);
        let _ = stream.set_read_timeout(Some(Duration::new(5, 0)));

        let mut input = String::new();

        loop {
            print!("[ IN ]:");
            let _ = io::stdout().flush();

            input.clear();
            if let Ok(size) = io::stdin().read_line(&mut input) {
                if size > 0 {
                    if size == 5 && input.eq("exit\n") {
                        break;
                    }

                    if let Err(e) = stream.write(input.as_bytes()) {
                        eprintln!("{:?}", e);
                        process::exit(0);
                    }
                }
            }

            let mut buffer = [0u8; 4096];
            loop {
                match stream.read(&mut buffer) {
                    Ok(size) => {
                        println!(
                            "[read] {:?} =>{}:{:?}",
                            Local::now(),
                            size,
                            String::from_utf8_lossy(&buffer[0..size])
                        );

                        if size < 4096 {
                            break;
                        }

                        let _ = io::stdout().write(&buffer[0..size]);
                    }
                    Err(e) => {
                        if e.kind() == ErrorKind::TimedOut {
                            println!("read err: {:?} {:?}", Local::now(), e);
                            break;
                        }
                        if e.kind() == ErrorKind::WouldBlock {
                        } else {
                            println!("read2: {:?}", e);
                            process::exit(0);
                        }
                    }
                }
                //thread::sleep(Duration::from_millis(10));
            }
        }
    }
    Ok(())
}
