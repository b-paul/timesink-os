//! Printing to screen with BIOS interrupts!

use super::BiosCaller;
use core::arch::asm;

// TODO I am unsure what encoding scheme we can assume is used. It appears to be an extension of
// ascii though, like codepage 437.
impl BiosCaller {
    /// Print a character using the teletype output BIOS interrupt.
    #[inline(never)]
    pub fn print_byte(&self, c: u8) {
        unsafe {
            asm!(
                "push bx",
                "mov ah, 0x0E",
                "mov al, {0}",
                "mov bh, 0",
                "int 0x10",
                "pop bx",
                in(reg_byte) c,
                out("ax") _,
                // We can't out("bx"), _ since llvm uses it or something (rust complains)
            )
        }
    }

    /// Obtain a `BiosWriter` for writing to the frame buffer with the `Write` trait.
    pub fn writer(&self) -> BiosWriter<'_> {
        BiosWriter { b: self }
    }
}

// ngl i kinda hate that this technically isn't a ZST but oh well
/// A type that implements the `Write` trait that writes to the framebuffer with BIOS interrupts.
pub struct BiosWriter<'a> {
    b: &'a BiosCaller,
}

use core::fmt;
impl<'a> fmt::Write for BiosWriter<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            // yeah don't feel like \r ing everything
            if b == b'\n' {
                self.b.print_byte(b'\r');
            }
            self.b.print_byte(b);
        }
        Ok(())
    }
}

// These print and println macros are half stolen from the standard library
/// Prints to the screen using bios interrupts. Note that no new line will be printed, for such
/// functionality, use the `println!` macro.
///
/// See `core::fmt` for format string usage.
///
/// # Examples
/// ```
/// print!("{} + {} =", 1, 1);
/// println!("{}", 1 + 1);
/// ```
#[macro_export]
macro_rules! print {
    ($writer:expr, $($arg:tt)*) => {{
        use core::fmt::Write;
        write!($writer, "{}", core::format_args!($($arg)*)).unwrap();
    }};
}

/// Prints to the screen using bios interrupts, with a new line appended to the end.
///
/// See `core::fmt` for format string usage.
///
/// # Examples
/// ```
/// for i in 0..10 {
///     println!("{i}");
/// }
/// ```
#[macro_export]
macro_rules! println {
    ($writer:expr) => {
        $crate::print!($writer, "\n")
    };
    ($writer:expr, $($arg:tt)*) => {{
        $crate::print!($writer, "{}\n", core::format_args!($($arg)*));
    }};
}
