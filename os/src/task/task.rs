/*!
 * task data structures
 * - [`TaskStatus`]
 * - [`TaskControlBlock`]
*/

use super::context::TaskContext;

/// for one task:
/// when init TASK_MANAGER: N/A -> `UnInit`
/// when init apps' task context at TASK_MANAGER: `UnInit` -> `Ready`
/// when `mark_current_suspended`: `Running` -> `Ready`
/// when task exit: `Running` -> `Exited`
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    UnInit,  // 未初始化
    Ready,   // 准备运行
    Running, // 正在运行
    Exited,  // 已退出
}

#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
}
