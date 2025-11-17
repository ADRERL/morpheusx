//! x86_64 PE relocation handling

use crate::pe::reloc::{RelocationEngine, RelocationEntry, RelocationType};
use crate::pe::{PeArch, PeError, PeResult};

pub struct X64RelocationEngine;

impl RelocationEngine for X64RelocationEngine {
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
                todo!("x86_64 DIR64 apply - use unrelocate.rs impl")
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
                todo!("x86_64 DIR64 unapply - use unrelocate.rs impl")
            }
            _ => Err(PeError::UnsupportedFormat),
        }
    }

    fn arch(&self) -> PeArch {
        PeArch::X64
    }
}
