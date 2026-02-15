# 地址转换

## 概述

地址转换（Address Translation）是把虚拟地址（VA）转换为物理地址（PA）的过程。

在启用分页后，CPU 每次访存都会：

1. 取虚拟地址
2. 通过页表查找对应的物理页号（PPN）
3. 拼上页内偏移得到物理地址
4. 若缺页或权限不符，触发异常

本仓库在 `src/kernel/paging.rs` 提供了一个简化的 `translate(vaddr)`，用来演示“页内偏移 + 页表查找”的基本形式。

## VA 拆分：页号 + 偏移

以 4KiB 页为例：

- 页内偏移：低 12 位（`offset = va & 0xFFF`）
- 虚拟页号：其余高位（按 Sv39 会再拆成 3 级索引）

本仓库的简化翻译就是：

```rust
let vpn = (vaddr >> 12) & 0x1FF;
let offset = vaddr & 0xFFF;
```

如果页表项有效，就返回 `paddr | offset`。

## TLB：为什么转换不会太慢

真实 CPU 不会每次访存都走完整页表遍历，而是使用 TLB（Translation Lookaside Buffer）缓存最近的转换结果。

当页表更新或切换地址空间时，需要：

- 刷新 TLB（RISC-V 使用 `sfence.vma`）

本项目目前以教学演示为主，尚未涉及 satp/TLB 刷新，但理解 TLB 的存在能帮助你把分页系统看成“可用且高效”的机制，而不是纯软件开销。

## 缺页与访问异常

地址转换可能失败的两类典型原因：

- **页不存在（not present）**：页表项无效 -> 触发 page fault
- **权限不足**：例如对不可写页执行 store -> 触发 access fault

这些异常最终会回到 trap handler（第 04 章），并通过 `mcause/mtval` 提供线索：

- `mtval` 通常会给出触发异常的虚拟地址（或相关值）

## 小结

本章你应该理解：

- 地址转换是“查页表 + 拼偏移”的过程
- TLB 缓存转换结果，提升性能
- 转换失败会触发异常，最终由 trap handler 处理
