// 第09章：内存分配
// 实现简单的内存分配器

use crate::common::types::ToString;
use crate::drivers::uart;

// 内存页大小 (4KB)
pub const PAGE_SIZE: usize = 4096;

// 内存分配器结构
pub struct Allocator {
    heap_start: usize,
    heap_end: usize,
    next_free: usize,
}

impl Allocator {
    pub const fn new() -> Self {
        Self {
            heap_start: 0,
            heap_end: 0,
            next_free: 0,
        }
    }

    // 初始化内存分配器
    pub fn init(&mut self, start: usize, size: usize) {
        self.heap_start = start;
        self.heap_end = start + size;
        self.next_free = start;
        uart::print("Memory allocator initialized: 0x");
        uart::print(&start.to_string());
        uart::print(" - 0x");
        uart::println(&self.heap_end.to_string());
    }

    // 分配内存
    pub fn allocate(&mut self, size: usize) -> Option<*mut u8> {
        // 对齐到8字节边界
        let aligned_size = (size + 7) & !7;

        if self.next_free + aligned_size > self.heap_end {
            return None;
        }

        let ptr = self.next_free;
        self.next_free += aligned_size;

        Some(ptr as *mut u8)
    }

    // 释放内存 (简单实现，不实际释放)
    pub fn deallocate(&mut self, _ptr: *mut u8, _size: usize) {
        // 简化实现，不实际释放内存
    }

    // 获取可用内存大小
    pub fn available(&self) -> usize {
        self.heap_end - self.next_free
    }
}

// 全局内存分配器
pub static mut ALLOCATOR: Allocator = Allocator::new();

// 全局分配函数
pub fn init_memory(start: usize, size: usize) {
    unsafe {
        ALLOCATOR.init(start, size);
    }
}

pub fn allocate(size: usize) -> Option<*mut u8> {
    unsafe { ALLOCATOR.allocate(size) }
}

pub fn deallocate(ptr: *mut u8, size: usize) {
    unsafe {
        ALLOCATOR.deallocate(ptr, size);
    }
}

// 简单的字符串分配
pub fn allocate_string(s: &str) -> Option<*mut u8> {
    let size = s.len() + 1;
    if let Some(ptr) = allocate(size) {
        unsafe {
            let slice = core::slice::from_raw_parts_mut(ptr, size);
            slice[..s.len()].copy_from_slice(s.as_bytes());
            slice[s.len()] = 0; // null terminator
        }
        Some(ptr)
    } else {
        None
    }
}

// 内存复制
pub unsafe fn memcpy(dst: *mut u8, src: *const u8, size: usize) {
    let dst_slice = core::slice::from_raw_parts_mut(dst, size);
    let src_slice = core::slice::from_raw_parts(src, size);
    dst_slice.copy_from_slice(src_slice);
}

// 内存设置
pub unsafe fn memset(ptr: *mut u8, value: u8, size: usize) {
    let slice = core::slice::from_raw_parts_mut(ptr, size);
    slice.fill(value);
}
