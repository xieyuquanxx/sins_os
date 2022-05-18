#![allow(unused)]
//? RustSBI服务
const SBI_SET_TIMER: usize = 0;
const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_CONSOLE_GETCHAR: usize = 2;
const SBI_CLEAR_IPI: usize = 3;
const SBI_SEND_IPI: usize = 4;
const SBI_REMOTE_FENCE_I: usize = 5;
const SBI_REMOTE_SFENCE_VMA: usize = 6;
const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;
const SBI_SHUTDOWN: usize = 8;

use core::arch::asm;

#[inline(always)]
/// 向RustSBI发起服务请求
/// which: 服务请求类型
/// arg0~arg2: 参数
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        asm!(
          "ecall",
          inlateout("x10") arg0 => ret, // x10 - a0
          in("x11") arg1, // x11 - a1
          in("x12") arg2, // x12 - a2
          in("x17") which // x17 - a7
        )
    }
    ret
}

/// 输出字符
pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}

/// 关机
pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    panic!("It should shutdown!");
}
