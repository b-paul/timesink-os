//! The rust `core` library assumes the existence of a `memcpy`, `memmove`, `memset`, `memcmp`,
//! `bcmp` and `strlen` symbol. We implement these functions here. They will have the same type
//! signature as the C functions from libc.

// Everything we implement here is from libc, so I don't think docs are necessary.
#![allow(missing_docs)]

use core::ffi::c_char;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    // TODO this is probably a bad implementation
    for offset in 0..n as isize {
        unsafe { *dest.offset(offset) = *src.offset(offset) }
    }
    dest
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    // We have to move src to dest, but src and dest are allowed to overlap at length n. There are
    // two cases. The simpler case is when src comes after dest, in which case we can just do a
    // normal copy, and the other case is when dest comes after src, in which case we can write
    // from the back.
    if src as usize > dest as usize {
        for offset in 0..n as isize {
            unsafe { *dest.offset(offset) = *src.offset(offset) }
        }
    } else {
        for offset in (0..n as isize).rev() {
            unsafe { *dest.offset(offset) = *src.offset(offset) }
        }
    }
    dest
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut offset = 0;
    while offset < n {
        unsafe {
            *s.offset(offset as isize) = c as u8;
        }
        offset += 1;
    }
    s
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcmp(_s1: *const u8, _s2: *const u8, _n: usize) -> i32 {
    todo!();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    unsafe { memcmp(s1, s2, n) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strlen(_s: *const c_char) -> usize {
    todo!();
}
