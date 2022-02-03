.section .text.init
.global _start

_start:
    // run only hart 0
    csrr t0, mhartid
    bnez t0, park

    // set gp pointer
    li gp, 0x82000000

    # Control registers, set the stack, mstatus, mepc,
    # and mtvec to return to the main function.
    # li		t5, 0xffff;
    # csrw	medeleg, t5
    # csrw	mideleg, t5
    li		sp, 0x81000000
    # We use mret here so that the mstatus register
    # is properly updated.
    # machine mode
    li		t0, (0b11 << 11) | (1 << 7) | (1 << 3)
    csrw	mstatus, t0
    la		t1, kmain
    csrw	mepc, t1
    la		t2, trap_vector
    csrw	mtvec, t2
    li		t3, (1 << 3) | (1 << 7) | (1 << 11)
    csrw	mie, t3
    la		ra, park
    mret

park:
    wfi
    j park