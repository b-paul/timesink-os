//! Bios interrupts

pub mod disk;
pub mod print;

struct Zst;

pub struct BiosCaller {
    _z: Zst,
}
