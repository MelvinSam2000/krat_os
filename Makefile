objdump:
	riscv64-unknown-elf-objdump -DSC target/riscv64gc-unknown-none-elf/debug/krat_os &> kratos.lst

gdb_start:
	riscv32-unknown-elf-gdb target/riscv64gc-unknown-none-elf/debug/krat_os