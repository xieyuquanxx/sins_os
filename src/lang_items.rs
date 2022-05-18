use core::panic::PanicInfo;

use crate::{println, sbi::shutdown};

#[panic_handler]
///* 打印错误信息并关机
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "Panic at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!("Panic: {}", info.message().unwrap());
    }
    shutdown()
}
