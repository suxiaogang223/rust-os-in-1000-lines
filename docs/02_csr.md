# 控制状态寄存器

## 概述

CSR（Control and Status Registers）是 RISC-V 特权架构中用于控制/查询 CPU 运行状态的寄存器集合。对内核来说，CSR 是：

- **异常/中断**的入口与现场信息来源（`mtvec/mepc/mcause/mtval`）
- **特权级切换**的关键控制点（`mstatus` 中的 `MPP/MIE/MPIE` 等）
- **中断使能**与屏蔽（`mie/mip`）

本仓库在 `src/arch/riscv.rs` 提供了一组最小 CSR 读写封装，用于后续异常处理、用户态切换等功能。

## 本项目用到的关键 CSR

### `mtvec`（Machine Trap-Vector Base Address）

- 作用：指定机器模式陷入（trap）入口地址
- 用法：在初始化异常处理时设置 `mtvec = handle_exception`（见 `src/kernel/exception.rs`）

### `mepc`（Machine Exception Program Counter）

- 作用：保存发生异常/中断时的 PC
- 常见用途：
  - 打印调试信息定位异常位置
  - 处理完异常后决定“回到哪执行”

### `mcause`（Machine Cause）

- 作用：表示 trap 的原因（异常 or 中断）
- 常见判定：
  - 最高位为 1 表示中断；为 0 表示异常
  - 低位的 cause code 表示具体类型（如 timer interrupt、illegal instruction、ecall 等）

本仓库里 `src/kernel/exception.rs` 用如下逻辑区分中断/异常：

```rust
if mcause & (1 << 63) != 0 { /* interrupt */ } else { /* exception */ }
```

### `mtval`（Machine Trap Value）

- 作用：提供与异常相关的额外信息（例如非法指令、访存地址等）
- 用途：调试时非常有价值，通常会打印出来

### `mstatus`（Machine Status）

`mstatus` 包含一堆位域，其中与本书最相关的是：

- `MIE`：全局中断使能
- `MPIE`：异常发生前的 `MIE` 备份
- `MPP`：异常发生前的特权级（用于 `mret` 返回时恢复）

本仓库提供了：

- `read_mstatus()/write_mstatus()`
- `write_mpp(level)`：修改 `MPP`
- `write_mie_global(bool)`：修改 `MIE`

## CSR 读写封装（src/arch/riscv.rs）

在 `no_std` 的内核里，我们一般会把内联汇编封装成小函数，避免到处散落 `unsafe asm!`。

本仓库的实现思路是：

- 用一个 `read_csr(csr_id)`/`write_csr(csr_id, value)` 统一入口
- 只覆盖当前教程需要的少量 CSR（通过 `match` 分发）

这种方式的优点是简单直观；缺点是可扩展性一般（更多 CSR 会让 `match` 变长）。如果你想做得更“库化”，也可以为每个 CSR 单独提供函数（比如 `read_mtvec()`/`write_mtvec()`），本仓库也已经这么做了一部分。

## 中断使能：`mie` 与全局开关

RISC-V 机器模式下，中断是否真正生效通常需要同时满足：

1. `mstatus.MIE = 1`（全局使能）
2. `mie` 对应中断位为 1（例如 machine timer/external）

初始化异常处理时，本仓库在 `src/kernel/exception.rs` 做了两步：

- 通过 `write_mie(read_mie() | ...)` 打开部分中断源
- 通过 `write_mie_global(true)` 打开全局中断

## 小结

到这里你应该能掌握：

- trap 入口由 `mtvec` 决定
- trap 原因在 `mcause`，现场 PC 在 `mepc`，附加信息在 `mtval`
- 中断开关分为“全局”和“源级”

下一章我们会回到“可见输出”，从串口通信与 UART 驱动开始，让内核能稳定地把信息打印到 QEMU 控制台上。
