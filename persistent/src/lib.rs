//! Bootloader self-persistence and data persistence
//!
//! Platform-neutral: PE parsing, memory capture, storage backends
//! Platform-specific: Relocation handling (arch modules)

#![no_std]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::new_without_default)]
#![allow(clippy::doc_lazy_continuation)]

extern crate alloc;

pub mod capture;
pub mod feedback;
pub mod pe;
pub mod storage;

#[cfg(target_arch = "x86_64")]
pub mod arch {
    pub mod x86_64;
}

#[cfg(target_arch = "aarch64")]
pub mod arch {
    pub mod aarch64;
}
