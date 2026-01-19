//! Types related to task management

use super::TaskContext;
use crate::mm::PhysPageNum;
use crate::sync::UPSafeCell;
use crate::task::KernelStack;
use crate::task::id::TaskUserRes;
use crate::task::id::kstack_alloc;
use crate::task::process::ProcessControlBlock;
use crate::trap::TrapContext;
use alloc::sync::{Arc, Weak};
use core::cell::RefMut;

pub struct TaskControlBlock {
    // immutable
    pub process: Weak<ProcessControlBlock>,
    pub kernel_stack: KernelStack,
    // mutable
    inner: UPSafeCell<TaskControlBlockInner>,
}

pub struct TaskControlBlockInner {
    pub res: Option<TaskUserRes>,
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    pub trap_cx_ppn: PhysPageNum,
    pub exit_code: Option<i32>,
}

impl TaskControlBlockInner {
    pub fn get_trap_cx(&self) -> &'static mut TrapContext {
        self.trap_cx_ppn.get_mut()
    }
    #[allow(unused)]
    fn get_status(&self) -> TaskStatus {
        self.task_status
    }
}

impl TaskControlBlock {
    pub fn inner_exclusive_access(&self) -> RefMut<'_, TaskControlBlockInner> {
        self.inner.exclusive_access()
    }

    pub fn get_user_token(&self) -> usize {
        let process = self.process.upgrade().unwrap();
        process.inner_exclusive_access().get_user_token()
    }

    pub fn new(
        process: Arc<ProcessControlBlock>,
        ustack_base: usize,
        alloc_user_res: bool,
    ) -> Self {
        let res = TaskUserRes::new(Arc::clone(&process), ustack_base, alloc_user_res);
        let trap_cx_ppn = res.trap_cx_ppn();
        let kstack = kstack_alloc();
        let kstack_top = kstack.get_top();
        Self {
            process: Arc::downgrade(&process),
            kernel_stack: kstack,
            inner: unsafe {
                UPSafeCell::new(TaskControlBlockInner {
                    res: Some(res),
                    task_status: TaskStatus::Ready,
                    task_cx: TaskContext::goto_trap_return(kstack_top),
                    trap_cx_ppn,
                    exit_code: None,
                })
            },
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    #[allow(unused)]
    UnInit,
    Ready,
    Running,
    Blocked,
    //Exited,
}
