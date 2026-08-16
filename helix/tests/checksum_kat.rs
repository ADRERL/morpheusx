//! kats + differential vs the crc crate for helixfs crc32c / crc64-xz / fnv1a-64.

use crc::{Crc, CRC_32_ISCSI, CRC_64_XZ};
use morpheus_helix::crc::{crc32c, crc32c_two, crc64, fnv1a_64};

const CRC32C: Crc<u32> = Crc::<u32>::new(&CRC_32_ISCSI);
const CRC64: Crc<u64> = Crc::<u64>::new(&CRC_64_XZ);

#[test]
fn crc32c_kat() {
    assert_eq!(crc32c(b""), 0x0000_0000);
    assert_eq!(crc32c(b"123456789"), 0xE306_9283); // published check value
    assert_eq!(crc32c(b"hello"), 0x9A71_BB4C);
    assert_eq!(crc32c(b"world"), 0x31AA_814E);
    assert_eq!(
        crc32c(b"The quick brown fox jumps over the lazy dog"),
        0x2262_0404
    );
}

#[test]
fn crc64_kat() {
    assert_eq!(crc64(b""), 0x0000_0000_0000_0000);
    assert_eq!(crc64(b"123456789"), 0x995D_C9BB_DF19_39FA); // published check value
    assert_eq!(crc64(b"hello"), 0x9B1E_DAE5_DBB9_37B1);
    assert_eq!(crc64(b"world"), 0x3E10_FCFA_54E1_58F8);
}

#[test]
fn fnv1a_64_kat() {
    assert_eq!(fnv1a_64(b""), 0xCBF2_9CE4_8422_2325); // offset basis
    assert_eq!(fnv1a_64(b"a"), 0xAF63_DC4C_8601_EC8C);
    assert_eq!(fnv1a_64(b"b"), 0xAF63_DF4C_8601_F1A5);
    assert_eq!(fnv1a_64(b"c"), 0xAF63_DE4C_8601_EFF2);
    assert_eq!(fnv1a_64(b"foobar"), 0x8594_4171_F739_67E8);
    assert_eq!(fnv1a_64(b"hello"), 0xA430_D846_80AA_BD0B);
}

#[test]
fn crc32c_two_concat_property() {
    let cases: &[(&[u8], &[u8])] = &[
        (b"", b""),
        (b"hello", b" world"),
        (b"123", b"456789"),
        (b"", b"123456789"),
        (b"123456789", b""),
    ];
    for (a, b) in cases {
        let mut cat = Vec::with_capacity(a.len() + b.len());
        cat.extend_from_slice(a);
        cat.extend_from_slice(b);
        assert_eq!(crc32c_two(a, b), crc32c(&cat), "two vs concat mismatch");
        assert_eq!(
            crc32c_two(a, b),
            CRC32C.checksum(&cat),
            "two vs crc-crate mismatch"
        );
    }
    assert_eq!(crc32c_two(b"hello", b" world"), 0xC994_65AA);
}

// fold-form fnv-1a-64, structurally independent of the crate impl
fn fnv_ref(data: &[u8]) -> u64 {
    data.iter().fold(0xCBF2_9CE4_8422_2325u64, |h, &b| {
        (h ^ b as u64).wrapping_mul(0x0000_0100_0000_01B3)
    })
}

#[test]
fn differential_vs_reference() {
    // fixed-seed lcg keeps the sweep deterministic
    let mut state: u64 = 0x2545_F491_4F6C_DD1D;
    let mut next = || {
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (state >> 33) as u8
    };
    let mut buf = [0u8; 512];
    for len in 0..=512usize {
        for b in buf.iter_mut().take(len) {
            *b = next();
        }
        let s = &buf[..len];
        assert_eq!(crc32c(s), CRC32C.checksum(s), "crc32c differs at len {len}");
        assert_eq!(crc64(s), CRC64.checksum(s), "crc64 differs at len {len}");
        assert_eq!(fnv1a_64(s), fnv_ref(s), "fnv1a_64 differs at len {len}");
    }
}
