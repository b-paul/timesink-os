//! This is a bootloader for a kernel that I have yet to write. What this bootloader will do is
//! find out stuff accessible with bios interrupts and store them somewhere, turn on protected
//! mode, load the kernel into memory, then jump to it.

// TODO references in docs

#![no_std]
#![no_main]
#![deny(missing_docs)]

use core::arch::global_asm;
use core::panic::PanicInfo;

// This contains the master boot record, whihc is what we get booted into! It loads this rust
// program into memory, then jumps to the `boot` function.
global_asm!(include_str!("mbr.S"));

mod bios;
pub mod mem_routines;
mod modes;

unsafe extern "C" {
    static _partition_boot_sectors: u32;
    static _partition_kernel_addr: u32;
    static _partition_kernel_sectors: u32;
    static _partition_kernel_block: u32;
}

static mut DISK_BUF: [u8; 512 * 0x80] = [0u8; 512 * 0x80];

/// Gets executed after initialisation in _start.
#[unsafe(no_mangle)]
pub extern "C" fn boot() {
    let real_mode = unsafe { modes::RealMode::begin() };
    let mut w = real_mode.writer();

    println!(w, "Hello, {}", "world!");

    // Before we load the kernel, it's a good idea to give ourselfs more than 1mb of memory
    let unreal_mode = real_mode.enter_unreal_mode();
    let mut w = unreal_mode.writer();

    unsafe {
        println!(w, "Bootloader sectors: {:08x}", _partition_boot_sectors);
        println!(w, "Kernel addr: {:08x}", _partition_kernel_addr);
        println!(w, "Kernel sectors: {:08x}", _partition_kernel_sectors);
        println!(w, "Kernel block index: {:08x}", _partition_kernel_block);
    }

    // Now we actually load the kernel into memory!
    read_kernel(&unreal_mode);

    // For now we'll print the first 128 bytes to check that we've read the correct thing.
    for i in 0..128 {
        unsafe {
            let b = *(_partition_kernel_addr as *const u8).offset(i);
            print!(w, "{b:02x} ");
        }
    }

    println!(w, "Jumping!");

    // TODO clean this up oh my god

    // Now we actually jump to the kernel
    // TODO enter long mode instead lol
    let prot = unreal_mode.enter_protected_mode();
    unsafe {
        // The address needs to be loaded from memory before we set cs, hence we push it to the
        // stack.
        core::arch::asm!(
            "push {entry:e}",
            entry = in(reg) _partition_kernel_addr
        );
        // Set cs by doing a funny jump, at&t syntax is needed for some reason :(
        core::arch::asm!("ljmp $0x8, $2f", "2:", options(att_syntax));
        // Set the rest of the segment registers to the data segment
        core::arch::asm!(
            "mov {0:x}, 0x10",
            "mov ds, {0:x}",
            "mov ss, {0:x}",
            "mov es, {0:x}",
            "mov fs, {0:x}",
            "mov gs, {0:x}",
            out(reg) _,
        );
        // Get the kernel start address and call it!
        // TODO THIS SHOULD NOT BE 16 BIT!!! SOMETHING IS VERY WRONG!!! A20 MIGHT NOT BE ENABLED OR
        // MAYBE PROTECTED MODE IS BROKEN OR SOMETHING AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
        core::arch::asm!(
            "pop {0:x}",
            "call {0:x}",
            out(reg) _,
        );
    }
    let unreal_mode = prot.enter_unreal_mode();
    let mut w = unreal_mode.writer();
    unsafe {
        core::arch::asm!(
            "xor {0:x}, {0:x}",
            "mov ds, {0:x}",
            "mov ss, {0:x}",
            "mov es, {0:x}",
            "mov fs, {0:x}",
            "mov gs, {0:x}",
            out(reg) _,
        );
    }
    println!(w, "Returned!");

    loop {}
}

/// Load the kernel from disk into memory.
fn read_kernel(unreal_mode: &modes::UnrealMode) {
    let mut w = unreal_mode.writer();

    let mut sectors_left = unsafe { _partition_kernel_sectors } as i32;
    let mut cur_addr = unsafe { _partition_kernel_addr };
    let mut cur_block = unsafe { _partition_kernel_block };

    println!(w, "Reading {sectors_left} blocks");

    while sectors_left > 0 {
        // We can load at most 0x7f sectors per packet.
        let blocks_read = sectors_left.min(0x7f) as u32;

        // We cannot load the disk contents into the kernel address location directly, since the
        // bios seems to crash writing past 1MB, so instead what we will do is load it into a
        // buffer and copy it over manually.
        let addr = &raw mut DISK_BUF as u32;
        let offset = (addr & 0xf) as u16;
        let segment = (addr >> 4) as u16;
        let packet = bios::disk::DiskAddressPacket::new(
            blocks_read.try_into().unwrap(),
            offset,
            segment,
            cur_block,
        )
        .expect("Invalid disk packet was created");

        unreal_mode.load_dap(&packet);

        unsafe {
            core::ptr::copy(
                &raw const DISK_BUF as *const u8,
                cur_addr as *mut u8,
                blocks_read as usize * 512,
            );
        }

        sectors_left -= blocks_read as i32;
        cur_addr += blocks_read * 512;
        cur_block += blocks_read;
    }
    println!(w, "Finished reading blocks");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // safety: i don't know what to do otherwise!!!
    let real_mode = unsafe { modes::RealMode::begin() };
    let mut w = real_mode.writer();
    println!(w, "{info}");
    loop {}
}
