# 块设备操作

## 概述

块设备（Block Device）以“固定大小的块”为单位读写数据。与之相对的是字符设备（如 UART），它以字节流的形式读写。

文件系统之所以倾向于建立在块设备之上，是因为：

- 块设备有明确的随机访问能力（按 LBA/sector 定位）
- 更容易做缓存、预读、写回等优化

本仓库的块设备接口体现在 `src/drivers/disk.rs`：

- `BLOCK_SIZE = 512`
- `read_disk_block(sector, buffer)`
- `write_disk_block(sector, buffer)`

## sector / block / LBA

在教程中我们可以把这些概念近似理解为：

- sector：磁盘最小寻址单元（常见 512B）
- block：文件系统/缓存层的读写单位（可等于 sector，也可更大）
- LBA：逻辑块地址（从 0 开始编号）

本仓库用 `sector` 表示“512B 单位的块编号”。

## 最小块设备 API 的设计要点

一个最小 API 通常需要：

- `init()`：识别设备容量、就绪状态
- `read(lba, buf)`：读取一个块
- `write(lba, buf)`：写入一个块
- `get_info()`：返回容量与块大小

本仓库的 `DiskDriver` 与全局函数基本覆盖了这些需求。

## 缓冲区约束

块设备读写通常要求：

- `buffer.len() >= BLOCK_SIZE`
- `buffer` 的地址/对齐可能有额外要求（DMA 场景）

本仓库在 `read_block/write_block` 里做了最小长度检查。

## 小结

本章你应该掌握：

- 块设备以固定大小块为单位访问，非常适合文件系统
- 最小块设备接口由 init/read/write/info 组成
- 缓冲区大小与对齐是块设备 API 需要明确的约束
