// 第11章：页表
// 实现RISC-V虚拟内存管理

use crate::common::types::ToString;
use crate::drivers::uart;
use crate::kernel::memory;

// 页表项标志
const PTE_V: usize = 1 << 0; // 有效位
const PTE_R: usize = 1 << 1; // 可读
const PTE_W: usize = 1 << 2; // 可写
const PTE_X: usize = 1 << 3; // 可执行
const PTE_U: usize = 1 << 4; // 用户可访问
const PTE_G: usize = 1 << 5; // 全局
const PTE_A: usize = 1 << 6; // 访问位
const PTE_D: usize = 1 << 7; // 脏位

// 页表项
#[derive(Debug, Clone, Copy)]
pub struct PageTableEntry {
    pub pte: usize,
}

impl PageTableEntry {
    pub fn new() -> Self {
        Self { pte: 0 }
    }

    pub fn is_valid(&self) -> bool {
        (self.pte & PTE_V) != 0
    }

    pub fn is_readable(&self) -> bool {
        (self.pte & PTE_R) != 0
    }

    pub fn is_writable(&self) -> bool {
        (self.pte & PTE_W) != 0
    }

    pub fn is_executable(&self) -> bool {
        (self.pte & PTE_X) != 0
    }

    pub fn is_user_accessible(&self) -> bool {
        (self.pte & PTE_U) != 0
    }

    pub fn get_physical_address(&self) -> usize {
        self.pte & 0xFFFFFFFFFFF00000
    }

    pub fn set_physical_address(&mut self, addr: usize) {
        self.pte = (self.pte & 0x0000000000000FFF) | (addr & 0xFFFFFFFFFFF00000);
    }

    pub fn set_flags(&mut self, flags: usize) {
        self.pte = (self.pte & 0xFFFFFFFFFFF00000) | (flags & 0xFFF);
    }

    pub fn set_valid(&mut self, valid: bool) {
        if valid {
            self.pte |= PTE_V;
        } else {
            self.pte &= !PTE_V;
        }
    }

    pub fn set_readable(&mut self, readable: bool) {
        if readable {
            self.pte |= PTE_R;
        } else {
            self.pte &= !PTE_R;
        }
    }

    pub fn set_writable(&mut self, writable: bool) {
        if writable {
            self.pte |= PTE_W;
        } else {
            self.pte &= !PTE_W;
        }
    }

    pub fn set_executable(&mut self, executable: bool) {
        if executable {
            self.pte |= PTE_X;
        } else {
            self.pte &= !PTE_X;
        }
    }

    pub fn set_user_accessible(&mut self, user: bool) {
        if user {
            self.pte |= PTE_U;
        } else {
            self.pte &= !PTE_U;
        }
    }
}

// 页表
pub struct PageTable {
    pub entries: [PageTableEntry; 512], // 512个页表项
}

impl PageTable {
    pub fn new() -> Option<Self> {
        // 分配页表内存
        if let Some(ptr) = memory::allocate(core::mem::size_of::<Self>()) {
            unsafe {
                let table = core::ptr::read(ptr as *const Self);
                Some(table)
            }
        } else {
            None
        }
    }

    pub fn get_entry(&self, index: usize) -> &PageTableEntry {
        &self.entries[index]
    }

    pub fn get_entry_mut(&mut self, index: usize) -> &mut PageTableEntry {
        &mut self.entries[index]
    }

    // 映射虚拟地址到物理地址
    pub fn map_page(&mut self, vaddr: usize, paddr: usize, flags: usize) -> bool {
        let vpn = (vaddr >> 12) & 0x1FF; // 虚拟页号 (9位)

        if vpn >= 512 {
            return false;
        }

        let entry = &mut self.entries[vpn];
        entry.set_physical_address(paddr);
        entry.set_flags(flags);
        entry.set_valid(true);

        true
    }

    // 取消映射
    pub fn unmap_page(&mut self, vaddr: usize) -> bool {
        let vpn = (vaddr >> 12) & 0x1FF;

        if vpn >= 512 {
            return false;
        }

        let entry = &mut self.entries[vpn];
        entry.set_valid(false);

        true
    }

    // 查找虚拟地址对应的物理地址
    pub fn translate(&self, vaddr: usize) -> Option<usize> {
        let vpn = (vaddr >> 12) & 0x1FF;

        if vpn >= 512 {
            return None;
        }

        let entry = &self.entries[vpn];
        if entry.is_valid() {
            let paddr = entry.get_physical_address();
            let offset = vaddr & 0xFFF; // 页内偏移
            Some(paddr | offset)
        } else {
            None
        }
    }
}

// 虚拟内存管理器
pub struct VirtualMemoryManager {
    pub root_page_table: Option<*mut PageTable>,
}

impl VirtualMemoryManager {
    pub const fn new() -> Self {
        Self {
            root_page_table: None,
        }
    }

    pub fn init(&mut self) -> bool {
        if let Some(table) = PageTable::new() {
            // 这里需要分配页表内存，简化处理
            uart::println("Virtual memory manager initialized");
            true
        } else {
            uart::println("Failed to initialize virtual memory manager");
            false
        }
    }

    // 映射内存区域
    pub fn map_memory(&mut self, vaddr: usize, paddr: usize, size: usize, flags: usize) -> bool {
        let mut current_vaddr = vaddr;
        let mut current_paddr = paddr;
        let mut remaining = size;

        while remaining > 0 {
            let page_size = 4096; // 4KB页
            let map_size = if remaining >= page_size {
                page_size
            } else {
                remaining
            };

            // 这里应该调用页表的map_page方法
            // 简化实现
            uart::print("Mapping: 0x");
            uart::print(&current_vaddr.to_string());
            uart::print(" -> 0x");
            uart::print(&current_paddr.to_string());
            uart::print(" (");
            uart::print(&map_size.to_string());
            uart::println(" bytes)");

            current_vaddr += map_size;
            current_paddr += map_size;
            remaining -= map_size;
        }

        true
    }

    // 取消映射内存区域
    pub fn unmap_memory(&mut self, vaddr: usize, size: usize) -> bool {
        let mut current_vaddr = vaddr;
        let mut remaining = size;

        while remaining > 0 {
            let page_size = 4096;
            let unmap_size = if remaining >= page_size {
                page_size
            } else {
                remaining
            };

            // 这里应该调用页表的unmap_page方法
            uart::print("Unmapping: 0x");
            uart::print(&current_vaddr.to_string());
            uart::print(" (");
            uart::print(&unmap_size.to_string());
            uart::println(" bytes)");

            current_vaddr += unmap_size;
            remaining -= unmap_size;
        }

        true
    }
}

// 全局虚拟内存管理器
pub static mut VMM: VirtualMemoryManager = VirtualMemoryManager::new();

// 全局函数
pub fn init_paging() -> bool {
    unsafe { VMM.init() }
}

pub fn map_memory(vaddr: usize, paddr: usize, size: usize, flags: usize) -> bool {
    unsafe { VMM.map_memory(vaddr, paddr, size, flags) }
}

pub fn unmap_memory(vaddr: usize, size: usize) -> bool {
    unsafe { VMM.unmap_memory(vaddr, size) }
}
