//! This is a bootloader for a kernel that I have yet to write. What this bootloader will do is
//! find out stuff accessible with bios interrupts and store them somewhere, turn on protected
//! mode, load the kernel into memory, then jump to it.

#![no_std]
#![no_main]

use core::arch::{asm, global_asm};
use core::panic::PanicInfo;

// This contains the master boot record, whihc is what we get booted into! It loads this rust
// program into memory, then jumps to the `boot` function.
global_asm!(include_str!("mbr.S"));

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
