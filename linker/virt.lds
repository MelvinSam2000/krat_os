OUTPUT_ARCH("riscv")
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
    PROVIDE(_memory_end = 0x8fff0000);

    /* ^ quick fix, cant access > 0x90000000
    PROVIDE(_memory_end = ORIGIN(RAM) + LENGTH(RAM));
    */
    .text : ALIGN(8)
    {
        PROVIDE(_text_start = .);
        *(.text.init)
        *(.text .text.*)
        PROVIDE(_text_end = .);
    } >RAM AT>RAM :text

    .rodata : ALIGN(8)
    {
        PROVIDE(_global_pointer = .);
        PROVIDE(_rodata_start = .);
        *(.rodata .rodata.*)
        PROVIDE(_rodata_end = .);
    } >RAM AT>RAM :rodata

    .data : ALIGN(4096)
    {
        PROVIDE(_data_start = .);
        *(.data .data.*)
        PROVIDE(_data_end = .);
    } >RAM AT>RAM :data

    .bss : ALIGN(8)
    {
        PROVIDE(_bss_start = .);
        *(.bss .bss.*)
        PROVIDE(_bss_end = .);
    } >RAM AT>RAM :bss

    . = ALIGN(4096);
    PROVIDE(_kstack_start = .);
    PROVIDE(_kstack_end = _kstack_start + KSTACK_SIZE - 1);

    PROVIDE(_kheap_start = _kstack_end + 1);
    PROVIDE(_kheap_end = _kheap_start + KHEAP_SIZE - 1);

    PROVIDE(_umemory_start = _kheap_end + 1);
    PROVIDE(_umemory_end = _memory_end - 1);
}
