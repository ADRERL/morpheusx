// hand-builds a valid gpt (protective mbr + header + entry array, correct
// crcs) in a vec-backed block device, runs it through scan_partitions /
// find_free_space, and cross-checks the parse against gpt_disk_types read
// directly off the same bytes. also pins the free-space gap math by hand.

use gpt_disk_io::{BlockIo, Disk};
use gpt_disk_types::{BlockSize, GptPartitionType, Lba};
use morpheus_storage_format::disk::gpt_ops::{find_free_space, scan_partitions, FreeRegion};
use morpheus_storage_format::disk::partition::{PartitionTable, PartitionType};

const SECTOR: usize = 512;
const TOTAL_SECTORS: usize = 2048;
const HDR_LBA: usize = 1;
const ENTRY_LBA: usize = 2;
const NUM_ENTRIES: u32 = 128;
const ENTRY_SIZE: u32 = 128;
const MY_LBA: u64 = 1;
const ALT_LBA: u64 = 2047;
const FIRST_USABLE: u64 = 34;
const LAST_USABLE: u64 = 2014;

const P0_START: u64 = 40;
const P0_END: u64 = 99;
const P1_START: u64 = 200;
const P1_END: u64 = 299;

// same crc-32 as the other gpt fixture (reflected poly 0xedb88320)
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

// plain vec-backed BlockIo, no unsafe: read/write are slice copies bounds
// checked by the vec itself.
struct VecBlockIo(Vec<u8>);

impl BlockIo for VecBlockIo {
    type Error = core::convert::Infallible;

    fn block_size(&self) -> BlockSize {
        BlockSize::BS_512
    }

    fn num_blocks(&mut self) -> Result<u64, Self::Error> {
        Ok((self.0.len() / SECTOR) as u64)
    }

    fn read_blocks(&mut self, start_lba: Lba, dst: &mut [u8]) -> Result<(), Self::Error> {
        let start = start_lba.0 as usize * SECTOR;
        dst.copy_from_slice(&self.0[start..start + dst.len()]);
        Ok(())
    }

