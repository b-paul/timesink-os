//! Functionality for reading from the disk into memory.

use core::arch::asm;

unsafe extern "C" {
    static _drive_number: u8;
}

/// Used in a BIOS interrupt (EXTENDED READ) to read from a disk into memory.
#[repr(C)]
#[derive(Debug)]
pub struct DiskAddressPacket {
    _size: u8,
    _reserved: u8,
    num_blocks: u16,
    offset: u16,
    segment: u16,
    block_address: u64,
}

impl DiskAddressPacket {
    /// Create a packet. At most 0x7F blocks can be read at once, so `None` is returned when
    /// `num_blocks` is greater than such.
    pub fn new(
        num_blocks: u16,
        offset: u16,
        segment: u16,
        block_address: u32,
    ) -> Option<DiskAddressPacket> {
        // TODO validate the address range
        if num_blocks > 0x007F {
            None
        } else {
            Some(DiskAddressPacket {
                _size: 16,
                _reserved: 0,
                num_blocks,
                offset,
                segment,
                block_address: block_address.into(),
            })
        }
    }

    /// Load the contents of the disk packet into memory.
    #[inline(never)]
    pub fn load(&self) {
        let mut _err: u16;
        unsafe {
            // Execute an EXTENDED READ interrupt. We have to push and pop si manually since it is
            // used internally by llvm.
            asm!("push si",
            "mov si, {0:x}",
            "mov ah, 0x42",
            "mov dl, {1}",
            "int 0x13",
            "pop si",
            in(reg) self as *const _ as usize,
            in(reg_byte) _drive_number,
            out("ax") _err);
        }
        // TODO return the error code in a result
    }
}
