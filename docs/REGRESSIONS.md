# regression ledger

one row per fixed bug, pinned by a host test. every regression test starts
with a `// BUG: RB-#### <one-line>` tag so `grep -rn 'BUG: RB-'` enumerates
live coverage.

- id: `RB-####`, allocated in order, never reused.
- owning test: fails if the bug returns; `-` when no host test exists yet.
- fix sha: `WORKTREE` while the fix is an uncommitted working-tree edit
  (baseline `fbd30ed7`); update the cell once committed.
- status: `PINNED` (a host test guards it) or `DEFER: <why no host test yet>`.

## ledger

| id | bug | one-line | owning test | fix sha | status |
| --- | --- | --- | --- | --- | --- |
| RB-0001 | PE `OptionalHeader64::parse` OOB read | bounds guard must cover `opt_offset+112` (the `number_of_rva_and_sizes` u32 read at +108), not +96, else a ~16 byte out-of-bounds read | `persistent/src/pe/header/optional_header.rs` `mod bounds_regression` | WORKTREE | PINNED |
| RB-0002 | `WaitStatus::signaled` i8-before-shift | `signaled()` must match glibc `WIFSIGNALED`; a stopped status `0x7f` must not classify as signaled | `morpheus-foundation/src/types.rs` `mod wait_status_regression` | WORKTREE | PINNED |
| RB-0003 | helix `read_extent_file` unbounded alloc | `file_size` beyond the extent node's mapped coverage is a corrupt inode and must return `HelixError::FileTooLarge` before allocating an attacker-chosen size | `helix/tests/regression_read_extent_cap.rs` | WORKTREE | PINNED |
| RB-0004 | hal `import_uefi_map` divide-by-zero | `descriptor_size == 0` must early-return before `map_size / descriptor_size` | `morpheus-hal-x86_64/tests/regression_import_uefi_map.rs` | WORKTREE | PINNED |
| RB-0005 | libmorpheus `surface_map` errno window | must reject the full errno window `[-4095,-1]` via `is_error`, not a narrower `[-256,-1]` check, so a mid-range errno is never handed back as a surface pointer | - | WORKTREE | DEFER: `libmorpheus` sets `[lib] test = false` (its `#[panic_handler]` collides with std under `cargo test`) and `surface_map` issues a real `syscall` instruction with no host seam |
| RB-0006 | xHCI bulk `configure_endpoints` | endpoint-context configuration for bulk IN/OUT during mass-storage bring-up | - | WORKTREE | DEFER: hardware path (xHCI MMIO); covered only by the qemu/real-hw e2e harness, no host unit seam |

## notes

- RB-0001 and RB-0002 predate the inline tag convention and live as
  `#[cfg(test)]` modules inside their crates' source files; they are registered
  here by path. the next change touching those files may prepend the
  `// BUG: RB-000x` line.
- RB-0005: the underlying `is_error` window in `morpheus-foundation/src/errno.rs`
  is host-testable on its own, but that does not reproduce the `surface_map`
  call-site regression, so RB-0005 stays deferred.