    fn write_blocks(&mut self, start_lba: Lba, src: &[u8]) -> Result<(), Self::Error> {
        let start = start_lba.0 as usize * SECTOR;
        self.0[start..start + src.len()].copy_from_slice(src);
        Ok(())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

struct Img {
    buf: Vec<u8>,
}

impl Img {
    fn dev(&self) -> VecBlockIo {
        VecBlockIo(self.buf.clone())
    }
}

fn write_entry(buf: &mut [u8], idx: usize, start: u64, end: u64, tguid: [u8; 16]) {
    let base = ENTRY_LBA * SECTOR + idx * ENTRY_SIZE as usize;
    buf[base..base + 16].copy_from_slice(&tguid); // partition type guid @0
    for i in 0..16 {
        buf[base + 16 + i] = 0x10 + idx as u8; // unique guid @16, just needs to be nonzero
    }
    wu64(buf, base + 32, start);
    wu64(buf, base + 40, end);
}

// header + entry array with correct crcs for whatever entries are given.
fn build_gpt(entries: &[(u64, u64, [u8; 16])]) -> Img {
    let mut buf = vec![0u8; TOTAL_SECTORS * SECTOR];
    buf[510] = 0x55; // protective-mbr boot signature
    buf[511] = 0xAA;

    let h = HDR_LBA * SECTOR;
    buf[h..h + 8].copy_from_slice(b"EFI PART");
    wu32(&mut buf, h + 8, 0x0001_0000); // revision
    wu32(&mut buf, h + 12, 92); // header_size
    wu64(&mut buf, h + 24, MY_LBA);
    wu64(&mut buf, h + 32, ALT_LBA);
    wu64(&mut buf, h + 40, FIRST_USABLE);
    wu64(&mut buf, h + 48, LAST_USABLE);
    for i in 0..16 {
        buf[h + 56 + i] = 0xA0 + i as u8; // disk guid, just needs to be nonzero
    }
    wu64(&mut buf, h + 72, ENTRY_LBA as u64);
    wu32(&mut buf, h + 80, NUM_ENTRIES);
    wu32(&mut buf, h + 84, ENTRY_SIZE);

    for (idx, (start, end, tguid)) in entries.iter().enumerate() {
        write_entry(&mut buf, idx, *start, *end, *tguid);
    }

    let a0 = ENTRY_LBA * SECTOR;
    let alen = NUM_ENTRIES as usize * ENTRY_SIZE as usize;
    let acrc = crc32(&buf[a0..a0 + alen]);
    wu32(&mut buf, h + 88, acrc); // array_crc32, must be set before header crc

    let hcrc = crc32(&buf[h..h + 92]);
    wu32(&mut buf, h + 16, hcrc);

    Img { buf }
}

fn esp_guid() -> [u8; 16] {
    GptPartitionType::EFI_SYSTEM.0.to_bytes()
}
fn basic_data_guid() -> [u8; 16] {
    GptPartitionType::BASIC_DATA.0.to_bytes()
}

fn build_valid_gpt() -> Img {
    build_gpt(&[
        (P0_START, P0_END, esp_guid()),
        (P1_START, P1_END, basic_data_guid()),
    ])
}

fn contains_range(regions: &[Option<FreeRegion>; 16], start: u64, end: u64) -> bool {
    regions
        .iter()
        .flatten()
        .any(|r| r.start_lba <= start && end <= r.end_lba)
}

#[test]
fn scan_matches_gpt_disk_types_parse() {
    let img = build_valid_gpt();

    let mut table = PartitionTable::new();
    scan_partitions(img.dev(), &mut table, SECTOR).expect("scan ok");
    assert!(table.has_gpt);
    assert_eq!(table.count(), 2);

    let ours: Vec<(u64, u64, PartitionType)> = table
        .iter()
        .map(|p| (p.start_lba, p.end_lba, p.partition_type))
        .collect();

    // parse the same bytes again, independently, straight through gpt_disk_types
    let mut disk = Disk::new(img.dev()).expect("disk new");
    let header = disk
        .read_primary_gpt_header(&mut [0u8; 512])
        .expect("header read");
    let layout = header.get_partition_entry_array_layout().expect("layout");
    let mut ebuf = [0u8; SECTOR];
    let mut parsed: Vec<(u64, u64, [u8; 16])> = Vec::new();
    for e in disk
        .gpt_partition_entry_array_iter(layout, &mut ebuf)
        .expect("entry iter")
    {
        let e = e.expect("entry");
        if e.is_used() {
            let t = e.partition_type_guid;
            parsed.push((
                e.starting_lba.to_u64(),
                e.ending_lba.to_u64(),
                t.0.to_bytes(),
            ));
        }
    }

    assert_eq!(parsed.len(), 2);
    assert_eq!(ours.len(), 2);

    for (i, (start, end, ty)) in ours.iter().enumerate() {
        assert_eq!(*start, parsed[i].0);
        assert_eq!(*end, parsed[i].1);
        let want_bytes = match ty {
            PartitionType::EfiSystem => esp_guid(),
            PartitionType::BasicData => basic_data_guid(),
            other => panic!("unexpected partition type {other:?}"),
        };
        assert_eq!(want_bytes, parsed[i].2);
    }

    assert_eq!(ours[0].0, P0_START);
    assert_eq!(ours[0].1, P0_END);
    assert_eq!(ours[1].0, P1_START);
    assert_eq!(ours[1].1, P1_END);
}

#[test]
fn free_regions_match_hand_computed_gaps() {
    let img = build_valid_gpt();

    // gaps: [34,39]=6, [100,199]=100, [300,2014]=1715 (largest)
    let regions = find_free_space(img.dev(), SECTOR).expect("free space");
    let got: Vec<(u64, u64)> = regions
        .iter()
        .flatten()
        .map(|r| (r.start_lba, r.end_lba))
        .collect();
    assert_eq!(got, vec![(34, 39), (100, 199), (300, 2014)]);

    let largest = regions
        .iter()
        .flatten()
        .max_by_key(|r| r.size_lba())
        .unwrap();
    assert_eq!(largest.start_lba, 300);
    assert_eq!(largest.end_lba, 2014);
    assert_eq!(largest.size_lba(), 1715);
}

#[test]
fn candidate_range_overlap_and_exact_fill() {
    let img = build_valid_gpt();
    let regions = find_free_space(img.dev(), SECTOR).expect("free space");

    // p0 is 40..=99: a candidate starting at 99 eats its last sector
    assert!(!contains_range(&regions, 99, 150));
    // p1 is 200..=299: a candidate ending at 200 eats its first sector
    assert!(!contains_range(&regions, 100, 200));
    // exact hole between p0 and p1, no slack on either side
    assert!(contains_range(&regions, 100, 199));
}

#[test]
fn one_sector_gap_is_reported_zero_gap_is_not() {
    // pa=[40,99], pb=[100,150] adjacent (no gap), pc=[152,300] leaves a
    // single free sector at 151. pins the "current < start" comparison in
    // find_free_space against an off-by-one that would use "<=" instead.
    let img = build_gpt(&[
        (40, 99, esp_guid()),
        (100, 150, esp_guid()),
        (152, 300, esp_guid()),
    ]);

    let regions = find_free_space(img.dev(), SECTOR).expect("free space");
    let got: Vec<(u64, u64)> = regions
        .iter()
        .flatten()
        .map(|r| (r.start_lba, r.end_lba))
        .collect();
    assert_eq!(got, vec![(34, 39), (151, 151), (301, 2014)]);

    let gap = regions
        .iter()
        .flatten()
        .find(|r| r.start_lba == 151)
        .unwrap();
    assert_eq!(gap.end_lba, 151);
    assert_eq!(gap.size_lba(), 1);
}

#[test]
fn bad_entry_size_is_rejected_not_panicked() {
    let mut img = build_valid_gpt();
    let h = HDR_LBA * SECTOR;
    wu32(&mut img.buf, h + 84, 96); // 96 is not a power of two >= 128

    // find_free_space surfaces this as an explicit error
    assert!(matches!(
        find_free_space(img.dev(), SECTOR),
        Err(morpheus_storage_format::disk::gpt_ops::GptError::InvalidHeader)
    ));

    // scan_partitions treats an unparsable header as "no gpt", not an error
    let mut table = PartitionTable::new();
    scan_partitions(img.dev(), &mut table, SECTOR).expect("scan does not error");
    assert!(!table.has_gpt);
    assert_eq!(table.count(), 0);
}

#[test]
fn corrupt_signature_alone_does_not_panic() {
    // scan_partitions / find_free_space only validate entry_size, not the
    // "EFI PART" signature or either crc; a bad signature with an otherwise
    // well-formed header and entries still parses. pinning that here so a
    // panic (not a wrong answer) is the only thing this test would catch.
    let mut img = build_valid_gpt();
    let h = HDR_LBA * SECTOR;
    img.buf[h..h + 8].copy_from_slice(b"NOTGPT!!");

    let mut table = PartitionTable::new();
    let scan_result = scan_partitions(img.dev(), &mut table, SECTOR);
    assert!(scan_result.is_ok());

    let free_result = find_free_space(img.dev(), SECTOR);
    assert!(free_result.is_ok());
}
