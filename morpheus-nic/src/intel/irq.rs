//! e1000e RX interrupt wiring. The ISR does NOT touch the net stack — it
//! read-clears ICR, bumps a counter, fires the installed wake hook (kernel
//! readiness `net_wake`), and EOIs the LAPIC. The epoll pump in syscall
//! context remains the only place smoltcp runs; polling backstops stay live
//! so an unwired or failed IRQ path degrades to the previous behavior.

use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

use morpheus_hal_api::{BusAddr, IsrFn, MsiError};
use morpheus_hal_x86_64::asm::mmio::{read32, write32};

use super::regs;

/// IDT vector for e1000e RX. 0x40 is xHCI MSI-X; timer is 0x20.
pub const NIC_RX_VECTOR: u8 = 0x41;

/// Diagnostic counter of observed NIC interrupts.
pub static NIC_IRQ_COUNT: AtomicU64 = AtomicU64::new(0);

static NIC_MMIO_BASE: AtomicU64 = AtomicU64::new(0);
static WAKE_HOOK: AtomicUsize = AtomicUsize::new(0);

extern "C" fn nic_isr_rust() {
    NIC_IRQ_COUNT.fetch_add(1, Ordering::Relaxed);

    // icr is read-to-clear; required to rearm the msi
    let mmio = NIC_MMIO_BASE.load(Ordering::Relaxed);
    if mmio != 0 {
        // SAFETY: mmio published by `wire_rx_irq` from the driver's verified
        // UC-mapped BAR; a 32-bit read of ICR is always safe on this part.
        let _ = unsafe { read32(mmio + regs::ICR as u64) };
    }

    let hook = WAKE_HOOK.load(Ordering::Acquire);
    if hook != 0 {
        // SAFETY: published by `wire_rx_irq` as a plain `fn()`; the installed
        // hook (`readiness::net_wake`) is IRQ-safe — kernel spinlocks disable
        // IRQs while held, so same-core deadlock is impossible.
        let f: fn() = unsafe { core::mem::transmute(hook) };
        f();
    }

    // SAFETY: LAPIC is online before any MSI can be programmed (wire_rx_irq
    // contract); EOI at end of every LAPIC-sourced ISR.
    unsafe { morpheus_hal_x86_64::cpu::apic::send_eoi() };
}

/// Thunk: save caller-saved GPRs, call the Rust handler (MS x64 ABI + shadow
/// space), restore, `iretq`.
#[unsafe(naked)]
unsafe extern "C" fn nic_isr_entry() {
    core::arch::naked_asm!(
        "push rax",
        "push rcx",
        "push rdx",
        "push rsi",
        "push rdi",
        "push r8",
        "push r9",
        "push r10",
        "push r11",
        "sub rsp, 32",
        "call {}",
        "add rsp, 32",
        "pop r11",
        "pop r10",
        "pop r9",
        "pop r8",
        "pop rdi",
        "pop rsi",
        "pop rdx",
        "pop rcx",
        "pop rax",
        "iretq",
        sym nic_isr_rust,
    );
}

/// Wire the NIC's MSI-X (or MSI fallback) to [`NIC_RX_VECTOR`] and unmask RX +
/// link-change interrupts. On any failure logs nothing fatal and returns —
/// polling backstops keep the stack functional.
///
/// `wake` is invoked from interrupt context on every NIC interrupt; it must be
/// IRQ-safe (intended: `morpheus_kernel::io::readiness::net_wake`).
///
/// # Safety
/// IDT initialized, BSP LAPIC enabled, `mmio_base` is the UC-mapped BAR0 of
/// the same device `dev` names, and the caller owns that device.
pub unsafe fn wire_rx_irq(
    intr: &dyn morpheus_hal_api::InterruptController,
    dev: BusAddr,
    mmio_base: u64,
    wake: fn(),
) -> bool {
    NIC_MMIO_BASE.store(mmio_base, Ordering::Relaxed);
    WAKE_HOOK.store(wake as usize, Ordering::Release);

    // gate installed before msi enable so a spurious vector can't triple-fault
    intr.set_handler(
        NIC_RX_VECTOR,
        IsrFn(nic_isr_entry as unsafe extern "C" fn()),
        0,
        0,
    );

    let apic_id = intr.read_lapic_id();
    let wired = match intr.enable_msix_single(dev, apic_id, NIC_RX_VECTOR) {
        Ok(_) => true,
        Err(MsiError::CapabilityNotFound) => {
            intr.enable_msi_single(dev, apic_id, NIC_RX_VECTOR).is_ok()
        },
        Err(_) => false,
    };
    if !wired {
        return false;
    }

    // throttle: itr units are 256ns; 0x3e8 = 256us min gap (~3.9k ints/s cap)
    // so a broadcast storm can't live-lock the cpu in irq context
    write32(mmio_base + regs::ITR as u64, 0x3E8);

    // read-clear stale causes, then unmask rx + link change
    let _ = read32(mmio_base + regs::ICR as u64);
    write32(
        mmio_base + regs::IMS as u64,
        regs::ICR_RXT0 | regs::ICR_RXDMT0 | regs::ICR_RXO | regs::ICR_LSC,
    );
    true
}
