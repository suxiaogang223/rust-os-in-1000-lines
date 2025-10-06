// Rust OS in 1000 Lines - 主入口文件
// 参考：https://operating-system-in-1000-lines.vercel.app/zh/

#![no_std]
#![no_main]

// 内核入口点
#[no_mangle]
pub extern "C" fn _start() -> ! {
    rust_os_in_1000::init();
    loop {}
}

// 第07章：内核恐慌 - panic处理函数
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    use rust_os_in_1000::common::types::ToString;
    use rust_os_in_1000::drivers::uart;

    uart::println("\n*** KERNEL PANIC ***");

    if let Some(location) = info.location() {
        uart::print("Panic at: ");
        uart::print(location.file());
        uart::print(":");
        uart::print(&location.line().to_string());
        uart::println("");
    }

    // 简化panic消息处理
    uart::print("Message: ");
    uart::println("Panic occurred");

    uart::println("System halted.");
    loop {}
}
