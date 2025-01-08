/*!
 * task
 * task (user app) execute and switch
 *
*/

mod context;
mod switch;
mod task;

use core::cell::RefCell;

use crate::config::MAX_APP_NUM;
use crate::loader::{get_app_data, get_app_num};
use crate::sbi::shutdown;
use crate::sync::UPSafeCell;
use crate::trap::context::TrapContext;
use alloc::vec::Vec;
use context::TaskContext;
use lazy_static::lazy_static;
use log::trace;
use switch::__switch;
use task::{TaskControlBlock, TaskStatus};

pub struct TaskManager {
    num_app: usize,
    inner: UPSafeCell<TaskManagerInner>, // use `UPSafeCell` so that we can visit inner globally and safely
}

struct TaskManagerInner {
    tasks: Vec<TaskControlBlock>,
    current_task: usize,
}

// // TASK_MANAGER will be initialized when first visited
// lazy_static! {
//     pub static ref TASK_MANAGER: TaskManager = {
//         // trace!("[kernel] TASK_MANAGER has been initialized");
//         let app_num = get_app_num();
//         let mut tasks = [
//             TaskControlBlock {
//                 task_status: TaskStatus::UnInit,
//                 task_cx: TaskContext::zero_init(),
//             };
//             MAX_APP_NUM
//         ];

//         // init apps' context and set their status as ready for the first run
//         for i in 0..app_num {
//             tasks[i].task_cx = TaskContext::goto_restore(init_app_cx(i));
//             tasks[i].task_status = TaskStatus::Ready;
//         }

//         TaskManager {
//             num_app: app_num,
//             inner: UPSafeCell::new(TaskManagerInner {
//                 tasks,
//                 current_task: 0,
//             })
//         }
//     };
// }

lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        println!("init TASK_MANAGER");
        let num_app = get_app_num();
        println!("num_app = {}", num_app);
        let mut tasks: Vec<TaskControlBlock> = Vec::new();
        for i in 0..num_app {
            tasks.push(TaskControlBlock::new(get_app_data(i), i));
        }
        TaskManager {
            num_app,
            inner: UPSafeCell::new(TaskManagerInner {
                tasks,
                current_task: 0,
            }),
        }
    };
}

impl TaskManager {
    /// Run the first task in task list.
    ///
    /// Generally, the first task in task list is an idle task (we call it zero process later).
    /// But in ch3, we load apps statically, so the first task is a real app.
    fn run_first_task(&self) -> ! {
        // trace!("[kernel] Running first app");
        let mut inner = self.inner.exclusive_access();
        let task0 = &mut inner.tasks[0];
        task0.task_status = TaskStatus::Running;
        let next_task_cx_ptr = &task0.task_cx as *const TaskContext;
        // log::trace!("111{:#x}", task0.task_cx.ra as usize);

        // from https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter3/3multiprogramming.html#sys-yield-sys-exit
        // 如果不手动 drop 的话，编译器会在 __switch 返回时，也就是当前应用被切换回来的时候才 drop，
        // 这期间我们都不能修改 TaskManagerInner ，甚至不能读（因为之前是可变借用），会导致内核 panic 报错退出。
        // 正因如此，我们需要在 __switch 前提早手动 drop 掉 inner 。
        drop(inner);

        let mut _unused = TaskContext::zero_init();
        // before this, we should drop local variables that must be dropped manually
        trace!("[kernel] Switching task context to the first app");
        unsafe {
            __switch(&mut _unused as *mut TaskContext, next_task_cx_ptr);
        }
        panic!("unreachable in run_first_task!");
    }

    /// Change the status of current `Running` task into `Ready`.
    /// Then it can be run when next time app loader is finding next runnable task
    fn mark_current_suspended(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Ready;
    }

    /// Change the status of current `Running` task into `Exited`.
    fn mark_current_exited(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Exited;
    }

    /// Find next task to run and return task id.
    ///
    /// In this case, we only return the first `Ready` task in task list.
    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        (current + 1..current + self.num_app + 1)
            .map(|id| id % self.num_app)
            .find(|id| inner.tasks[*id].task_status == TaskStatus::Ready)
    }

    /// Switch current `Running` task to the task we have found,
    /// or there is no `Ready` task and we can exit with all applications completed
    fn run_next_task(&self) {
        if let Some(next) = self.find_next_task() {
            let mut inner = self.inner.exclusive_access();
            let current = inner.current_task;
            inner.tasks[next].task_status = TaskStatus::Running;
            inner.current_task = next;
            let current_task_cx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContext;
            let next_task_cx_ptr = &inner.tasks[next].task_cx as *const TaskContext;
            drop(inner);
            // before this, we should drop local variables that must be dropped manually
            unsafe {
                __switch(current_task_cx_ptr, next_task_cx_ptr);
            }
            // go back to user mode
        } else {
            trace!("[kernel] All applications completed! Shutdown now.");
            shutdown(false);
        }
    }
}

/// run first task
pub fn run_first_task() {
    TASK_MANAGER.run_first_task();
}

/// rust next task
fn run_next_task() {
    TASK_MANAGER.run_next_task();
}

/// suspend current task
fn mark_current_suspended() {
    TASK_MANAGER.mark_current_suspended();
}

/// exit current task
fn mark_current_exited() {
    TASK_MANAGER.mark_current_exited();
}

/// suspend current task, then run next task
pub fn suspend_current_and_run_next() {
    mark_current_suspended();
    run_next_task();
}

/// exit current task, then run next task
pub fn exit_current_and_run_next() {
    mark_current_exited();
    run_next_task();
}

/// provide current task's id
pub fn get_cur_task_id() -> usize {
    let inner = TASK_MANAGER.inner.exclusive_access();
    let cur_task_id = inner.current_task;
    drop(inner);
    cur_task_id
}

impl TaskManager {
    /// Get the current 'Running' task's token.
    fn get_current_token(&self) -> usize {
        let inner = self.inner.exclusive_access();
        inner.tasks[inner.current_task].get_user_token()
    }

    /// Get the current 'Running' task's trap contexts.
    fn get_current_trap_cx(&self) -> &'static mut TrapContext {
        let inner = self.inner.exclusive_access();
        inner.tasks[inner.current_task].get_trap_cx()
    }
}

pub fn current_user_token() -> usize {
    TASK_MANAGER.get_current_token()
}

pub fn current_trap_cx() -> &'static mut TrapContext {
    TASK_MANAGER.get_current_trap_cx()
}
