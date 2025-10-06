// 内核模块
// 第01章：入门 - 基本内核结构

use crate::common::types::ToString;
use crate::drivers::uart;

pub mod conclusion;
pub mod exception;
pub mod memory;
pub mod paging;
pub mod process;
pub mod syscall;
pub mod usermode;

pub fn init() {
    // 第05章：Hello World - 初始化UART并输出Hello World
    uart::UART.init();
    uart::println("Hello, Rust OS!");
    uart::println("Welcome to OS in 1000 Lines!");

    // 第08章：异常处理 - 初始化异常处理
    exception::init_exception_handling();
    uart::println("Exception handling initialized");

    // 第09章：内存分配 - 初始化内存分配器
    // 假设从0x20000000开始有1MB的堆内存
    memory::init_memory(0x20000000, 1024 * 1024);

    // 测试内存分配
    if let Some(ptr) = memory::allocate(1024) {
        uart::print("Memory allocated at: 0x");
        uart::println(&(ptr as usize).to_string());
    } else {
        uart::println("Memory allocation failed!");
    }

    // 第10章：进程管理 - 初始化进程调度器
    process::init_scheduler();

    // 创建几个测试进程
    if let Some(pid1) = process::create_process(4096) {
        uart::print("Created process with PID: ");
        uart::println(&pid1.to_string());
    }

    if let Some(pid2) = process::create_process(4096) {
        uart::print("Created process with PID: ");
        uart::println(&pid2.to_string());
    }

    // 列出所有进程
    process::list_processes();

    // 第11章：页表 - 初始化虚拟内存管理
    if paging::init_paging() {
        uart::println("Paging initialized successfully");

        // 测试内存映射
        paging::map_memory(0x10000000, 0x20000000, 4096, 0x7); // 可读可写可执行
        paging::unmap_memory(0x10000000, 4096);
    } else {
        uart::println("Failed to initialize paging");
    }

    // 第13章：用户模式 - 初始化用户态管理
    usermode::init_user_mode();
    usermode::list_user_programs();

    // 第14章：系统调用 - 初始化系统调用处理
    syscall::init_syscalls();

    // 第15章：磁盘I/O - 初始化磁盘驱动
    if crate::drivers::disk::init_disk() == crate::drivers::disk::DiskResult::Success {
        uart::println("Disk driver initialized successfully");

        // 测试磁盘读写
        let mut buffer = [0u8; 512];
        if crate::drivers::disk::read_disk_block(0, &mut buffer)
            == crate::drivers::disk::DiskResult::Success
        {
            uart::println("Disk read test successful");
        }
    } else {
        uart::println("Failed to initialize disk driver");
    }

    // 第16章：文件系统 - 初始化文件系统
    if crate::fs::fat::init_filesystem() {
        uart::println("File system initialized successfully");
        crate::fs::fat::list_files();

        // 测试文件操作
        if let Some(mut file) = crate::fs::fat::open_file("HELLO") {
            let mut buffer = [0u8; 100];
            let bytes_read = crate::fs::fat::read_file(&mut file, &mut buffer);
            uart::print("Read ");
            uart::print(&bytes_read.to_string());
            uart::println(" bytes from file");
            crate::fs::fat::close_file(&mut file);
        }
    } else {
        uart::println("Failed to initialize file system");
    }

    // 第12章：应用程序 - 启动shell
    uart::println("Starting shell...");
    // crate::user::shell::run_shell(); // 注释掉以避免阻塞

    // 第17章：结语 - 总结和展望
    conclusion::print_conclusion();

    // 第07章：内核恐慌 - 演示panic处理
    // panic!("This is a kernel panic test");
}
