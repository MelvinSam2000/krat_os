.section .text
.global trap_vector
trap_vector:
    csrr    t0, scause
    wfi
    sret