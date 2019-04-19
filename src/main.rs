extern crate nix;

use std::usize;
use std::io::{stdin, stdout, BufReader, Read, Write};
use std::fs::File;
use nix::unistd::Pid;
use nix::sys::ptrace;

macro_rules! print_o {
    ($x:expr) => {
        print!("{}", $x);
        stdout().flush().unwrap();
    }
}

fn main() {
    println!("pdb written by penta2himajin.");
    print_o!("Process ID: ");
    let pid = read_pid();
    println!("{}", get_mem_addr(pid));
    print_o!("Process Address: ");
    println!("{:?}", ptrace::read(pid, read_addr()));
}

fn read<T: std::str::FromStr>() -> T {
    let mut input = String::new();
    stdin().read_line(&mut input)
        .expect("Couldn't receive correct input");
    input.trim().parse().ok()
        .expect("Couldn't unwrap input")
}

fn read_pid() -> Pid {
    Pid::from_raw(read::<i32>())
}

fn read_addr() -> ptrace::AddressType {
    let addr = read::<String>();
    usize::from_str_radix(
        addr.trim_start_matches("0x"),
        16
    ).unwrap() as ptrace::AddressType
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
    contents
}
