//! File and filesystem-related syscalls

use crate::{
    batch::{
        stack::{USER_STACK, USER_STACK_SIZE},
        APP_BASE_ADDRESS, APP_SIZE_LIMIT,
    },
    print,
};

const FD_STDOUT: usize = 1;

/// write buf of length `len`  to a file with `fd`
pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            // 每个应用程序只能访问自己的空间
            if ((buf as usize) >= USER_STACK.get_sp() - USER_STACK_SIZE
                && (buf as usize + len) <= USER_STACK.get_sp())
                || ((buf as usize) >= APP_BASE_ADDRESS
                    && (buf as usize) <= APP_BASE_ADDRESS + APP_SIZE_LIMIT)
            {
                let slice = unsafe { core::slice::from_raw_parts(buf, len) };
                let str = core::str::from_utf8(slice).unwrap();
                print!("{}", str);
                len as isize
            } else {
                panic!("Cannot access buf:[{}]", buf as usize);
            }
        }
        _ => {
            panic!("Unsupported fd in sys_write!");
        }
    }
}
