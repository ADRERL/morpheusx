//! UEFI protocol management

#[cfg(target_os = "uefi")]
pub mod uefi;

#[cfg(target_os = "uefi")]
pub use uefi::ProtocolManager;
