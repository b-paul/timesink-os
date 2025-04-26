#![no_std]
#![no_main]

use core::panic::PanicInfo;

// This program is meant to be empty, I made it just for testing the builder script

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() {
    loop {}
}
