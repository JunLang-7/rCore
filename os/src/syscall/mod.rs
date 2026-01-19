const SYS_DUP: usize = 24;
const SYS_UNLINKAT: usize = 35;
const SYS_LINKAT: usize = 37;
const SYS_OPEN: usize = 56;
const SYS_CLOSE: usize = 57;
const SYS_PIPE: usize = 59;
const SYS_READ: usize = 63;
const SYS_WRITE: usize = 64;
const SYS_FSTAT: usize = 80;
const SYS_EXIT: usize = 93;
const SYS_SLEEP: usize = 101;
const SYS_YIELD: usize = 124;
const SYS_KILL: usize = 129;
const SYS_GET_TIME: usize = 169;
const SYS_GETPID: usize = 172;
const SYS_FORK: usize = 220;
const SYS_EXEC: usize = 221;
const SYS_WAITPID: usize = 260;
const SYS_THREAD_CREATE: usize = 1000;
const SYS_GETTID: usize = 1001;
const SYS_WAITTID: usize = 1002;
const SYS_MUTEX_CREATE: usize = 1010;
const SYS_MUTEX_LOCK: usize = 1011;
const SYS_MUTEX_UNLOCK: usize = 1012;

mod fs;
mod process;
mod sync;
mod thread;

use fs::*;
use process::*;
use sync::*;
use thread::*;

use crate::fs::Stat;

pub fn syscall(syscall_id: usize, args: [usize; 4]) -> isize {
    match syscall_id {
        SYS_DUP => sys_dup(args[0]),
        SYS_LINKAT => sys_linkat(args[1] as *const u8, args[3] as *const u8),
        SYS_UNLINKAT => sys_unlinkat(args[1] as *const u8),
        SYS_OPEN => sys_open(args[0] as *const u8, args[1] as u32),
        SYS_CLOSE => sys_close(args[0]),
        SYS_PIPE => sys_pipe(args[0] as *mut usize),
        SYS_READ => sys_read(args[0], args[1] as *const u8, args[2]),
        SYS_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYS_FSTAT => sys_fstat(args[0], args[1] as *mut Stat),
        SYS_EXIT => sys_exit(args[0] as i32),
        SYS_SLEEP => sys_sleep(args[0]),
        SYS_YIELD => sys_yield(),
        SYS_KILL => sys_kill(args[0] as usize, args[1] as i32),
        SYS_GET_TIME => sys_get_time(args[0] as *mut TimeVal, args[1]),
        SYS_GETPID => sys_getpid(),
        SYS_FORK => sys_fork(),
        SYS_EXEC => sys_exec(args[0] as *const u8, args[1] as *const usize),
        SYS_WAITPID => sys_waitpid(args[0] as isize, args[1] as *mut i32),
        SYS_THREAD_CREATE => sys_thread_create(args[0], args[1]),
        SYS_GETTID => sys_gettid(),
        SYS_WAITTID => sys_waittid(args[0]) as isize,
        SYS_MUTEX_CREATE => sys_mutex_create(args[0] == 1),
        SYS_MUTEX_LOCK => sys_mutex_lock(args[0]),
        SYS_MUTEX_UNLOCK => sys_mutex_unlock(args[0]),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}
