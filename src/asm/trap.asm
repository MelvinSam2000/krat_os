.section .text
.global trap_vector
trap_vector:
    wfi
    mret