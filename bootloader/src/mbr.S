# Mark the mbr section as allocatable, writable and executable.
.section .mbr, "awx"
.global _start
.code16

_start:
    # initialise registers
    xor ax, ax
    mov ds, ax
    mov ss, ax
    mov sp, 0x7c00
    cld
    cli

    # Load the gdt
    lgdt [gdtr]

    # Our goal at the moment is to load the rest of the boot loader into
    # memory and jump to it. We will use the disk reading BIOS interrupts
    # to do this.

    # We will perform an EXTENDED READ interrupt. The arguments are as follows
    # - AH = 0x42
    # - DL = drive number
    # - DS:SI = address of a "disk address packet" struct
    # On boot, we know that the DL register *will* contain the disk number.
    # The disk address packet is defined lower in this file.

    # We will dynamically update the number of blocks to read, since it can
    # change after assembly.
    mov ax, _partition_boot_sectors
    mov [OFFSET dap + 2], ax

    # And now execute the interrupt
    mov si, OFFSET dap
    mov ah, 0x42
    int 0x13

    # If the interrupt succeeded, the carry flag will be unset. We want to
    # catch errors (which have been occuring while developing) so we add this
    # check. Maybe in the future I'll be bothered to write an error logger
    # here! The main error that has been occuring has been from reading more
    # from the disk than is available, so some process to figure out the
    # correct amount should probably be made.
    jc hang

success:

    # Jump to our rust code
    jmp _entry

    hang:
        jmp hang

gdtr:
    .2byte (gdt_end - gdt  - 1) # size of gdt (minus 1 for some reason)
    .4byte gdt # offset of the gdt

gdt:
    .8byte 0 # entry 0 is null
    # I think this is wrong TODO
    # We don't really want to support proper segmentation in the bootloader,
    # the kernel can do that when it wants to. We do still need to set up a gdt
    # though, to get into protected mode. We have to make entries for code and
    # data segments, and we will make simple entries with little restrictions.
    # We have the base set to 0, and the limit set to 0xfffff. We also use the
    # granularity 4kb flag, so this is 0xfffff 4kb blocks as the limit. Hence
    # all of memory fits into these sectors. TODO I can't be bothered to
    # document the rest of this lol it's not that important
    .8byte 0x00cf9b000000ffff # code segment
    .8byte 0x00cf9f000000ffff # idk
gdt_end:

.global _partition_boot_sectors

.align 4
.global dap
dap:
    .byte 0x10 # size of packet
    .byte 0 # zeroed
    .2byte 0x00 # number of blocks to read, this is updated at run time
    .2byte 0x7e00 # Transfer address offset
    .2byte 0 # Transfer addres sector
    .4byte 1 
    .4byte 0

# The boot sector will be placed outside of the MBR! The _entry label is where
# we will jump to to start executing code sitting outside of the MBR.
.section .boot, "awx"

.global _entry
_entry:
    call boot

    # If we return from the boot function, we should do something still, so we
    # hang.
    jmp hang
