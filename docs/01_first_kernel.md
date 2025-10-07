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
│   └── types.rs       # 类型定义
├── drivers/           # 设备驱动
│   ├── mod.rs
│   └── uart.rs        # UART驱动
└── kernel/            # 内核核心
    ├── mod.rs
    └── init.rs        # 内核初始化
```

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

```rust
// RISC-V架构相关函数和常量

// 控制状态寄存器地址
const MSTATUS: usize = 0x300;
const MIE: usize = 0x304;
const MTVEC: usize = 0x305;
const MEPC: usize = 0x341;
const MCAUSE: usize = 0x342;
const MTVAL: usize = 0x343;

// 读取控制状态寄存器
#[inline(always)]
pub fn read_csr(csr: usize) -> usize {
    let value: usize;
    unsafe {
        asm!("csrr {}, {}", out(reg) value, const csr);
    }
    value
}

// 写入控制状态寄存器
#[inline(always)]
pub fn write_csr(csr: usize, value: usize) {
    unsafe {
        asm!("csrw {}, {}", const csr, in(reg) value);
    }
}

// 读取mstatus寄存器
pub fn read_mstatus() -> usize {
    read_csr(MSTATUS)
}

// 写入mstatus寄存器
pub fn write_mstatus(value: usize) {
    write_csr(MSTATUS, value);
}

// 读取mepc寄存器
pub fn read_mepc() -> usize {
    read_csr(MEPC)
}

// 写入mepc寄存器
pub fn write_mepc(value: usize) {
    write_csr(MEPC, value);
}

// 读取mcause寄存器
pub fn read_mcause() -> usize {
    read_csr(MCAUSE)
}

// 读取mtval寄存器
pub fn read_mtval() -> usize {
    read_csr(MTVAL)
}

// 读取mie寄存器
pub fn read_mie() -> usize {
    read_csr(MIE)
}

// 写入mie寄存器
pub fn write_mie(value: usize) {
    write_csr(MIE, value);
}

// 读取mtvec寄存器
pub fn read_mtvec() -> usize {
    read_csr(MTVEC)
}

// 写入mtvec寄存器
pub fn write_mtvec(value: usize) {
    write_csr(MTVEC, value);
}

// 读取mpp位
pub fn read_mpp() -> usize {
    (read_mstatus() >> 11) & 0x3
}

// 写入mpp位
pub fn write_mpp(value: usize) {
    let mstatus = read_mstatus();
    let new_mstatus = (mstatus & !(0x3 << 11)) | ((value & 0x3) << 11);
    write_mstatus(new_mstatus);
}

// 写入mie全局中断使能
pub fn write_mie_global(enable: bool) {
    let mstatus = read_mstatus();
    let new_mstatus = if enable {
        mstatus | (1 << 3)  // 设置MIE位
    } else {
        mstatus & !(1 << 3) // 清除MIE位
    };
    write_mstatus(new_mstatus);
}

// 异常和中断代码
pub const EXCEPTION_INSTRUCTION_ADDRESS_MISALIGNED: usize = 0;
pub const EXCEPTION_INSTRUCTION_ACCESS_FAULT: usize = 1;
pub const EXCEPTION_ILLEGAL_INSTRUCTION: usize = 2;
pub const EXCEPTION_BREAKPOINT: usize = 3;
pub const EXCEPTION_LOAD_ADDRESS_MISALIGNED: usize = 4;
pub const EXCEPTION_LOAD_ACCESS_FAULT: usize = 5;
pub const EXCEPTION_STORE_AMO_ADDRESS_MISALIGNED: usize = 6;
pub const EXCEPTION_STORE_AMO_ACCESS_FAULT: usize = 7;
pub const EXCEPTION_ECALL_FROM_U_MODE: usize = 8;
pub const EXCEPTION_ECALL_FROM_S_MODE: usize = 9;
pub const EXCEPTION_ECALL_FROM_M_MODE: usize = 11;
pub const EXCEPTION_INSTRUCTION_PAGE_FAULT: usize = 12;
pub const EXCEPTION_LOAD_PAGE_FAULT: usize = 13;
pub const EXCEPTION_STORE_AMO_PAGE_FAULT: usize = 15;

pub const INTERRUPT_SOFTWARE: usize = 3;
pub const INTERRUPT_TIMER: usize = 7;
pub const INTERRUPT_EXTERNAL: usize = 11;
```

## 通用类型 (src/common/types.rs)

```rust
// 通用类型定义和ToString trait

// 自定义ToString trait，用于no_std环境
pub trait ToString {
    fn to_string(&self) -> &'static str;
}

impl ToString for usize {
    fn to_string(&self) -> &'static str {
        // 简化实现，返回固定字符串
        "usize"
    }
}

impl ToString for u32 {
    fn to_string(&self) -> &'static str {
        // 简化实现，返回固定字符串
        "u32"
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
    pub const fn new(base: usize) -> Self {
        Self { base }
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
pub static UART: Uart = Uart::new(UART_BASE);

// 便捷函数
pub fn print(s: &str) {
    UART.put_str(s);
}

pub fn println(s: &str) {
    UART.put_str(s);
    UART.put_str("\n");
}

pub fn put_char(c: u8) {
    UART.put_char(c);
}

pub fn put_str(s: &str) {
    UART.put_str(s);
}
```

## 内核初始化 (src/kernel/init.rs)

```rust
// 内核初始化

use crate::drivers::uart;

pub fn init() {
    // 初始化UART
    uart::UART.init();
    
    // 输出欢迎信息
    uart::println("Welcome to OS in 1000 Lines!");
    uart::println("Hello, Rust OS!");
    
    // 内核主循环
    loop {
        uart::println("Welcome to OS in 1000 Lines!");
        uart::println("Hello, Rust OS!");
    }
}
```

## 模块声明

### src/arch/mod.rs
```rust
pub mod riscv;
```

### src/common/mod.rs
```rust
pub mod types;
```

### src/drivers/mod.rs
```rust
pub mod uart;
```

### src/kernel/mod.rs
```rust
pub mod init;

pub fn init() {
    init::init();
}
```

## 构建和运行

### 1. 构建内核
```bash
cargo build --release
```

### 2. 生成二进制文件
```bash
rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/kernel -O binary target/riscv64gc-unknown-none-elf/release/kernel.stripped
```

### 3. 运行内核
```bash
qemu-system-riscv64 \
    -machine virt \
    -cpu rv64 \
    -m 128M \
    -smp 1 \
    -nographic \
    -kernel target/riscv64gc-unknown-none-elf/release/kernel.stripped
```

## 预期输出

运行成功后，你应该看到：

```
Welcome to OS in 1000 Lines!
Hello, Rust OS!
Welcome to OS in 1000 Lines!
Hello, Rust OS!
Welcome to OS in 1000 Lines!
Hello, Rust OS!
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
