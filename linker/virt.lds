OUTPUT_ARCH("riscv64gc")
ENTRY(entry)

MEM_SIZE = 256M;
KSTACK_SIZE = 16K;
KHEAP_SIZE = 16K;

PHDRS
{
    text PT_LOAD;
    rodata PT_LOAD;
    data PT_LOAD;
    bss PT_LOAD;
}

MEMORY
{
    RAM (wxa) : ORIGIN = 0x80200000, LENGTH = MEM_SIZE
}

SECTIONS
{
    PROVIDE(_memory_start = ORIGIN(RAM));

    .text : ALIGN(8)
    {
        PROVIDE(_text_start = .);
        KEEP(*(.text.init))
        *(.text .text.*)
        PROVIDE(_text_end = .);
    } >RAM AT>RAM :text

    .rodata : ALIGN(4096)
    {
        PROVIDE(_global_pointer = .);
        PROVIDE(_rodata_start = .);
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        PROVIDE(_rodata_end = .);
    } >RAM AT>RAM :rodata

    .data : ALIGN(4096)
    {
        PROVIDE(_data_start = .);
        *(.data .data.*)
        *(.sdata .sdata.*)
        PROVIDE(_data_end = .);
    } >RAM AT>RAM :data

    .bss : ALIGN(8)
    {
        PROVIDE(_bss_start = .);
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        PROVIDE(_bss_end = .);
    } >RAM AT>RAM :bss

    . = ALIGN(4096);
    PROVIDE(_kheap_start = .);
    PROVIDE(_kheap_end = _kheap_start + KHEAP_SIZE - 1);

    PROVIDE(_kstack_start = _kheap_end + 1);
    PROVIDE(_kstack_end = _kstack_start + KSTACK_SIZE - 1);

    PROVIDE(_umemory_start = _kstack_end + 1);
}
