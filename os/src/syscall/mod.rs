const SYS_WRITE: usize = 64;
const SYS_EXIT: usize = 93;

mod fs;
mod process;

use fs::*;
use process::*;

pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYS_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYS_EXIT => sys_exit(args[0] as i32),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}
