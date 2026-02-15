# 中断处理

## 概述

中断（Interrupt）是内核“被动响应外部事件”的机制。典型用途包括：

- 定时器：时间片调度、延时、时钟
- 外设：UART 收到数据、磁盘 IO 完成、网络包到达

本节先讲“RISC-V 中断使能与分发”的最小模型，并结合本仓库的实现解释目前有哪些简化。

## 中断链路：从硬件事件到 handler

一条典型链路可以概括为：

1. 外设产生中断信号
2. CPU 进入 trap（跳转到 `mtvec`）
3. handler 读取 `mcause` 判断是中断
4. 根据中断编号分发到具体处理函数
5. 清除中断源/确认中断，返回继续执行

在真实系统里，“外部中断”往往还要经过中断控制器（如 PLIC），并需要读取/写回 claim/complete 寄存器。

## 中断使能：全局开关 + 源开关

机器模式下，中断生效通常需要同时满足：

- `mstatus.MIE = 1`（全局中断使能）
- `mie` 中对应中断位为 1（例如 machine timer/external）

本仓库在 `src/kernel/exception.rs:init_exception_handling()`：

- 设置 `mtvec`
- 打开 machine timer + machine external
- 打开全局中断

## 目前的实现：打印 + 占位

`src/kernel/exception.rs` 对中断的处理是演示性质：

- Timer interrupt：打印 `"Timer interrupt"`
- External interrupt：打印 `"External interrupt"`

还没有实现：

- 定时器中断的清除/重设（CLINT）
- 外部中断的分发（PLIC）
- 与进程调度的联动（时间片到期触发切换）

这些内容可以作为后续扩展练习。

## 练习：用定时器中断驱动调度（思路）

当你已经有了进程调度器（第 07 章），可以尝试：

1. 在定时器中断里更新 tick 计数
2. 每 N 个 tick 触发一次 `schedule()`
3. 在上下文切换完成后返回到新进程执行

要做到这一点，你需要补齐：

- 保存/恢复寄存器上下文（第 07 章“上下文切换”）
- 正确的 trap 返回路径（`mret` 或类似机制）

## 小结

本章你应该理解：

- 中断是 trap 机制的一支，入口仍由 `mtvec` 决定
- 中断使能需要同时打开“全局 + 源级”
- 早期内核可以先用“打印”验证链路，后续再逐步实现 CLINT/PLIC/调度联动
