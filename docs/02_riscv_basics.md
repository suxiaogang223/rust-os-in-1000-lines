# 第02章：RISC-V架构基础

## 概述

RISC-V是一个开源的指令集架构(ISA)，具有简洁、模块化的设计。本章将介绍RISC-V架构的基础知识，包括寄存器、指令集和特权级别。

## RISC-V架构特点

### 1. 开源设计
- **完全开源** - 无专利限制
- **标准化** - 由RISC-V基金会维护
- **可扩展** - 支持自定义指令

### 2. 模块化设计
- **基础指令集** - RV32I/RV64I
- **扩展指令集** - M(乘法)、A(原子)、F(浮点)等
- **特权架构** - Machine、Supervisor、User模式

### 3. 简洁高效
- **指令数量少** - 基础指令集仅40条
- **编码规整** - 指令格式统一
- **实现简单** - 易于硬件实现

## 寄存器架构

### 通用寄存器

RISC-V定义了32个通用寄存器：

```
x0  - 零寄存器 (始终为0)
x1  - 返回地址寄存器 (ra)
x2  - 栈指针寄存器 (sp)
x3  - 全局指针寄存器 (gp)
x4  - 线程指针寄存器 (tp)
x5  - 临时寄存器 (t0)
x6  - 临时寄存器 (t1)
x7  - 临时寄存器 (t2)
x8  - 保存寄存器 (s0/fp)
x9  - 保存寄存器 (s1)
x10 - 参数/返回值寄存器 (a0)
x11 - 参数/返回值寄存器 (a1)
x12 - 参数寄存器 (a2)
x13 - 参数寄存器 (a3)
x14 - 参数寄存器 (a4)
x15 - 参数寄存器 (a5)
x16 - 参数寄存器 (a6)
x17 - 参数寄存器 (a7)
x18 - 保存寄存器 (s2)
x19 - 保存寄存器 (s3)
x20 - 保存寄存器 (s4)
x21 - 保存寄存器 (s5)
x22 - 保存寄存器 (s6)
x23 - 保存寄存器 (s7)
x24 - 保存寄存器 (s8)
x25 - 保存寄存器 (s9)
x26 - 保存寄存器 (s10)
x27 - 保存寄存器 (s11)
x28 - 临时寄存器 (t3)
x29 - 临时寄存器 (t4)
x30 - 临时寄存器 (t5)
x31 - 临时寄存器 (t6)
```

### 寄存器使用约定

```rust
// 在Rust中访问寄存器
use core::arch::asm;

// 读取寄存器
let value: usize;
unsafe {
    asm!("mv {}, x1", out(reg) value);
}

// 写入寄存器
let value = 42;
unsafe {
    asm!("mv x1, {}", in(reg) value);
}
```

## 控制状态寄存器(CSR)

### 重要CSR寄存器

| 寄存器 | 地址 | 名称 | 描述 |
|--------|------|------|------|
| mstatus | 0x300 | 机器状态寄存器 | 全局状态信息 |
| mie | 0x304 | 机器中断使能 | 中断使能控制 |
| mtvec | 0x305 | 机器陷阱向量 | 异常处理入口 |
| mepc | 0x341 | 机器异常PC | 异常返回地址 |
| mcause | 0x342 | 机器异常原因 | 异常/中断原因 |
| mtval | 0x343 | 机器陷阱值 | 异常相关信息 |

### CSR操作指令

```rust
// CSR操作函数
#[inline(always)]
pub fn read_csr(csr: usize) -> usize {
    let value: usize;
    unsafe {
        asm!("csrr {}, {}", out(reg) value, const csr);
    }
    value
}

#[inline(always)]
pub fn write_csr(csr: usize, value: usize) {
    unsafe {
        asm!("csrw {}, {}", const csr, in(reg) value);
    }
}

#[inline(always)]
pub fn set_csr(csr: usize, value: usize) {
    unsafe {
        asm!("csrs {}, {}", const csr, in(reg) value);
    }
}

#[inline(always)]
pub fn clear_csr(csr: usize, value: usize) {
    unsafe {
        asm!("csrc {}, {}", const csr, in(reg) value);
    }
}
```

## 特权级别

### 三个特权级别

1. **Machine模式 (M-mode)**
   - 最高特权级别
   - 可以访问所有资源
   - 操作系统内核运行在此模式

2. **Supervisor模式 (S-mode)**
   - 中等特权级别
   - 用于虚拟内存管理
   - 用户程序监控

