TARGET_NAME = riscv_vm
all: 
	rm -f $(TARGET_NAME)
	cd riscv_vm_prj && \
	cargo build 
	cp -f riscv_vm_prj/target/debug/$(TARGET_NAME) $(TARGET_NAME)