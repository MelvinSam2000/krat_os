objdump:
	riscv64-unknown-elf-objdump -DC target/riscv64gc-unknown-none-elf/debug/krat_os &> kratos.lst

gdb_start:
	riscv64-unknown-elf-gdb target/riscv64gc-unknown-none-elf/debug/krat_os \
		-ex 'target remote :1234'

BOARD=spike
dts:
	qemu-system-riscv64 -cpu rv64 -smp 1 -m 256M -machine dumpdtb=${BOARD}.dtb
	dtc -I dtb -O dts -o ${BOARD}.dts ${BOARD}.dtb
	rm ${BOARD}.dtb