//!Implementation of [`TaskManager`]
use super::TaskControlBlock;
use crate::sync::UPSafeCell;
use alloc::collections::BinaryHeap;
use alloc::sync::Arc;
use core::cmp;
use lazy_static::*;

/// When a task is switched out by the scheduler, its scheduler stride increases
/// by one pass, where `pass = BIG_STRIDE / task.sched_prio`.
pub const BIG_STRIDE: usize = 512;

/// A array of `TaskControlBlock` that is thread-safe
pub struct TaskManager {
    ready_queue: BinaryHeap<HeapItem>,
}

struct HeapItem(pub Arc<TaskControlBlock>);

impl cmp::PartialEq for HeapItem {
    fn eq(&self, other: &Self) -> bool {
        self.0.inner_exclusive_access().sched_stride
            == other.0.inner_exclusive_access().sched_stride
    }
}
impl cmp::PartialOrd for HeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        let me = self.0.inner_exclusive_access().sched_stride;
        let other = other.0.inner_exclusive_access().sched_stride;

        if usize::abs_diff(me, other) <= BIG_STRIDE / 2 {
            other.partial_cmp(&me)
        } else {
            me.partial_cmp(&other)
        }
    }
}
impl cmp::Eq for HeapItem {}
impl cmp::Ord for HeapItem {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let me = self.0.inner_exclusive_access().sched_stride;
        let other = other.0.inner_exclusive_access().sched_stride;

        if usize::abs_diff(me, other) <= BIG_STRIDE / 2 {
            other.cmp(&me)
        } else {
            me.cmp(&other)
        }
    }
}

/// A simple FIFO scheduler.
impl TaskManager {
    ///Creat an empty TaskManager
    pub fn new() -> Self {
        Self {
            ready_queue: BinaryHeap::new(),
        }
    }
    /// Add process back to ready queue
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push(HeapItem(task));
    }
    /// Take a process out of the ready queue
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.ready_queue.pop().map(|item| item.0)
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
