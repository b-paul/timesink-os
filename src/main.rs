//! This crate is for an MBR image that can be booted from by a BIOS (or qemu).
//!
//! Currently we don't do anything on boot but print some letters but that will change soon!

#![no_std]
#![no_main]

use core::arch::{asm, global_asm};
use core::panic::PanicInfo;

global_asm!(
    "
# Mark the boot section as allocatable, writable and executable.
.section .boot, \"awx\"
.global _start
.code16

_start:
    # initialise registers
    xor ax, ax
    mov ds, ax
    mov ss, ax
    mov sp, 0x7c00
    cld

    # Jump to our rust code
    call boot

    hang:
        jmp hang
"
);

// TODO I am unsure what encoding scheme we can assume is used. It appears to be an extension of
// ascii though, like codepage 437.
/// Print a character using the teletype output BIOS interrupt.
#[inline(never)]
fn print(c: u8) {
    unsafe {
        asm!(
            "push bx",
            "mov ah, 0x0E",
            "mov al, {0}",
            "mov bh, 0",
            "int 0x10",
            "pop bx",
            in(reg_byte) c
        )
    }
}

/// Print an 8 bit number in decimal.
fn print_num(n: u8) {
    if n >= 100 {
        print(b'0' + n / 100);
    }
    if n >= 10 {
        print(b'0' + (n % 100) / 10);
    }
    print(b'0' + n % 10);
}

/// Gets executed after initialisation in _start.
#[unsafe(no_mangle)]
pub extern "C" fn boot() {
    // For now we will just print out all of the characters. The font looks like codepage 437!
    for i in 0..=255 {
        if i == b'\n' || i == b'\r' {
            continue;
        }
        print_num(i);
        print(b':');
        print(0);
        print(i);
        print(0);
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
