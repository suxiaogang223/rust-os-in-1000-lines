# 系统调用接口

## 概述

系统调用接口（Syscall Interface）定义了“用户态如何请求内核服务”的一整套约定，包括：

- 系统调用号的分配
- 参数如何传递（寄存器/栈）
- 返回值与错误码如何表示
- 哪些调用是允许的、哪些是非法的

本仓库在 `src/kernel/syscall.rs` 中定义了一组 syscall 号，并提供了：

- `SyscallArgs`：从寄存器读取参数的结构体
- `SyscallHandler`：按 syscall 号分发到具体实现
- 一些简化的封装函数：`sys_write/sys_exit/sys_getpid`

## syscall 号（示例）

本仓库的 syscall 号与 Linux RISC-V 常见编号保持一致（节选）：

- `SYS_WRITE = 64`
- `SYS_EXIT = 93`
- `SYS_GETPID = 172`
- `SYS_FORK = 220`
- `SYS_EXECVE = 221`
- `SYS_WAITPID = 260`

在学习阶段保持编号“看起来熟悉”有助于后续参考资料，但你完全可以定义自己的一套编号（只要用户态与内核一致）。

## 参数与返回值约定

本仓库的 `SyscallArgs` 假设：

- `a7`：syscall number
- `a0~a5`：参数

返回值约定：

- 把返回值写回 `a0`

目前 `SyscallHandler::handle_syscall()` 返回一个 `usize`，作为“返回值”的概念表达。

## 错误码策略（建议）

教学项目里可以先用简单策略：

- 成功返回非负值（或具体值）
- 失败返回 `0xFFFF_FFFF` 作为错误（本仓库当前就是这种简化）

但如果你希望更接近真实系统，建议采用：

- 返回 `isize`
- 负值表示错误（如 `-EINVAL/-EBADF`），正值/0 表示成功

并在用户态封装层把错误转换为 `Result<T, Errno>`。

## 从接口到机制

系统调用接口是“约定”，系统调用机制是“如何触发 trap 并进入内核”。

下一章（第 10 章）会把视角放到 `ecall`、trap 分发、以及 syscall handler 的实现路径上。

## 小结

本章你应该理解：

- syscall 接口包含编号、参数、返回值/错误码等约定
- `a7`/`a0~a5` 是常见的寄存器传参方式
- 设计一套清晰稳定的接口，是后续文件系统/进程等能力的基础
