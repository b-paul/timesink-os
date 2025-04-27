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

mod print;

unsafe extern "C" {
    static _partition_boot_sectors: u64;
    static _partition_kernel_addr: u64;
    static _partition_kernel_sectors: u64;
}

fn enter_protected_mode() {
    unsafe {
        asm!("mov eax, cr0", "or eax, 1", "mov cr0, eax");
    }
}

fn exit_protected_mode() {
    unsafe {
        asm!("mov eax, cr0", "and eax, ~1", "mov cr0, eax");
    }
}

fn enter_unreal_mode() {
    unsafe {
        asm!("push ds");
    }

    enter_protected_mode();

    unsafe {
        // TODO which gdt entry should we be using ?!
        asm!("push bx", "mov bx, 0x08", "mov ds, bx", "pop bx");
    }

    exit_protected_mode();

    unsafe {
        asm!("pop ds");
    }
}

/// Gets executed after initialisation in _start.
#[unsafe(no_mangle)]
pub extern "C" fn boot() {
    println!("Hello, {}", "world!");

    enter_unreal_mode();

    println!("Bootloader sectors: {:08x}", unsafe {
        _partition_boot_sectors
    });
    println!("Kernel addr: {:08x}", unsafe { _partition_kernel_addr });
    println!("Kernel sectors: {:08x}", unsafe {
        _partition_kernel_sectors
    });

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    loop {}
}
