// KERNEL EXEC SECTIONS

.section .rodata

.global TEXT_START
TEXT_START: .dword _text_start

.global TEXT_END
TEXT_END: .dword _text_end

.global DATA_START
DATA_START: .dword _data_start

.global DATA_END
DATA_END: .dword _data_end

.global RODATA_START
RODATA_START: .dword _rodata_start

.global RODATA_END
RODATA_END: .dword _rodata_end

.global BSS_START
BSS_START: .dword _bss_start

.global BSS_END
BSS_END: .dword _bss_end

// KERNEL RW MEMORY SECTIONS

.global KHEAP_START
KHEAP_START: .dword _kheap_start

.global KHEAP_END
KHEAP_END: .dword _kheap_end

.global KSTACK_START
KSTACK_START: .dword _kstack_start

.global KSTACK_END
KSTACK_END: .dword _kstack_end

.global UMEMORY_START
UMEMORY_START: .dword _umemory_start

.global UMEMORY_END
UMEMORY_END: .dword _umemory_end

// Other symbols

.global GLOBAL_POINTER
GLOBAL_POINTER: .dword _global_pointer

.global TRAMP_VECTOR
TRAMP_VECTOR: .dword _tramp_vector

.global TRAMP_FRAME
TRAMP_FRAME: .dword _tramp_frame