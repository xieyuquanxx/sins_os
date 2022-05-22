#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod batch;
mod console;
mod lang_items;
mod logger;
mod sbi;
mod sync;
mod syscall;
mod trap;

use core::arch::global_asm;

use log::*;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
fn rust_main() -> ! {
    clear_bss();
    // 初始化log
    logger::init().unwrap();
    trap::init();
    batch::init();

    error!("Hello SinsOS");

    batch::run_next_app();
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
