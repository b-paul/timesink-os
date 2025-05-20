#![no_std]
#![no_main]

use core::panic::PanicInfo;

// This program is meant to be empty, I made it just for testing the builder script

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
#[inline(never)]
extern "C" fn get_instruction_pointer() -> u32 {
    let ip: u32;
    unsafe { core::arch::asm!("mov {}, [esp]", out(reg) ip) }
    ip
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".entry")]
pub extern "C" fn _start() {
    unsafe { (0xb8000 as *mut u32).write(0x07690748) };

    for i in 0..80 * 25 {
        unsafe {
            //fb.offset(i).write(b' ' as u16);
            core::arch::asm!(
                "ds mov WORD PTR [{0:e}*2 + 0xb8000], 0x0020",
                //"mov DWORD PTR ds:0xb8000, 0x0020",
                in(reg) i
            )
        };
    }

    // Print the instruction pointer
    let mut ip = get_instruction_pointer();

    let mut x = 15i32;
    for _ in 0..16u32 {
        let k = ip % 16;
        ip /= 16;
        let char = 0x07 << 8 | (k as u8 + b'0') as u16;
        unsafe {
            core::arch::asm!(
                "ds mov WORD PTR [{0:e}*2 + 0xb8000], {1:x}",
                in(reg) x,
                in(reg) char,
            )
        };
        x -= 1;
    }

    loop {}
}
