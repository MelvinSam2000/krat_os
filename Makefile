build_rustc:
	rustc --target riscv64gc-unknown-none-elf src/lib.rs

objdump:
	riscv64-unknown-elf-objdump -DSC target/riscv64gc-unknown-none-elf/debug/krat_os &> kratos.lst