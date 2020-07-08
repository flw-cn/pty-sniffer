use chrono::prelude::*;
use nix::pty::openpty;
use std::{
    env,
    fs::File,
    io::{self, Read, Write},
    os::unix::io::FromRawFd,
    process::{Child, Command, Stdio},
};

#[derive(Debug)]
pub struct Pty {
    process: Child,
    fd: i32,
}

fn create_pty(arg0: &str, args: &[String]) -> Pty {
    let pty = openpty(None, None).expect("Error: openpty()");
    let mut cmd = Command::new(arg0);
    cmd.stdin(Stdio::inherit());
    cmd.stdout(unsafe { Stdio::from_raw_fd(pty.slave) });
    cmd.stderr(unsafe { Stdio::from_raw_fd(pty.slave) });
    cmd.args(args);

    let process = cmd.spawn().expect("Error: spawn()");

    return Pty {
        process: process,
        fd: pty.master,
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let cmd = &args[1];
    let args = &args[2..];
    let pty = create_pty(cmd, args);
    let output = unsafe { File::from_raw_fd(pty.fd) };

    let now = Local::now();
    let log_file = format!("{}", now.format("pty-sniffer-%Y%m%d-%H%M%S.log"));

    let mut log = File::create(&log_file).unwrap();

    for b in output.bytes() {
        let buf = [b.unwrap() as u8];

        io::stdout().write(&buf[..]).unwrap();
        log.write(&buf[..]).unwrap();
    }

    println!("\x1b[0mLogs are generated at: {}", log_file);
}
