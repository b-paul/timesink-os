#!/bin/sh

# Converts an integer into a byte string.
int_to_bytes() {
    # TODO if I care I should bound check the number

    # https://stackoverflow.com/a/9955198
    printf "0: %.8x" $1 | sed -E 's/0: (..)(..)(..)(..)/0: \4\3\2\1/' | xxd -r -g0
}

BUILD_DIR=$(dirname $1)
ELF="$BUILD_DIR/bootloader"
BOOTLOADER_BIN="$BUILD_DIR/bootloader.bin"

# Convert the linked elf into a raw binary file
objcopy -I elf32-i386 -O binary $ELF $BOOTLOADER_BIN || exit

# We have to update the partition information to say something still
# We can put a kernel which does nothing but return!

KERNEL_NASM="$BUILD_DIR/kernel.nasm"
KERNEL_BIN="$BUILD_DIR/kernel.bin"

echo "ret" > $KERNEL_NASM
nasm -o $KERNEL_BIN $KERNEL_NASM || exit

# then we actually build the image
IMAGE_BIN="$BUILD_DIR/image.bin"

dd if=$BOOTLOADER_BIN of=$IMAGE_BIN bs=512
dd if=$KERNEL_BIN of=$IMAGE_BIN conv=notrunc bs=512 seek=128

bootloader_size=$(wc -c $BOOTLOADER_BIN | cut -f1 -d' ')
bootloader_sectors=$((($bootloader_size - 1)/512))

# boot_sectors
int_to_bytes $bootloader_sectors | dd of=$IMAGE_BIN conv=notrunc bs=1 seek=384
# kernel_addr
int_to_bytes 0x100000 | dd of=$IMAGE_BIN conv=notrunc bs=1 seek=392 # idk random memory offset i might change it later
# kernel_sectors
int_to_bytes 1 | dd of=$IMAGE_BIN conv=notrunc bs=1 seek=400
# kernel_block
int_to_bytes 128 | dd of=$IMAGE_BIN conv=notrunc bs=1 seek=408

qemu-system-i386 -hda $IMAGE_BIN
