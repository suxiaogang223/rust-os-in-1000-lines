// 第15章：磁盘I/O
// 实现磁盘驱动

use crate::common::types::ToString;
use crate::drivers::uart;

// 磁盘块大小
pub const BLOCK_SIZE: usize = 512;

// 磁盘操作结果
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DiskResult {
    Success,
    Error,
    Timeout,
    InvalidSector,
}

// 磁盘驱动结构
pub struct DiskDriver {
    pub base_address: usize,
    pub sector_count: usize,
    pub initialized: bool,
}

impl DiskDriver {
    pub const fn new() -> Self {
        Self {
            base_address: 0x10001000, // 假设磁盘控制器地址
            sector_count: 0,
            initialized: false,
        }
    }

    // 初始化磁盘驱动
    pub fn init(&mut self) -> DiskResult {
        uart::println("Initializing disk driver...");

        // 模拟磁盘初始化
        self.sector_count = 1024; // 假设有1024个扇区
        self.initialized = true;

        uart::print("Disk initialized: ");
        uart::print(&self.sector_count.to_string());
        uart::println(" sectors");

        DiskResult::Success
    }

    // 读取磁盘块
    pub fn read_block(&self, sector: usize, buffer: &mut [u8]) -> DiskResult {
        if !self.initialized {
            uart::println("Disk not initialized");
            return DiskResult::Error;
        }

        if sector >= self.sector_count {
            uart::println("Invalid sector number");
            return DiskResult::InvalidSector;
        }

        if buffer.len() < BLOCK_SIZE {
            uart::println("Buffer too small");
            return DiskResult::Error;
        }

        uart::print("Reading sector ");
        uart::print(&sector.to_string());
        uart::println("...");

        // 模拟磁盘读取
        unsafe {
            // 从磁盘控制器读取数据
            let disk_addr = self.base_address + (sector * BLOCK_SIZE);
            let disk_data = core::slice::from_raw_parts(disk_addr as *const u8, BLOCK_SIZE);
            buffer[..BLOCK_SIZE].copy_from_slice(disk_data);
        }

        uart::println("Sector read successfully");
        DiskResult::Success
    }

    // 写入磁盘块
    pub fn write_block(&self, sector: usize, buffer: &[u8]) -> DiskResult {
        if !self.initialized {
            uart::println("Disk not initialized");
            return DiskResult::Error;
        }

        if sector >= self.sector_count {
            uart::println("Invalid sector number");
            return DiskResult::InvalidSector;
        }

        if buffer.len() < BLOCK_SIZE {
            uart::println("Buffer too small");
            return DiskResult::Error;
        }

        uart::print("Writing sector ");
        uart::print(&sector.to_string());
        uart::println("...");

        // 模拟磁盘写入
        unsafe {
            // 向磁盘控制器写入数据
            let disk_addr = self.base_address + (sector * BLOCK_SIZE);
            let disk_data = core::slice::from_raw_parts_mut(disk_addr as *mut u8, BLOCK_SIZE);
            disk_data.copy_from_slice(&buffer[..BLOCK_SIZE]);
        }

        uart::println("Sector written successfully");
        DiskResult::Success
    }

    // 获取磁盘信息
    pub fn get_info(&self) -> (usize, usize) {
        (self.sector_count, BLOCK_SIZE)
    }
}

// 全局磁盘驱动
pub static mut DISK: DiskDriver = DiskDriver::new();

// 全局函数
pub fn init_disk() -> DiskResult {
    unsafe { DISK.init() }
}

pub fn read_disk_block(sector: usize, buffer: &mut [u8]) -> DiskResult {
    unsafe { DISK.read_block(sector, buffer) }
}

pub fn write_disk_block(sector: usize, buffer: &[u8]) -> DiskResult {
    unsafe { DISK.write_block(sector, buffer) }
}

pub fn get_disk_info() -> (usize, usize) {
    unsafe { DISK.get_info() }
}
