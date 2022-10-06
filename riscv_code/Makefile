
ARCH32 = riscv32-linux
ARCH64 = riscv64-unknown-elf
ARCH = $(ARCH32)
CC32 = -march=rv32ima -mabi=ilp32
CC = $(CC32)

#all:
#	riscv32-unknown-elf-gcc $(SRC) -o $(SRC).elf -static
#	riscv32-unknown-elf-objdump -D $(SRC).elf > $(SRC).s
#	#riscv32-unknown-elf-strip -s $(SRC).elf
#	xxd $(SRC).elf > $(SRC).xxd
#	~/Tools/elf2hex/elf2hex --input $(SRC).elf --output $(SRC).hex --bit-width 32

customlink:
	$(ARCH)-gcc $(SRC) -o $(SRC).o -c -mcmodel=medany -ffreestanding $(CC)
	$(ARCH)-objdump -D $(SRC).o > $(SRC).o.obj
	$(ARCH)-as -c entry.s -o entry.o $(CC)
	$(ARCH)-ld -T linker.ld $(SRC).o entry.o -o $(SRC).elf 
	$(ARCH)-objdump -D $(SRC).elf > $(SRC).elf.obj
	xxd $(SRC).elf > $(SRC).xxd
	~/Tools/elf2hex/elf2hex --input $(SRC).elf --output $(SRC).hex --bit-width 32

clean:
	rm -f *.elf
	rm -f *.hex
	rm -f *.o
	rm -f *.xxd
	rm -f *.obj
