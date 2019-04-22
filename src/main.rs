extern crate nix;

use std::{
    str,
    u64,
    fs::File,
    io::{
        stdin,
        stdout,
        BufReader,
        Read,
        Write
    }
};
use nix::{
    unistd::Pid,
    sys::{
        ptrace::{
            attach,
            detach,
            read,
            getregs,
            AddressType
        }
    }
};


macro_rules! print_ol {
    ($x:expr) => {
        print!("{}", $x);
        stdout().flush().unwrap();
    }
}

macro_rules! print_hex {
    ($x:expr) => {
        println!("{}", format!("{:x}", $x));
    }
}


fn main() {
    println!("\n ***** pdb ***** \n");
    print_ol!("Process ID: ");
    let pid = read_pid();
    attach(pid).unwrap();
    let regs = getregs(pid).unwrap();
    println!("{}", get_mem_addr(pid));
    print_hex!(regs.rsp);
    print_hex!(read(pid, read_addr()).unwrap());
    detach(pid).unwrap();
}

fn read_any<T: str::FromStr>() -> T {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().parse().ok().unwrap()
}

fn read_pid() -> Pid {
    Pid::from_raw(read_any::<i32>())
}

fn read_addr() -> AddressType {
    let addr = read_any::<String>();
    u64::from_str_radix(
        addr.trim_start_matches("0x"),
        16
    ).unwrap() as AddressType
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
