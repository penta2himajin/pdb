extern crate nix;

use std::io;
use std::ptr::null_mut;
use nix::unistd::Pid;
use nix::sys::ptrace::{ptrace, Request};

fn main() {
    println!("pdb written by penta2himajin.");
    let pid = Pid::from_raw(read::<i32>());
    proc_trace(pid);
}

fn read<T: std::str::FromStr>() -> T {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Couldn't receive correct input");
    return input.trim().parse().ok().unwrap();
}

fn proc_trace(pid: Pid){
    unsafe{
        ptrace(Request::PTRACE_ATTACH, pid, null_mut(), null_mut());
    }
}
