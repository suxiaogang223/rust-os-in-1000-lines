# 系统调用实现

## 概述

在机制打通之后，系统调用实现关注的是：

- 每个 syscall 的语义是什么
- 如何验证参数（尤其是用户指针）
- 如何与内核子系统协作（进程/文件系统/驱动等）
- 如何返回结果与错误码

本仓库在 `src/kernel/syscall.rs` 提供了一个 `SyscallHandler`，实现了若干常见 syscall 的“教学版”处理流程。

## SyscallHandler：分发器骨架

`handle_syscall(args)` 做了：

1. 计数（`syscall_count += 1`，便于观察调用次数）
2. 打印 syscall number（便于调试）
3. `match args.syscall_num` 分发到具体 handler

这是一种最常见的写法：简单、清晰，且易于扩展。

## write：最小可用的输出 syscall

`write(fd, buf, count)` 是最常见也最有用的 syscall 之一，因为它能把用户态输出映射到某个设备（串口、文件、管道等）。

本仓库的 `handle_write()` 做了简化：

- 只支持 `fd == 1`（stdout）和 `fd == 2`（stderr）
- 直接把用户缓冲区的字节写到 UART

注意：这里的实现没有做用户指针校验，是教学演示代码。真实内核必须验证：

- 指针是否落在用户地址空间
- 缓冲区范围是否可读

否则用户态可以用任意指针读写内核地址，破坏隔离。

## exit/getpid/fork/execve/waitpid：教学占位

本仓库还提供了其他 syscall 的骨架：

- `exit(code)`：终止当前进程（当前实现用占位 PID）
- `getpid()`：返回当前 PID（当前实现返回固定值）
- `fork()`：创建新进程（当前实现调用 `process::create_process()`）
- `execve()`：占位，不真正加载程序
- `waitpid()`：占位，写入 status 并返回 pid

这些实现的价值在于：把“系统调用层与内核子系统的连接点”先搭好。

当你准备把它们做得更真实时，需要补齐：

- 当前进程的概念（调度器持有 current）
- 进程地址空间与用户内存拷贝
- 程序加载器（把磁盘上的 ELF/二进制加载到内存）
- 进程同步与回收

## 小结

本章你应该理解：

- syscall 实现的关键在于：参数校验 + 子系统协作 + 返回值约定
- `write` 往往是第一个真正“有用”的 syscall
- 其他 syscall 可以先搭骨架，再逐步增强语义与安全性
