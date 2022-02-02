.section .text.init
.global _start

_start:
    // run only hart 0
    csrr t0, mhartid
    bnez t0, harts_park

    // go to rust
    call kmain

harts_park:
    wfi
    j harts_park