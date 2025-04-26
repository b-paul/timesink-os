/* We output a binary which gets booted by QEMU. In the future it may be
 * preferable to emit an elf that gets linked with later sections of the
 * booting process / with the kernel. */
OUTPUT_FORMAT(binary)

ENTRY(_start)

SECTIONS
{
    /* The boot sector will be loaded by the bios at address 0x7c00 */
    . = 0x7c00;

    /* We have a dedicated section to the mbr which loads the rest of the code
     * past the first 512 bytes into memory, then jumps to it */
    .mbr : { *(.mbr .mbr.*) }

    /* The BIOS may check for these two magic bytes at the end of the block to
     * determine that it is bootable. */
    . = 0x7c00 + 510;
    .signature : {
        BYTE(0x55)
        BYTE(0xaa)
    }

    /* After the mbr is the actual code for the bootloader! */
    . = 0x7e00;
    /* TODO some section wildcards may be missing! */
    .boot : { *(.boot) }
    /* .text.* is necessary since rust seems to emit stuff like .text.run */
    .text : { *(.text .text.*) }
    .data : { *(.data) }
    .got : { *(.got .got.*) }
}
