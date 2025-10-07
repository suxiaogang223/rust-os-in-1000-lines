# 第03章：UART驱动实现

## 概述

UART(Universal Asynchronous Receiver/Transmitter)是串行通信接口，是操作系统内核与外部世界通信的重要方式。本章将详细介绍UART的工作原理和驱动实现。

## UART原理

### 串行通信基础

UART是一种异步串行通信协议，具有以下特点：

- **异步通信** - 不需要时钟信号
- **全双工** - 可同时发送和接收
- **起始位** - 标识数据开始
- **数据位** - 实际数据(5-8位)
- **停止位** - 标识数据结束
- **奇偶校验** - 可选的错误检测

### 通信参数

```rust
// UART通信参数
pub const UART_BAUD_RATE: u32 = 115200;    // 波特率
pub const UART_DATA_BITS: u8 = 8;          // 数据位
pub const UART_STOP_BITS: u8 = 1;          // 停止位
pub const UART_PARITY: u8 = 0;             // 无奇偶校验
```

## UART寄存器

### 寄存器映射

在QEMU virt机器中，UART的寄存器映射如下：

```rust
// UART寄存器地址
const UART_BASE: usize = 0x10000000;

// UART寄存器偏移
const UART_RBR: usize = 0x00;  // 接收缓冲寄存器 (Read)
const UART_THR: usize = 0x00;  // 发送保持寄存器 (Write)
const UART_IER: usize = 0x01;  // 中断使能寄存器
const UART_IIR: usize = 0x02;  // 中断标识寄存器
const UART_FCR: usize = 0x02;  // FIFO控制寄存器
const UART_LCR: usize = 0x03;  // 线路控制寄存器
const UART_MCR: usize = 0x04;  // 调制解调器控制寄存器
const UART_LSR: usize = 0x05;  // 线路状态寄存器
const UART_MSR: usize = 0x06;  // 调制解调器状态寄存器
const UART_SCR: usize = 0x07;  // 暂存寄存器
```

### 重要寄存器详解

#### 线路控制寄存器 (LCR)
```
位7: DLAB (Divisor Latch Access Bit)
位6: 设置中断
位5: 粘性奇偶校验
位4: 偶校验选择
位3: 奇偶校验使能
位2: 停止位数量
位1-0: 数据位长度
```

#### 线路状态寄存器 (LSR)
```
位7: 错误标志
位6: 发送移位寄存器空
位5: 发送保持寄存器空
位4: 中断标识
位3: 帧错误
位2: 奇偶校验错误
位1: 溢出错误
位0: 数据就绪
```

## UART驱动实现

### 基础结构

```rust
// UART驱动结构体
pub struct Uart {
    base: usize,
}

impl Uart {
    // 创建UART实例
    pub const fn new(base: usize) -> Self {
        Self { base }
    }

    // 初始化UART
    pub fn init(&self) {
        // 设置线路控制寄存器
        // 8位数据位，1位停止位，无奇偶校验
        self.write_reg(UART_LCR, 0x03);
        
        // 禁用中断
        self.write_reg(UART_IER, 0x00);
        
        // 禁用FIFO
        self.write_reg(UART_FCR, 0x00);
        
        // 设置调制解调器控制
        self.write_reg(UART_MCR, 0x00);
    }

    // 读取寄存器
    fn read_reg(&self, offset: usize) -> u8 {
        unsafe {
            core::ptr::read_volatile((self.base + offset) as *const u8)
        }
    }

    // 写入寄存器
    fn write_reg(&self, offset: usize, value: u8) {
        unsafe {
            core::ptr::write_volatile((self.base + offset) as *mut u8, value);
        }
    }
}
```

### 字符发送

```rust
impl Uart {
    // 发送单个字符
    pub fn put_char(&self, c: u8) {
        // 等待发送缓冲区空闲
        while !self.is_transmit_empty() {}
        
        // 发送字符
        self.write_reg(UART_THR, c);
    }

    // 检查发送缓冲区是否为空
    fn is_transmit_empty(&self) -> bool {
        let lsr = self.read_reg(UART_LSR);
        (lsr & 0x20) != 0  // 第5位表示发送保持寄存器空
    }

    // 发送字符串
    pub fn put_str(&self, s: &str) {
        for byte in s.bytes() {
            self.put_char(byte);
        }
    }
}
```

