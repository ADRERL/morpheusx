// Kernel boot handoff

use super::{KernelImage, LinuxBootParams};

pub enum HandoffError {
    ExitBootServicesFailed,
    InvalidKernel,
}

// Jump to kernel entry point
// This function does not return!
pub unsafe fn boot_kernel(
    kernel: &KernelImage,
    boot_params: *mut LinuxBootParams,
    _system_table: *mut (),
) -> ! {
    // According to Linux boot protocol for x86_64:
    // 
    // Register state on entry:
    //   %rsi = address of boot_params (zero page)
    //   %rsp = stack pointer (must be valid)
    //   All other registers undefined
    //
    // The kernel is entered in 64-bit mode with:
    //   - Paging enabled (identity mapped)
    //   - Interrupts disabled
    //   - Direction flag cleared

    let entry_point = kernel.kernel_base();
    let boot_params_addr = boot_params as u64;

    // Jump to kernel
    // According to Linux x86 boot protocol:
    // - rsi = boot params pointer
    // - All other GPRs = 0 (except rsp)
    // - Interrupts disabled (IF=0)
    // - Direction flag cleared (DF=0)
    core::arch::asm!(
        "cli",              // Disable interrupts
        "cld",              // Clear direction flag
        "xor rax, rax",     // Zero all registers as per protocol
        "xor rbx, rbx",
        "xor rcx, rcx",
        "xor rdx, rdx",
        "xor rdi, rdi",
        "xor rbp, rbp",
        "xor r8, r8",
        "xor r9, r9",
        "xor r10, r10",
        "xor r11, r11",
        "xor r12, r12",
        "xor r13, r13",
        "xor r14, r14",
        "xor r15, r15",
        "mov rsi, {boot_params}",  // Set boot params
        "jmp {entry}",             // Jump to kernel (no return)
        boot_params = in(reg) boot_params_addr,
        entry = in(reg) entry_point,
        options(noreturn)
    );
}
