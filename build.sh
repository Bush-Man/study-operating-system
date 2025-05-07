#!/usr/bin/env bash
set -e  #Exit immediately if a command exits with a non-zero status
set -u  #Treat unset variables as an error when substituting
set -o pipefail  #The return value of a pipeline is the status of the last command to exit with a non-zero status
set -x  #Print commands and their arguments


# Compile assembly boot file
i686-elf-as src/boot.s -o boot.o

# Build Rust kernel
cargo build --release

# Link objects using the proper linker script
i686-elf-ld -T linker.ld -o kernel.bin boot.o target/i686-elf-unknown-none/release/libstudyos.a

# Create ISO
mkdir -p isodir/boot/grub
cp kernel.bin isodir/boot/
cat > isodir/boot/grub/grub.cfg << EOF
menuentry "STUDY OS" {
    multiboot /boot/kernel.bin
    boot
}
EOF

grub-mkrescue -o os.iso isodir
qemu-system-i386 -cdrom os.iso