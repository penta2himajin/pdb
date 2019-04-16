extern crate nix;

use std::io;
use std::io::{BufReader, Read};
use std::fs::File;
use nix::unistd::Pid;
use nix::sys::signal::{kill, Signal};
use nix::sys::ptrace::{attach};

fn main() {
    println!("pdb written by penta2himajin.");
    let pid = read();
    let addr = get_mem_addr(pid);
    println!("{}", addr);
    proc_trace(pid);
}

fn read() -> Pid {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Couldn't receive correct input");
    return Pid::from_raw(input.trim().parse().ok().unwrap());
}

fn proc_trace(pid: Pid) {
    attach(pid).expect(&format!("Couldn't attach process {}", pid));
    kill(pid, Signal::SIGSTOP)
        .expect(&format!("Couldn't 'kill()' process {}", pid));
}

fn get_mem_addr(pid: Pid) -> String {
    let pid_str = pid.to_string();
    let mut buf_reader = BufReader::new(
        File::open(
            format!("/proc/{}/maps", pid_str)
        ).expect(&format!("Couldn't open /proc/{}/maps", pid_str))
    );
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)
        .expect(&format!("Couldn't read /proc/{}/maps", pid_str));
    return contents;
}
