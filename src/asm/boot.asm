.section .text.init
.global _start

_start:
    // run only hart 0
    csrr    t0, mhartid
    bnez    t0, park

    // set gp pointer
    la      gp, _global_pointer

    // set sp pointer
    la      sp, _kstack_end

    // clear bss section
    call    clear_bss
    
    // set cpu to machine mode
    li		t0, (0b11 << 11) | (1 << 7) | (1 << 3)
    csrw	mstatus, t0

    // set machine expection to kmain
    la		t1, kmain
    csrw	mepc, t1
    
    // set trap vector csr
    la		t2, trap_vector
    csrw	mtvec, t2
    
    // enable interrupts
    li		t3, (1 << 3) | (1 << 7) | (1 << 11)
    csrw	mie, t3

    // go to Rust (kmain) 
    la		ra, park
    mret

park:
    wfi
    j       park

clear_bss:
    la      t0, _bss_start
    la      t1, _bss_end
1:
    sd      zero, (t0)
    addi    t0, t0, 8
    bne     t0, t1, 1b
    ret