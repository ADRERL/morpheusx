//! `FileFlags` byte conversions (ISO 9660 §9.1.6).

use crate::types::FileFlags;

impl FileFlags {
    /// Decode the directory-record file-flags byte.
    pub fn from_byte(byte: u8) -> Self {
        Self {
            hidden: byte & 0x01 != 0,
            directory: byte & 0x02 != 0,
            associated: byte & 0x04 != 0,
            extended_format: byte & 0x08 != 0,
            extended_permissions: byte & 0x10 != 0,
            not_final: byte & 0x80 != 0,
        }
    }
}
