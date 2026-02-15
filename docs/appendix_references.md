# 参考资源

本附录列出学习/实现本教程时常用的参考资料（按主题分类）。链接可能会随着时间变化，但关键关键词一般稳定可搜到。

## 原始教程与相关项目

- 《1000行代码的操作系统》（原版教程）：https://operating-system-in-1000-lines.vercel.app/zh/
- RISC-V 官方主页：https://riscv.org/

## RISC-V 规范

- The RISC-V Instruction Set Manual（Unprivileged Spec）：https://riscv.org/technical/specifications/
- The RISC-V Privileged Architecture（Privileged Spec）：https://riscv.org/technical/specifications/

建议阅读顺序：

1. Unprivileged（指令集、寄存器、基本指令编码）
2. Privileged（异常/中断、CSR、特权级、分页 `satp` 等）

## QEMU / virt 机器

- QEMU 官方文档：https://www.qemu.org/documentation/
- `qemu-system-riscv64` 帮助：在本地运行 `qemu-system-riscv64 -help`

建议重点关注：

- `-machine virt`
- `-nographic`、串口/monitor 重定向
- virtio-blk（当你准备实现真实磁盘驱动时）

## Rust（no_std / 内联汇编）

- The Rust Reference / Inline Assembly：https://doc.rust-lang.org/reference/inline-assembly.html
- Rust Embedded Book（no_std 思维非常通用）：https://docs.rust-embedded.org/book/

## 操作系统/实现技巧

- OSDev Wiki（经典 OS 实现百科）：https://wiki.osdev.org/

## mdBook（本书的构建工具）

- mdBook 文档：https://rust-lang.github.io/mdBook/

常用命令：

- `cd docs && mdbook serve`：本地预览
- `cd docs && mdbook build`：生成静态输出
