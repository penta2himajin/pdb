extern crate nix;

use std::io;
use std::io::{stdin, stdout, BufReader, Read, Write};
use std::fs::File;
use std::ptr::null_mut;
use std::ffi::c_void;
use std::mem::size_of;
use nix::unistd::Pid;
use nix::sys::signal::{kill, Signal};
use nix::sys::ptrace::{ptrace, Request};

fn main() {
    println!("pdb written by penta2himajin.");
    print!("Process ID: ");
    stdout().flush();
    let pid = Pid::from_raw(read::<i32>());
    proc_stop(pid);
    println!("{}", get_mem_addr(pid));
    print!("Process Address: ");
    stdout().flush();
    println!("{}", get_proc_data(pid, read::<usize>() as *mut c_void));
}

fn read<T: std::str::FromStr>() -> T {
    let mut input = String::new();
    stdin().read_line(&mut input)
        .expect("Couldn't receive correct input");
    return input.trim().parse().ok()
        .expect("Couldn't unwrap input");
}

fn proc_stop(pid: Pid) {
    kill(pid, Signal::SIGSTOP)
        .expect(&format!("Couldn't exec 'kill(SIGSTOP)' process {}", pid));
}

fn get_proc_data(pid: Pid, addr: *mut c_void) -> String {
    let mut data;
    unsafe{
        data = ptrace(Request::PTRACE_PEEKDATA, pid, addr, null_mut())
            .expect(&format!("Couldn't exec 'ptrace(PEEKDATA)' process {}", pid));
    }
    return data.to_string();
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
