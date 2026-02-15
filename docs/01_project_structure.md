# 第01章：项目结构

## 概述

写操作系统内核时，工程结构比“把代码跑起来”更重要：它决定了模块边界、可维护性、以及后续扩展（内存、进程、文件系统等）能否自然落地。

本章会带你快速熟悉本仓库的目录布局、核心入口、以及各模块之间的依赖关系，帮助你在阅读后续章节时能“知道去哪里找代码”。

## 代码与文档的关系

- `src/`：内核与用户态示例的 Rust 代码（`no_std`）。
- `docs/`：教程正文（本书），使用 mdBook 生成静态页面。

建议的阅读顺序：

1. `docs/` 按章节学习概念与实现思路
2. 对照 `src/` 里的模块实现验证理解
3. 用 `make run` 或 `./run.sh` 在 QEMU 里跑起来观察串口输出

## 顶层目录说明

```
.
├── Cargo.toml              # Rust 包配置（no_std 内核）
├── Makefile                # 常用开发命令封装（build/run/debug/fmt/check）
├── linker.ld               # 链接脚本：定义内核镜像布局、入口符号等
├── build.sh / run.sh       # 构建/运行脚本（便于快速上手）
├── debug.sh                # 调试脚本（QEMU + GDB stub）
├── test.sh                 # 本地验证脚本（格式化/检查/构建等）
├── docs/                   # 本书源码（mdBook）
└── src/                    # 内核源码
```

## 内核源码结构（src/）

```
src/
├── main.rs                 # 裸机入口：_start + panic_handler
├── lib.rs                  # crate 根：声明模块并提供 init()
├── arch/                   # 架构相关（当前是 RISC-V）
│   ├── mod.rs
│   └── riscv.rs             # CSR/异常码/中断码/特权级别常量与读写封装
├── common/                 # 通用工具（no_std 下的“迷你 libc”/辅助类型）
│   ├── mod.rs
│   ├── string.rs
│   └── types.rs
├── drivers/                # 设备驱动（UART、Disk）
│   ├── mod.rs
│   ├── uart.rs
│   └── disk.rs
├── kernel/                 # 核心子系统（异常、内存、进程、分页、系统调用…）
│   ├── mod.rs               # kernel::init() 统一初始化入口
│   ├── exception.rs
│   ├── memory.rs
│   ├── process.rs
│   ├── paging.rs
│   ├── usermode.rs
│   ├── syscall.rs
│   └── conclusion.rs
├── fs/                     # 文件系统（当前是简化的 FAT）
│   ├── mod.rs
│   └── fat.rs
└── user/                   # 用户态示例（Shell 等）
    ├── mod.rs
    └── shell.rs
```

## 启动流程（从 _start 到内核子系统）

最小启动链路如下：

1. `src/main.rs`：`_start()` 是入口（裸机环境下没有标准 `main`）。
2. `src/lib.rs`：`init()` 作为统一入口，便于拆分 crate 模块。
3. `src/kernel/mod.rs`：`kernel::init()` 完成串口初始化、异常/内存/进程/分页等子系统初始化，并输出一些自检信息。

你可以把它理解成：

```
_start() -> rust_os_in_1000::init() -> kernel::init()
```

## “章节”与“模块”的映射建议

为了让阅读更顺滑，本书的章节与源码模块大致可以对应：

- RISC-V 基础/寄存器/CSR：`src/arch/riscv.rs`
- UART 与串口输出：`src/drivers/uart.rs`
- Panic/异常/中断：`src/main.rs`（panic）与 `src/kernel/exception.rs`
- 内存分配/堆：`src/kernel/memory.rs`
- 进程与调度：`src/kernel/process.rs`
- 分页与地址转换：`src/kernel/paging.rs`
- 用户模式：`src/kernel/usermode.rs`
- 系统调用：`src/kernel/syscall.rs`（以及异常入口里的 ecall 分支）
- 磁盘与块设备：`src/drivers/disk.rs`
- FAT 文件系统：`src/fs/fat.rs`
- Shell：`src/user/shell.rs`

## 小结

到这里你应该能回答两个问题：

1. “我想看某个功能的实现，该去哪找？”（按模块定位）
2. “内核启动时做了哪些初始化？”（按启动链路定位）

下一章我们会继续实现/完善最小内核并让它在 QEMU 上稳定输出信息。