### 字符接收

```rust
impl Uart {
    // 接收单个字符
    pub fn get_char(&self) -> Option<u8> {
        if self.is_data_ready() {
            Some(self.read_reg(UART_RBR))
        } else {
            None
        }
    }

    // 检查是否有数据可读
    fn is_data_ready(&self) -> bool {
        let lsr = self.read_reg(UART_LSR);
        (lsr & 0x01) != 0  // 第0位表示数据就绪
    }

    // 阻塞式接收字符
    pub fn get_char_blocking(&self) -> u8 {
        while !self.is_data_ready() {}
        self.read_reg(UART_RBR)
    }
}
```

### 高级功能

```rust
impl Uart {
    // 检查UART是否可用
    pub fn is_available(&self) -> bool {
        let lsr = self.read_reg(UART_LSR);
        (lsr & 0x20) != 0  // 发送缓冲区空
    }

    // 清空接收缓冲区
    pub fn flush_receive(&self) {
        while self.is_data_ready() {
            let _ = self.read_reg(UART_RBR);
        }
    }

    // 设置波特率（需要访问DLAB）
    pub fn set_baud_rate(&self, baud_rate: u32) {
        // 计算分频器值
        let clock_freq = 25_000_000;  // 25MHz时钟
        let divisor = clock_freq / (baud_rate * 16);
        
        // 设置DLAB位
        let lcr = self.read_reg(UART_LCR);
        self.write_reg(UART_LCR, lcr | 0x80);
        
        // 写入分频器
        self.write_reg(UART_RBR, (divisor & 0xFF) as u8);
        self.write_reg(UART_IER, ((divisor >> 8) & 0xFF) as u8);
        
        // 清除DLAB位
        self.write_reg(UART_LCR, lcr & !0x80);
    }
}
```

## 全局UART实例

### 单例模式

```rust
// 全局UART实例
pub static UART: Uart = Uart::new(UART_BASE);

// 便捷函数
pub fn print(s: &str) {
    UART.put_str(s);
}

pub fn println(s: &str) {
    UART.put_str(s);
    UART.put_str("\n");
}

pub fn put_char(c: u8) {
    UART.put_char(c);
}

pub fn put_str(s: &str) {
    UART.put_str(s);
}

pub fn get_char() -> Option<u8> {
    UART.get_char()
}

pub fn get_char_blocking() -> u8 {
    UART.get_char_blocking()
}
```

### 格式化输出

```rust
// 数字转字符串（简化实现）
pub fn print_number(num: usize) {
    if num == 0 {
        print("0");
        return;
    }
    
    let mut digits = [0u8; 20];
    let mut i = 0;
    let mut n = num;
    
    while n > 0 {
        digits[i] = (n % 10) as u8 + b'0';
        n /= 10;
        i += 1;
    }
    
    for j in (0..i).rev() {
        put_char(digits[j]);
    }
}

// 十六进制输出
pub fn print_hex(num: usize) {
    print("0x");
    if num == 0 {
        print("0");
        return;
    }
    
    let mut digits = [0u8; 16];
    let mut i = 0;
    let mut n = num;
    
    while n > 0 {
        let digit = (n & 0xF) as u8;
        digits[i] = if digit < 10 {
            digit + b'0'
        } else {
            digit - 10 + b'A'
        };
        n >>= 4;
        i += 1;
    }
    
    for j in (0..i).rev() {
        put_char(digits[j]);
    }
}
```

## 中断驱动UART

### 中断处理

