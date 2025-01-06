/*
    all task switch operation happens in S mode. no S/U mode change.

    before `__switch`, it must have been invoked `__alltraps` to get into S mode.
    The U mode task's TaskContext has been saved to it's own kernel stack.

    then __switch will:
    - save kernel `sp` (stack pointer) , `ra` and `s0`~`s11` as the task_cx of current task to a0 address (current_task_cx_ptr)
    - load next task's task_cx to CPU registers
    - ret, then CPU will continue what kernel was doing for the next task

    finally kernel will use the trap context and `sret` in __restore, return to U mode and continue task.
*/

use super::context::TaskContext;
use core::arch::global_asm;

global_asm!(include_str!("switch.S"));

extern "C" {
    pub fn __switch(current_task_cx_ptr: *mut TaskContext, next_task_cx_ptr: *const TaskContext);
}
