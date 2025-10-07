# Rust OS in 1000 Lines - 教程

一个用Rust语言实现的1000行代码操作系统教程，参考了[《1000行代码的操作系统》](https://operating-system-in-1000-lines.vercel.app/zh/)教程。

## 📚 教程目录

### 第01章：入门
- [环境设置](./01_environment_setup.md)
- [项目结构](./01_project_structure.md)
- [第一个内核](./01_first_kernel.md)

### 第02章：RISC-V 101
- [RISC-V架构基础](./02_riscv_basics.md)
- [寄存器操作](./02_registers.md)
- [控制状态寄存器](./02_csr.md)

### 第03章：UART驱动
- [UART原理](./03_uart_principle.md)
- [串口通信](./03_serial_communication.md)
- [驱动实现](./03_uart_driver.md)

### 第04章：内核恐慌
- [异常处理](./04_exception_handling.md)
- [Panic处理](./04_panic_handler.md)
- [错误恢复](./04_error_recovery.md)

### 第05章：异常处理
- [异常类型](./05_exception_types.md)
- [中断处理](./05_interrupt_handling.md)
- [系统调用](./05_syscalls.md)

### 第06章：内存管理
- [内存分配器](./06_memory_allocator.md)
- [堆管理](./06_heap_management.md)
- [内存保护](./06_memory_protection.md)

### 第07章：进程管理
- [进程概念](./07_process_concept.md)
- [进程调度](./07_process_scheduling.md)
- [上下文切换](./07_context_switching.md)

### 第08章：虚拟内存
- [分页机制](./08_paging.md)
- [页表管理](./08_page_tables.md)
- [地址转换](./08_address_translation.md)

### 第09章：用户模式
- [特权级别](./09_privilege_levels.md)
- [用户态切换](./09_user_mode_switch.md)
- [系统调用接口](./09_syscall_interface.md)

### 第10章：系统调用
- [系统调用机制](./10_syscall_mechanism.md)
- [系统调用实现](./10_syscall_implementation.md)
- [用户接口](./10_user_interface.md)

### 第11章：磁盘I/O
- [磁盘驱动](./11_disk_driver.md)
- [块设备操作](./11_block_device.md)
- [I/O调度](./11_io_scheduling.md)

### 第12章：文件系统
- [FAT文件系统](./12_fat_filesystem.md)
- [文件操作](./12_file_operations.md)
- [目录管理](./12_directory_management.md)

### 第13章：Shell应用
- [命令行界面](./13_cli.md)
- [命令解析](./13_command_parsing.md)
- [用户交互](./13_user_interaction.md)

## 🚀 快速开始

1. **环境准备**
   ```bash
   # 安装Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # 安装QEMU
   brew install qemu  # macOS
   # 或
   sudo apt install qemu-system-misc  # Ubuntu
   ```

2. **克隆项目**
   ```bash
   git clone <your-repo-url>
   cd rust-os-in-1000-lines
   ```

3. **运行测试**
   ```bash
   ./test.sh
   ```

4. **启动系统**
   ```bash
   ./run.sh
   ```

## 🛠️ 开发工具

- **Rust工具链**: 1.70+
- **QEMU模拟器**: 支持RISC-V64
- **目标架构**: riscv64gc-unknown-none-elf

## 📖 学习目标

通过本教程，你将学会：

1. **Rust系统编程** - 在no_std环境中开发
2. **操作系统原理** - 内核、进程、内存管理
3. **RISC-V架构** - 寄存器、异常、中断
4. **设备驱动** - UART、磁盘、文件系统
5. **系统调用** - 用户态与内核态交互

## 🤝 贡献

欢迎提交Issue和Pull Request来改进教程！

## 📄 许可证

MIT License
