use core::arch::asm;

use crate::{
    batch::stack::{KERNEL_STACK, USER_STACK},
    sync::up::UPSafeCell,
    trap::TrapContext,
};
use lazy_static::lazy_static;
use log::info;
pub mod stack;
// 最大应用数
const MAX_APP_NUM: usize = 5;
// 应用程序的首地址
pub const APP_BASE_ADDRESS: usize = 0x8040_0000;
pub const APP_SIZE_LIMIT: usize = 0x2_0000;

// 维护应用的信息
struct AppManager {
    // 应用个数
    num_app: usize,
    // 当前运行的应用下标
    current_app: usize,
    // 保存应用地址
    app: [usize; MAX_APP_NUM + 1],
}

//? 在第一次使用APP_MANAGER时执行
lazy_static! {
    static ref APP_MANAGER: UPSafeCell<AppManager> = unsafe {
        UPSafeCell::new({
            extern "C" {
                fn _num_app();
            }
            // 指向 _num_app
            let num_app_ptr = _num_app as usize as *const usize;
            // 读出里面的值
            let num_app = num_app_ptr.read_volatile();

            let mut app: [usize; MAX_APP_NUM + 1] = [0; MAX_APP_NUM + 1];
            // 获取app的首地址
            let app_raw: &[usize] = core::slice::from_raw_parts(num_app_ptr.add(1), num_app+1);
            // copy过去
            app[..=num_app].copy_from_slice(app_raw);
            AppManager {
                num_app,
                current_app: 0,
                app,
            }
        })
    };
}

impl AppManager {
    /// 加载应用程序到约定的位置
    unsafe fn load_app(&self, app_id: usize) {
        if app_id >= self.num_app {
            panic!("All applications completed!");
        }

        info!("[kernel] loading app_{}", app_id);
        // clear icache
        asm!("fence.i");
        // 清空应用区域
        core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, APP_SIZE_LIMIT).fill(0);
        // 应用程序的源地址
        let app_src = core::slice::from_raw_parts(
            self.app[app_id] as *const u8,
            self.app[app_id + 1] - self.app[app_id],
        );
        // 应用程序的目标地址
        let app_dst = core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, app_src.len());
        // 拷贝过去即可
        app_dst.copy_from_slice(app_src);
    }
    /// 加载下一个应用程序
    pub fn move_to_next_app(&mut self) {
        self.current_app += 1;
    }

    pub fn get_current_app(&self) -> usize {
        self.current_app
    }

    pub fn print_app_info(&self) {
        info!("[kernel] num_app = {}", self.num_app);
        for i in 0..self.num_app {
            info!(
                "[kernel] app_{} [{:#x}, {:#x})",
                i,
                self.app[i],
                self.app[i + 1]
            );
        }
    }
}

pub fn run_next_app() -> ! {
    let mut app_manager = APP_MANAGER.exclusive_access();
    let current_app = app_manager.get_current_app();
    unsafe {
        app_manager.load_app(current_app);
    }
    app_manager.move_to_next_app();
    drop(app_manager);
    // before this we have to drop local variables related to resources manually
    // and release the resources
    extern "C" {
        fn __restore(cx_addr: usize);
    }
    unsafe {
        __restore(KERNEL_STACK.push_context(TrapContext::app_init_context(
            APP_BASE_ADDRESS,
            USER_STACK.get_sp(),
        )) as *const _ as usize);
    }
    panic!("Unreachable in batch::run_current_app!");
}

/// init batch subsystem
pub fn init() {
    print_app_info();
}

/// print apps info
pub fn print_app_info() {
    APP_MANAGER.exclusive_access().print_app_info();
}
