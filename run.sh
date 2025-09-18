#!/usr/bin/env bash
set -e

# 1. Build UEFI target
cargo build --target x86_64-unknown-uefi

# 2. Copy EFI to ESP
cp target/x86_64-unknown-uefi/debug/bootloader.efi esp/EFI/BOOT/BOOTX64.EFI

# 3. Launch QEMU
qemu-system-x86_64 \
  -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.4m.fd \
  -drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.4m.fd \
  -drive format=raw,file=fat:rw:esp

