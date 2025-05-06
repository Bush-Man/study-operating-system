#!/usr/bin/env bash
export RUST_TARGET_PATH=$(pwd)

# Build the Rust kernel with Cargo
cargo build --release

# Copy the Cargo-built binary
cp target/i686-elf-unknown-none/release/studyos kernel.bin

# Create ISO structure
mkdir -p isodir/boot/grub
cp kernel.bin isodir/boot/

# Create GRUB config
cat > isodir/boot/grub/grub.cfg << EOF
menuentry "Rust OS" {
    multiboot /boot/kernel.bin
    boot
}
EOF

# Build ISO
grub-mkrescue -o os.iso isodir || exit 1

# Run in QEMU
qemu-system-i386 -cdrom os.iso