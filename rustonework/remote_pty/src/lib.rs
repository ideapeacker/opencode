use std::env;
use std::io::{self, Error};
use std::process::Command;

pub mod args;
pub mod self_cert;

pub fn get_shell_path() -> io::Result<String> {
    match env::var("SHELL") {
        Ok(shell) => Ok(shell),
        Err(_) => {
            if let Ok(output) = Command::new("which").arg("sh").output() {
                if output.status.success() {
                    Ok(String::from_utf8_lossy(&output.stdout).to_string())
                } else {
                    Err(Error::new(
                        io::ErrorKind::NotFound,
                        String::from_utf8_lossy(&output.stderr).to_string(),
                    ))
                }
            } else {
                Err(Error::new(io::ErrorKind::NotFound, "sh not found."))
            }
        }
    }
}
