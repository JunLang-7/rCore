use crate::batch::{get_running_app, run_next_app};

#[repr(C)]
pub struct TaskInfo {
    id: usize,
    name: [u8; 32],
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    run_next_app();
}

/// task info syscall
pub fn sys_get_taskinfo(ti: *mut TaskInfo) -> isize {
    let running_id = get_running_app();

    let mut info = TaskInfo {
        id: running_id,
        name: [0u8; 32],
    };

    let name_prefix = b"app_";
    info.name[0..4].copy_from_slice(name_prefix);
    info.name[4] = (running_id as u8) + b'0';

    unsafe { *ti = info; }
    0
}
