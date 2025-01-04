MODE ?= release

TARGET = target/riscv64gc-unknown-none-elf/$(MODE)/os
BIN = target/riscv64gc-unknown-none-elf/$(MODE)/os.bin

.PHONY: build run run-gdb fmt fmt-fix

build:
	if [ "$(MODE)" = "release" ]; then \
		cargo build --release; \
	else \
		cargo build; \
	fi
	rust-objcopy --strip-all $(TARGET) -O binary $(BIN)

run-gdb: build
	qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios ../bootloader/rustsbi-qemu.bin \
    -device loader,file=$(BIN),addr=0x80200000 \
    -s -S

run: build
	qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios ../bootloader/rustsbi-qemu.bin \
    -device loader,file=$(BIN),addr=0x80200000

fmt:
	cargo fmt -- --check

fmt-fix:
	cargo fmt

clean:
	cargo clean