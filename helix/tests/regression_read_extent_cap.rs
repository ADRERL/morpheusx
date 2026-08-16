//! BUG: RB-0003 read_extent_file must return FileTooLarge before allocating
//! when file_size exceeds the extent node's mapped coverage (helixfs is not
//! sparse), else a corrupt inode drives an attacker-chosen unbounded alloc.

mod common;

use common::MemBio;
use morpheus_helix::extent::{read_extent_file, write_extent_node};
use morpheus_helix::HelixError;

// one run covering a single 4096-byte fs block: partition at lba 0, data
// region at block 0, 512-byte sectors, extent node at fs block 10 (lba 80)
const PART_LBA: u64 = 0;
const DATA_START: u64 = 0;
const DEV_BS: u32 = 512;
const NODE_BLOCK: u64 = 10;
const BLOCK: u64 = 4096;

fn one_block_extent_disk() -> MemBio {
    let mut dev = MemBio::new(128);
    write_extent_node(
        &mut dev,
        PART_LBA,
        DATA_START,
        DEV_BS,
        NODE_BLOCK,
        &[(0, 1, 1)],
    )
    .expect("write_extent_node");
    dev
}

// BUG: RB-0003 -- file_size beyond mapped extent coverage must be rejected.
#[test]
fn file_size_over_extent_coverage_is_rejected() {
    let mut dev = one_block_extent_disk();
    // tight boundary (covered + 1) and a value that would be a ~1 tib alloc
    for bad in [BLOCK + 1, 1u64 << 40] {
        let r = read_extent_file(&mut dev, PART_LBA, DATA_START, DEV_BS, NODE_BLOCK, bad);
        assert!(
            matches!(r, Err(HelixError::FileTooLarge)),
            "file_size {bad} exceeds covered {BLOCK}; expected FileTooLarge, got {r:?}"
        );
    }
}

// BUG: RB-0003 -- legitimate file_size (<= coverage) must still read back.
#[test]
fn file_size_within_extent_coverage_still_reads() {
    let mut dev = one_block_extent_disk();
    let full = read_extent_file(&mut dev, PART_LBA, DATA_START, DEV_BS, NODE_BLOCK, BLOCK)
        .expect("file_size == coverage must succeed");
    assert_eq!(full.len(), BLOCK as usize);
    let partial = read_extent_file(&mut dev, PART_LBA, DATA_START, DEV_BS, NODE_BLOCK, 2048)
        .expect("file_size < coverage must succeed");
    assert_eq!(partial.len(), 2048);
}
