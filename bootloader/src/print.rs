//! Printing to screen with BIOS interrupts!

use core::arch::asm;

// TODO I am unsure what encoding scheme we can assume is used. It appears to be an extension of
// ascii though, like codepage 437.
/// Print a character using the teletype output BIOS interrupt.
#[inline(never)]
pub fn print_byte(c: u8) {
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


/// A ZST that implements the Write trait that writes to the framebuffer with BIOS interrupts.
pub struct BiosWriter;

use core::fmt;
impl fmt::Write for BiosWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            // yeah don't feel like \r ing everything
            if b == b'\n' { print_byte(b'\r'); }
            print_byte(b);
        }
        Ok(())
    }
}

// These print and println macros are half stolen from the standard library
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        use crate::print::BiosWriter;
        use core::fmt::Write;
        write!(BiosWriter, "{}", core::format_args!($($arg)*)).unwrap();
    }};
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        $crate::print!("{}\n", core::format_args!($($arg)*));
    }};
}
