//! parses a hand-built spec-valid gpt with both the in-tree byte-offset parser
//! and the gpt_disk_io/gpt_disk_types oracle, and checks the free-space /
//! range-free math against hand-computed answers.

use gpt_disk_io::Disk;
use morpheus_block::transfer::disk::{DiskError, GptOps};
use morpheus_block::MemBlockDevice;

// lbas, 512-byte sectors
const SECTOR: usize = 512;
const TOTAL_SECTORS: usize = 2048; // >= ENTRY_LBA+32; GptOps always reads 32 array sectors
const HDR_LBA: usize = 1;
const ENTRY_LBA: usize = 2;
const NUM_ENTRIES: u32 = 128;
const ENTRY_SIZE: u32 = 128;
const MY_LBA: u64 = 1;
const ALT_LBA: u64 = 2047;
const FIRST_USABLE: u64 = 34;
const LAST_USABLE: u64 = 2014;

// p0 = real esp type guid (on-disk order), p1 = synthetic nonzero guid
const GUID_A: [u8; 16] = [
    0x28, 0x73, 0x2A, 0xC1, 0x1F, 0xF8, 0xD2, 0x11, 0xBA, 0x4B, 0x00, 0xA0, 0xC9, 0x3E, 0xC9, 0x3B,
];
const GUID_B: [u8; 16] = [
    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10,
];
const P0_START: u64 = 40;
const P0_END: u64 = 99;
const P1_START: u64 = 200;
const P1_END: u64 = 299;

// same crc-32 as the in-tree impl (reflected poly 0xedb88320)
fn crc32(data: &[u8]) -> u32 {
    const POLY: u32 = 0xEDB8_8320;
    let mut crc = 0xFFFF_FFFFu32;
    for &b in data {
        crc ^= b as u32;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ POLY;
            } else {
                crc >>= 1;
            }
        }
    }
    !crc
}

fn wu32(b: &mut [u8], off: usize, v: u32) {
    b[off..off + 4].copy_from_slice(&v.to_le_bytes());
}
fn wu64(b: &mut [u8], off: usize, v: u64) {
    b[off..off + 8].copy_from_slice(&v.to_le_bytes());
}

struct Img {
    buf: Vec<u8>,
}
impl Img {
    fn dev(&mut self) -> MemBlockDevice {
        // SAFETY: buf is a live heap region that outlives every device handed
        // out; callers only read, and only within total_bytes
        unsafe { MemBlockDevice::new(self.buf.as_mut_ptr(), self.buf.len(), SECTOR as u32) }
    }
}

fn write_entry(buf: &mut [u8], idx: usize, start: u64, end: u64, tguid: [u8; 16]) {
    let base = ENTRY_LBA * SECTOR + idx * ENTRY_SIZE as usize;
    buf[base..base + 16].copy_from_slice(&tguid); // partition type GUID @0
    for i in 0..16 {
        buf[base + 16 + i] = 0x10 + idx as u8; // unique GUID @16 (nonzero)
    }
    wu64(buf, base + 32, start); // starting LBA @32
    wu64(buf, base + 40, end); // ending LBA @40
}

// correct header + array crcs, two used partitions p0 (40..=99) and p1 (200..=299)
fn build_valid_gpt() -> Img {
    let mut buf = vec![0u8; TOTAL_SECTORS * SECTOR];
    buf[510] = 0x55; // protective-MBR boot signature
    buf[511] = 0xAA;

    let h = HDR_LBA * SECTOR;
    buf[h..h + 8].copy_from_slice(b"EFI PART"); // signature @0
    wu32(&mut buf, h + 8, 0x0001_0000); // revision @8
    wu32(&mut buf, h + 12, 92); // header_size @12
                                // header_crc32 @16 -> filled last (left zero for now)
                                // reserved @20 = 0
    wu64(&mut buf, h + 24, MY_LBA); // my_lba @24
    wu64(&mut buf, h + 32, ALT_LBA); // alternate_lba @32
    wu64(&mut buf, h + 40, FIRST_USABLE); // first_usable_lba @40
    wu64(&mut buf, h + 48, LAST_USABLE); // last_usable_lba @48
    for i in 0..16 {
        buf[h + 56 + i] = 0xA0 + i as u8; // disk_guid @56 (nonzero)
    }
    wu64(&mut buf, h + 72, ENTRY_LBA as u64); // partition_entry_lba @72
    wu32(&mut buf, h + 80, NUM_ENTRIES); // num_entries @80
    wu32(&mut buf, h + 84, ENTRY_SIZE); // entry_size @84

    write_entry(&mut buf, 0, P0_START, P0_END, GUID_A);
    write_entry(&mut buf, 1, P1_START, P1_END, GUID_B);

    // array crc over the whole 128*128 = 16384-byte entry array (32 sectors)
    let a0 = ENTRY_LBA * SECTOR;
    let alen = NUM_ENTRIES as usize * ENTRY_SIZE as usize;
    let acrc = crc32(&buf[a0..a0 + alen]);
    wu32(&mut buf, h + 88, acrc); // array_crc32 @88 (must be set BEFORE header crc)

    // header crc over 92 bytes with the crc field (@16) still zero
    let hcrc = crc32(&buf[h..h + 92]);
    wu32(&mut buf, h + 16, hcrc);

    Img { buf }
}

#[test]
fn crc32_matches_ieee_check_value() {
    assert_eq!(crc32(b""), 0x0000_0000);
    assert_eq!(crc32(b"123456789"), 0xCBF4_3926);
}

