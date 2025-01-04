use log::trace;

// 经过多次debug，最终发现是缺了这里。
// 如果不加，经过gdb检查发现__switch处加载TaskContext（在a1寄存器）后ra为0，导致访问无效内存0x0从而使内核卡死，
// 而检查发现TaskContext发现内部的ra有效，于是考虑内存布局问题，从而解决。
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TaskContext {
    ra: usize,
    sp: usize,
    s: [usize; 12],
}

impl TaskContext {
    /// init task context
    pub fn zero_init() -> Self {
        Self {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }

    /// set task context {__restore ASM funciton, kernel stack, s_0..12 }
    /// Now we has multiple apps (tasks) and each app (task) has it's own kernel stack.
    /// so we need to give the specific kstack_ptr.
    pub fn goto_restore(kstack_ptr: usize) -> Self {
        extern "C" {
            fn __restore();
        }
        // trace!("[kernel] goto_restore: __restore at {:#x}", __restore as usize);
        Self {
            ra: __restore as usize,
            sp: kstack_ptr,
            s: [0; 12],
        }
    }
}
