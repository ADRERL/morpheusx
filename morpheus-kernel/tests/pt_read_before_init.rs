//! any process-table read before init_process_table must panic. own test file:
//! the guard flags are process-global, so this must run with no prior init.

#[test]
#[should_panic(expected = "process table read before init_process_table")]
fn read_before_init_panics() {
    unsafe {
        let _ = morpheus_kernel::schedular::process_table();
    }
}
