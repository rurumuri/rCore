#!/bin/bash

# build user
cd user
cargo clean
make build
cd ..
# build os
cd os
cargo clean
cargo build --release
# obj-copy
rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/os \
    -O binary target/riscv64gc-unknown-none-elf/release/os.bin
# start qemu
qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios ../bootloader/rustsbi-qemu.bin \
    -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000