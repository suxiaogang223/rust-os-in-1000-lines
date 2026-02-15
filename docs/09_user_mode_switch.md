# 用户态切换

## 概述

“用户态切换”指的是：让 CPU 从内核态进入用户态运行一段用户代码，并在发生系统调用/异常/中断时再回到内核态处理。

这背后依赖三件事：

1. **特权级控制**：设置返回目标（`mstatus.MPP`）并执行返回指令（`mret`）
2. **上下文准备**：设置用户程序入口 PC 与用户栈 SP
3. **trap 返回路径**：用户态 `ecall`/异常进入内核后，内核能正确返回

本仓库在 `src/kernel/usermode.rs` 提供了一个教学版的用户态管理器，用于展示“注册用户程序 + 创建上下文 + 切换意图”。

## 用户态上下文：需要保存什么

本仓库定义了：

- `UserContext { regs: [usize; 32], pc, sp }`

它表达的核心思想是：用户态的寄存器状态可以被当作一个“可保存/恢复的数据结构”。

真实实现里，上下文的保存/恢复往往发生在 trap handler 中，并且需要严格对应 ABI/寄存器布局。

## 用户程序描述与注册

`UserProgram` 包含：

- `name`
- `entry_point`
- `stack_size`

`UserModeManager::init()` 里注册了两个示例程序（演示用途）：

- `hello`
- `test`

并提供了 `list_programs()` 在串口上列出可用程序。

## 切换流程（概念版）

本仓库的 `switch_to_user(program_index)` 做了几件关键动作：

1. 选择一个 `UserProgram`
2. 创建 `UserContext`
3. 设置 `mstatus.MPP = U`（通过 `write_mpp(PRIVILEGE_USER)`）
4. 设置 `mepc = entry_point`
5. 打开部分用户态中断位（演示）

注意：在真实硬件上，完成这些准备后还需要执行 `mret` 才会真正跳转到用户态开始执行。

本项目目前主要展示结构与流程，`mret` 与真实返回路径仍属于后续扩展内容（并会与异常处理和上下文切换紧密结合）。

## 从用户态回到内核态

常见触发点：

- 用户态执行 `ecall`（系统调用）
- 用户态触发非法指令/访存错误等异常
- 用户态发生中断（定时器/外设）

本仓库在 `UserModeManager::handle_user_exception()` 里演示了对 `ecall/illegal instruction/load/store fault` 的处理策略，并在需要时调用 `return_to_kernel()` 清理上下文并恢复到机器态配置（演示）。

## 小结

本章你应该理解：

- 用户态切换的关键是准备好 PC/SP/权限位，并通过 `mret` 返回到目标级别
- 用户态进入内核的主要通道是 `ecall` 与各种异常/中断
- 本仓库目前提供了用户态管理的骨架，真实执行与返回路径仍可作为后续练习补齐
