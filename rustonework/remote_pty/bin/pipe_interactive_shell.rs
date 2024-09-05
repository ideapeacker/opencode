use std::io::{self, Read};
use std::process::{Command, Stdio};

fn main() -> io::Result<()> {
    let shell = remote_pty::get_shell_path()?;

    let mut child = Command::new(shell);

    let child = child
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    if let Ok(mut child) = child.spawn() {
        let mut stdin = child.stdin.take().expect("Failed to open stdin");

        std::thread::spawn(move || {
            io::copy(&mut io::stdin(), &mut stdin).unwrap();
            // stdin
            //     .write_all("Hello, world!".as_bytes())
            //     .expect("Failed to write to stdin");
        });
        let mut output = child.stdout.take().expect("Failed to open stdout.");
        std::thread::spawn(move || {
            //io::copy(&mut output, &mut io::stdout()).unwrap();
            let mut buffer = [0u8; 2048];
            loop {
                match output.read(&mut buffer) {
                    Ok(size) => {
                        println!("{}=>{:?}", size, String::from_utf8_lossy(&buffer[0..size]));
                    }
                    Err(e) => {
                        eprintln!("{:?}", e);
                    }
                }
            }
        });
        let mut stderr = child.stderr.take().expect("Failed to open stdout.");
        std::thread::spawn(move || {
            //io::copy(&mut stderr, &mut io::stdout()).unwrap();
            let mut buffer = [0u8; 2048];
            loop {
                match stderr.read(&mut buffer) {
                    Ok(size) => {
                        println!("{}=>{:?}", size, String::from_utf8_lossy(&buffer[0..size]));
                    }
                    Err(e) => {
                        eprintln!("{:?}", e);
                    }
                }
            }
        });

        child.wait().expect("command wasn't running");
        println!("Child has finished its execution!");
    } else {
        println!("ls command didn't start");
    }
    Ok(())
}
