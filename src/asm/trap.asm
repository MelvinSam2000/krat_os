.section .text
.global trap_vector
.option norvc
trap_vector:

    // swap sscratch with x31
    csrrw   x31, sscratch, x31

    // store general purpose registers into trap frame (except x31)
    sd      x1, 8(x31)
    sd      x2, 16(x31)
    sd      x3, 24(x31)
    sd      x4, 32(x31)
    sd      x5, 40(x31)
    sd      x6, 48(x31)
    sd      x7, 56(x31)
    sd      x8, 64(x31)
    sd      x9, 72(x31)
    sd      x10, 80(x31)
    sd      x11, 88(x31)
    sd      x12, 96(x31)
    sd      x13, 104(x31)
    sd      x14, 112(x31)
    sd      x15, 120(x31)
    sd      x16, 128(x31)
    sd      x17, 136(x31)
    sd      x18, 144(x31)
    sd      x19, 152(x31)
    sd      x20, 160(x31)
    sd      x21, 168(x31)
    sd      x22, 176(x31)
    sd      x23, 184(x31)
    sd      x24, 192(x31)
    sd      x25, 200(x31)
    sd      x26, 208(x31)
    sd      x27, 216(x31)
    sd      x28, 224(x31)
    sd      x29, 232(x31)
    sd      x30, 240(x31)

    // store x31
    csrr    t0, sscratch
    sd      t0, 248(x31)

    // store fp registers into trap frame
    fsd     f0, 256(x31)
    fsd     f1, 264(x31)
    fsd     f2, 272(x31)
    fsd     f3, 280(x31)
    fsd     f4, 288(x31)
    fsd     f5, 296(x31)
    fsd     f6, 304(x31)
    fsd     f7, 312(x31)
    fsd     f8, 320(x31)
    fsd     f9, 328(x31)
    fsd     f10, 336(x31)
    fsd     f11, 344(x31)
    fsd     f12, 352(x31)
    fsd     f13, 360(x31)
    fsd     f14, 368(x31)
    fsd     f15, 376(x31)
    fsd     f16, 384(x31)
    fsd     f17, 392(x31)
    fsd     f18, 400(x31)
    fsd     f19, 408(x31)
    fsd     f20, 416(x31)
    fsd     f21, 424(x31)
    fsd     f22, 432(x31)
    fsd     f23, 440(x31)
    fsd     f24, 448(x31)
    fsd     f25, 456(x31)
    fsd     f26, 464(x31)
    fsd     f27, 472(x31)
    fsd     f28, 480(x31)
    fsd     f29, 488(x31)
    fsd     f30, 496(x31)
    fsd     f31, 504(x31)

    // store satp into trap frame
    csrr    t0, satp
    sd      t0, 512(x31)

    // get status registers for trap handler
    csrr    a0, sepc
    csrr    a1, stval
    csrr    a2, scause
    csrr    a3, sstatus

    // get trap frame
    mv      a4, x31

    // restore sscratch
    csrw    sscratch, x31

    // enter Rust trap_handler
    call    trap_handler

    // update return pc
    csrw    sepc, a0

    // swap sscratch with x31
    csrrw   x31, sscratch, x31

    // load general purpose registers from trap frame
    ld      x1, 8(x31)
    ld      x2, 16(x31)
    ld      x3, 24(x31)
    ld      x4, 32(x31)
    ld      x5, 40(x31)
    ld      x6, 48(x31)
    ld      x7, 56(x31)
    ld      x8, 64(x31)
    ld      x9, 72(x31)
    ld      x10, 80(x31)
    ld      x11, 88(x31)
    ld      x12, 96(x31)
    ld      x13, 104(x31)
    ld      x14, 112(x31)
    ld      x15, 120(x31)
    ld      x16, 128(x31)
    ld      x17, 136(x31)
    ld      x18, 144(x31)
    ld      x19, 152(x31)
    ld      x20, 160(x31)
    ld      x21, 168(x31)
    ld      x22, 176(x31)
    ld      x23, 184(x31)
    ld      x24, 192(x31)
    ld      x25, 200(x31)
    ld      x26, 208(x31)
    ld      x27, 216(x31)
    ld      x28, 224(x31)
    ld      x29, 232(x31)
    ld      x30, 240(x31)

    // load fp registers from trap frame
    fld     f0, 256(x31)
    fld     f1, 264(x31)
    fld     f2, 272(x31)
    fld     f3, 280(x31)
    fld     f4, 288(x31)
    fld     f5, 296(x31)
    fld     f6, 304(x31)
    fld     f7, 312(x31)
    fld     f8, 320(x31)
    fld     f9, 328(x31)
    fld     f10, 336(x31)
    fld     f11, 344(x31)
    fld     f12, 352(x31)
    fld     f13, 360(x31)
    fld     f14, 368(x31)
    fld     f15, 376(x31)
    fld     f16, 384(x31)
    fld     f17, 392(x31)
    fld     f18, 400(x31)
    fld     f19, 408(x31)
    fld     f20, 416(x31)
    fld     f21, 424(x31)
    fld     f22, 432(x31)
    fld     f23, 440(x31)
    fld     f24, 448(x31)
    fld     f25, 456(x31)
    fld     f26, 464(x31)
    fld     f27, 472(x31)
    fld     f28, 480(x31)
    fld     f29, 488(x31)
    fld     f30, 496(x31)
    fld     f31, 504(x31)

    // load x31
    ld      x31, 248(x31)

    sret