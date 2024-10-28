//! Types related to task management

use super::TaskContext;

const MAX_SYSCALL_NUM: usize = 500;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// The task relative infomation
    pub task_info: TaskInfo,
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}

/// the relative information of task
#[derive(Copy, Clone)]
pub struct TaskInfo {
    /// task status (not used, always Runnings)
    pub status: TaskStatus,
    /// the struct recorded the sys_call times.
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// time task been scheduled
    pub start_time: usize,
}

impl TaskInfo {
    /// create a new taskinfo
    pub fn new() -> Self {
        TaskInfo {
            status: TaskStatus::UnInit,
            syscall_times: [0; MAX_SYSCALL_NUM],
            start_time: 0,
        }
    }
    /// update target taskinfo's information
    pub fn update(&mut self, sys_call_idx: u32) {
        self.syscall_times[sys_call_idx as usize] += 1;
    }
}
