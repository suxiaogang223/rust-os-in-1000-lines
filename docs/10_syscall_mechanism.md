# 系统调用机制

## 概述

系统调用机制描述的是“用户态如何进入内核、内核如何再返回用户态”的完整链路。

以 RISC-V 为例，最典型的链路是：

1. 用户态把 syscall number 放入 `a7`，把参数放入 `a0~a5`
2. 执行 `ecall`
3. CPU 触发 trap，跳到 `mtvec` 指向的 handler
4. handler 识别 `mcause = ecall from user`
5. 内核读取寄存器参数，分发到对应 syscall 实现
6. 把返回值写回 `a0`，更新 `mepc`，执行 trap return（`mret`）

本仓库目前以“骨架 + 打印演示”为主：

- trap 入口与分发：`src/kernel/exception.rs`
- syscall 参数抽取与分发器：`src/kernel/syscall.rs`

## ecall 会发生什么

`ecall` 是一条特权相关指令：

- 在用户态执行 `ecall` 会触发异常 `EXCEPTION_ECALL_FROM_USER`
- `mepc` 记录 `ecall` 的地址
- `mcause` 记录异常原因
- CPU 跳转到 `mtvec` 执行 handler

因此系统调用在硬件视角上是一种“可控的异常”。

## handler 如何区分系统调用

handler 读取 `mcause` 后：

- 若是异常并且 code == `EXCEPTION_ECALL_FROM_USER`，则走 syscall 分支

本仓库在 `src/kernel/exception.rs` 的 `handle_exception_code()` 中展示了这种匹配方式。

## 参数读取：a0~a7

进入内核后，要从寄存器中取出：

- `a7`：syscall number
- `a0~a5`：参数

本仓库提供了两种层次的做法：

- 在异常处理里直接读 `a7` 并打印（演示）
- 在 `src/kernel/syscall.rs` 中用 `SyscallArgs::from_registers()` 一次性读取 `a0~a7`

## 返回用户态：返回值与 mepc

真实系统调用必须处理两个关键点：

1. **返回值**：写回 `a0`
2. **前移 mepc**：跳过 `ecall`，否则会重复陷入

通常会做：

- `mepc += 4`

然后执行 `mret` 返回到用户态继续执行。

本仓库当前没有完整的 `mret` 返回路径实现，因此更像是在讲清楚“机制应该长什么样”。

## 小结

本章你应该理解：

- 系统调用的硬件机制本质是：`ecall -> trap -> handler -> return`
- 参数通过 `a0~a5`，编号通过 `a7` 传递
- 返回用户态时需要写回返回值并前移 `mepc`
