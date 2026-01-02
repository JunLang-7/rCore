#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
extern crate alloc;

use ::user_lib::{OpenFlags, close, open, read};
use alloc::string::String;

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    let fd = open("filea\0file", OpenFlags::RDONLY);
    if fd == -1 {
        panic!("Error occured when opening file");
    }
    let fd = fd as usize;
    let mut buffer = [0u8; 16];
    let mut s = String::new();
    loop {
        let size = read(fd, &mut buffer) as usize;
        if size == 0 {
            break;
        }
        s.push_str(core::str::from_utf8(&buffer[..size]).unwrap());
    }
    println!("{}", s);
    close(fd);
    0
}
