use riscv::register::sstatus::{self, Sstatus, SPP};

#[repr(C)]
pub struct TrapContext {
    pub x: [usize; 32],   // x0-x31
    pub sstatus: Sstatus, // environment info (S or U mode) in SPP
    pub sepc: usize,      // last instruction's address before trap, if trap is an exception.
}

impl TrapContext {
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }

    /// construct the initial TrapContext when firstly running app
    /// so that we can reuse the `__restore`
    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User);
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry,
        };
        cx.set_sp(sp);
        cx
    }
}
