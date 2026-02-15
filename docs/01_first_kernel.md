# 第01章：第一个内核

## 概述

本章将创建我们的第一个Rust操作系统内核。我们将学习如何设置no_std环境，创建内核入口点，并实现基本的UART输出功能。

## 项目结构

首先创建基本的项目结构：

```
src/
├── main.rs            # 内核入口点
├── lib.rs             # 库文件
├── arch/              # 架构相关
│   ├── mod.rs
│   └── riscv.rs       # RISC-V架构支持
├── common/            # 通用工具
│   ├── mod.rs
│   ├── string.rs
│   └── types.rs
├── drivers/           # 设备驱动
│   ├── mod.rs
│   ├── uart.rs        # UART驱动
│   └── disk.rs        # 磁盘驱动（教学骨架）
└── kernel/            # 内核核心
    ├── mod.rs         # kernel::init() 初始化入口
    ├── exception.rs   # 异常/中断处理
    ├── memory.rs      # 内存分配（bump allocator）
    ├── process.rs     # 进程与调度（教学骨架）
    ├── paging.rs      # 分页/页表（教学骨架）
    ├── usermode.rs    # 用户态切换（教学骨架）
    ├── syscall.rs     # 系统调用分发（教学骨架）
    └── conclusion.rs  # 结语输出
```

更完整的仓库布局说明请参考上一节：[项目结构](./01_project_structure.md)。

## 内核入口点 (src/main.rs)

```rust
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
```

## 库文件 (src/lib.rs)

```rust
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
```

## RISC-V架构支持 (src/arch/riscv.rs)

本仓库把 RISC-V 相关的常量与 CSR 读写封装在 `src/arch/riscv.rs`。

它主要用于后续章节的：

- 异常/中断处理（`mtvec/mepc/mcause/mtval/mie/mstatus`）
- 用户态切换（`mstatus.MPP` 等）

详细解释请阅读第 02 章：[RISC-V架构基础](./02_riscv_basics.md)、[寄存器操作](./02_registers.md)、[控制状态寄存器](./02_csr.md)。

## 通用类型 (src/common/types.rs)

```rust
// 通用类型定义和ToString trait

// 自定义ToString trait，用于no_std环境
pub trait ToString {
    fn to_string(&self) -> &'static str;
}

impl ToString for usize {
    fn to_string(&self) -> &'static str {
        // 简化实现：目前返回占位字符串
        "123"
    }
}

impl ToString for u32 {
    fn to_string(&self) -> &'static str {
        // 简化实现：目前返回占位字符串
        "123"
    }
}

impl ToString for &str {
    fn to_string(&self) -> &'static str {
        "str"
    }
}
```

## UART驱动 (src/drivers/uart.rs)

```rust
// UART驱动实现

// UART寄存器地址（QEMU virt机器）
const UART_BASE: usize = 0x10000000;
const UART_RBR: usize = 0x00; // 接收缓冲寄存器
const UART_THR: usize = 0x00; // 发送保持寄存器
const UART_IER: usize = 0x01; // 中断使能寄存器
const UART_IIR: usize = 0x02; // 中断标识寄存器
const UART_FCR: usize = 0x02; // FIFO控制寄存器
const UART_LCR: usize = 0x03; // 线路控制寄存器
const UART_MCR: usize = 0x04; // 调制解调器控制寄存器
const UART_LSR: usize = 0x05; // 线路状态寄存器
const UART_MSR: usize = 0x06; // 调制解调器状态寄存器
const UART_SCR: usize = 0x07; // 暂存寄存器

// UART结构体
pub struct Uart {
    base: usize,
}

impl Uart {
    pub const fn new() -> Self {
        Self { base: UART_BASE }
    }

    // 初始化UART
    pub fn init(&self) {
        // 设置波特率（115200）
        // 8位数据位，1位停止位，无奇偶校验
        unsafe {
            core::ptr::write_volatile((self.base + UART_LCR) as *mut u8, 0x03);
            core::ptr::write_volatile((self.base + UART_IER) as *mut u8, 0x00);
            core::ptr::write_volatile((self.base + UART_FCR) as *mut u8, 0x00);
            core::ptr::write_volatile((self.base + UART_MCR) as *mut u8, 0x00);
        }
    }

    // 发送字符
    pub fn put_char(&self, c: u8) {
        // 等待发送缓冲区空闲
        while unsafe { core::ptr::read_volatile((self.base + UART_LSR) as *const u8) } & 0x20 == 0 {}
        
        // 发送字符
        unsafe {
            core::ptr::write_volatile((self.base + UART_THR) as *mut u8, c);
        }
    }

    // 发送字符串
    pub fn put_str(&self, s: &str) {
        for byte in s.bytes() {
            self.put_char(byte);
        }
    }
}

// 全局UART实例
pub static UART: Uart = Uart::new();

// 便捷函数
pub fn print(s: &str) {
    UART.put_str(s);
}

pub fn println(s: &str) {
    UART.put_str(s);
    UART.put_char(b'\n');
}

pub fn put_char(c: u8) {
    UART.put_char(c);
}

pub fn put_str(s: &str) {
    UART.put_str(s);
}
```

