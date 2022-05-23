use core::arch::global_asm;

use log::info;
use riscv::register::{
    scause::{self, Exception, Trap},
    stval, stvec,
};

use crate::{batch::run_next_app, syscall::syscall};

pub use self::context::TrapContext;

mod context;

global_asm!(include_str!("trap.S"));

/// Trap初始化
pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        stvec::write(
            __alltraps as usize,
            riscv::register::utvec::TrapMode::Direct,
        );
    }
}

#[no_mangle]
/// Trap处理函数
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        // 如果是用户的系统调用
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            info!("[kernel] PageFault in application, kernel killed it.");
            run_next_app();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            info!("[kernel] IllegalInstruction in application, kernel killed it.");
            run_next_app();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    cx
}
