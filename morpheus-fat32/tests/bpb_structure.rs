// bpb boot-sector parsing against a hand-built byte block. Bpb::parse is a
// pure fn over &[u8], so no block device is needed to drive it.

use morpheus_fat32::bpb::Bpb;
use morpheus_fat32::error::Fat32Error;
use proptest::prelude::*;

const SECTOR: usize = 512;

// writes a valid fat32 boot sector; every field lands at its fatgen103
// offset. total_sectors_16 stays zero so the 32-bit field is authoritative.
fn build_sector(
    bytes_per_sector: u16,
    sectors_per_cluster: u8,
    reserved_sectors: u16,
    num_fats: u8,
    total_sectors_32: u32,
    sectors_per_fat_32: u32,
    root_cluster: u32,
) -> [u8; SECTOR] {
    let mut b = [0u8; SECTOR];
    b[11..13].copy_from_slice(&bytes_per_sector.to_le_bytes());
    b[13] = sectors_per_cluster;
    b[14..16].copy_from_slice(&reserved_sectors.to_le_bytes());
    b[16] = num_fats;
    b[17..19].copy_from_slice(&0u16.to_le_bytes()); // root entry count, 0 for fat32
    b[19..21].copy_from_slice(&0u16.to_le_bytes()); // total_16 unused, forces total_32
    b[22..24].copy_from_slice(&0u16.to_le_bytes()); // sectors/fat 16, 0 marks fat32
    b[32..36].copy_from_slice(&total_sectors_32.to_le_bytes());
    b[36..40].copy_from_slice(&sectors_per_fat_32.to_le_bytes());
    b[44..48].copy_from_slice(&root_cluster.to_le_bytes());
    b[82..90].copy_from_slice(b"FAT32   ");
    b[510] = 0x55;
    b[511] = 0xAA;
    b
}

fn valid_sector() -> [u8; SECTOR] {
    build_sector(512, 8, 32, 2, 200_000, 1000, 2)
}

#[test]
fn parses_every_field_from_a_valid_sector() {
    let buf = valid_sector();
    let bpb = Bpb::parse(&buf).expect("valid sector must parse");

    assert_eq!(bpb.bytes_per_sector, 512);
    assert_eq!(bpb.sectors_per_cluster, 8);
    assert_eq!(bpb.reserved_sectors, 32);
    assert_eq!(bpb.num_fats, 2);
    assert_eq!(bpb.sectors_per_fat, 1000);
    assert_eq!(bpb.root_cluster, 2);
    assert_eq!(bpb.total_sectors, 200_000);

    // derived fields, cheap to pin here since the inputs are already known
    assert_eq!(bpb.fat_start_sector(), 32);
    assert_eq!(bpb.data_start_sector(), 32 + 2 * 1000);
    assert_eq!(bpb.bytes_per_cluster(), 512 * 8);
}

#[test]
fn rejects_non_power_of_two_bytes_per_sector() {
    let buf = build_sector(511, 8, 32, 2, 200_000, 1000, 2);
    assert_eq!(Bpb::parse(&buf).unwrap_err(), Fat32Error::BadGeometry);
}

#[test]
fn rejects_zero_sectors_per_cluster() {
    let buf = build_sector(512, 0, 32, 2, 200_000, 1000, 2);
    assert_eq!(Bpb::parse(&buf).unwrap_err(), Fat32Error::BadGeometry);
}

#[test]
fn rejects_missing_boot_signature() {
    let mut buf = valid_sector();
    buf[510] = 0x00;
    buf[511] = 0x00;
    assert_eq!(Bpb::parse(&buf).unwrap_err(), Fat32Error::NotFat32);
}

proptest! {
    // any valid geometry must yield a cluster count that fits the data
    // region it was computed from, and a nonzero cluster size.
    #[test]
    fn cluster_count_stays_in_bounds_for_valid_geometry(
        bytes_per_sector in prop::sample::select(vec![512u16, 1024, 2048, 4096]),
        spc_shift in 0u32..7,
        reserved_sectors in 1u16..64,
        num_fats in 1u8..=2,
        sectors_per_fat in 1u32..2000,
        extra_data_sectors in 0u32..5000,
    ) {
        let sectors_per_cluster = (1u32 << spc_shift) as u8;
        let data_start = reserved_sectors as u32 + num_fats as u32 * sectors_per_fat;
        let total_sectors = data_start + extra_data_sectors + sectors_per_cluster as u32;

        let buf = build_sector(
            bytes_per_sector,
            sectors_per_cluster,
            reserved_sectors,
            num_fats,
            total_sectors,
            sectors_per_fat,
            2,
        );
        let bpb = Bpb::parse(&buf).expect("constructed geometry must be valid");

        let cluster_count = bpb.cluster_count();
        let data_sectors = total_sectors - bpb.data_start_sector();
        prop_assert!(cluster_count * bpb.sectors_per_cluster <= data_sectors);
        prop_assert!(bpb.bytes_per_cluster() > 0);
    }
}
