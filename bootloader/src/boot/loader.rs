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
    image_handle: *mut (),
    kernel_data: &[u8],
    cmdline: &str,
) -> ! {
    morpheus_core::logger::log("Parsing kernel...");
    
    // Parse kernel image
    let kernel = match KernelImage::parse(kernel_data) {
        Ok(k) => k,
        Err(_) => panic!("Failed to parse kernel"),
    };

    morpheus_core::logger::log("Allocating kernel memory...");
    
    // Allocate memory for kernel
    let kernel_dest = match allocate_kernel_memory(boot_services, &kernel) {
        Ok(d) => d,
        Err(_) => panic!("Failed to allocate kernel memory"),
    };

    morpheus_core::logger::log("Loading kernel to memory...");
    
    // Load kernel into memory
    let _ = load_kernel_image(&kernel, kernel_dest);

    morpheus_core::logger::log("Setting up boot params...");
    
    // Allocate boot parameters
    let boot_params = match allocate_boot_params(boot_services) {
        Ok(b) => b,
        Err(_) => panic!("Failed to allocate boot params"),
    };

    // CRITICAL: Copy the setup header from kernel to boot params
    // The kernel expects to see its own setup header in boot_params
    (*boot_params).copy_setup_header(kernel.setup_header_ptr());
    
    // Setup boot params
    (*boot_params).set_loader_type(0xFF); // 0xFF = undefined loader
    (*boot_params).set_video_mode(); // Basic text mode
    
    // Allocate and set command line
    if !cmdline.is_empty() {
        if let Ok(cmdline_ptr) = allocate_cmdline(boot_services, cmdline) {
            (*boot_params).set_cmdline(cmdline_ptr as u32);
        }
    }

    morpheus_core::logger::log("Exiting boot services...");
    
    // Get memory map before exiting boot services
    let mut map_size: usize = 0;
    let mut map_key: usize = 0;
    let mut descriptor_size: usize = 0;
    let mut descriptor_version: u32 = 0;
    
    // First call to get size
    let _ = (boot_services.get_memory_map)(
        &mut map_size,
        core::ptr::null_mut(),
        &mut map_key,
        &mut descriptor_size,
        &mut descriptor_version,
    );
    
    // Exit boot services - kernel now owns hardware
    // This terminates UEFI runtime and gives full control to kernel
    let exit_status = (boot_services.exit_boot_services)(
        image_handle,
        map_key,
    );
    
    // If ExitBootServices fails, retry once
    if exit_status != 0 {
        // Get updated map key
        let _ = (boot_services.get_memory_map)(
            &mut map_size,
            core::ptr::null_mut(),
            &mut map_key,
            &mut descriptor_size,
            &mut descriptor_version,
        );
        let _ = (boot_services.exit_boot_services)(
            image_handle,
            map_key,
        );
    }

    // CRITICAL: After ExitBootServices, we can't use UEFI services anymore
    // No more logging, no more panics - we're on our own
    
    // Jump to kernel (never returns)
    // kernel still has the setup header from original bzImage
    // kernel_dest is where we actually loaded the kernel code
    boot_kernel(&kernel, boot_params, system_table, kernel_dest)
}
