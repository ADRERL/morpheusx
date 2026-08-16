//! BUG: RB-0004 import_uefi_map computed map_size / descriptor_size without
//! guarding descriptor_size == 0, panicking during early boot on a bad
//! uefi hand-off.

use morpheus_hal_x86_64::memory::MemoryRegistry;

// BUG: RB-0004 -- descriptor_size == 0 must not divide by zero.
#[test]
fn zero_descriptor_size_returns_without_dividing_by_zero() {
    let mut reg = MemoryRegistry::new();
    // null map_ptr is safe here: the guard returns before the division and
    // before any descriptor deref. if the guard regresses this panics.
    unsafe {
        reg.import_uefi_map(
            core::ptr::null(),
            4096, // non-zero map_size makes the division reachable if unguarded
            0,    // the bug trigger
            0,
            0,
            0,
            0,
            0,
            &[],
        );
    }
    // no panic == pass
}
