//! This is a bootloader for a kernel that I have yet to write. What this bootloader will do is
//! find out stuff accessible with bios interrupts and store them somewhere, turn on protected
//! mode, load the kernel into memory, then jump to it.

#![no_std]
#![no_main]
#![deny(missing_docs)]

use core::arch::global_asm;
use core::panic::PanicInfo;

// This contains the master boot record, whihc is what we get booted into! It loads this rust
// program into memory, then jumps to the `boot` function.
global_asm!(include_str!("mbr.S"));

mod disk;
pub mod mem_routines;
mod modes;
mod print;

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
    println!("Hello, {}", "world!");

    modes::enter_unreal_mode();

    println!("Bootloader sectors: {:08x}", unsafe {
        _partition_boot_sectors
    });
    println!("Kernel addr: {:08x}", unsafe { _partition_kernel_addr });
    println!("Kernel sectors: {:08x}", unsafe {
        _partition_kernel_sectors
    });
    println!("Kernel block index: {:08x}", unsafe {
        _partition_kernel_block
    });

    // Now we actually load the kernel into memory!
    read_kernel();

    // For now we'll print the first 128 bytes to check that we've read the correct thing.
    for i in 0..128 {
        unsafe {
            let b = *(_partition_kernel_addr as *const u8).offset(i);
            print!("{b:02x} ");
        }
    }

    // Next problem is figuring out where to jump to lol
    //unsafe {
    //core::mem::transmute::<_, extern "C" fn()>(_partition_kernel_addr as usize)();
    //}

    loop {}
}

/// Load the kernel from disk into memory.
fn read_kernel() {
    let mut sectors_left = unsafe { _partition_kernel_sectors } as i32;
    let mut cur_addr = unsafe { _partition_kernel_addr };
    let mut cur_block = unsafe { _partition_kernel_block };

    println!("Reading {sectors_left} many blocks");

    while sectors_left > 0 {
        // We can load at most 0x7f sectors per packet.
        let blocks_read = sectors_left.min(0x7f) as u32;

        // We cannot load the disk contents into the kernel address location directly, since the
        // bios seems to crash writing past 1MB, so instead what we will do is load it into a
        // buffer and copy it over manually.
        let addr = &raw mut DISK_BUF as u32;
        let offset = (addr & 0xf) as u16;
        let segment = (addr >> 4) as u16;
        let packet = disk::DiskAddressPacket::new(
            blocks_read.try_into().unwrap(),
            offset,
            segment,
            cur_block,
        )
        .expect("Invalid disk packet was created");

        packet.load();

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
    println!("Finished reading blocks");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    loop {}
}
