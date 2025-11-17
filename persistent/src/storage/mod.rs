//! Persistence storage backends

pub mod esp;

use crate::pe::PeError;

pub trait PersistenceBackend {
    fn store_bootloader(&mut self, data: &[u8]) -> Result<(), PeError>;
    fn retrieve_bootloader(&mut self) -> Result<alloc::vec::Vec<u8>, PeError>;
    fn is_persisted(&mut self) -> Result<bool, PeError>;
    fn name(&self) -> &str;
}
