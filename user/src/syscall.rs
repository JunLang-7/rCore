use crate::{SignalAction, Stat, TimeVal};
use core::arch::asm;

const SYS_DUP: usize = 24;
const SYS_OPEN: usize = 56;
const SYS_CLOSE: usize = 57;
const SYS_UNLINKAT: usize = 35;
const SYS_LINKAT: usize = 37;
const SYS_PIPE: usize = 59;
const SYS_READ: usize = 63;
const SYS_WRITE: usize = 64;
const SYS_FSTAT: usize = 80;
const SYS_EXIT: usize = 93;
const SYS_SLEEP: usize = 101;
const SYS_YIELD: usize = 124;
const SYS_KILL: usize = 129;
const SYS_SIGACTION: usize = 134;
const SYS_SIGPROCMASK: usize = 135;
const SYS_SIGRETURN: usize = 139;
const SYS_SET_PRIORITY: usize = 140;
const SYS_GET_TIME: usize = 169;
const SYS_GETPID: usize = 172;
const SYS_SBRK: usize = 214;
const SYS_FORK: usize = 220;
const SYS_EXEC: usize = 221;
const SYS_WAITPID: usize = 260;
const SYS_SPAWN: usize = 400;
const SYS_THREAD_CREATE: usize = 1000;
const SYS_GETTID: usize = 1001;
const SYS_WAITTID: usize = 1002;
const SYS_MUTEX_CREATE: usize = 1010;
const SYS_MUTEX_LOCK: usize = 1011;
const SYS_MUTEX_UNLOCK: usize = 1012;
const SYS_SEMAPHORE_CREATE: usize = 1020;
const SYS_SEMAPHORE_UP: usize = 1021;
const SYS_SEMAPHORE_DOWN: usize = 1022;
const SYS_CONDVAR_CREATE: usize = 1030;
const SYS_CONDVAR_SIGNAL: usize = 1031;
const SYS_CONDVAR_WAIT: usize = 1032;

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

pub fn sys_dup(fd: usize) -> isize {
    syscall(SYS_DUP, [fd, 0, 0])
}

pub fn sys_open(path: &str, flags: u32) -> isize {
    syscall(SYS_OPEN, [path.as_ptr() as usize, flags as usize, 0])
}

pub fn sys_close(fd: usize) -> isize {
    syscall(SYS_CLOSE, [fd, 0, 0])
}

pub fn sys_pipe(pipe: &mut [usize]) -> isize {
    syscall(SYS_PIPE, [pipe.as_mut_ptr() as usize, 0, 0])
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

pub fn sys_sleep(sleep_ms: usize) -> isize {
    syscall(SYS_SLEEP, [sleep_ms, 0, 0])
}

pub fn sys_yield() -> isize {
    syscall(SYS_YIELD, [0, 0, 0])
}

pub fn sys_kill(pid: usize, signum: i32) -> isize {
    syscall(SYS_KILL, [pid, signum as usize, 0])
}

pub fn sys_sigaction(
    signum: i32,
    action: *const SignalAction,
    old_action: *mut SignalAction,
) -> isize {
    syscall(
        SYS_SIGACTION,
        [signum as usize, action as usize, old_action as usize],
    )
}

pub fn sys_sigprocmask(mask: u32) -> isize {
    syscall(SYS_SIGPROCMASK, [mask as usize, 0, 0])
}

pub fn sys_sigreturn() -> isize {
    syscall(SYS_SIGRETURN, [0, 0, 0])
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

pub fn sys_exec(path: &str, args: &[*const u8]) -> isize {
    syscall(
        SYS_EXEC,
        [path.as_ptr() as usize, args.as_ptr() as usize, 0],
    )
}

pub fn sys_waitpid(pid: isize, exit_code: *mut i32) -> isize {
    syscall(SYS_WAITPID, [pid as usize, exit_code as usize, 0])
}

pub fn sys_spawn(path: &str) -> isize {
    syscall(SYS_SPAWN, [path.as_ptr() as usize, 0, 0])
}

pub fn sys_thread_create(entry: usize, arg: usize) -> isize {
    syscall(SYS_THREAD_CREATE, [entry, arg, 0])
}

pub fn sys_gettid() -> isize {
    syscall(SYS_GETTID, [0, 0, 0])
}

pub fn sys_waittid(tid: usize) -> isize {
    syscall(SYS_WAITTID, [tid, 0, 0])
}

pub fn sys_mutex_create(blocking: bool) -> isize {
    syscall(SYS_MUTEX_CREATE, [blocking as usize, 0, 0])
}

pub fn sys_mutex_lock(mutex_id: usize) -> isize {
    syscall(SYS_MUTEX_LOCK, [mutex_id, 0, 0])
}

pub fn sys_mutex_unlock(mutex_id: usize) -> isize {
    syscall(SYS_MUTEX_UNLOCK, [mutex_id, 0, 0])
}

pub fn sys_semaphore_create(res_count: usize) -> isize {
    syscall(SYS_SEMAPHORE_CREATE, [res_count, 0, 0])
}

pub fn sys_semaphore_up(sem_id: usize) -> isize {
    syscall(SYS_SEMAPHORE_UP, [sem_id, 0, 0])
}

pub fn sys_semaphore_down(sem_id: usize) -> isize {
    syscall(SYS_SEMAPHORE_DOWN, [sem_id, 0, 0])
}

pub fn sys_condvar_create() -> isize {
    syscall(SYS_CONDVAR_CREATE, [0, 0, 0])
}

pub fn sys_condvar_signal(condvar_id: usize) -> isize {
    syscall(SYS_CONDVAR_SIGNAL, [condvar_id, 0, 0])
}

pub fn sys_condvar_wait(condvar_id: usize, mutex_id: usize) -> isize {
    syscall(SYS_CONDVAR_WAIT, [condvar_id, mutex_id, 0])
}
