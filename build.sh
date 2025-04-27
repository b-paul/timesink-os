#!/bin/sh
# This script builds a bootable image!

# Converts an integer into a byte string.
int_to_bytes() {
    # TODO if I care I should bound check the number

    # https://stackoverflow.com/a/9955198
    printf "0: %.8x" $1 | sed -E 's/0: (..)(..)(..)(..)/0: \4\3\2\1/' | xxd -r -g0
}

# Build the bootloader, it will be stored at ./target/x86-unknown-bootloader/bootloader/bootloader as a raw binary.
cargo b --profile bootloader -p bootloader --target bootloader/x86-unknown-bootloader.json -Zbuild-std=core || exit

# Build the kernel, it will be stored at ./target/x86_64-unknown-none/debug/kernel
# TODO handle release mode
cargo b -p kernel --target x86_64-unknown-none || exit
# Copy the kernel into a binary for creating the image
# idk if it should be elf32-i386 feels wrong
objcopy -I elf32-i386 -O binary target/x86_64-unknown-none/debug/kernel target/kernel.bin

bootloader="./target/x86-unknown-bootloader/bootloader/bootloader"
kernel="./target/kernel.bin"

bootloader_size=$(wc -c $bootloader | cut -f1 -d' ')
bootloader_sectors=$((($bootloader_size - 1)/512))

kernel_size=$(wc -c $kernel | cut -f1 -d' ')
kernel_sectors=$((($kernel_size + 511)/512))

# Use dd to build an actual image
dd if=./target/x86-unknown-bootloader/bootloader/bootloader of=./target/image.bin bs=512
dd if=./target/kernel.bin of=./target/image.bin conv=notrunc bs=512 seek=128
# We write the amount of sectors into the partition table of the bootloader
int_to_bytes $bootloader_sectors | dd of=./target/image.bin conv=notrunc bs=1 seek=384
int_to_bytes 0x100000 | dd of=./target/image.bin conv=notrunc bs=1 seek=392 # idk random memory offset i might change it later
int_to_bytes $kernel_sectors | dd of=./target/image.bin conv=notrunc bs=1 seek=400

# Now you can run the image with `qemu-system-i386 -hda target/image.bin`!!
