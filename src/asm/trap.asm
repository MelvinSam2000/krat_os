.section .text
.global trap_vector
trap_vector:
    
    // get hart id (?)
    add     a0, zero, zero  

    // get status registers for trap handler
    csrr    a1, sepc
    csrr    a2, stval
    csrr    a3, scause
    csrr    a4, sstatus
    
    // get trap frame (?)
    add     a5, zero, zero

    call    trap_handler  

    sret