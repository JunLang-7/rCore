use core::{arch::asm, ptr};

pub unsafe fn print_stack_trace() {
    let mut fp: *const usize;
    unsafe {
        asm!("mv {}, fp", out(reg) fp);
    }

    println!("== Begin stack trace ==");
    while fp != ptr::null() {
        let ra = unsafe { *fp.sub(1) };
        let sf = unsafe { *fp.sub(1) };
        println!("0x{:016x}, fp=0x{:016x}", ra, sf);
        fp = sf as *const usize;
    }
    println!("== End stack trace ==");
}
