# Building
TARGET := riscv64gc-unknown-none-elf
MODE := debug
KERNEL_ELF := target/$(TARGET)/$(MODE)/os
KERNEL_BIN := $(KERNEL_ELF).bin
KERNEL_ASM := $(KERNEL_ELF).asm


CPUS := 1
MEM := 128M

CC := riscv64-linux-gnu-gcc
LD := riscv64-linux-gnu-ld
GDB := riscv64-linux-gun-gdb
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

QEMU = qemu-system-riscv64

#board
BOOTLOADER := ../bootloader/rustsbi-qemu.bin

# 
KERNEL_ENTRY_PA := 0x80200000

# 

QEMUOPTS = -machine virt -nographic -m $(MEM) -smp $(CPUS)
QEMUOPTS += -bios $(BOOTLOADER)
QEMUOPTS += -device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA)


build: env $(KERNEL_BIN)

$(KERNEL_BIN): kernel
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@

env:
	(rustup target list | grep "riscv64gc-unknown-none-elf (installed)") || rustup target add $(TARGET)
	rustup component add rust-src
	rustup component add llvm-tools-preview
	cargo install cargo-binutils

kernel:
	@cargo build

asm: kernel
	@$(OBJDUMP) $(KERNEL_ELF) -S > $(KERNEL_ASM)

run: build
	@$(QEMU) $(QEMUOPTS)

clean:
	@cargo clean

.PHONY: run clean build env kernel asm debug
