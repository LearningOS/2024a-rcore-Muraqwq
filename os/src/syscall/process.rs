//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    mm::translated_byte_buffer,
    task::{
        change_program_brk, current_user_token, exit_current_and_run_next, get_current_task_info,
        mmap, suspend_current_and_run_next, unmap, TaskStatus,
    },
    timer::{get_time_ms, get_time_us},
};

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
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let mut p_ts = translated_byte_buffer(
        current_user_token(),
        _ts as *const u8,
        core::mem::size_of::<TimeVal>(),
    );
    unsafe {
        let buf: &mut [u8] = p_ts[0];

        let us = get_time_us();
        let time_val = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
        let time_byte = &time_val as *const TimeVal as *const u8;
        let time_val_bytes: &[u8] =
            core::slice::from_raw_parts(time_byte, core::mem::size_of::<TimeVal>());

        buf.copy_from_slice(time_val_bytes);
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");
    let kernel_info = get_current_task_info();
    let mut p_ti = translated_byte_buffer(
        current_user_token(),
        _ti as *const u8,
        core::mem::size_of::<TaskInfo>(),
    );

    unsafe {
        let buf: &mut [u8] = p_ti[0];

        let task_info = TaskInfo {
            status: TaskStatus::Running,
            syscall_times: kernel_info.syscall_times,
            time: get_time_ms() - kernel_info.start_time,
        };

        let info_byte = &task_info as *const TaskInfo as *const u8;
        let task_info_bytes: &[u8] =
            core::slice::from_raw_parts(info_byte, core::mem::size_of::<TaskInfo>());
        buf.copy_from_slice(task_info_bytes);
    }
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    mmap(_start, _len, _port)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    unmap(_start, _len)
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
