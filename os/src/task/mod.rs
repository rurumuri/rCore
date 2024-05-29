mod task;
mod switch;
mod context;

use crate::sync::UPSafeCell;
use task::{TaskControlBlock, TaskStatus};
use crate::config::MAX_APP_NUM;
use lazy_static::lazy_static;
use context::TaskContext;


pub struct TaskManager {
    num_app: usize,
    inner: UPSafeCell<TaskManagerInner>,
}

struct TaskManagerInner {
    tasks: [TaskControlBlock; MAX_APP_NUM],
    current_task: usize,
}