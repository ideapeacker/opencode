use std::io::{self};
use std::process::{Command, Stdio};

fn main() -> io::Result<()> {
    let shell = remote_pty::get_shell_path()?;

    println!("shell env: {:?}", &shell);

    let mut command = Command::new(shell);
    let command = command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(io::stdout());

    if let Ok(mut child) = command.spawn() {
        child.wait().expect("command wasn't running");
        println!("Child has finished its execution!");
    } else {
        println!("ls command didn't start");
    }
    Ok(())
}
