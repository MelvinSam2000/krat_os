# Krat OS

Krat OS is a small RISC-V OS kernel written in Rust. 

Currently targets the RV64GC ISA, and the QEMU virt machine. May support other RISC V boards in the future, as well as real physical devices.  

## Installation

Cargo, as well as Qemu RISC-V are required. 

Will update this later...


## Usage

Compile kernel with cargo

```bash
cargo build
```

Run OS with qemu
```bash
cargo run
```

## TODO

### Low priority

- [ ] Virtio and file system
- [ ] Fix virtual page deallocation
- [ ] Implement fdt parser
- [ ] Refactor code
- [ ] Elf parser
- [ ] Userspace library
- [ ] Userspace programs
- [ ] Decide monolith or microkernel
- [ ] Enable multicore

### High priority

- [ ] Enable 32 bit compatibility
- [ ] Replace trap handler memory scheme (fix release mode)
- [ ] Refactor physical page allocator
- [ ] Add unit tests
 
## License
[MIT](https://choosealicense.com/licenses/mit/)
