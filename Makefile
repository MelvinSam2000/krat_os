objdump:
	riscv64-unknown-elf-objdump -DC target/riscv64gc-unknown-none-elf/debug/krat_os &> kratos.lst

gdb_start:
	riscv64-unknown-elf-gdb target/riscv64gc-unknown-none-elf/debug/krat_os