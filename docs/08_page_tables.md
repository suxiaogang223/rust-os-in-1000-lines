# 页表管理

## 概述

页表（Page Table）是分页系统的核心数据结构：它把“虚拟页号（VPN）”映射到“物理页号（PPN）”，并携带权限信息。

本仓库在 `src/kernel/paging.rs` 定义了：

- `PageTableEntry`：页表项（PTE）封装
- `PageTable`：包含 512 项的页表数组（教学简化）

本节重点解释：

1. 页表项有哪些字段/标志
2. 512 项从何而来（9 位索引）
3. 映射/取消映射在数据结构上做了什么

## 页表项（PTE）与标志位

真实的 RISC-V PTE 格式在 Sv39/Sv48 下有固定布局，包含：

- 有效位 V
- 权限位 R/W/X/U
- 访问/脏位 A/D
- 物理页号 PPN（位域）

本仓库为了教学，把常见标志位抽成常量（节选）：

```rust
const PTE_V: usize = 1 << 0;
const PTE_R: usize = 1 << 1;
const PTE_W: usize = 1 << 2;
const PTE_X: usize = 1 << 3;
const PTE_U: usize = 1 << 4;
```

并通过 `PageTableEntry` 提供一些便捷方法：

- `is_valid()/set_valid()`
- `set_readable()/set_writable()/set_executable()`
- `set_physical_address()/get_physical_address()`

这些方法的目的主要是：让你在阅读代码时能“按语义”理解，而不是直接位运算。

## 页表大小：为什么是 512 项

在 Sv39 的每一级页表中：

- 索引是 9 位（`2^9 = 512`）
- 每项 8 字节（64 位 PTE）
- 整张页表大小 `512 * 8 = 4096` 字节，正好一页

因此“页表本身也用一页存放”是非常自然的设计。

本仓库把 `PageTable` 的 entries 定义为 `[PageTableEntry; 512]`，正是这个原因（即使当前实现做了简化）。

## 映射与取消映射

最小映射流程是：

1. 根据虚拟地址取出虚拟页号 VPN
2. 找到页表项位置
3. 写入物理地址与 flags，并置有效位

本仓库的 `map_page()` 用了一个简化的 VPN 计算：

```rust
let vpn = (vaddr >> 12) & 0x1FF;
```

这相当于只使用“最低一级”的 9 位索引（教学简化）。

取消映射则把 `valid` 清掉即可。

## 练习：演进到多级页表（思路）

当你准备实现更接近真实的 Sv39，你需要：

- 计算 `VPN[2], VPN[1], VPN[0]`
- 每一级页表项要么指向下一级页表，要么是叶子映射
- 动态分配中间页表页（需要页级分配器或从堆申请整页）

## 小结

本章你应该理解：

- PTE 是“映射 + 权限”的组合
- 512 项来自 9 位索引（Sv39 每级页表）
- 本仓库当前页表实现用于教学骨架，后续可按 Sv39 逐步补齐多级结构与 satp 配置