```rust
// UART中断处理
pub fn handle_uart_interrupt() {
    let iir = UART.read_reg(UART_IIR);
    let interrupt_type = iir & 0x0F;
    
    match interrupt_type {
        0x02 => {
            // 发送中断
            handle_transmit_interrupt();
        },
        0x04 => {
            // 接收中断
            handle_receive_interrupt();
        },
        0x06 => {
            // 线路状态中断
            handle_line_status_interrupt();
        },
        _ => {
            // 其他中断或伪中断
        }
    }
}

// 发送中断处理
fn handle_transmit_interrupt() {
    // 处理发送完成
    // 可以发送下一个字符
}

// 接收中断处理
fn handle_receive_interrupt() {
    // 处理接收到的字符
    if let Some(c) = UART.get_char() {
        // 处理接收到的字符
        process_received_char(c);
    }
}

// 线路状态中断处理
fn handle_line_status_interrupt() {
    let lsr = UART.read_reg(UART_LSR);
    
    if (lsr & 0x80) != 0 {
        // 溢出错误
        println("UART: Overflow error");
    }
    if (lsr & 0x40) != 0 {
        // 奇偶校验错误
        println("UART: Parity error");
    }
    if (lsr & 0x20) != 0 {
        // 帧错误
        println("UART: Framing error");
    }
}
```

### 中断使能

```rust
// 启用UART中断
pub fn enable_uart_interrupts() {
    // 启用接收中断
    UART.write_reg(UART_IER, 0x01);
    
    // 启用全局中断
    write_mie_global(true);
}

// 禁用UART中断
pub fn disable_uart_interrupts() {
    UART.write_reg(UART_IER, 0x00);
}
```

## 缓冲区和队列

### 环形缓冲区

```rust
// 环形缓冲区结构
pub struct RingBuffer {
    buffer: [u8; 256],
    head: usize,
    tail: usize,
    size: usize,
}

impl RingBuffer {
    pub const fn new() -> Self {
        Self {
            buffer: [0; 256],
            head: 0,
            tail: 0,
            size: 0,
        }
    }

    pub fn push(&mut self, byte: u8) -> bool {
        if self.size >= 256 {
            return false;  // 缓冲区满
        }
        
        self.buffer[self.tail] = byte;
        self.tail = (self.tail + 1) % 256;
        self.size += 1;
        true
    }

    pub fn pop(&mut self) -> Option<u8> {
        if self.size == 0 {
            return None;
        }
        
        let byte = self.buffer[self.head];
        self.head = (self.head + 1) % 256;
        self.size -= 1;
        Some(byte)
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size >= 256
    }
}
```

### 带缓冲区的UART

```rust
// 带缓冲区的UART驱动
pub struct BufferedUart {
    uart: Uart,
    rx_buffer: RingBuffer,
    tx_buffer: RingBuffer,
}

impl BufferedUart {
    pub fn new(base: usize) -> Self {
        Self {
            uart: Uart::new(base),
            rx_buffer: RingBuffer::new(),
            tx_buffer: RingBuffer::new(),
        }
    }

    pub fn init(&self) {
        self.uart.init();
    }

    // 非阻塞发送
    pub fn send(&mut self, byte: u8) -> bool {
        if self.uart.is_available() {
            self.uart.put_char(byte);
            true
        } else {
            self.tx_buffer.push(byte)
        }
    }

    // 非阻塞接收
    pub fn receive(&mut self) -> Option<u8> {
        // 首先从缓冲区读取
        if let Some(byte) = self.rx_buffer.pop() {
            return Some(byte);
        }
        
        // 然后从UART读取
        if let Some(byte) = self.uart.get_char() {
            Some(byte)
        } else {
            None
        }
    }

    // 处理中断
    pub fn handle_interrupt(&mut self) {
        // 处理接收中断
        if let Some(byte) = self.uart.get_char() {
            if !self.rx_buffer.push(byte) {
                // 缓冲区满，丢弃字符
            }
        }
        
        // 处理发送中断
        if self.uart.is_available() {
            if let Some(byte) = self.tx_buffer.pop() {
                self.uart.put_char(byte);
            }
        }
    }
}
```

## 实践练习

### 练习1：基本UART操作
实现一个简单的UART测试程序：

