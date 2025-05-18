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
extern "C" fn get_instruction_pointer() -> u64 {
    let ip: u64;
    unsafe { core::arch::asm!("pop rax", "mov {}, [rsp]", "push rax", out(reg) ip) }
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
                "ds mov WORD PTR [{0:e} + {0:e}*1 + 0xb8000], 0x0020",
                in(reg) i
            )
        };
    }

    // Print the instruction pointer
    let mut ip = get_instruction_pointer();

    let mut x = 7;
    while ip > 0 {
        let k = ip % 16;
        ip /= 16;
        let char = 0x07 << 8 | (k as u8 + b'0') as u16;
        unsafe {
            //fb.offset(i).write(b' ' as u16);
            core::arch::asm!(
                "mov {1:x}, {2:x}",
                "ds mov WORD PTR [{0:e} + {0:e}*1 + 0xb8000], {1:x}",
                in(reg) x,
                out(reg) _,
                in(reg) char,
            )
        };
        x -= 1;
    }
    // IT'S AT 0x00029??????? THAT'S NOT 0x100000!!! AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA

    loop {}
}
