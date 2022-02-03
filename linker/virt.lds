OUTPUT_ARCH("riscv")
ENTRY(_start)

SECTIONS
{ 
    . = 0x80000000;
    .text.init : { *(.text.init) }
    .text : { *(.text .text*) }
    .rodata : { *(.rodata .rodata*) }
    .data : { *(.data .data*) }
    .bss : { *(.bss .bss*) }
}
