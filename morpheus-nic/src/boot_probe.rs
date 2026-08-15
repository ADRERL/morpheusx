//! Dynamic NIC probe and driver factory.
//!
//! Probes PCI bus and creates appropriate driver based on detected hardware.
//! This is the main entry point for automatic driver selection.
//!
//! # Supported Devices
//! - VirtIO-net (QEMU, cloud VMs)
//! - Intel e1000e family (ThinkPad T450s, T520, etc.)
//!
//! # Usage
//!
//! ```ignore
//! use morpheus_nic::boot_probe::{probe_and_create_driver, ProbeResult};
//!
//! let result = unsafe { probe_and_create_driver(&dma, tsc_freq)? };
//! match result {
//!     ProbeResult::VirtIO(driver) => { /* use driver */ }
//!     ProbeResult::Intel(driver) => { /* use driver */ }
//! }
//! ```

use crate::intel::{
    enable_device, find_intel_nic, validate_mmio_access, E1000eConfig, E1000eDriver, E1000eError,
    IntelNicInfo,
};
use crate::virtio::{VirtioConfig, VirtioInitError, VirtioNetDriver};
use morpheus_hal_x86_64::pci::capability::probe_virtio_caps;
use morpheus_hal_x86_64::pci::{offset, pci_cfg_read16, pci_cfg_read32, PciAddr};
use morpheus_virtio::dma::DmaRegion;
use morpheus_virtio::transport::{PciModernConfig, VirtioTransport};

/// VirtIO vendor ID
const VIRTIO_VENDOR_ID: u16 = 0x1AF4;
/// VirtIO-net device ID range start
const VIRTIO_NET_DEVICE_START: u16 = 0x1000;
/// VirtIO-net modern device ID
const VIRTIO_NET_MODERN: u16 = 0x1041;

/// Probe and initialization errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProbeError {
    /// No network device found
    NoDevice,
    /// VirtIO initialization failed
    VirtioInitFailed,
    /// Intel e1000e initialization failed
    IntelInitFailed,
    /// BAR mapping failed
    BarMappingFailed,
    /// Device not responding
    DeviceNotResponding,
}

crate::impl_from!(VirtioInitError => ProbeError : VirtioInitFailed(_));
crate::impl_from!(E1000eError => ProbeError : IntelInitFailed(_));

/// Information about a detected network device.
#[derive(Debug, Clone, Copy)]
pub enum DetectedNic {
    /// VirtIO network device
    VirtIO { pci_addr: PciAddr, mmio_base: u64 },
    /// Intel e1000e network device
    Intel(IntelNicInfo),
}

/// Result of successful probe and initialization.
#[allow(clippy::large_enum_variant)]
pub enum ProbeResult {
    /// VirtIO driver
    VirtIO(VirtioNetDriver),
    /// Intel e1000e driver
    Intel(E1000eDriver),
}

/// Scan PCI bus for supported network devices.
///
/// Returns the first supported NIC found, preferring Intel over VirtIO
/// (for real hardware priority).
pub fn scan_for_nic() -> Option<DetectedNic> {
    // First try to find Intel NIC (real hardware)
    if let Some(info) = find_intel_nic() {
        return Some(DetectedNic::Intel(info));
    }

    // Fall back to VirtIO (QEMU, VMs)
    if let Some((pci_addr, mmio_base)) = find_virtio_nic() {
        return Some(DetectedNic::VirtIO {
            pci_addr,
            mmio_base,
        });
    }

    None
}

