//! checks WaitStatus against glibc <sys/wait.h> for every 16-bit wait(2)
//! status word. the oracle table is generated at build time by build.rs from
//! oracle_probes/gen_wstatus_oracle.c.
//!
//! row columns: 0 WIFEXITED, 1 WIFSIGNALED, 2 WIFSTOPPED, 3 WIFCONTINUED,
//! 4 WEXITSTATUS, 5 WTERMSIG, 6 WSTOPSIG, 7 WCOREDUMP

use morpheus_foundation::types::WaitStatus as W;

// pub static WSTATUS_ORACLE: [[u8; 8]; 65536]
include!(concat!(env!("OUT_DIR"), "/gen_wstatus_oracle_glibc.rs"));

// exit_status/term_sig/stop_sig return i32, so mask before the u8 cast
fn classify(si: i32) -> [u8; 8] {
    [
        W::exited(si) as u8,
        W::signaled(si) as u8,
        W::stopped(si) as u8,
        W::continued(si) as u8,
        (W::exit_status(si) & 0xff) as u8,
        (W::term_sig(si) & 0xff) as u8,
        (W::stop_sig(si) & 0xff) as u8,
        W::core_dumped(si) as u8,
    ]
}

#[test]
fn oracle_spot_check_known_words() {
    assert_eq!(WSTATUS_ORACLE.len(), 65536);
    let known: [(usize, [u8; 8]); 7] = [
        (0x0000, [1, 0, 0, 0, 0, 0, 0, 0]),       // exit(0)
        (0x0100, [1, 0, 0, 0, 1, 0, 1, 0]),       // exit(1)
        (0x0009, [0, 1, 0, 0, 0, 9, 0, 0]),       // killed by sigkill, no core
        (0x008b, [0, 1, 0, 0, 0, 11, 0, 1]),      // sigsegv with core dump (0x80)
        (0x007f, [0, 0, 1, 0, 0, 127, 0, 0]),     // stopped, stop-signal 0
        (0x137f, [0, 0, 1, 0, 19, 127, 19, 0]),   // stopped, stop-signal 19
        (0xffff, [0, 0, 0, 1, 255, 127, 255, 1]), // continued
    ];
    for (word, expected) in known {
        assert_eq!(WSTATUS_ORACLE[word], expected, "oracle row {word:#06x}");
        assert_eq!(
            classify(word as i32),
            expected,
            "WaitStatus row {word:#06x}"
        );
    }
}

// pins the signaled() i8-before-shift fix: 0x7f classifies stopped, not signaled
#[test]
fn oracle_total_decode_all_65536_words() {
    for s in 0..=0xffffu32 {
        let si = s as i32;
        assert_eq!(
            classify(si),
            WSTATUS_ORACLE[s as usize],
            "WaitStatus disagrees with glibc for status word {s:#06x}"
        );
    }
}

// the kernel encodes (sig & 0x7f) | ((code & 0xff) << 8). sig stays below 64
// so the built word never hits the 0x7f stopped/continued encodings
#[test]
fn encode_wstatus_roundtrip() {
    for sig in 0..64i32 {
        for code in 0..256i32 {
            let word = (sig & 0x7f) | ((code & 0xff) << 8);
            let row = WSTATUS_ORACLE[word as usize];

            assert_eq!(
                classify(word),
                row,
                "encode sig={sig} code={code} word={word:#06x}: helper vs glibc"
            );

            if sig == 0 {
                assert_eq!(row[0], 1, "sig=0 code={code}: expected WIFEXITED");
                assert_eq!(row[4], code as u8, "sig=0 code={code}: WEXITSTATUS");
                assert_eq!(row[1], 0, "sig=0 code={code}: not WIFSIGNALED");
                assert_eq!(row[2], 0, "sig=0 code={code}: not WIFSTOPPED");
                assert_eq!(
                    W::exit_status(word),
                    code,
                    "sig=0 code={code}: exit_status()"
                );
                assert!(W::exited(word), "sig=0 code={code}: exited()");
            } else {
                assert_eq!(row[1], 1, "sig={sig} code={code}: expected WIFSIGNALED");
                assert_eq!(row[5], sig as u8, "sig={sig} code={code}: WTERMSIG");
                assert_eq!(row[0], 0, "sig={sig} code={code}: not WIFEXITED");
                assert_eq!(row[2], 0, "sig={sig} code={code}: not WIFSTOPPED");
                assert_eq!(W::term_sig(word), sig, "sig={sig} code={code}: term_sig()");
                assert!(W::signaled(word), "sig={sig} code={code}: signaled()");
            }
        }
    }
}
