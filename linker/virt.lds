OUTPUT_ARCH("riscv")
ENTRY(_start)

MEM_SIZE = 256M;
STACK_SIZE = 1M;
UPAGES_SIZE = 64M;

PHDRS
{
    text PT_LOAD;
    rodata PT_LOAD;
    data PT_LOAD;
    bss PT_LOAD;
}

MEMORY
{
    RAM (wxa) : ORIGIN = 0x80000000, LENGTH = MEM_SIZE
}

SECTIONS
{
    PROVIDE(_memory_start = ORIGIN(RAM));
    PROVIDE(_memory_end = ORIGIN(RAM) + LENGTH(RAM));

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

    .data : ALIGN(8)
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

    PROVIDE(_kstack_start = _bss_end);
    PROVIDE(_kstack_end = _kstack_start + STACK_SIZE);

    . = ALIGN(4096);
    PROVIDE(_upages_start = .);
    PROVIDE(_upages_end = _upages_start + UPAGES_SIZE);
}