/// Scan for VirtIO network device.
fn find_virtio_nic() -> Option<(PciAddr, u64)> {
    for bus in 0..=255u8 {
        for device in 0..32u8 {
            for function in 0..8u8 {
                let addr = PciAddr::new(bus, device, function);

                let vendor_id = pci_cfg_read16(addr, offset::VENDOR_ID);
                if vendor_id == 0xFFFF {
                    if function == 0 {
                        break;
                    }
                    continue;
                }

                if vendor_id != VIRTIO_VENDOR_ID {
                    if function == 0 {
                        let header = pci_cfg_read16(addr, offset::HEADER_TYPE) & 0x80;
                        if header == 0 {
                            break;
                        }
                    }
                    continue;
                }

                let device_id = pci_cfg_read16(addr, offset::DEVICE_ID);

                // Check for VirtIO-net (transitional or modern)
                if device_id != VIRTIO_NET_DEVICE_START && device_id != VIRTIO_NET_MODERN {
                    continue;
                }

                // Read BAR0
                let bar0 = pci_cfg_read32(addr, offset::BAR0);
                if bar0 & 0x01 != 0 {
                    // I/O BAR - skip (need MMIO)
                    continue;
                }

                let is_64bit = (bar0 & 0x06) == 0x04;
                let mmio_base = if is_64bit {
                    let bar1 = pci_cfg_read32(addr, offset::BAR1);
                    ((bar1 as u64) << 32) | ((bar0 & 0xFFFFFFF0) as u64)
                } else {
                    (bar0 & 0xFFFFFFF0) as u64
                };

                return Some((addr, mmio_base));
            }
        }
    }

    None
}

/// Probe for network device and create appropriate driver.
///
/// # Safety
/// - DMA region must be properly allocated with correct bus addresses
/// - TSC frequency must be calibrated
pub unsafe fn probe_and_create_driver(
    dma: &DmaRegion,
    tsc_freq: u64,
) -> Result<ProbeResult, ProbeError> {
    let detected = scan_for_nic().ok_or(ProbeError::NoDevice)?;

    match detected {
        DetectedNic::Intel(info) => {
            // Enable device (bus mastering, memory space)
            enable_device(info.pci_addr);

            // Validate MMIO access
            if !validate_mmio_access(info.mmio_base) {
                return Err(ProbeError::DeviceNotResponding);
            }

            // Create driver config
            let config = E1000eConfig {
                dma_cpu_base: dma.cpu_base(),
                dma_bus_base: dma.bus_base(),
                rx_queue_size: 32,
                tx_queue_size: 32,
                buffer_size: 2048,
                tsc_freq,
            };

            // Create driver
            let driver = E1000eDriver::new(info.mmio_base, config)?;
            Ok(ProbeResult::Intel(driver))
        },

        DetectedNic::VirtIO {
            pci_addr,
            mmio_base,
        } => {
            // Enable device
            let cmd = pci_cfg_read16(pci_addr, offset::COMMAND);
            morpheus_hal_x86_64::pci::pci_cfg_write16(pci_addr, offset::COMMAND, cmd | 0x06);

            // Create VirtIO config
            let config = VirtioConfig {
                dma_cpu_base: dma.cpu_base(),
                dma_bus_base: dma.bus_base(),
                dma_size: dma.size(),
                queue_size: 32,
                buffer_size: 2048,
            };

            // Prefer PCI modern transport; fallback to legacy MMIO for old QEMU.
            let caps = probe_virtio_caps(pci_addr);
            let driver = if caps.has_required() {
                let pci_cfg = PciModernConfig {
                    common_cfg: caps.common_cfg_addr().unwrap_or(0),
                    notify_cfg: caps.notify_addr().unwrap_or(0),
                    notify_off_multiplier: caps.notify_multiplier(),
                    isr_cfg: caps.isr_addr().unwrap_or(0),
                    device_cfg: caps.device_cfg_addr().unwrap_or(0),
                    pci_cfg: 0,
                };
                let transport = VirtioTransport::pci_modern(pci_cfg);
                VirtioNetDriver::new_with_transport(transport, config, tsc_freq)?
            } else {
                VirtioNetDriver::new(mmio_base, config)?
            };
            Ok(ProbeResult::VirtIO(driver))
        },
    }
}
