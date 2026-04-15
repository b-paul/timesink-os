#!/bin/sh
# This script builds a bootable image!

# Converts an integer into a byte string.
int_to_bytes() {
    # TODO if I care I should bound check the number

    # https://stackoverflow.com/a/9955198
    printf "0: %.8x" $1 | sed -E 's/0: (..)(..)(..)(..)/0: \4\3\2\1/' | xxd -r -g0
}

KERNEL_DIR=$(dirname $1)
KERNEL_ELF="$KERNEL_DIR/kernel"
KERNEL_BIN="../target/kernel.bin"

BOOTLOADER_DIR="../target/mbr-32bit-bootloader/debug"
BOOTLOADER_ELF="$BOOTLOADER_DIR/mbr-32bit"
BOOTLOADER_BIN="$BOOTLOADER_DIR/mbr-32bit.bin"

IMAGE_BIN="$KERNEL_DIR/image.bin"

# TODO make this better
# Build the bootloader
cargo b -p mbr-32bit --target ../bootloaders/mbr-32bit/mbr-32bit-bootloader.json -Zjson-target-spec -Zbuild-std=core || exit
objcopy -I elf32-i386 -O binary $BOOTLOADER_ELF $BOOTLOADER_BIN || exit

# Build the kernel
objcopy -I elf32-i386 -O binary $KERNEL_ELF $KERNEL_BIN || exit


BOOTLOADER_SIZE=$(wc -c $BOOTLOADER_BIN | cut -f1 -d' ')
BOOTLOADER_SECTORS=$((($BOOTLOADER_SIZE - 1)/512))

KERNEL_SIZE=$(wc -c $KERNEL_BIN | cut -f1 -d' ')
KERNEL_SECTORS=$((($KERNEL_SIZE + 511)/512))

# Use dd to build an actual image
dd if=$BOOTLOADER_BIN of=$IMAGE_BIN bs=512
dd if=$KERNEL_BIN of=$IMAGE_BIN conv=notrunc bs=512 seek=128
# We write the amount of sectors into the partition table of the bootloader
int_to_bytes $BOOTLOADER_SECTORS | dd of=$IMAGE_BIN conv=notrunc bs=1 seek=384
int_to_bytes 0x100000 | dd of=$IMAGE_BIN conv=notrunc bs=1 seek=392 # idk random memory offset i might change it later
int_to_bytes $KERNEL_SECTORS | dd of=$IMAGE_BIN conv=notrunc bs=1 seek=400
int_to_bytes 128 | dd of=$IMAGE_BIN conv=notrunc bs=1 seek=408
