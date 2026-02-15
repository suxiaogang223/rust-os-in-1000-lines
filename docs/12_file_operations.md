# 文件操作

## 概述

文件操作是文件系统对外最常用的接口：

- `open`：根据路径/文件名找到目录项并创建句柄
- `read`：从文件当前位置读取数据并推进偏移
- `close`：关闭句柄、释放资源

本仓库在 `src/fs/fat.rs` 定义了：

- `FileInfo`：目录项抽象（文件名、属性、大小、首簇等）
- `FileHandle`：打开文件的运行时状态（当前位置、是否打开）
- `open_file/read_file/close_file`：教学版文件 API

## FileInfo：目录项信息

FAT 目录项核心信息包括：

- 8.3 文件名（8 字节主名 + 3 字节扩展名）
- 属性位（目录/只读/隐藏等）
- 文件大小
- 首簇号（first cluster）

本仓库的 `FileInfo` 把这些信息抽成字段，并提供了：

- `is_directory()`
- `is_regular_file()`

以便调用方按语义判断条目类型。

## open_file：查找根目录并打开

当前实现的 `open_file(filename)`：

- 仅在“根目录数组”里线性查找
- 用最简单的字符串比较匹配文件名

注意：FAT 的 8.3 文件名通常用空格填充到固定长度，本仓库的示例文件名是 `HELLO   TXT`（主名部分含空格）。

为了让 `open_file("HELLO")` 之类调用更自然，你可以在后续改进：

- 去掉尾部空格后比较
- 支持大小写不敏感匹配
- 支持带扩展名的匹配（`HELLO.TXT`）

## read_file：按当前位置读取

`read_file(handle, buffer)` 的最小语义是：

1. 计算还能读多少（`remaining = size - pos`）
2. 确定本次读取的字节数 `to_read`
3. 读取数据填充 `buffer[..to_read]`
4. 推进 `handle.current_position += to_read`

本仓库目前用演示数据填充 `buffer`，并打印读取进度，方便观察接口行为。

当你把文件系统做真实后，这里需要改成：

- 根据首簇号 + FAT 表遍历簇链
- 把簇号换算成 LBA 扇区号
- 调用块设备读取对应数据

## close_file：关闭句柄

当前 `close_file()` 只做：

- `handle.open = false`

真实实现可能还需要：

- 写回缓存
- 减少引用计数
- 释放内核对象等

## 小结

本章你应该理解：

- 文件操作 API 的最小语义：open/read/close
- 文件句柄需要维护“当前位置”
- 教学实现可先用演示数据跑通接口，后续再接入真实簇链与块设备读写
