# Panic处理

## 概述

在 `no_std` 内核里没有标准库提供的 panic 处理逻辑；当代码触发 `panic!` 时，必须由内核自己提供 `#[panic_handler]`，否则无法链接。

一个“可用的” panic handler 通常要做到：

1. **尽可能把信息打印出来**（文件、行号、简要消息）
2. **阻止系统继续以未知状态运行**（进入死循环或停机）

本仓库在 `src/main.rs` 提供了一个最小 `panic_handler`，会通过 UART 输出 panic 位置并停机。

## 为什么 panic 在内核里很重要

和用户态程序不同，内核 panic 往往意味着：

- 出现了无法恢复的一致性错误
- 数据结构可能已损坏（继续运行可能造成二次破坏）
- 需要立刻停止并留下足够的调试线索

所以“panic 打印”通常是早期内核最核心的调试设施之一。

## 本项目的 panic_handler 做了什么

`src/main.rs` 的逻辑大致是：

- 打印 `*** KERNEL PANIC ***`
- 如果 `PanicInfo` 带有 `location()`：
  - 打印 `file:line`
- 打印一条简化的 message
- 进入 `loop {}` 停机

在 `no_std` 环境下，很多格式化与字符串拼接都需要你自己实现/裁剪，因此本仓库的输出是“最小可用”的版本。

## 实战建议：panic 输出要包含哪些信息

当你把内核做复杂以后，建议 panic 输出至少包含：

- 当前特权级/CPU 核心号（多核时）
- `mepc/mcause/mtval`（如果 panic 发生在异常上下文）
- 当前进程/线程 ID（如果有调度器）
- 一段回溯（backtrace，能做则更好）

这些信息越早打通，后面调 bug 会省很多时间。

## 和异常处理的关系

- panic：由 Rust 语义触发（`panic!` / `unwrap()` 等）
- 异常：由硬件触发（非法指令、访存错误、ecall、中断等）

在真实内核里，你往往会让“不可恢复的异常”最终也走到 panic（或同等级的 fatal handler），从而统一输出与停机流程。

## 小结

本章你应该能回答：

- 为什么 `no_std` 内核必须自己提供 `#[panic_handler]`
- panic handler 的最小职责是什么
- 如何把 panic 输出当作早期内核的“救命绳”
