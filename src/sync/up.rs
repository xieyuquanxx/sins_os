use core::cell::{RefCell, RefMut};

pub struct UPSafeCell<T> {
    /// inner data
    inner: RefCell<T>,
}

unsafe impl<T> Sync for UPSafeCell<T> {}

impl<T> UPSafeCell<T> {
    /// 运行在单处理器上
    pub unsafe fn new(data: T) -> Self {
        Self {
            inner: RefCell::new(data),
        }
    }

    /// 独占访问权，取得可变借用，panic if the data has been borrowed
    pub fn exclusive_access(&self) -> RefMut<'_, T> {
        self.inner.borrow_mut()
    }
}
