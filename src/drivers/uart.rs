// UART 驱动
// 第05章：Hello World - 实现串口输出

// UART 寄存器地址 (QEMU virt 机器)
const UART_BASE: usize = 0x10000000;

// UART 寄存器偏移
const UART_RBR: usize = 0x00; // 接收缓冲区寄存器
const UART_THR: usize = 0x00; // 发送保持寄存器
const UART_IER: usize = 0x01; // 中断使能寄存器
const UART_IIR: usize = 0x02; // 中断标识寄存器
const UART_FCR: usize = 0x02; // FIFO控制寄存器
const UART_LCR: usize = 0x03; // 线路控制寄存器
const UART_MCR: usize = 0x04; // 调制解调器控制寄存器
const UART_LSR: usize = 0x05; // 线路状态寄存器
const UART_MSR: usize = 0x06; // 调制解调器状态寄存器
const UART_SCR: usize = 0x07; // 暂存寄存器

// 线路状态寄存器位
const LSR_THRE: u8 = 0x20; // 发送保持寄存器空
const LSR_DR: u8 = 0x01; // 数据就绪

pub struct Uart {
    base: usize,
}

impl Uart {
    pub const fn new() -> Self {
        Self { base: UART_BASE }
    }

    // 初始化UART
    pub fn init(&self) {
        unsafe {
            // 设置波特率 (115200)
            // 这里简化处理，实际需要设置除数锁存器
            self.write_reg(UART_LCR, 0x80); // 启用除数锁存器访问
            self.write_reg(UART_THR, 0x00); // 除数低字节
            self.write_reg(UART_IER, 0x00); // 除数高字节
            self.write_reg(UART_LCR, 0x03); // 8位数据，1个停止位，无奇偶校验
            self.write_reg(UART_FCR, 0x00); // 禁用FIFO
            self.write_reg(UART_MCR, 0x00); // 禁用调制解调器控制
        }
    }

    // 写入寄存器
    unsafe fn write_reg(&self, offset: usize, value: u8) {
        let addr = self.base + offset;
        core::ptr::write_volatile(addr as *mut u8, value);
    }

    // 读取寄存器
    unsafe fn read_reg(&self, offset: usize) -> u8 {
        let addr = self.base + offset;
        core::ptr::read_volatile(addr as *const u8)
    }

    // 检查发送缓冲区是否为空
    pub fn is_transmit_empty(&self) -> bool {
        unsafe { (self.read_reg(UART_LSR) & LSR_THRE) != 0 }
    }

    // 发送一个字符
    pub fn put_char(&self, c: u8) {
        // 等待发送缓冲区为空
        while !self.is_transmit_empty() {}

        unsafe {
            self.write_reg(UART_THR, c);
        }
    }

    // 发送字符串
    pub fn put_str(&self, s: &str) {
        for &b in s.as_bytes() {
            self.put_char(b);
        }
    }

    // 检查是否有数据可读
    pub fn has_data(&self) -> bool {
        unsafe { (self.read_reg(UART_LSR) & LSR_DR) != 0 }
    }

    // 读取一个字符
    pub fn get_char(&self) -> Option<u8> {
        if self.has_data() {
            unsafe { Some(self.read_reg(UART_RBR)) }
        } else {
            None
        }
    }
}

// 全局UART实例
pub static UART: Uart = Uart::new();

// 全局打印函数
pub fn print(s: &str) {
    UART.put_str(s);
}

pub fn println(s: &str) {
    UART.put_str(s);
    UART.put_char(b'\n');
}

// 兼容性函数
pub fn put_char(c: u8) {
    UART.put_char(c);
}

pub fn put_str(s: &str) {
    UART.put_str(s);
}
