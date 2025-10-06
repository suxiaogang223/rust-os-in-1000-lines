// 第16章：文件系统
// 实现简单的FAT文件系统

use crate::common::types::ToString;
use crate::drivers::disk::*;
use crate::drivers::uart;

// FAT文件系统常量
const FAT_SIGNATURE: u16 = 0xAA55;
const BOOT_SECTOR_SIZE: usize = 512;
const FAT_ENTRY_SIZE: usize = 2; // FAT16
const ROOT_DIR_ENTRIES: usize = 512;
const DIR_ENTRY_SIZE: usize = 32;

// 文件属性
const ATTR_READ_ONLY: u8 = 0x01;
const ATTR_HIDDEN: u8 = 0x02;
const ATTR_SYSTEM: u8 = 0x04;
const ATTR_VOLUME_ID: u8 = 0x08;
const ATTR_DIRECTORY: u8 = 0x10;
const ATTR_ARCHIVE: u8 = 0x20;

// 文件类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileType {
    Regular,
    Directory,
    Unknown,
}

// 文件信息
#[derive(Debug, Clone, Copy)]
pub struct FileInfo {
    pub name: [u8; 11],
    pub attributes: u8,
    pub size: u32,
    pub first_cluster: u16,
    pub file_type: FileType,
}

impl FileInfo {
    pub fn new() -> Self {
        Self {
            name: [0; 11],
            attributes: 0,
            size: 0,
            first_cluster: 0,
            file_type: FileType::Unknown,
        }
    }

    pub fn is_directory(&self) -> bool {
        (self.attributes & ATTR_DIRECTORY) != 0
    }

    pub fn is_regular_file(&self) -> bool {
        !self.is_directory() && (self.attributes & ATTR_VOLUME_ID) == 0
    }
}

// 文件句柄
pub struct FileHandle {
    pub file_info: FileInfo,
    pub current_position: usize,
    pub open: bool,
}

impl FileHandle {
    pub fn new() -> Self {
        Self {
            file_info: FileInfo::new(),
            current_position: 0,
            open: false,
        }
    }
}

// FAT文件系统
pub struct FatFileSystem {
    pub boot_sector: [u8; BOOT_SECTOR_SIZE],
    pub fat_table: [u16; 1024], // 简化的FAT表
    pub root_directory: [FileInfo; ROOT_DIR_ENTRIES],
    pub initialized: bool,
    pub total_sectors: usize,
    pub sectors_per_cluster: usize,
    pub fat_sectors: usize,
    pub root_dir_sectors: usize,
    pub data_sectors: usize,
}

impl FatFileSystem {
    pub const fn new() -> Self {
        Self {
            boot_sector: [0; BOOT_SECTOR_SIZE],
            fat_table: [0; 1024],
            root_directory: [FileInfo {
                name: [0; 11],
                attributes: 0,
                size: 0,
                first_cluster: 0,
                file_type: FileType::Unknown,
            }; ROOT_DIR_ENTRIES],
            initialized: false,
            total_sectors: 0,
            sectors_per_cluster: 0,
            fat_sectors: 0,
            root_dir_sectors: 0,
            data_sectors: 0,
        }
    }

    // 初始化文件系统
    pub fn init(&mut self) -> bool {
        uart::println("Initializing FAT file system...");

        // 读取引导扇区
        if read_disk_block(0, &mut self.boot_sector) != DiskResult::Success {
            uart::println("Failed to read boot sector");
            return false;
        }

        // 解析引导扇区
        if !self.parse_boot_sector() {
            uart::println("Invalid boot sector");
            return false;
        }

        // 读取FAT表
        if !self.load_fat_table() {
            uart::println("Failed to load FAT table");
            return false;
        }

        // 读取根目录
        if !self.load_root_directory() {
            uart::println("Failed to load root directory");
            return false;
        }

        self.initialized = true;
        uart::println("FAT file system initialized successfully");
        true
    }

    // 解析引导扇区
    fn parse_boot_sector(&mut self) -> bool {
        // 检查FAT签名
        let signature = u16::from_le_bytes([self.boot_sector[510], self.boot_sector[511]]);
        if signature != FAT_SIGNATURE {
            uart::println("Invalid FAT signature");
            return false;
        }

        // 解析文件系统参数
        self.total_sectors =
            u16::from_le_bytes([self.boot_sector[19], self.boot_sector[20]]) as usize;
        self.sectors_per_cluster = self.boot_sector[13] as usize;
        self.fat_sectors =
            u16::from_le_bytes([self.boot_sector[22], self.boot_sector[23]]) as usize;
        self.root_dir_sectors = ((ROOT_DIR_ENTRIES * DIR_ENTRY_SIZE) + BLOCK_SIZE - 1) / BLOCK_SIZE;
        self.data_sectors = self.total_sectors - 1 - self.fat_sectors - self.root_dir_sectors;

        uart::print("Total sectors: ");
        uart::println(&self.total_sectors.to_string());
        uart::print("Sectors per cluster: ");
        uart::println(&self.sectors_per_cluster.to_string());

        true
    }

