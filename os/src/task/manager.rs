//!Implementation of [`TaskManager`]

use super::TaskControlBlock;
use crate::sync::UPSafeCell;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use lazy_static::*;
///A array of `TaskControlBlock` that is thread-safe
pub struct TaskManager {
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
}

/// A simple FIFO scheduler.
impl TaskManager {
    ///Creat an empty TaskManager
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }
    /// Add process back to ready queue
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push_back(task);
    }
    /// Take a process out of the ready queue
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.get_next_and_add_pass()
    }

    fn get_next_and_add_pass(&mut self) -> Option<Arc<TaskControlBlock>> {
        let next_task_idx = self
            .ready_queue
            .iter()
            // .filter(|tcb| {
            //     tcb.inner_exclusive_access().task_status == TaskStatus::Ready
            //         || tcb.inner_exclusive_access().task_status == TaskStatus::UnInit
            // })
            .enumerate()
            .min_by_key(|(_, tcb)| tcb.get_stride())
            .map(|(idx, _)| idx);
        if let Some(idx) = next_task_idx {
            let next_task = self.ready_queue.remove(idx).unwrap();
            next_task.add_pass();
            return Some(next_task);
        }
        None
    }
}

lazy_static! {
    /// TASK_MANAGER instance through lazy_static!
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> =
        unsafe { UPSafeCell::new(TaskManager::new()) };
}

/// Add process to ready queue
pub fn add_task(task: Arc<TaskControlBlock>) {
    //trace!("kernel: TaskManager::add_task");
    TASK_MANAGER.exclusive_access().add(task);
}

/// Take a process out of the ready queue
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    //trace!("kernel: TaskManager::fetch_task");
    TASK_MANAGER.exclusive_access().fetch()
}
