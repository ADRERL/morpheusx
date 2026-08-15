//! 2 MB DMA region for one VirtIO net device.
//!
//! ```text
//! Offset      Size        Content
//! 0x00000     0x0200      RX Descriptor Table (32 × 16 bytes)
//! 0x00200     0x0048      RX Available Ring
//! 0x00400     0x0108      RX Used Ring
//! 0x00800     0x0200      TX Descriptor Table (32 × 16 bytes)
//! 0x00A00     0x0048      TX Available Ring
//! 0x00C00     0x0108      TX Used Ring
//! 0x01000     0x10000     RX Buffers (32 × 2KB)
//! 0x11000     0x10000     TX Buffers (32 × 2KB)
//! ```

#[derive(Clone)]
pub struct DmaRegion {
    pub cpu_ptr: *mut u8,
    pub bus_addr: u64,
    pub size: usize,
}

impl DmaRegion {
    pub const MIN_SIZE: usize = 2 * 1024 * 1024;

    pub const RX_DESC_OFFSET: usize = 0x0000;
    pub const RX_AVAIL_OFFSET: usize = 0x0200;
    pub const RX_USED_OFFSET: usize = 0x0400;
    pub const TX_DESC_OFFSET: usize = 0x0800;
    pub const TX_AVAIL_OFFSET: usize = 0x0A00;
    pub const TX_USED_OFFSET: usize = 0x0C00;
    pub const RX_BUFFERS_OFFSET: usize = 0x1000;
    pub const TX_BUFFERS_OFFSET: usize = 0x11000;

    /// # Safety
    /// - `cpu_ptr` must point to valid DMA-capable memory
    /// - `bus_addr` must be the corresponding device-visible address
    /// - Region must be page-aligned
    pub unsafe fn new(cpu_ptr: *mut u8, bus_addr: u64, size: usize) -> Self {
        debug_assert!(size >= Self::MIN_SIZE, "DMA region too small");
        Self {
            cpu_ptr,
            bus_addr,
            size,
        }
    }

    pub fn cpu_base(&self) -> *mut u8 {
        self.cpu_ptr
    }

    pub fn bus_base(&self) -> u64 {
        self.bus_addr
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

// SAFETY: DmaRegion wraps a raw pointer to a static, identity-mapped DMA block owned exclusively
// by the driver that constructed it; this bring-up is single-threaded so no cross-core aliasing occurs.
unsafe impl Send for DmaRegion {}
// SAFETY: cpu_ptr/bus_addr/size are read-only fields once constructed; concurrent readers see a consistent view.
unsafe impl Sync for DmaRegion {}
