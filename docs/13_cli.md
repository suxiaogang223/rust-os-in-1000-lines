# 命令行界面

## 概述

当内核拥有基础输出（UART）与一些子系统（进程、文件系统）之后，一个最直观的“可交互界面”就是 Shell/CLI：

- 用文本命令触发内核功能
- 便于调试与演示
- 不需要图形系统

本仓库在 `src/user/shell.rs` 实现了一个教学版 Shell，用于展示：

- REPL 循环（prompt -> 读输入 -> 解析 -> 执行）
- 一组最小命令：`help/list/create/exit`

## Shell 的基本结构

教学版 Shell 的核心循环一般是：

1. 打印提示符 `> `
2. 读取一行输入
3. 解析为命令结构
4. 执行命令并打印结果

本仓库的 `Shell::run()` 正是这个结构。

## 命令集合（当前实现）

本仓库定义的命令枚举：

- `help`：打印帮助
- `list`：列出进程
- `create`：创建新进程
- `exit`：退出 shell

它们对应的执行逻辑在 `Shell::execute_command()` 中实现。

## Shell 与内核子系统的连接点

Shell 的价值在于：它把“抽象 API”变成可见的交互行为。

例如：

- `list` 调用 `process::list_processes()`（第 07 章）
- `create` 调用 `process::create_process()` 创建一个新进程

当你把文件系统做得更真实后，也可以添加命令：

- `ls/cat/cd`：目录与文件操作（第 12 章）
- `run`：加载并执行用户程序（第 09/10 章）

## 小结

本章你应该理解：

- CLI/Shell 是最轻量的交互界面，非常适合教学内核
- Shell 的核心是 REPL 循环与命令分发
- Shell 是连接“用户输入”与“内核子系统 API”的最佳试验田
