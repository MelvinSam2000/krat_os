.section .rodata

.global TEXT_START
TEXT_START: .dword _text_start

.global TEXT_END
TEXT_END: .dword _text_end

.global DATA_START
DATA_START: .dword _data_start

.global DATA_END
DATA_END: .dword _data_end

.global GLOBAL_POINTER
GLOBAL_POINTER: .dword _global_pointer

.global RODATA_START
RODATA_START: .dword _rodata_start

.global RODATA_END
RODATA_END: .dword _rodata_end

.global BSS_START
BSS_START: .dword _bss_start

.global BSS_END
BSS_END: .dword _bss_end

.global KSTACK_START
KSTACK_START: .dword _kstack_start

.global KSTACK_END
KSTACK_END: .dword _kstack_end

.global HEAP_START
HEAP_START: .dword _heap_start

.global HEAP_END
HEAP_END: .dword _heap_end
