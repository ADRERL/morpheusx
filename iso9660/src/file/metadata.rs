//! `FileEntry` convenience accessors.

use crate::types::FileEntry;

impl FileEntry {
    /// The decoded entry name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// True for regular files.
    pub fn is_file(&self) -> bool {
        !self.flags.directory
    }

    /// True for directories.
    pub fn is_directory(&self) -> bool {
        self.flags.directory
    }
}
