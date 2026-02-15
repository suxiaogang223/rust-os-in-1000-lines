# 异常处理

## 概述

当 CPU 遇到“无法继续按正常流程执行”的事件时（非法指令、访问错误、系统调用、中断等），它会触发 **trap**，并跳转到内核设置的处理入口。

在 RISC-V 机器模式下，trap 处理的核心问题是：

1. trap 入口在哪里？（`mtvec`）
2. trap 是什么原因？（`mcause`）
3. 发生时的 PC 在哪？（`mepc`）
4. 有无额外信息？（`mtval`）
5. 处理完后如何恢复/终止？（更新 `mepc` 或切换上下文）

本仓库在 `src/kernel/exception.rs` 实现了一个最小 trap 处理框架，用于打印原因并演示系统调用分支。

## trap 入口：mtvec

初始化异常处理时，需要把 `mtvec` 指向一个 trap handler：

- `mtvec` 存放 handler 的地址（base）
- 发生 trap 时 CPU 自动跳到 `mtvec` 指定的入口执行

本仓库在 `src/kernel/exception.rs` 的 `init_exception_handling()` 做了：

- `write_mtvec(handle_exception as usize)`
- 打开部分中断源（写 `mie`）
- 打开全局中断（设置 `mstatus.MIE`）

## 读取 trap 信息：mcause / mepc / mtval

进入 handler 后，第一件事通常是读取现场信息并记录：

- `mcause`：区分“中断/异常”并拿到类型编码
- `mepc`：异常发生的 PC（用于定位与恢复）
- `mtval`：异常附加值（非法指令/访存地址等）

本仓库的 `handle_exception()` 做了最小读取与分发：

- `mcause` 最高位为 1：中断（interrupt）
- 否则：异常（exception）

## 异常处理的最小分发结构

一个常见的最小结构是：

1. 先打印/记录 cause 与现场信息（便于调试）
2. 对可恢复的异常做恢复（如页故障可映射页面；系统调用可前进 `mepc`）
3. 对不可恢复的异常终止当前任务/直接 panic

本仓库目前以“演示”为主：

- 对中断：打印 `"Timer interrupt"` / `"External interrupt"`
- 对异常：按异常码打印原因，并在遇到 `ecall from user` 时走系统调用分支

## 系统调用（ecall）与异常处理的衔接

系统调用通常通过 `ecall` 指令触发 trap：

- 用户态执行 `ecall`
- `mcause` 变为 `EXCEPTION_ECALL_FROM_USER`
- 内核 handler 读取 `a7`（syscall number）与 `a0~a5`（参数）

本仓库在 `src/kernel/exception.rs` 中演示了读取 `a7` 并打印系统调用号。

在更完整的实现里，你通常会：

- 在 trap handler 里构造 `SyscallArgs`（见 `src/kernel/syscall.rs`）
- 交给统一的系统调用分发器处理
- 把返回值写回 `a0`
- 把 `mepc` 前移到 `ecall` 的下一条指令（否则会反复陷入）

## 练习与改进方向

如果你希望把异常处理做得更接近真实内核，可尝试：

- 在 trap handler 中保存/恢复通用寄存器现场（见第 07 章上下文切换）
- 对 `ecall` 处理：更新 `mepc += 4` 并返回
- 增加对常见异常（非法指令、访存 fault）的策略：打印 + 终止当前进程

## 小结

本章你应该理解：

- trap 入口由 `mtvec` 决定
- `mcause/mepc/mtval` 是定位问题的三件套
- “异常处理”与“系统调用/中断”是同一条机制上的不同分支
