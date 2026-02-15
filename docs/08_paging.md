# 分页机制

## 概述

分页（Paging）是虚拟内存的基础。它把内存管理从“线性地址”变成“按页映射”：

- 虚拟地址（VA）：程序看到的地址
- 物理地址（PA）：实际 RAM 地址
- 页（Page）：固定大小的块（常见 4KiB）

分页带来的关键能力：

- 进程隔离（每个进程一套地址空间）
- 权限控制（R/W/X/U 等）
- 更灵活的内存布局（映射文件、共享内存、懒分配等）

本仓库在 `src/kernel/paging.rs` 给出了一个教学用途的分页/页表框架，用来串起“页表项、映射、翻译”的概念。

## 页大小与对齐

本仓库在 `src/kernel/memory.rs` 定义了：

- `PAGE_SIZE = 4096`

分页系统通常要求：

- 页起始地址按页大小对齐
- 映射长度是页大小的整数倍（或最后一页处理部分映射）

## RISC-V 的分页模式（Sv39 简介）

在 RV64 上，常见分页模式是 Sv39：

- 虚拟地址有效位 39 位
- 三级页表（VPN[2], VPN[1], VPN[0]）
- 每级页表 512 项（9 位索引）

真实实现会：

- 构造/维护多级页表
- 把根页表地址写入 `satp`
- 在切换地址空间后执行 `sfence.vma` 刷新 TLB

本仓库目前为了演示，把页表结构简化为“单级 + 线性索引”的思路，并用打印来展示映射过程。

## 映射（map）与取消映射（unmap）

分页系统最核心的 API 往往是：

- `map(vaddr, paddr, flags)`
- `unmap(vaddr)`
- `translate(vaddr) -> paddr`

本仓库在 `src/kernel/paging.rs` 中提供了：

- `PageTable::map_page/unmap_page/translate`
- `VirtualMemoryManager::map_memory/unmap_memory`（目前主要是打印演示）

在 `src/kernel/mod.rs` 的初始化过程中，也演示了调用 `map_memory()` 和 `unmap_memory()`。

## 权限位（flags）的意义

页表项通常包含一组权限位：

- V：有效
- R/W/X：读/写/执行
- U：用户可访问

这些位共同决定了 CPU 对该页的访问是否允许。权限错误通常会触发异常（例如 load/store access fault）。

## 小结

本章你应该理解：

- 分页把 VA 映射到 PA，并以“页”为单位管理
- 页表项包含映射关系与权限位
- 本仓库的分页实现以教学骨架为主，真实隔离需要补齐 satp、多级页表、TLB 刷新等环节
