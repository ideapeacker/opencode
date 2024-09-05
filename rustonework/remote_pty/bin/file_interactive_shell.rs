use std::fs::File;
use std::io::{self};
use std::process::{Command, Stdio};

fn main() -> io::Result<()> {
    let fd = File::create_new("a.txt")?;

    let shell = remote_pty::get_shell_path()?;
    let i = r#"(sss)"#;

    let mut command = Command::new(shell);
    let command = command
        .current_dir("/home/kali/Desktop")
        .env("PS1", r#""[\u@\h \w]\\$ ""#)
        .stdin(Stdio::inherit())
        .stdout(fd.try_clone()?)
        .stderr(fd.try_clone()?);

    if let Ok(mut child) = command.spawn() {
        child.wait().expect("command wasn't running");
        println!("Child has finished its execution!");
    } else {
        println!("ls command didn't start");
    }
    Ok(())
}
