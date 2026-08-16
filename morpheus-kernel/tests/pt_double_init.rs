//! a second init_process_table call must panic, not silently wipe live slots.

#[test]
#[should_panic(expected = "init_process_table called twice")]
fn double_init_panics() {
    unsafe {
        morpheus_kernel::schedular::init_process_table();
        morpheus_kernel::schedular::init_process_table();
    }
}
