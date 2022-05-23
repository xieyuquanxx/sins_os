use core::panic::PanicInfo;

use log::error;

use crate::sbi::shutdown;

#[panic_handler]
/// 打印错误信息并关机
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        error!(
            "Panic at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        error!("Panic: {}", info.message().unwrap());
    }
    shutdown()
}
