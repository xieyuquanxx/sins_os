TARGET := riscv64gc-unknown-none-elf
MODE := release
KERNEL_ELF := target/$(TARGET)/$(MODE)/sins_os
KERNEL_BIN := $(KERNEL_ELF).bin
# kernel entry
KERNEL_ENTRY := 0x80200000


BOOTLOADER := ../bootloader/rustsbi-qemu.bin

# Binutils
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

all: clean run

build: $(KERNEL_BIN)

run: build
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY)

debug: build
	@tmux new -s debug -d \
		"qemu-system-riscv64 -machine virt -nographic -bios $(BOOTLOADER) -device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY) -s -S" && \
		tmux split-window -h "riscv64-unknown-elf-gdb -ex 'file $(KERNEL_ELF)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'" && \
		tmux -2 attach-session -d

$(KERNEL_BIN): kernel user
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@
	@python3 tools/link_app.py

kernel:
	@cargo build --$(MODE)

# 编译用户应用程序
user:
	cd /Users/xieyuquan/os/user && make build

clean:
	@cargo clean

.PHONY: kernel clean build run debug user