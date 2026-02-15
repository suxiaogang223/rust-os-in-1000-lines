# 系统调用

## 概述

系统调用（Syscall）是用户态程序请求内核服务的标准入口。它解决两个核心问题：

1. **权限隔离**：用户态不能直接访问硬件/内核数据结构，只能通过受控接口请求服务
2. **ABI 约定**：用固定的寄存器/编号约定描述“我想做什么”和“参数是什么”

在 RISC-V 上，最常见的系统调用触发方式是执行 `ecall`，CPU 会进入 trap，然后由内核根据 `mcause` 和寄存器内容完成分发。

本仓库相关代码主要在：

- `src/kernel/exception.rs`：trap 入口里演示了 `ecall from user` 分支
- `src/kernel/syscall.rs`：系统调用参数结构与分发器（更完整的处理框架）

## RISC-V 系统调用寄存器约定（常见）

在 RV64 的常见 ABI 里：

- `a7`：系统调用号
- `a0~a5`：参数（最多 6 个）
- `a0`：返回值（也常用于第一个参数）

因此“读取系统调用号”的最小代码就是读 `a7`：

```rust
let syscall_num: usize;
unsafe { core::arch::asm!("mv {}, a7", out(reg) syscall_num); }
```

本仓库在 `src/kernel/syscall.rs` 进一步把它抽象成了 `SyscallArgs::from_registers()`，把 `a0~a7` 都读出来。

## 系统调用分发（dispatch）

一个系统调用子系统通常包含：

1. **编号定义**：如 `SYS_WRITE/SYS_EXIT/...`
2. **参数提取**：从寄存器或用户栈取参数
3. **权限检查与拷贝**：用户指针需要验证、必要时拷贝到内核缓冲区
4. **功能实现**：例如写串口、创建进程、打开文件
5. **返回值写回**：把结果写回 `a0` 并返回用户态

本仓库目前实现了演示性质的 syscall handler：

- `write`：把用户缓冲区的字节写到 UART（仅支持 `fd=1/2`）
- `exit/getpid/fork/execve/waitpid`：大多是简化/占位实现，用于展示结构

## 关键注意点：mepc 前移

`ecall` 会触发 trap。如果返回后 `mepc` 仍指向同一条 `ecall`，CPU 会再次执行它，导致“无限陷入”。

因此真实实现通常会在处理 `ecall` 后做：

- `mepc += 4`（RV64I 下指令长度通常 4 字节）

本仓库目前以打印演示为主，你可以把这一步作为练习补上。

## 小结

本章你应该掌握：

- 系统调用是用户态进入内核的受控入口
- RISC-V 里 `a7` 是 syscall number，`a0~a5` 是参数，`a0` 也是返回值
- 处理 `ecall` 后通常需要前移 `mepc`，否则会反复陷入
