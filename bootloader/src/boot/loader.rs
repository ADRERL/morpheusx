// Boot orchestrator - high-level API for booting a kernel

use super::{KernelImage, LinuxBootParams, boot_kernel};
use super::memory::{allocate_kernel_memory, allocate_boot_params, allocate_cmdline, load_kernel_image};

pub enum BootError {
    ParseFailed,
    MemoryAllocationFailed,
    LoadFailed,
}

// Boot a Linux kernel from a bzImage in memory
// kernel_data: Raw bzImage file contents
// cmdline: Kernel command line (e.g., "root=/dev/sda1 ro quiet")
// This function never returns - it jumps to kernel
pub unsafe fn boot_linux_kernel(
    boot_services: &crate::BootServices,
    system_table: *mut (),
    kernel_data: &[u8],
    cmdline: &str,
) -> ! {
    // Parse kernel image
    let kernel = match KernelImage::parse(kernel_data) {
        Ok(k) => k,
        Err(_) => panic!("Failed to parse kernel"),
    };

    // Allocate memory for kernel
    let kernel_dest = match allocate_kernel_memory(boot_services, &kernel) {
        Ok(d) => d,
        Err(_) => panic!("Failed to allocate kernel memory"),
    };

    // Load kernel into memory
    let _ = load_kernel_image(&kernel, kernel_dest);

    // Allocate boot parameters
    let boot_params = match allocate_boot_params(boot_services) {
        Ok(b) => b,
        Err(_) => panic!("Failed to allocate boot params"),
    };

    // Setup boot params
    (*boot_params).set_loader_type(0xFF); // 0xFF = undefined loader

    // Allocate and set command line
    if !cmdline.is_empty() {
        if let Ok(cmdline_ptr) = allocate_cmdline(boot_services, cmdline) {
            (*boot_params).set_cmdline(cmdline_ptr as u32);
        }
    }

    // TODO: Exit boot services here
    // This is critical - kernel expects to own hardware
    // For now we skip it for testing

    // Jump to kernel (never returns)
    // Need to create new KernelImage pointing to loaded location
    let loaded_kernel = match KernelImage::parse(
        core::slice::from_raw_parts(kernel_dest, kernel.kernel_size())
    ) {
        Ok(k) => k,
        Err(_) => panic!("Failed to parse loaded kernel"),
    };

    boot_kernel(&loaded_kernel, boot_params, system_table)
}
