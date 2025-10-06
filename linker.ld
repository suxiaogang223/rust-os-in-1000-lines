/* Rust OS in 1000 Lines - Linker Script for RISC-V64 */

ENTRY(_start)

MEMORY
{
    /* QEMU virt machine memory layout */
    RAM : ORIGIN = 0x80000000, LENGTH = 128M
}

SECTIONS
{
    /* 内核入口点 */
    . = 0x80000000;
    
    .text : {
        *(.text.start)
        *(.text*)
        . = ALIGN(4);
    } > RAM
    
    .rodata : {
        *(.rodata*)
        . = ALIGN(4);
    } > RAM
    
    .data : {
        *(.data*)
        . = ALIGN(4);
    } > RAM
    
    .bss : {
        *(.bss*)
        *(COMMON)
        . = ALIGN(4);
    } > RAM
    
    /* 栈空间 */
    . = ALIGN(16);
    _stack_start = .;
    . += 0x10000;  /* 64KB 栈空间 */
    _stack_end = .;
    
    /* 堆空间 */
    . = ALIGN(4);
    _heap_start = .;
    . += 0x100000; /* 1MB 堆空间 */
    _heap_end = .;
    
    /* 符号定义 */
    _end = .;
    
    /* 丢弃不需要的段 */
    /DISCARD/ : {
        *(.comment)
        *(.note*)
        *(.debug*)
    }
}
