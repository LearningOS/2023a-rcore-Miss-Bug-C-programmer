//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus},
    timer::get_time_us,
};

use crate::syscall::{SYSCALL_TASK_INFO, SYSCALL_YIELD, SYSCALL_GET_TIME, SYSCALL_EXIT};
use crate::task::{current_task_info, mynew_syscall};
#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    pub(crate) status: TaskStatus,
    /// The numbers of syscall called by task
    pub(crate) syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    pub(crate) time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    mynew_syscall(SYSCALL_EXIT);
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    mynew_syscall(SYSCALL_YIELD);
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    mynew_syscall(SYSCALL_GET_TIME);
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    mynew_syscall(SYSCALL_TASK_INFO);
    trace!("kernel: sys_task_info");
    unsafe {
        (*_ti) = current_task_info()
    }
    0
}
