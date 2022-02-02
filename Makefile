build_rustc:
	rustc --target riscv64gc-unknown-none-elf src/lib.rs

objdump:
	riscv64-unknown-elf-objdump -ds target/riscv64gc-unknown-none-elf/debug/libkrat_os.rlib &> kratos.lst