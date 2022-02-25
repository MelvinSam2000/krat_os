# Krat OS

Krat OS is a small RISC-V OS kernel written in Rust. 

Currently targets the RV64GC ISA, and the QEMU virt machine. May support other RISC V boards in the future, as well as real physical devices.  

## Installation

Cargo, as well as qemu RISC-V are required. 

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

<<<<<<< Updated upstream
- [x] Replace physical page allocator with free lists
- [ ] Fix virtual page deallocation
=======
- [ ] Replace physical page allocator with free lists
>>>>>>> Stashed changes
- [ ] Implement fdt parser

### High priority

- [x] Fix timer interrupt
- [x] Fix PLIC 
- [x] Better logging
<<<<<<< Updated upstream
- [ ] Proper debug logging
=======
- [ ] Proper Debug logging
>>>>>>> Stashed changes
- [ ] Basic Syscalls
- [ ] Basic Scheduler

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
[MIT](https://choosealicense.com/licenses/mit/)