// Memory management for kernel loading

use super::{KernelImage, LinuxBootParams};

pub enum MemoryError {
    AllocationFailed,
    InvalidAddress,
}

// Allocate memory for kernel at preferred address
pub unsafe fn allocate_kernel_memory(
    boot_services: &crate::BootServices,
    kernel: &KernelImage,
) -> Result<*mut u8, MemoryError> {
    // Get kernel's preferred load address
    let pref_addr = if kernel.is_relocatable() {
        kernel.pref_address()
    } else {
        0x0100_0000 // Default 16MB if not relocatable
    };

    // Calculate pages needed (UEFI uses 4KB pages)
    let kernel_size = kernel.init_size() as usize;
    let pages = (kernel_size + 0xFFF) / 0x1000;

    // Try to allocate at preferred address
    let mut buffer: *mut u8 = core::ptr::null_mut();
    let result = (boot_services.allocate_pages)(
        2, // AllocateAddress
        2, // EfiLoaderData
        pages,
        pref_addr,
    );

    if result == 0 {
        // Success - got preferred address
        return Ok(pref_addr as *mut u8);
    }

    // Fallback: allocate anywhere and hope kernel is relocatable
    if kernel.is_relocatable() {
        let result = (boot_services.allocate_pool)(
            2, // EfiLoaderData
            kernel_size,
            &mut buffer as *mut *mut u8,
        );

        if result == 0 {
            Ok(buffer)
        } else {
            Err(MemoryError::AllocationFailed)
        }
    } else {
        Err(MemoryError::AllocationFailed)
    }
}

// Allocate memory for boot params (zero page)
pub unsafe fn allocate_boot_params(
    boot_services: &crate::BootServices,
) -> Result<*mut LinuxBootParams, MemoryError> {
    let mut buffer: *mut u8 = core::ptr::null_mut();
    let size = core::mem::size_of::<LinuxBootParams>();

    let result = (boot_services.allocate_pool)(
        2, // EfiLoaderData
        size,
        &mut buffer as *mut *mut u8,
    );

    if result == 0 {
        // Zero the structure
        core::ptr::write_bytes(buffer, 0, size);
        Ok(buffer as *mut LinuxBootParams)
    } else {
        Err(MemoryError::AllocationFailed)
    }
}

// Allocate memory for command line string
pub unsafe fn allocate_cmdline(
    boot_services: &crate::BootServices,
    cmdline: &str,
) -> Result<*mut u8, MemoryError> {
    let mut buffer: *mut u8 = core::ptr::null_mut();
    let size = cmdline.len() + 1; // +1 for null terminator

    let result = (boot_services.allocate_pool)(
        2, // EfiLoaderData
        size,
        &mut buffer as *mut *mut u8,
    );

    if result == 0 {
        // Copy string
        core::ptr::copy_nonoverlapping(
            cmdline.as_ptr(),
            buffer,
            cmdline.len(),
        );
        // Null terminate
        *buffer.add(cmdline.len()) = 0;
        Ok(buffer)
    } else {
        Err(MemoryError::AllocationFailed)
    }
}

// Load kernel image into allocated memory
pub unsafe fn load_kernel_image(
    kernel: &KernelImage,
    dest: *mut u8,
) -> Result<(), MemoryError> {
    // Copy compressed kernel to destination
    let kernel_data = core::slice::from_raw_parts(
        kernel.kernel_base(),
        kernel.kernel_size(),
    );

    core::ptr::copy_nonoverlapping(
        kernel_data.as_ptr(),
        dest,
        kernel_data.len(),
    );

    Ok(())
}
