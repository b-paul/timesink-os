ENTRY(_start)

SECTIONS
{
    . = 0x100000;
    .entry : { *(.entry) }
    .text : { *(.text .text.*) }
    .data : { *(.data .data.*) }
    .rodata : { *(.rodata .rodata.*) }
}