3. **User模式 (U-mode)**
   - 最低特权级别
   - 用户程序运行
   - 受限的资源访问

### 特权级别切换

```rust
// 读取当前特权级别
pub fn read_mpp() -> usize {
    (read_mstatus() >> 11) & 0x3
}

// 设置特权级别
pub fn write_mpp(level: usize) {
    let mstatus = read_mstatus();
    let new_mstatus = (mstatus & !(0x3 << 11)) | ((level & 0x3) << 11);
    write_mstatus(new_mstatus);
}

// 特权级别常量
pub const PRIVILEGE_USER: usize = 0;
pub const PRIVILEGE_SUPERVISOR: usize = 1;
pub const PRIVILEGE_MACHINE: usize = 3;
```

## 异常和中断

### 异常类型

| 异常代码 | 名称 | 描述 |
|----------|------|------|
| 0 | 指令地址不对齐 | 指令地址不是4字节对齐 |
| 1 | 指令访问错误 | 指令访问权限错误 |
| 2 | 非法指令 | 不支持的指令 |
| 3 | 断点 | 调试断点 |
| 4 | 加载地址不对齐 | 加载地址不对齐 |
| 5 | 加载访问错误 | 加载访问权限错误 |
| 6 | 存储地址不对齐 | 存储地址不对齐 |
| 7 | 存储访问错误 | 存储访问权限错误 |
| 8 | 用户态系统调用 | ecall from U-mode |
| 9 | 监管态系统调用 | ecall from S-mode |
| 11 | 机器态系统调用 | ecall from M-mode |
| 12 | 指令页错误 | 指令页错误 |
| 13 | 加载页错误 | 加载页错误 |
| 15 | 存储页错误 | 存储页错误 |

### 中断类型

| 中断代码 | 名称 | 描述 |
|----------|------|------|
| 3 | 软件中断 | 软件触发的中断 |
| 7 | 定时器中断 | 定时器到期中断 |
| 11 | 外部中断 | 外部设备中断 |

### 异常处理实现

```rust
// 异常处理函数
pub fn handle_exception(mcause: usize, mepc: usize, mtval: usize) {
    let is_interrupt = (mcause >> 63) & 1;
    let exception_code = mcause & 0x7FFFFFFF;
    
    if is_interrupt != 0 {
        handle_interrupt(exception_code);
    } else {
        handle_exception_code(exception_code, mepc, mtval);
    }
}

// 中断处理
fn handle_interrupt(code: usize) {
    match code {
        INTERRUPT_SOFTWARE => {
            // 处理软件中断
        },
        INTERRUPT_TIMER => {
            // 处理定时器中断
        },
        INTERRUPT_EXTERNAL => {
            // 处理外部中断
        },
        _ => {
            // 未知中断
        }
    }
}

// 异常处理
fn handle_exception_code(code: usize, mepc: usize, mtval: usize) {
    match code {
        EXCEPTION_ILLEGAL_INSTRUCTION => {
            // 处理非法指令
        },
        EXCEPTION_ECALL_FROM_U_MODE => {
            // 处理用户态系统调用
        },
        _ => {
            // 其他异常
        }
    }
}
```

## 内存管理

### 地址空间

- **物理地址空间** - 硬件直接访问的地址
- **虚拟地址空间** - 程序使用的地址
- **地址转换** - 通过页表实现

### 页表结构

```rust
// 页表项结构
#[repr(C)]
pub struct PageTableEntry {
    pub bits: usize,
}

impl PageTableEntry {
    // 页表项标志位
    const VALID: usize = 1 << 0;
    const READ: usize = 1 << 1;
    const WRITE: usize = 1 << 2;
    const EXECUTE: usize = 1 << 3;
    const USER: usize = 1 << 4;
    const GLOBAL: usize = 1 << 5;
    const ACCESSED: usize = 1 << 6;
    const DIRTY: usize = 1 << 7;
    
    // 检查是否有效
    pub fn is_valid(&self) -> bool {
        (self.bits & Self::VALID) != 0
    }
    
    // 获取物理地址
    pub fn get_physical_address(&self) -> usize {
        self.bits & 0xFFFFFFFFFFF000
    }
}
```

## 指令集基础

### 指令格式

RISC-V有6种基本指令格式：

1. **R型** - 寄存器-寄存器操作
2. **I型** - 立即数操作
3. **S型** - 存储操作
4. **B型** - 分支操作
5. **U型** - 高位立即数
6. **J型** - 跳转操作

### 常用指令

