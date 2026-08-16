//! after init_process_table the table reads as all-None slots.

use morpheus_kernel::schedular::{init_process_table, process_table};

#[test]
fn init_then_read_all_none() {
    unsafe {
        init_process_table();
        let table = process_table();
        assert_eq!(table.len(), morpheus_kernel::process::MAX_PROCESSES);
        assert!(table.iter().all(|slot| slot.is_none()));
        // slots are writable through the accessor
        table[0] = None;
        assert!(table[0].is_none());
    }
}
