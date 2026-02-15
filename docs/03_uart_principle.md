# UART原理

## 概述

在最小内核阶段，我们还没有图形界面、没有文件系统、甚至没有“标准输出”。想要观察内核是否正常运行，最可靠的手段之一就是 **串口（UART）输出**。

本节先讲 UART 的基本工作原理与 16550 兼容寄存器模型，下一节再讲“串口通信如何接到 QEMU/终端”，最后在 `docs/03_uart_driver.md` 完成驱动实现。

## UART 是什么

UART（Universal Asynchronous Receiver/Transmitter）是一种 **异步串行通信**外设：

- 异步：通信双方不共享时钟
- 串行：一次传输 1 bit
- 全双工：可同时发送与接收

一帧典型格式：

```
| start(1) | data(5~8) | parity(可选) | stop(1~2) |
```

常见参数组合：`115200-8-N-1`（波特率 115200、8 数据位、无校验、1 停止位）。

## 为什么 OS 教程喜欢用 UART

- **实现门槛低**：最小只要“写一个寄存器”就能输出字符
- **调试友好**：系统一崩溃还能打印信息（panic/异常）
- **QEMU 支持好**：virt 机器自带串口设备，终端即可看到输出

## 16550 寄存器模型（常用子集）

QEMU `virt` 机器提供的 UART 通常是 16550 兼容（或行为类似）。本仓库使用的寄存器布局（见 `src/drivers/uart.rs`）：

```rust
const UART_BASE: usize = 0x1000_0000;

const UART_RBR: usize = 0x00; // 接收缓冲寄存器 (Read)
const UART_THR: usize = 0x00; // 发送保持寄存器 (Write)
const UART_IER: usize = 0x01; // 中断使能
const UART_FCR: usize = 0x02; // FIFO 控制
const UART_LCR: usize = 0x03; // 线路控制
const UART_LSR: usize = 0x05; // 线路状态
```

### `LSR`（Line Status Register）关键位

本仓库使用了两个常见标志位：

- `LSR_THRE (0x20)`：发送保持寄存器空（可以写 `THR`）
- `LSR_DR (0x01)`：数据就绪（可以读 `RBR`）

驱动里会做“轮询等待 THR 空”：

```rust
while !self.is_transmit_empty() {}
self.write_reg(UART_THR, c);
```

这是一种最简单可靠的输出方式（但会忙等，占用 CPU）。

## UART 初始化的最小目标

对“只想打印字符”的最小内核来说，UART 初始化的目标通常是：

- 设置数据格式（8N1）
- 关闭/配置 FIFO（简化实现可以直接关闭）
- 配置波特率（16550 通常通过 DLAB + divisor 设置；本仓库做了简化）

你可以在 `src/drivers/uart.rs` 的 `Uart::init()` 看到一个最小可用的初始化流程。

## 小结

本节你应该理解：

- UART 帧格式与常见通信参数
- 16550 兼容 UART 的常用寄存器与状态位
- 最小输出驱动通常通过“轮询 + 写 THR”实现

下一节我们会把 UART 输出接到 QEMU/终端上，让你能在命令行里看到内核的打印信息。