#[test]
fn fixture_crc_fields_are_correct() {
    let img = build_valid_gpt();
    let h = HDR_LBA * SECTOR;
    let a0 = ENTRY_LBA * SECTOR;
    let alen = NUM_ENTRIES as usize * ENTRY_SIZE as usize;

    let stored_acrc = u32::from_le_bytes(img.buf[h + 88..h + 92].try_into().unwrap());
    assert_eq!(stored_acrc, crc32(&img.buf[a0..a0 + alen]));
    assert_ne!(stored_acrc, 0);

    let mut hdr = img.buf[h..h + 92].to_vec();
    let stored_hcrc = u32::from_le_bytes(hdr[16..20].try_into().unwrap());
    hdr[16..20].copy_from_slice(&[0u8; 4]);
    assert_eq!(stored_hcrc, crc32(&hdr));
    assert_ne!(stored_hcrc, 0);
}

#[test]
fn differential_scan_matches_gpt_disk_types() {
    let mut img = build_valid_gpt();

    let mut dev = img.dev();
    let (parts, count) = GptOps::scan_partitions(&mut dev).expect("ours: scan ok");
    let mut ours: Vec<(u64, u64, [u8; 16])> = Vec::new();
    for p in parts.iter().take(count) {
        ours.push((p.start_lba, p.end_lba, p.type_guid));
    }

    let dev2 = img.dev();
    let mut disk = Disk::new(dev2).expect("oracle: disk new");
    let header = disk
        .read_primary_gpt_header(&mut [0u8; 512])
        .expect("oracle: header");
    let layout = header
        .get_partition_entry_array_layout()
        .expect("oracle: layout");
    let mut ebuf = [0u8; 512];
    let mut oracle: Vec<(u64, u64, [u8; 16])> = Vec::new();
    for e in disk
        .gpt_partition_entry_array_iter(layout, &mut ebuf)
        .expect("oracle: iter")
    {
        let e = e.expect("oracle: entry");
        if e.is_used() {
            // copy packed fields to locals before use (repr(c, packed))
            let ptype = e.partition_type_guid;
            oracle.push((
                e.starting_lba.to_u64(),
                e.ending_lba.to_u64(),
                ptype.0.to_bytes(),
            ));
        }
    }

    // copy the packed signature field to a local before comparing
    let sig_field = header.signature;
    let sig_bytes: [u8; 8] = sig_field.0 .0;
    assert_eq!(sig_bytes, *b"EFI PART");
    assert_eq!(header.first_usable_lba.to_u64(), FIRST_USABLE);
    assert_eq!(header.last_usable_lba.to_u64(), LAST_USABLE);
    assert_eq!(header.partition_entry_lba.to_u64(), ENTRY_LBA as u64);
    assert_eq!(header.number_of_partition_entries.to_u32(), NUM_ENTRIES);
    assert_eq!(header.size_of_partition_entry.to_u32(), ENTRY_SIZE);

    assert_eq!(count, 2);
    assert_eq!(oracle.len(), 2);
    assert_eq!(ours, oracle);
    assert_eq!(ours[0], (P0_START, P0_END, GUID_A));
    assert_eq!(ours[1], (P1_START, P1_END, GUID_B));
}

#[test]
fn find_free_space_returns_largest_gap() {
    let mut img = build_valid_gpt();
    let mut dev = img.dev();
    // gaps: [34,39]=6, [100,199]=100, [300,2014]=1715 (largest)
    let range = GptOps::find_free_space(&mut dev).expect("free space");
    assert_eq!(range, (300, 2014));
}

#[test]
fn verify_range_free_data_loss_cases() {
    let mut img = build_valid_gpt();

    // exact hole between P0 and P1 -> free
    {
        let mut d = img.dev();
        assert_eq!(GptOps::verify_range_free(&mut d, 100, 199), Ok(true));
    }
    // overlap P0 by exactly one sector (99) -> not free
    {
        let mut d = img.dev();
        assert_eq!(GptOps::verify_range_free(&mut d, 99, 150), Ok(false));
    }
    // overlap P1 by exactly one sector (200) -> not free
    {
        let mut d = img.dev();
        assert_eq!(GptOps::verify_range_free(&mut d, 100, 200), Ok(false));
    }
    // below first_usable (34) -> not free
    {
        let mut d = img.dev();
        assert_eq!(GptOps::verify_range_free(&mut d, 10, 20), Ok(false));
    }
    // above last_usable (2014) -> not free
    {
        let mut d = img.dev();
        assert_eq!(GptOps::verify_range_free(&mut d, 2000, 2020), Ok(false));
    }
}

#[test]
fn invalid_signature_is_invalid_gpt_not_panic() {
    let mut img = build_valid_gpt();
    let h = HDR_LBA * SECTOR;
    img.buf[h..h + 8].copy_from_slice(b"NOTGPT!!");
    let mut dev = img.dev();
    // map to () because ([PartitionInfo; 16], usize) is not PartialEq
    assert_eq!(
        GptOps::scan_partitions(&mut dev).map(|_| ()),
        Err(DiskError::InvalidGpt)
    );
}

#[test]
fn invalid_entry_size_is_invalid_gpt_not_panic() {
    let mut img = build_valid_gpt();
    let h = HDR_LBA * SECTOR;
    wu32(&mut img.buf, h + 84, 96); // entry_size 96 != 128
    {
        let mut dev = img.dev();
        assert_eq!(
            GptOps::scan_partitions(&mut dev).map(|_| ()),
            Err(DiskError::InvalidGpt)
        );
    }
    {
        let mut dev = img.dev();
        assert_eq!(
            GptOps::find_free_space(&mut dev),
            Err(DiskError::InvalidGpt)
        );
    }
}
