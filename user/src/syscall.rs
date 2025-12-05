use core::arch::asm;

const SYS_WRITE: usize = 64;
const SYS_EXIT: usize = 93;
const SYS_YIELD: usize = 124;
const SYS_GET_TIME: usize = 169;
const SYS_SBRK: usize = 214;

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

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYS_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

pub fn sys_exit(xstate: i32) -> isize {
    syscall(SYS_EXIT, [xstate as usize, 0, 0])
}

pub fn sys_yield() -> isize {
    syscall(SYS_YIELD, [0, 0, 0])
}

pub fn sys_get_time() -> isize {
    syscall(SYS_GET_TIME, [0, 0, 0])
}

pub fn sys_sbrk(size: i32) -> isize {
    syscall(SYS_SBRK, [size as usize, 0, 0])
}