```rust
pub fn uart_test() {
    UART.init();
    
    println("UART Test Started");
    println("Type characters (press Enter to exit):");
    
    loop {
        if let Some(c) = get_char() {
            if c == b'\r' || c == b'\n' {
                println("");
                break;
            }
            put_char(c);  // 回显
        }
    }
    
    println("UART Test Completed");
}
```

### 练习2：字符串处理
实现一个简单的命令行解析器：

```rust
pub fn command_parser() {
    let mut buffer = [0u8; 256];
    let mut index = 0;
    
    println("Command Parser Started");
    println("Enter commands (type 'exit' to quit):");
    
    loop {
        if let Some(c) = get_char_blocking() {
            if c == b'\r' || c == b'\n' {
                buffer[index] = 0;
                let command = core::str::from_utf8(&buffer[..index]).unwrap_or("");
                
                match command {
                    "exit" => break,
                    "help" => println("Available commands: help, exit, test"),
                    "test" => println("Test command executed"),
                    _ => println("Unknown command"),
                }
                
                index = 0;
                print("> ");
            } else if index < 255 {
                buffer[index] = c;
                index += 1;
                put_char(c);  // 回显
            }
        }
    }
}
```

### 练习3：格式化输出
实现一个简单的printf函数：

```rust
pub fn printf(format: &str, args: &[&str]) {
    let mut arg_index = 0;
    let mut chars = format.chars();
    
    while let Some(c) = chars.next() {
        if c == '%' {
            if let Some(next) = chars.next() {
                match next {
                    's' => {
                        if arg_index < args.len() {
                            print(args[arg_index]);
                            arg_index += 1;
                        }
                    },
                    'd' => {
                        if arg_index < args.len() {
                            // 简化的数字输出
                            print(args[arg_index]);
                            arg_index += 1;
                        }
                    },
                    '%' => put_char(b'%'),
                    _ => put_char(next as u8),
                }
            }
        } else {
            put_char(c as u8);
        }
    }
}
```

## 性能优化

### 批量传输

```rust
// 批量发送字符串
pub fn print_batch(strings: &[&str]) {
    for s in strings {
        UART.put_str(s);
    }
}

// 批量接收
pub fn read_line(buffer: &mut [u8]) -> usize {
    let mut index = 0;
    
    while index < buffer.len() - 1 {
        if let Some(c) = get_char_blocking() {
            if c == b'\r' || c == b'\n' {
                break;
            }
            buffer[index] = c;
            index += 1;
        }
    }
    
    buffer[index] = 0;
    index
}
```

### 中断优化

```rust
// 高效的中断处理
pub fn optimized_uart_interrupt() {
    // 批量处理接收数据
    let mut count = 0;
    while count < 16 && UART.is_data_ready() {
        if let Some(c) = UART.get_char() {
            // 处理字符
            process_char(c);
            count += 1;
        }
    }
    
    // 批量处理发送数据
    while UART.is_available() && !tx_queue.is_empty() {
        if let Some(c) = tx_queue.pop() {
            UART.put_char(c);
        }
    }
}
```

## 总结

本章我们学习了：

1. **UART原理** - 串行通信的基础知识
2. **寄存器操作** - UART寄存器的读写方法
3. **驱动实现** - 完整的UART驱动实现
4. **字符传输** - 发送和接收字符的方法
5. **高级功能** - 中断处理、缓冲区管理
6. **性能优化** - 批量传输和中断优化
7. **实践练习** - 实际应用场景的代码示例

UART驱动是操作系统内核与外部世界通信的基础，为后续实现更复杂的功能提供了重要的通信手段。

## 下一步

掌握了UART驱动后，我们可以开始学习异常处理机制。请继续阅读 [异常处理](./04_exception_handling.md) 章节。

## 参考资源

- [UART通信协议](https://en.wikipedia.org/wiki/Universal_asynchronous_receiver-transmitter)
- [串行通信基础](https://www.electronics-tutorials.ws/io/io_4.html)
- [RISC-V中断处理](https://riscv.org/wp-content/uploads/2019/12/riscv-privileged-20191213.pdf)
- [QEMU virt机器文档](https://qemu.readthedocs.io/en/latest/system/riscv/virt.html)