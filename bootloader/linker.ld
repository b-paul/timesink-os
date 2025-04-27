/* We output a binary which gets booted by QEMU. */
OUTPUT_FORMAT(binary)

ENTRY(_start)

SECTIONS
{
    /* The boot sector will be loaded by the bios at address 0x7c00 */
    . = 0x7c00;

    /* We have a dedicated section to the mbr which loads the rest of the code
     * past the first 512 bytes into memory, then jumps to it */
    .mbr : { *(.mbr .mbr.*) }

    /* Here we store the partition information for the rest of the bootloader,
     * and the kernel itself. We will modify its values at build time, but they
     * will be initialised to 0. */
    . = 0x7d80;
    _partition_boot_sectors = 0x7d80;
    .partition.boot_sectors : {
        /* TODO modify this region with a build script */
        QUAD(0)
    }
    _partition_kernel_addr = 0x7d88;
    .partition.kernel_addr : {
        QUAD(0)
    }
    _partition_kernel_sectors = 0x7d90;
    .partition.kernel_sectors : {
        QUAD(0)
    }

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
