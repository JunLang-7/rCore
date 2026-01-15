use crate::{Stat, TimeVal};
use core::arch::asm;

const SYS_OPEN: usize = 56;
const SYS_CLOSE: usize = 57;
const SYS_UNLINKAT: usize = 35;
const SYS_LINKAT: usize = 37;
const SYS_READ: usize = 63;
const SYS_WRITE: usize = 64;
const SYS_FSTAT: usize = 80;
const SYS_EXIT: usize = 93;
const SYS_YIELD: usize = 124;
const SYS_SET_PRIORITY: usize = 140;
const SYS_GET_TIME: usize = 169;
const SYS_GETPID: usize = 172;
const SYS_SBRK: usize = 214;
const SYS_FORK: usize = 220;
const SYS_EXEC: usize = 221;
const SYS_WAITPID: usize = 260;
const SYS_SPAWN: usize = 400;

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id
        );
    }
    ret
}

pub fn syscall6(id: usize, args: [usize; 6]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x13") args[3],
            in("x14") args[4],
            in("x15") args[5],
            in("x17") id
        );
    }
    ret
}

pub fn sys_open(path: &str, flags: u32) -> isize {
    syscall(SYS_OPEN, [path.as_ptr() as usize, flags as usize, 0])
}

pub fn sys_close(fd: usize) -> isize {
    syscall(SYS_CLOSE, [fd, 0, 0])
}

pub fn sys_read(fd: usize, buffer: &mut [u8]) -> isize {
    syscall(SYS_READ, [fd, buffer.as_mut_ptr() as usize, buffer.len()])
}

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYS_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

pub fn sys_linkat(
    old_dirfd: usize,
    old_path: &str,
    new_dirfd: usize,
    new_path: &str,
    flags: usize,
) -> isize {
    syscall6(
        SYS_LINKAT,
        [
            old_dirfd,
            old_path.as_ptr() as usize,
            new_dirfd,
            new_path.as_ptr() as usize,
            flags,
            0,
        ],
    )
}

pub fn sys_unlinkat(dirfd: usize, path: &str, flags: usize) -> isize {
    syscall(SYS_UNLINKAT, [dirfd, path.as_ptr() as usize, flags])
}

pub fn sys_fstat(fd: usize, st: &mut Stat) -> isize {
    syscall(SYS_FSTAT, [fd, st as *const _ as usize, 0])
}

pub fn sys_exit(xstate: i32) -> ! {
    syscall(SYS_EXIT, [xstate as usize, 0, 0]);
    panic!("sys_exit never returns!");
}

pub fn sys_yield() -> isize {
    syscall(SYS_YIELD, [0, 0, 0])
}

pub fn sys_set_priority(prio: isize) -> isize {
    syscall(SYS_SET_PRIORITY, [prio as usize, 0, 0])
}

pub fn sys_get_time(time: &mut TimeVal, tz: usize) -> isize {
    syscall(SYS_GET_TIME, [time as *const _ as usize, tz, 0])
}

pub fn sys_getpid() -> isize {
    syscall(SYS_GETPID, [0, 0, 0])
}

pub fn sys_sbrk(size: i32) -> isize {
    syscall(SYS_SBRK, [size as usize, 0, 0])
}

pub fn sys_fork() -> isize {
    syscall(SYS_FORK, [0, 0, 0])
}

pub fn sys_exec(path: &str) -> isize {
    syscall(SYS_EXEC, [path.as_ptr() as usize, 0, 0])
}

pub fn sys_waitpid(pid: isize, exit_code: *mut i32) -> isize {
    syscall(SYS_WAITPID, [pid as usize, exit_code as usize, 0])
}

pub fn sys_spawn(path: &str) -> isize {
    syscall(SYS_SPAWN, [path.as_ptr() as usize, 0, 0])
}
