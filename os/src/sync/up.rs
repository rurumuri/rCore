use core::cell::{RefCell, RefMut};

pub struct UPSafeCell<T> {
    inner: RefCell<T>,
}

unsafe impl<T> Sync for UPSafeCell<T> {}

impl<T> UPSafeCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: RefCell::new(value),
        }
    }
    pub fn exclusive_access(&self) -> RefMut<T> {
        self.inner.borrow_mut()
    }
}
