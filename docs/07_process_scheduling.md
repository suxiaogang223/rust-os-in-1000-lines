# 进程调度

## 概述

调度（Scheduling）决定“下一次谁来用 CPU”。在单核系统里，它的核心任务就是：

- 在一组就绪进程中选择一个来运行
- 维护进程状态（Ready/Running/Blocked/Terminated）
- 在合适的时机触发切换（时间片/主动让出/阻塞）

本仓库在 `src/kernel/process.rs` 实现了一个最简单的 **轮转调度（Round-robin）** 框架。

## 调度器的数据结构

`Scheduler` 维护：

- `processes: [Option<Process>; 16]`：最多 16 个进程槽位
- `current_pid: Option<usize>`：当前正在运行的进程 PID（简化）
- `next_pid: usize`：分配 PID 用

这种固定数组 + Option 的方式非常适合早期内核：

- 不依赖复杂的动态分配
- 查找/遍历简单
- 便于在串口上输出进程列表

## 创建进程

`create_process(stack_size)` 会：

1. 找到一个空槽位
2. 创建 `Process`（分配栈）
3. 分配一个新的 PID
4. 进程初始状态设为 Ready

在 `kernel::init()` 中，代码会创建两个测试进程并打印进程列表（见 `src/kernel/mod.rs`）。

## 轮转调度（Round-robin）思路

轮转调度的核心思想：

- 把就绪队列当成一个环
- 每次从“上一次运行的后一个”开始找 Ready 进程

本仓库的 `schedule()` 做的事情很直观：

- 计算一个起始位置
- 循环扫描 `processes`
- 找到第一个 `Ready` 的进程：
  - 把旧的 Running 进程设为 Ready（简化）
  - 把新的进程设为 Running，并更新 `current_pid`

## 何时触发调度

调度需要一个“触发点”，常见有：

- 定时器中断：时间片轮转（最常见）
- 系统调用：例如 `yield()` 主动让出
- 阻塞事件：等待 IO、等待锁

本仓库目前主要展示调度器结构，尚未实现“在定时器中断里触发真正切换”。这部分会与第 07 章“上下文切换”与第 05 章“中断处理”结合起来。

## 小结

本章你应该理解：

- 调度决定“谁用 CPU”，上下文切换负责“怎么切过去”
- 固定数组 + Option 是早期内核管理进程的常见简化
- 轮转调度实现简单，适合作为第一版调度器
