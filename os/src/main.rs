#![no_std]
#![no_main]

#[macro_use]
mod console;
mod lang_items;
mod sbi;
#[macro_use]
mod logging;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    unsafe extern "C" {
        safe fn stext(); // begin addr of text segment
        safe fn etext(); // end addr of text segment
        safe fn srodata(); // start addr of Read-Only data segment
        safe fn erodata(); // end addr of Read-Only data ssegment
        safe fn sdata(); // start addr of data segment
        safe fn edata(); // end addr of data segment
        safe fn sbss(); // start addr of BSS segment
        safe fn ebss(); // end addr of BSS segment
        safe fn boot_stack_lower_bound(); // stack lower bound
        safe fn boot_stack_top(); // stack top
    }
    clear_bss();
    trace!("This is a TRACE message");
    debug!("This is a DEBUG message");
    info!("This is an INFO message");
    warn!("This is a WARN message");
    error!("This is an ERROR message");

    info!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    info!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    info!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    debug!(
        ".boot_stack [{:#x}, {:#x})",
        boot_stack_lower_bound as usize, boot_stack_top as usize
    );

    panic!("shut down!");
}

fn clear_bss() {
    unsafe extern "C" {
        safe fn sbss();
        safe fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
