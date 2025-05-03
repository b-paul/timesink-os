//! Manage cpu states, such as protected mode, unreal mode and long mode.

use core::arch::asm;

// TODO should these be unsafe because of the unsafety that could come from running interrupts in
// protected mode?
/// Enter protected mode. In protected mode, BIOS interrupts will be mostly disabled, so caution
/// should be made so that no interrupts are executed. To exit, call `exit_protected_mode`.
///
/// It is valid to call this function within protected mode in which case, nothing will happen.
pub fn enter_protected_mode() {
    unsafe {
        asm!("mov eax, cr0", "or eax, 1", "mov cr0, eax");
    }
}

/// Exit protected mode to reenter real mode. Within real mode, it is safe to call bios interrupts.
///
/// It is valid to call this function within real mode, in which case nothing will happen.
pub fn exit_protected_mode() {
    unsafe {
        asm!("mov eax, cr0", "and eax, ~1", "mov cr0, eax");
    }
}

/// Enter unreal mode, which will give us full access to the 32 bit address space, while still
/// being able to call BIOS interrupts.
pub fn enter_unreal_mode() {
    unsafe {
        asm!("push ds");
    }

    enter_protected_mode();

    // Set bs to use the segment we created that can access full the 32 bit address space.
    // As stated on the OS dev wiki, the bits 3-15 in the segment registers correspond to gdt
    // entries when in protected mode, and for some reason when we set the segment and switch back
    // to real mode the information from this is preserved.
    unsafe {
        asm!("push bx", "mov bx, 0x08", "mov ds, bx", "pop bx");
    }

    exit_protected_mode();

    unsafe {
        asm!("pop ds");
    }
}

