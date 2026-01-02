#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{OpenFlags, close, open, read, write};

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    let test_str = "Hello, world!";
    let filea = "filea\0";
    let fd = open(filea, OpenFlags::CREATE | OpenFlags::WRONLY);
    assert!(fd > 0);
    let fd = fd as usize;
    write(fd, test_str.as_bytes());
    close(fd);

    let fd = open(filea, OpenFlags::RDONLY);
    assert!(fd > 0);
    let fd = fd as usize;
    let mut buf = [0u8; 100];
    let read_len = read(fd, &mut buf) as usize;
    close(fd);

    assert_eq!(test_str, core::str::from_utf8(&buf[..read_len]).unwrap());
    println!("filetest_simple passed!");
    0
}
