//! ARM64 PE relocation handling
//!
//! NOTE: May require ADRP/ADD instruction pair handling for position-independent code.
//! Initial impl handles data pointers only.

use crate::pe::reloc::{RelocationEngine, RelocationEntry, RelocationType};
use crate::pe::{PeArch, PeError, PeResult};

pub struct Aarch64RelocationEngine;

impl RelocationEngine for Aarch64RelocationEngine {
    fn apply_relocation(
        &self,
        image_data: &mut [u8],
        entry: RelocationEntry,
        page_rva: u32,
        delta: i64,
    ) -> PeResult<()> {
        match entry.reloc_type() {
            RelocationType::Absolute => Ok(()),
            RelocationType::Dir64 => {
                todo!("ARM64 DIR64 apply - data pointers first, ADRP/ADD later")
            }
            _ => Err(PeError::UnsupportedFormat),
        }
    }

    fn unapply_relocation(
        &self,
        image_data: &mut [u8],
        entry: RelocationEntry,
        page_rva: u32,
        delta: i64,
    ) -> PeResult<()> {
        match entry.reloc_type() {
            RelocationType::Absolute => Ok(()),
            RelocationType::Dir64 => {
                todo!("ARM64 DIR64 unapply")
            }
            _ => Err(PeError::UnsupportedFormat),
        }
    }

    fn arch(&self) -> PeArch {
        PeArch::ARM64
    }
}
