//! Manage cpu states, such as protected mode, unreal mode and long mode.

use core::arch::asm;
use core::marker::PhantomData;

struct Zst;

pub struct RealMode {
    _z: Zst,
}

use super::bios::BiosCaller;

impl core::ops::Deref for RealMode {
    type Target = BiosCaller;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const RealMode as *const BiosCaller) }
    }
}

pub struct UnrealMode {
    _z: Zst,
}

impl core::ops::Deref for UnrealMode {
    type Target = BiosCaller;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const UnrealMode as *const BiosCaller) }
    }
}

pub struct ProtectedMode<U> {
    _p: PhantomData<U>,
}

impl RealMode {
    /// Acquire a witness of being in real mode, given from startup.
    ///
    /// # Safety
    /// This function should only be called at the very start of the boot process, as then it is
    /// guaranteed that we are in real mode.
    pub unsafe fn begin() -> Self {
        RealMode { _z: Zst }
    }

    /// Exit real mode and enter protected mode.
    pub fn enter_protected_mode(self) -> ProtectedMode<RealMode> {
        unsafe {
            asm!("mov eax, cr0", "or eax, 1", "mov cr0, eax");
        }
        ProtectedMode { _p: PhantomData }
    }

    /// Enter unreal mode, which will give us full access to the 32 bit address space, while still
    /// being able to call BIOS interrupts.
    pub fn enter_unreal_mode(self) -> UnrealMode {
        unsafe {
            asm!("push ds");
        }

        let prot = self.enter_protected_mode();

        // Set ds to use the segment we created that can access full the 32 bit address space.
        // As stated on the OS dev wiki, the bits 3-15 in the segment registers correspond to gdt
        // entries when in protected mode, and for some reason when we set the segment and switch back
        // to real mode the information from this is preserved.
        unsafe {
            asm!("mov {0}, 0x10", "mov ds, {0}", out(reg) _);
        }

        let _ = prot.enter_real_mode();

        unsafe {
            asm!("pop ds");
        }

        UnrealMode { _z: Zst }
    }
}

impl UnrealMode {
    /// Exit real mode and enter protected mode.
    pub fn enter_protected_mode(self) -> ProtectedMode<UnrealMode> {
        unsafe {
            asm!("mov eax, cr0", "or eax, 1", "mov cr0, eax");
        }
        ProtectedMode { _p: PhantomData }
    }
}

impl ProtectedMode<RealMode> {
    /// Exit protected mode and reenter real mode.
    pub fn enter_real_mode(self) -> RealMode {
        unsafe {
            asm!("mov eax, cr0", "and eax, ~1", "mov cr0, eax");
        }
        RealMode { _z: Zst }
    }
}

impl ProtectedMode<UnrealMode> {
    /// Exit protected mode and reenter unreal mode.
    pub fn enter_unreal_mode(self) -> UnrealMode {
        unsafe {
            asm!("mov eax, cr0", "and eax, ~1", "mov cr0, eax");
        }
        UnrealMode { _z: Zst }
    }
}

/// Enter long mode. In long mode, BIOS interrupts will be mostly disabled, so caution should be
/// made so that no interrupts are executed. To exit, call `exit_long_mode`.
///
/// This function is always valid to call.
#[allow(unused)]
pub fn enter_long_mode() {
    todo!()
}

/// Exit long mode and reenter real mode, where BIOS interrupts are safe to call.
///
/// This function is always valid to call.
#[allow(unused)]
pub fn exit_long_mode() {
    todo!()
}
