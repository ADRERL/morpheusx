//! kats + differential for the el torito 16-bit boot-catalog checksum.

use iso9660::utils::checksum::{checksum_16, verify_checksum_16};

// wrapping sum of le 16-bit words; trailing odd byte ignored, matching
// the chunks_exact(2) semantics of the impl under test
fn cksum16_ref(data: &[u8]) -> u16 {
    let mut sum: u16 = 0;
    let mut i = 0;
    while i + 2 <= data.len() {
        sum = sum.wrapping_add(u16::from_le_bytes([data[i], data[i + 1]]));
        i += 2;
    }
    sum
}

#[test]
fn el_torito_kat() {
    assert_eq!(checksum_16(&[0x01, 0x00, 0x02, 0x00]), 0x0003);
    // 0x0001 + 0xffff wraps to 0, so validation passes
    assert_eq!(checksum_16(&[0x01, 0x00, 0xFF, 0xFF]), 0x0000);
    assert!(verify_checksum_16(&[0x01, 0x00, 0xFF, 0xFF]));
    // a validation entry needs the word sum to be zero
    assert_eq!(checksum_16(&[0xAA, 0x55, 0x55, 0xAA]), 0xFFFF);
    assert!(!verify_checksum_16(&[0xAA, 0x55, 0x55, 0xAA]));
    assert_eq!(checksum_16(&[]), 0x0000);
    assert!(verify_checksum_16(&[]));
}

#[test]
fn trailing_odd_byte_ignored() {
    let even = [0x11u8, 0x22, 0x33, 0x44];
    let mut odd = [0u8; 5];
    odd[..4].copy_from_slice(&even);
    odd[4] = 0x99;
    assert_eq!(checksum_16(&odd), checksum_16(&even));
}

#[test]
fn differential_vs_reference() {
    // fixed-seed lcg keeps the sweep deterministic
    let mut state: u64 = 0x9E37_79B9_7F4A_7C15;
    let mut next = || {
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (state >> 33) as u8
    };
    let mut buf = [0u8; 256];
    for len in 0..=256usize {
        for b in buf.iter_mut().take(len) {
            *b = next();
        }
        let s = &buf[..len];
        assert_eq!(
            checksum_16(s),
            cksum16_ref(s),
            "checksum_16 differs at len {len}"
        );
        assert_eq!(verify_checksum_16(s), checksum_16(s) == 0);
    }
}
