#![no_std]
#![no_main]

// 禁用标准库，使用no_std
// 这是操作系统内核的基本要求

pub mod arch;
pub mod common;
pub mod drivers;
pub mod fs;
pub mod kernel;
pub mod user;

// 内核初始化函数
pub fn init() {
    kernel::init();
}
