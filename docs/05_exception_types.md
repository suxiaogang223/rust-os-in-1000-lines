# 异常类型

## 概述

RISC-V 把 trap 的原因编码在 `mcause` 里：

- 最高位：0 表示 **异常（Exception）**，1 表示 **中断（Interrupt）**
- 其余位：具体的异常/中断编号（cause code）

理解这些编号非常关键，因为异常处理器就是根据它做分发的。

本仓库把常用异常/中断编号定义在 `src/arch/riscv.rs`，异常处理逻辑在 `src/kernel/exception.rs`。

## 常见异常（Exception）

以下是学习内核时最常碰到的一些异常类型（并非完整列表）：

- **Illegal instruction**：执行了非法指令（可能是跳到错误地址，或使用了不支持的扩展）
- **Instruction access fault**：取指失败（权限/映射/总线错误）
- **Load/Store access fault**：数据访存失败
- **Address misaligned**：地址未对齐
- **Breakpoint**：断点（调试/`ebreak`）
- **Environment call (ecall)**：系统调用入口（用户态/内核态均可触发，常用的是 user）

本仓库定义的异常码（节选）：

- `EXCEPTION_ILLEGAL_INSTRUCTION`
- `EXCEPTION_LOAD_ACCESS_FAULT`
- `EXCEPTION_STORE_ACCESS_FAULT`
- `EXCEPTION_ECALL_FROM_USER`

你可以在 `src/arch/riscv.rs` 查到具体数值。

## 常见中断（Interrupt）

中断通常来自“外部事件”，例如定时器或外设：

- Machine timer interrupt：定时器中断（驱动时间片/时钟）
- Machine external interrupt：外部中断（例如 PLIC 转发的设备中断）

本仓库在 `src/kernel/exception.rs` 里演示了对两类中断打印信息：

- `INTERRUPT_MACHINE_TIMER`
- `INTERRUPT_MACHINE_EXTERNAL`

## 如何从 mcause 解析类型

典型逻辑：

1. 取 `interrupt = mcause >> (XLEN - 1)` 判断是否中断
2. `code = mcause & ((1 << (XLEN - 1)) - 1)` 得到编号

本仓库用 64 位环境下的写法：

```rust
if mcause & (1 << 63) != 0 {
    let interrupt_code = mcause & 0x7FFF_FFFF;
    /* ... */
} else {
    let exception_code = mcause;
    /* ... */
}
```

## 调试建议：配合 mepc/mtval 一起看

只看异常码往往不够。建议你在异常发生时同时记录：

- `mepc`：异常发生的 PC（定位代码位置）
- `mtval`：异常相关值（非法指令/访问地址等）

本仓库的异常 handler 会把这三者都打印出来，方便你在早期阶段定位问题。

## 小结

本章你应该掌握：

- `mcause` 如何编码异常/中断
- 常见异常码与中断码分别意味着什么
- 排查问题时要结合 `mepc/mtval` 一起看
