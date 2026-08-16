//! the table must stay a flat pointer array; a cleared slot frees its box.

use core::mem::size_of;
use morpheus_kernel::process::{Process, MAX_PROCESSES};
use morpheus_kernel::schedular::process_table;

#[test]
fn table_is_pointer_sized() {
    assert_eq!(
        size_of::<[Option<Box<Process>>; MAX_PROCESSES]>(),
        MAX_PROCESSES * size_of::<usize>()
    );
}

#[test]
fn slot_lifecycle() {
    unsafe {
        let t = process_table();
        assert!(t[7].is_none());
        t[7] = Some(Box::new(Process::empty()));
        assert!(t[7].is_some());
        t[7] = None;
        assert!(t[7].is_none());
    }
}
