// cross-checks morpheus_persistent's pe header parser against goblin: both
// on field values for a well-formed image, and on refusing images truncated
// inside the optional header instead of reading past the buffer.

use goblin::pe::PE;
use morpheus_persistent::pe::header::PeHeaders;

const E_LFANEW: u32 = 0x80;
const OPT_HEADER_SIZE: u16 = 112;
const ENTRY_POINT: u32 = 0x1234;
const IMAGE_BASE: u64 = 0x1_4000_0000;
const SUBSYSTEM: u16 = 3; // windows cui
const SIZE_OF_IMAGE: u32 = 0x9000;
const SIZE_OF_HEADERS: u32 = 0x400;

// no sections, no data directories: only the coff and optional header
// fields morpheus_persistent reads carry meaning here.
fn build_valid_image() -> Vec<u8> {
    let mut buf = vec![0u8; E_LFANEW as usize];
    buf[0..2].copy_from_slice(b"MZ");
    buf[0x3C..0x40].copy_from_slice(&E_LFANEW.to_le_bytes());

    buf.extend_from_slice(b"PE\0\0");

    // coff header
    buf.extend_from_slice(&0x8664u16.to_le_bytes()); // machine: amd64
    buf.extend_from_slice(&0u16.to_le_bytes()); // number_of_sections
    buf.extend_from_slice(&0u32.to_le_bytes()); // time_date_stamp
    buf.extend_from_slice(&0u32.to_le_bytes()); // pointer_to_symbol_table
    buf.extend_from_slice(&0u32.to_le_bytes()); // number_of_symbols
    buf.extend_from_slice(&OPT_HEADER_SIZE.to_le_bytes());
    buf.extend_from_slice(&0u16.to_le_bytes()); // characteristics

    // optional header standard fields
    buf.extend_from_slice(&0x20Bu16.to_le_bytes()); // magic: pe32+
    buf.push(0); // major_linker_version
    buf.push(0); // minor_linker_version
    buf.extend_from_slice(&0u32.to_le_bytes()); // size_of_code
    buf.extend_from_slice(&0u32.to_le_bytes()); // size_of_initialized_data
    buf.extend_from_slice(&0u32.to_le_bytes()); // size_of_uninitialized_data
    buf.extend_from_slice(&ENTRY_POINT.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes()); // base_of_code

    // optional header windows fields
    buf.extend_from_slice(&IMAGE_BASE.to_le_bytes());
    buf.extend_from_slice(&0x1000u32.to_le_bytes()); // section_alignment
    buf.extend_from_slice(&0x200u32.to_le_bytes()); // file_alignment
    buf.extend_from_slice(&0u16.to_le_bytes()); // major_os_version
    buf.extend_from_slice(&0u16.to_le_bytes()); // minor_os_version
    buf.extend_from_slice(&0u16.to_le_bytes()); // major_image_version
    buf.extend_from_slice(&0u16.to_le_bytes()); // minor_image_version
    buf.extend_from_slice(&6u16.to_le_bytes()); // major_subsystem_version
    buf.extend_from_slice(&0u16.to_le_bytes()); // minor_subsystem_version
    buf.extend_from_slice(&0u32.to_le_bytes()); // win32_version_value
    buf.extend_from_slice(&SIZE_OF_IMAGE.to_le_bytes());
    buf.extend_from_slice(&SIZE_OF_HEADERS.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes()); // checksum
    buf.extend_from_slice(&SUBSYSTEM.to_le_bytes());
    buf.extend_from_slice(&0u16.to_le_bytes()); // dll_characteristics
    buf.extend_from_slice(&0u64.to_le_bytes()); // size_of_stack_reserve
    buf.extend_from_slice(&0u64.to_le_bytes()); // size_of_stack_commit
    buf.extend_from_slice(&0u64.to_le_bytes()); // size_of_heap_reserve
    buf.extend_from_slice(&0u64.to_le_bytes()); // size_of_heap_commit
    buf.extend_from_slice(&0u32.to_le_bytes()); // loader_flags
    buf.extend_from_slice(&0u32.to_le_bytes()); // number_of_rva_and_sizes

    buf
}

#[test]
fn fields_agree_with_goblin() {
    let image = build_valid_image();

    let parsed = unsafe { PeHeaders::parse(image.as_ptr(), image.len()) }
        .expect("in-tree parser should accept a well-formed image");
    let pe = PE::parse(&image).expect("goblin should accept the same image");

    let coff = &pe.header.coff_header;
    assert_eq!(parsed.coff.machine, coff.machine);
    assert_eq!(parsed.coff.number_of_sections, coff.number_of_sections);
    assert_eq!(
        parsed.coff.size_of_optional_header,
        coff.size_of_optional_header
    );
    assert_eq!(parsed.coff.characteristics, coff.characteristics);

    let opt = pe
        .header
        .optional_header
        .expect("goblin should also find an optional header");
    assert_eq!(parsed.optional.magic, opt.standard_fields.magic);
    assert_eq!(
        parsed.optional.address_of_entry_point as u64,
        opt.standard_fields.address_of_entry_point
    );
    assert_eq!(parsed.optional.image_base, opt.windows_fields.image_base);
    assert_eq!(
        parsed.optional.section_alignment,
        opt.windows_fields.section_alignment
    );
    assert_eq!(
        parsed.optional.file_alignment,
        opt.windows_fields.file_alignment
    );
    assert_eq!(
        parsed.optional.size_of_image,
        opt.windows_fields.size_of_image
    );
    assert_eq!(
        parsed.optional.size_of_headers,
        opt.windows_fields.size_of_headers
    );
    assert_eq!(parsed.optional.checksum, opt.windows_fields.check_sum);
    assert_eq!(parsed.optional.subsystem, opt.windows_fields.subsystem);
    assert_eq!(
        parsed.optional.number_of_rva_and_sizes,
        opt.windows_fields.number_of_rva_and_sizes
    );

    // goblin recomputes these from the same header; cross-check them too.
    assert_eq!(parsed.optional.address_of_entry_point as usize, pe.entry);
    assert_eq!(parsed.optional.image_base as usize, pe.image_base);
}

#[test]
fn truncated_optional_header_is_rejected_by_both() {
    let image = build_valid_image();
    let opt_offset = E_LFANEW as usize + 24;
    let full_len = opt_offset + OPT_HEADER_SIZE as usize;
    assert_eq!(image.len(), full_len, "built image must be header-sized");

    // each length lands inside the 112-byte optional header the in-tree
    // parser reads; this is the same boundary an off-by-16 guard once missed.
    for len in [
        opt_offset,
        opt_offset + 1,
        opt_offset + 96,
        opt_offset + 100,
        opt_offset + 111,
    ] {
        let truncated = &image[..len];

        let in_tree = unsafe { PeHeaders::parse(truncated.as_ptr(), truncated.len()) };
        assert!(
            in_tree.is_err(),
            "in-tree parser accepted a {len}-byte image"
        );

        let via_goblin = PE::parse(truncated);
        assert!(via_goblin.is_err(), "goblin accepted a {len}-byte image");
    }
}