    // 加载FAT表
    fn load_fat_table(&mut self) -> bool {
        uart::println("Loading FAT table...");

        // 简化实现，不实际读取FAT表
        for i in 0..self.fat_table.len() {
            self.fat_table[i] = 0xFFFF; // 标记为未使用
        }

        uart::println("FAT table loaded");
        true
    }

    // 加载根目录
    fn load_root_directory(&mut self) -> bool {
        uart::println("Loading root directory...");

        // 简化实现，创建一些示例文件
        let mut file_info = FileInfo::new();
        file_info.name = [
            b'H', b'E', b'L', b'L', b'O', b' ', b' ', b' ', b'T', b'X', b'T',
        ];
        file_info.attributes = ATTR_ARCHIVE;
        file_info.size = 13;
        file_info.first_cluster = 2;
        file_info.file_type = FileType::Regular;
        self.root_directory[0] = file_info;

        let mut dir_info = FileInfo::new();
        dir_info.name = [
            b'T', b'E', b'S', b'T', b' ', b' ', b' ', b' ', b'D', b'I', b'R',
        ];
        dir_info.attributes = ATTR_DIRECTORY;
        dir_info.size = 0;
        dir_info.first_cluster = 3;
        dir_info.file_type = FileType::Directory;
        self.root_directory[1] = dir_info;

        uart::println("Root directory loaded");
        true
    }

    // 列出根目录
    pub fn list_directory(&self) {
        uart::println("Directory listing:");

        for (i, file_info) in self.root_directory.iter().enumerate() {
            if file_info.name[0] != 0 && file_info.name[0] != 0xE5 {
                uart::print("  ");
                uart::print(&i.to_string());
                uart::print(": ");

                // 打印文件名
                let name_str = core::str::from_utf8(&file_info.name[..8]).unwrap_or("???");
                uart::print(name_str);

                if file_info.name[8] != b' ' {
                    uart::put_char(b'.');
                    let ext_str = core::str::from_utf8(&file_info.name[8..11]).unwrap_or("???");
                    uart::print(ext_str);
                }

                uart::print(" (");
                uart::print(&file_info.size.to_string());
                uart::print(" bytes, ");

                if file_info.is_directory() {
                    uart::print("DIR");
                } else {
                    uart::print("FILE");
                }

                uart::println(")");
            }
        }
    }

    // 打开文件
    pub fn open_file(&self, filename: &str) -> Option<FileHandle> {
        uart::print("Opening file: ");
        uart::println(filename);

        // 在根目录中查找文件
        for file_info in &self.root_directory {
            if file_info.name[0] != 0 && file_info.name[0] != 0xE5 {
                // 比较文件名
                let name_str = core::str::from_utf8(&file_info.name[..8]).unwrap_or("");
                if name_str == filename {
                    let mut handle = FileHandle::new();
                    handle.file_info = *file_info;
                    handle.open = true;
                    uart::println("File opened successfully");
                    return Some(handle);
                }
            }
        }

        uart::println("File not found");
        None
    }

    // 读取文件
    pub fn read_file(&self, handle: &mut FileHandle, buffer: &mut [u8]) -> usize {
        if !handle.open {
            return 0;
        }

        let remaining = handle.file_info.size as usize - handle.current_position;
        let to_read = if buffer.len() < remaining {
            buffer.len()
        } else {
            remaining
        };

        if to_read == 0 {
            return 0;
        }

        uart::print("Reading ");
        uart::print(&to_read.to_string());
        uart::print(" bytes from file...");

        // 简化实现，不实际读取文件内容
        for i in 0..to_read {
            buffer[i] = b'A' + (i % 26) as u8;
        }

        handle.current_position += to_read;
        uart::println(" done");

        to_read
    }

    // 关闭文件
    pub fn close_file(&mut self, handle: &mut FileHandle) {
        if handle.open {
            handle.open = false;
            uart::println("File closed");
        }
    }
}

// 全局文件系统
pub static mut FS: FatFileSystem = FatFileSystem::new();

// 全局函数
pub fn init_filesystem() -> bool {
    unsafe { FS.init() }
}

pub fn list_files() {
    unsafe {
        FS.list_directory();
    }
}

pub fn open_file(filename: &str) -> Option<FileHandle> {
    unsafe { FS.open_file(filename) }
}

pub fn read_file(handle: &mut FileHandle, buffer: &mut [u8]) -> usize {
    unsafe { FS.read_file(handle, buffer) }
}

pub fn close_file(handle: &mut FileHandle) {
    unsafe {
        FS.close_file(handle);
    }
}
