# 串口通信

## 概述

“UART 驱动”解决的是：内核如何往 UART 的 MMIO 寄存器写字节。

“串口通信”解决的是：这些字节如何从 QEMU 里出来，被你的终端看到，以及你如何与 QEMU 交互（退出、切换 monitor、调试等）。

本节以本仓库的 `Makefile`/脚本为准，介绍最常用的串口输出配置方式。

## QEMU virt：串口输出到终端

本仓库默认使用：

- `-nographic`：不启用图形窗口，把串口等重定向到当前终端
- `-monitor stdio`：把 QEMU monitor 放到当前终端（便于退出、查看状态等）

对应 `Makefile` 的 `run` 目标（简化说明）：

```bash
qemu-system-riscv64 \
  -machine virt -cpu rv64 -m 128M -smp 1 \
  -nographic -monitor stdio \
  -kernel target/.../kernel.stripped
```

当内核通过 UART 输出字符时，你会在终端里直接看到。

## 退出 QEMU

如果终端被 QEMU 占用，常见退出方式：

- `Ctrl+A`，然后按 `X`

如果启用了 `-monitor stdio`，也可以在 monitor 中输入 `quit`（具体取决于 monitor/串口复用方式）。

## “串口”和“monitor”同时在 stdio 的注意点

当串口与 monitor 复用同一个 stdio 时，可能出现：

- 内核输出与 monitor 提示符混在一起
- 键盘输入到底是发给 monitor 还是串口，不够直观

这是学习阶段常见的折中方案：它最省事，但交互体验一般。

如果你希望更清晰的体验，可以把 monitor 关闭、让串口独占 stdio，例如：

```bash
qemu-system-riscv64 -nographic -monitor none -serial stdio ...
```

（你可以在后续根据自己的习惯调整 `Makefile` 或 `run.sh`。）

## 从“只输出”到“可输入”

输出只需要写 `THR`；输入则需要读取 `RBR` 并检查 `LSR_DR`。

本仓库的 UART 驱动已经提供：

- `has_data()`：检查是否有数据可读
- `get_char()`：读取一个字节（若无数据返回 `None`）

不过目前 `src/user/shell.rs` 的输入读取仍是简化实现（返回固定字符串）。当你准备实现真正的交互式 Shell 时，本节的串口输入/回显将会派上用场（参见第 13 章）。

## 小结

本节你应该掌握：

- 如何用 QEMU 在终端看到串口输出
- 如何退出 QEMU、以及 monitor/串口复用的基本概念
- 输入能力来自 UART 的 `RBR/LSR`，并会影响后续 Shell 的实现

下一节开始我们将进入 UART 驱动的实现细节。
