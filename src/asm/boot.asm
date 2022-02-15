.section .text.init
.global _start

_start:

    // set gp pointer
.option push
.option norelax
    la      gp, _global_pointer
.option pop

    // set sp pointer
    la      sp, _kstack_end

    // clear bss section
    call    clear_bss
    
    // set trap vector csr
    la		t0, trap_vector
    csrw	stvec, t0

    // go to Rust (kmain)
    call    kmain

clear_bss:
    la      t0, _bss_start
    la      t1, _bss_end
1:
    sd      zero, (t0)
    addi    t0, t0, 8
    bne     t0, t1, 1b
    ret