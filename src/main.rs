#![no_std]
#![no_main]

mod lang_items;
mod sbi;

use core::arch::global_asm;

use sbi::{console_putchar, shutdown};
global_asm!(include_str!("entry.asm"));

#[no_mangle]
fn rust_main() -> ! {
    clear_bss();

    console_putchar('O' as usize);
    console_putchar('K' as usize);

    shutdown();
}

///* bss 段存放未初始化的全局或静态变量，只有初始化后才能够读写 \
///* 清空bss段
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|addr| unsafe {
        (addr as *mut u8).write_volatile(0);
    })
}
