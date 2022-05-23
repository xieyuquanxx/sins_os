use crate::trap::TrapContext;

/// 用户栈 8KB
pub const USER_STACK_SIZE: usize = 4096 * 2;
/// 内核栈 8KB
pub const KERNEL_STACK_SIZE: usize = 4096 * 2;

#[repr(align(4096))]
pub struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

#[repr(align(4096))]
pub struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

pub static KERNEL_STACK: KernelStack = KernelStack {
    data: [0; KERNEL_STACK_SIZE],
};

pub static USER_STACK: UserStack = UserStack {
    data: [0; USER_STACK_SIZE],
};

impl KernelStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }
    pub fn push_context(&self, cx: TrapContext) -> &'static mut TrapContext {
        let cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *cx_ptr = cx;
        }
        unsafe { cx_ptr.as_mut().unwrap() }
    }
}

impl UserStack {
    /// 返回栈顶指针，只需要返回数组data的最后一个元素
    /// risc-v的栈是向下增长的
    /// 切换应用时，只需设置sp为get_sp()的返回值即可
    pub fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}