```rust
// 算术指令
unsafe {
    asm!("add x1, x2, x3");     // x1 = x2 + x3
    asm!("sub x1, x2, x3");     // x1 = x2 - x3
    asm!("addi x1, x2, 42");    // x1 = x2 + 42
}

// 逻辑指令
unsafe {
    asm!("and x1, x2, x3");     // x1 = x2 & x3
    asm!("or x1, x2, x3");      // x1 = x2 | x3
    asm!("xor x1, x2, x3");     // x1 = x2 ^ x3
}

// 移位指令
unsafe {
    asm!("sll x1, x2, x3");     // x1 = x2 << x3
    asm!("srl x1, x2, x3");     // x1 = x2 >> x3
    asm!("slli x1, x2, 5");     // x1 = x2 << 5
}

// 比较指令
unsafe {
    asm!("slt x1, x2, x3");     // x1 = (x2 < x3) ? 1 : 0
    asm!("sltu x1, x2, x3");   // x1 = (x2 < x3) ? 1 : 0 (无符号)
}

// 分支指令
unsafe {
    asm!("beq x1, x2, label");  // if (x1 == x2) goto label
    asm!("bne x1, x2, label");  // if (x1 != x2) goto label
    asm!("blt x1, x2, label");  // if (x1 < x2) goto label
}

// 跳转指令
unsafe {
    asm!("jal x1, label");      // x1 = pc + 4; goto label
    asm!("jalr x1, x2, 0");     // x1 = pc + 4; goto x2
}
```

## 系统调用

### 系统调用机制

```rust
// 系统调用号定义
pub const SYSCALL_WRITE: usize = 64;
pub const SYSCALL_EXIT: usize = 93;
pub const SYSCALL_GETPID: usize = 172;
pub const SYSCALL_FORK: usize = 220;
pub const SYSCALL_EXECVE: usize = 221;
pub const SYSCALL_WAITPID: usize = 260;

// 系统调用处理
pub fn handle_syscall(args: &SyscallArgs) -> usize {
    match args.syscall_number {
        SYSCALL_WRITE => handle_write(args),
        SYSCALL_EXIT => handle_exit(args),
        SYSCALL_GETPID => handle_getpid(args),
        SYSCALL_FORK => handle_fork(args),
        SYSCALL_EXECVE => handle_execve(args),
        SYSCALL_WAITPID => handle_waitpid(args),
        _ => {
            // 未知系统调用
            0xFFFFFFFF
        }
    }
}
```

## 实践练习

### 练习1：寄存器操作
编写一个函数，交换两个寄存器的值：

```rust
pub fn swap_registers(a: &mut usize, b: &mut usize) {
    let temp = *a;
    *a = *b;
    *b = temp;
}
```

### 练习2：CSR操作
实现一个函数，安全地修改mstatus寄存器的特定位：

```rust
pub fn set_mstatus_bit(bit: usize, value: bool) {
    let mask = 1 << bit;
    let mstatus = read_mstatus();
    let new_mstatus = if value {
        mstatus | mask
    } else {
        mstatus & !mask
    };
    write_mstatus(new_mstatus);
}
```

### 练习3：异常处理
实现一个简单的异常处理框架：

```rust
pub fn setup_exception_handling() {
    // 设置异常处理入口
    write_mtvec(exception_handler as usize);
    
    // 启用全局中断
    write_mie_global(true);
}
```

## 总结

本章我们学习了：

1. **RISC-V架构特点** - 开源、模块化、简洁
2. **寄存器系统** - 32个通用寄存器的使用约定
3. **控制状态寄存器** - 重要的CSR寄存器及其操作
4. **特权级别** - Machine、Supervisor、User模式
5. **异常和中断** - 异常类型和处理机制
6. **内存管理** - 地址空间和页表结构
7. **指令集基础** - 6种指令格式和常用指令
8. **系统调用** - 用户态与内核态交互

这些知识为后续开发操作系统内核奠定了坚实的基础。

## 下一步

掌握了RISC-V架构基础后，我们可以开始实现UART驱动。请继续阅读 [UART驱动实现](./03_uart_driver.md) 章节。

## 参考资源

- [RISC-V规范](https://riscv.org/technical/specifications/)
- [RISC-V指令集手册](https://riscv.org/wp-content/uploads/2017/05/riscv-spec-v2.2.pdf)
- [RISC-V特权架构](https://riscv.org/wp-content/uploads/2019/12/riscv-privileged-20191213.pdf)
- [RISC-V汇编语言](https://github.com/riscv/riscv-asm-manual)