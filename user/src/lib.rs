#![no_std]
#![feature(linkage)]

#[macro_use]
pub mod console;
mod lang_items;
mod syscall;

#[repr(C)]
pub struct TaskInfo {
    id: isize,
    name: [u8; 32],
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.entry")]
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
    panic!("unreachable after sys_exit!");
}

#[linkage = "weak"]
#[unsafe(no_mangle)]
fn main() -> i32 {
    panic!("Cannot find main!");
}

fn clear_bss() {
    unsafe extern "C" {
        safe fn sbss();
        safe fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

use syscall::*;

pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}

pub fn exit(exit_code: i32) -> isize {
    sys_exit(exit_code)
}

pub fn get_taskinfo(ti: &mut TaskInfo) -> isize {
    sys_get_taskinfo(ti as *mut TaskInfo as *mut u8)
}

pub fn print_taskinfo() {
    let mut ti = TaskInfo {
        id: 0,
        name: [0u8; 32],
    };
    if let 0 = get_taskinfo(&mut ti) {
        println!("Task ID: {}", ti.id);
        print!("Task Name: ");
        for c in ti.name.iter() {
            if *c == 0 {
                break;
            }
            print!("{}", *c as char);
        }
        println!("");
    } else {
        println!("Failed to get the task info");
    }
}
