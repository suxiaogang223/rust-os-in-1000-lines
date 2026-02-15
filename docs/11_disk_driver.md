# 磁盘驱动

## 概述

磁盘驱动（或更一般的“块设备驱动”）的目标是提供一个稳定的接口：

- 读写固定大小的块（通常 512B 或 4KiB）
- 屏蔽具体硬件控制器的细节

一旦块设备接口可用，文件系统（第 12 章）就可以建立在它之上。

本仓库在 `src/drivers/disk.rs` 提供了一个教学版的磁盘驱动骨架：

- `BLOCK_SIZE = 512`
- `init/read_block/write_block/get_info`
- `DiskResult` 用于返回成功/错误

## 真实世界里“磁盘”通常是什么

在 QEMU `virt` 机器里，最常见的块设备是 **virtio-blk**：

- 通过 MMIO 寄存器与 virtqueue 进行交互
- 需要描述符链、队列通知、中断处理等

本仓库的 `Makefile` 也提供了 `run-with-disk`，会在 QEMU 中挂载一个 `virtio-blk-device`。

不过目前 `src/drivers/disk.rs` 的实现更像是一个“内存映射的模拟磁盘”，用于展示接口形态；如果你想让它真正读写 `disk.img`，需要把驱动替换为 virtio-blk 的实现。

## 本项目 DiskDriver 的接口设计

`DiskDriver` 的核心字段：

- `base_address`：控制器基址（当前是占位）
- `sector_count`：扇区数（初始化时设置为 1024，演示）
- `initialized`：是否已初始化

核心方法：

- `init()`：初始化并设置容量信息
- `read_block(sector, buffer)`：读一个扇区到缓冲区
- `write_block(sector, buffer)`：写一个扇区

这些接口足以支撑一个最小 FAT 读取/目录列出示例。

## 练习：对接 virtio-blk（路线图）

如果你希望实现真实的 virtio-blk，可按以下路线推进：

1. 在 QEMU 启动参数中确认块设备存在（`run-with-disk` 已提供）
2. 实现 virtio-mmio 的寄存器访问与设备初始化
3. 构造 virtqueue，提交读写请求
4. 处理中断或轮询完成
5. 在 `read_block/write_block` 中封装上述细节

完成后，文件系统章节的代码就可以真正读写 `disk.img` 上的数据结构。

## 小结

本章你应该理解：

- 块设备驱动的目标是提供“按块读写”的稳定接口
- 本仓库当前提供的是教学骨架；要与 QEMU 的 `virtio-blk` 对接需要进一步实现 virtio 协议
