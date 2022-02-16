.section .text
.global trap_vector
.option norvc
trap_vector:

    // get status registers for trap handler
    csrr    a0, sepc
    csrr    a1, stval
    csrr    a2, scause
    csrr    a3, sstatus
    
    // get trap frame (?)
    add     a4, zero, zero

    call    trap_handler  

    sret