更完整的 UART 原理与驱动实现请阅读第 03 章：[UART原理](./03_uart_principle.md)、[串口通信](./03_serial_communication.md)、[驱动实现](./03_uart_driver.md)。

## 内核初始化 (src/kernel/mod.rs)

```rust
use crate::drivers::uart;

pub fn init() {
    // 初始化UART
    uart::UART.init();

    // 输出欢迎信息
    uart::println("Hello, Rust OS!");
    uart::println("Welcome to OS in 1000 Lines!");

    // 后续会逐步初始化更多子系统：
    // - 异常/中断
    // - 内存分配
    // - 进程/调度
    // - 分页/用户态/系统调用/磁盘/文件系统/应用等
}
```

## 模块声明

### src/arch/mod.rs
```rust
pub mod riscv;
```

### src/common/mod.rs
```rust
pub mod string;
pub mod types;
```

### src/drivers/mod.rs
```rust
pub mod disk;
pub mod uart;
```

### src/kernel/mod.rs
```rust
pub mod conclusion;
pub mod exception;
pub mod memory;
pub mod paging;
pub mod process;
pub mod syscall;
pub mod usermode;
```

## 构建和运行

推荐使用 Makefile 或脚本：

```bash
make build
make run
```

或：

```bash
./build.sh
./run.sh
```

## 预期输出

运行成功后，你应该看到：

```
Hello, Rust OS!
Welcome to OS in 1000 Lines!
Exception handling initialized
...
```

## 代码解析

### 1. no_std环境
```rust
#![no_std]
#![no_main]
```
- `#![no_std]`: 禁用标准库
- `#![no_main]`: 禁用标准main函数

### 2. 内核入口点
```rust
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // 内核初始化
}
```
- `#[no_mangle]`: 防止函数名被修改
- `extern "C"`: 使用C调用约定
- `-> !`: 永不返回的函数

### 3. Panic处理
```rust
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // 处理panic
}
```

### 4. UART通信
- 通过内存映射I/O访问UART寄存器
- 轮询方式等待发送缓冲区空闲
- 支持字符和字符串输出

## 常见问题

### 问题1：链接错误
检查 `linker.ld` 文件是否正确，确保 `_start` 符号存在。

### 问题2：QEMU启动失败
检查QEMU版本，确保支持RISC-V64架构。

### 问题3：无输出
检查UART寄存器地址是否正确，确保QEMU配置正确。

## 下一步

第一个内核运行成功后，我们可以开始学习RISC-V架构的详细信息。请继续阅读 [RISC-V架构基础](./02_riscv_basics.md) 章节。

## 总结

本章我们学习了：

1. **no_std环境设置** - 如何配置Rust进行系统编程
2. **内核入口点** - 创建 `_start` 函数作为内核入口
3. **UART驱动** - 实现基本的串口通信
4. **Panic处理** - 处理内核异常情况
5. **项目结构** - 组织内核代码的模块化结构

这是构建操作系统内核的第一步，为后续更复杂的功能奠定了基础。
