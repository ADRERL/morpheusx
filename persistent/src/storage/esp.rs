//! ESP (EFI System Partition) backend

use super::PersistenceBackend;
use crate::pe::PeError;

pub struct EspBackend {}

impl EspBackend {
    pub fn new() -> Self {
        todo!("ESP backend creation")
    }
}

impl PersistenceBackend for EspBackend {
    fn store_bootloader(&mut self, data: &[u8]) -> Result<(), PeError> {
        todo!("Use morpheus_core::fs::fat32_ops::write_file")
    }

    fn retrieve_bootloader(&mut self) -> Result<alloc::vec::Vec<u8>, PeError> {
        todo!("Read from ESP")
    }

    fn is_persisted(&mut self) -> Result<bool, PeError> {
        todo!("Check file exists")
    }

    fn name(&self) -> &str {
        "ESP"
    }
}
