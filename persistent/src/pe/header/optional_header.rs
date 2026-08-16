use super::super::{PeError, PeResult};
use super::utils::{read_u16, read_u32, read_u64};

#[derive(Debug, Clone, Copy)]
pub struct OptionalHeader64 {
    pub magic: u16,
    pub address_of_entry_point: u32,
    pub image_base: u64, // UEFI rewrites this to the actual load address.
    pub section_alignment: u32,
    pub file_alignment: u32,
    pub size_of_image: u32,
    pub size_of_headers: u32,
    pub checksum: u32,
    pub subsystem: u16,
    pub number_of_rva_and_sizes: u32,
}

impl OptionalHeader64 {
    pub const MAGIC_PE32PLUS: u16 = 0x20B;
    pub const IMAGE_BASE_OFFSET: usize = 24;

    /// # Safety
    ///
    /// `data` must be readable for at least `size` bytes and `data + pe_offset`
    /// must reference a PE image whose optional header lies within `size`.
    pub unsafe fn parse(data: *const u8, pe_offset: u32, size: usize) -> PeResult<Self> {
        // Optional header = pe_offset + 4 (sig) + 20 (COFF).
        let opt_offset = pe_offset as usize + 24;

        // 112 not 96: number_of_rva_and_sizes is read at opt_offset+108 (u32 -> +112).
        if opt_offset + 112 > size {
            return Err(PeError::InvalidOffset);
        }

        let magic = read_u16(data, opt_offset);
        if magic != Self::MAGIC_PE32PLUS {
            return Err(PeError::UnsupportedFormat);
        }

        let address_of_entry_point = read_u32(data, opt_offset + 16);
        let image_base = read_u64(data, opt_offset + 24);
        let section_alignment = read_u32(data, opt_offset + 32);
        let file_alignment = read_u32(data, opt_offset + 36);
        let size_of_image = read_u32(data, opt_offset + 56);
        let size_of_headers = read_u32(data, opt_offset + 60);
        let checksum = read_u32(data, opt_offset + 64);
        let subsystem = read_u16(data, opt_offset + 68);
        let number_of_rva_and_sizes = read_u32(data, opt_offset + 108);

        Ok(OptionalHeader64 {
            magic,
            address_of_entry_point,
            image_base,
            section_alignment,
            file_alignment,
            size_of_image,
            size_of_headers,
            checksum,
            subsystem,
            number_of_rva_and_sizes,
        })
    }

    /// # Safety
    ///
    /// `data` must contain a valid PE image with consistent DOS and COFF
    /// headers; the caller is responsible for ensuring the buffer is the real
    /// image and that overwriting its `ImageBase` field is intended.
    pub unsafe fn patch_image_base(data: &mut [u8], new_image_base: u64) -> PeResult<()> {
        if data.len() < 0x40 {
            return Err(PeError::InvalidOffset);
        }

        let e_lfanew =
            u32::from_le_bytes([data[0x3C], data[0x3D], data[0x3E], data[0x3F]]) as usize;

        let image_base_offset = e_lfanew + 24 + Self::IMAGE_BASE_OFFSET;

        if image_base_offset + 8 > data.len() {
            return Err(PeError::InvalidOffset);
        }

        let bytes = new_image_base.to_le_bytes();
        data[image_base_offset..image_base_offset + 8].copy_from_slice(&bytes);

        Ok(())
    }
}

#[cfg(test)]
mod bounds_regression {
    use super::OptionalHeader64 as O;
    use crate::pe::PeError;

    // Regression: parse read number_of_rva_and_sizes at opt_offset+108 (u32 ->
    // +112) but only guarded opt_offset+96 — a ~16B OOB read for a buffer sized
    // in [96,112). The guard must be +112.
    #[test]
    fn rejects_optional_header_shorter_than_full_read() {
        // one 136-byte backing buffer; the `size` arg drives the guard. pe_offset
        // 0 -> opt_offset 24, so a claimed size under 24+112=136 must be rejected
        // before the +108 read (never reached).
        let buf = [0u8; 24 + 112];
        for size in [24 + 96usize, 24 + 100, 24 + 111] {
            let r = unsafe { O::parse(buf.as_ptr(), 0, size) };
            assert!(
                matches!(r, Err(PeError::InvalidOffset)),
                "size {size} must be rejected"
            );
        }
        // 136 bytes is enough to read every field; magic 0x020B -> parses.
        let mut full = [0u8; 24 + 112];
        full[24] = 0x0B;
        full[25] = 0x02; // MAGIC_PE32PLUS little-endian
        let r = unsafe { O::parse(full.as_ptr(), 0, full.len()) };
        assert!(r.is_ok(), "full-size buffer with PE32+ magic must parse");
    }
}
