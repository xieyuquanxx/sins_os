#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod console;
mod lang_items;
mod logger;
mod sbi;

use core::{arch::global_asm, panic};

use log::*;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
fn rust_main() -> ! {
    clear_bss();
    // 初始化log
    logger::init().unwrap();

    println!("Hello World!");
    error!("log error");
    warn!("log warn");
    info!("log info");
    debug!("log debug");
    trace!("log trace");

    panic!("Shutdown machine!");
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
