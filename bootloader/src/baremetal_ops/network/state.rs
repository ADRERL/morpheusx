use core::net::Ipv4Addr;

use morpheus_hal_x86_64::sync::{SpinLock, SpinLockGuard};
use morpheus_net_stack::stack::{DnsQueryHandle, NetInterface, SocketHandle};
use morpheus_nic::device::UnifiedNetDevice;

const MAX_TCP_HANDLES: usize = 128;
const MAX_UDP_HANDLES: usize = 128;
const MAX_DNS_QUERIES: usize = 64;
const DNS_QUERY_TTL_MS: u64 = 60_000;

pub(super) struct NetState {
    pub driver: Option<UnifiedNetDevice>,
    pub stack: Option<NetInterface<UnifiedNetDevice>>,
    pub dma: Option<morpheus_virtio::dma::DmaRegion>,
    pub tsc_freq: u64,
    pub hostname: [u8; 64],
    pub hostname_len: usize,
    pub tcp: [Option<SocketHandle>; MAX_TCP_HANDLES],
    pub udp: [Option<SocketHandle>; MAX_UDP_HANDLES],
    pub dns: [Option<DnsQueryHandle>; MAX_DNS_QUERIES],
    pub dns_starts: [u64; MAX_DNS_QUERIES],
}

impl NetState {
    const fn new() -> Self {
        Self {
            driver: None,
            stack: None,
            dma: None,
            tsc_freq: 0,
            hostname: [0; 64],
            hostname_len: 0,
            tcp: [None; MAX_TCP_HANDLES],
            udp: [None; MAX_UDP_HANDLES],
            dns: [None; MAX_DNS_QUERIES],
            dns_starts: [0; MAX_DNS_QUERIES],
        }
    }

    pub(super) fn device_mut(&mut self) -> Option<&mut UnifiedNetDevice> {
        if let Some(stack) = self.stack.as_mut() {
            return Some(stack.device_mut());
        }
        self.driver.as_mut()
    }

    pub(super) fn clear_net_handle_tables(&mut self) {
        self.tcp.fill(None);
        self.udp.fill(None);
        self.dns.fill(None);
    }

    pub(super) fn alloc_tcp_slot(&mut self, handle: SocketHandle) -> Option<i64> {
        #[allow(clippy::needless_range_loop)]
        for idx in 0..MAX_TCP_HANDLES {
            if self.tcp[idx].is_none() {
                self.tcp[idx] = Some(handle);
                return Some(slot_to_user_handle(idx));
            }
        }
        None
    }

    pub(super) fn get_tcp_slot(&self, handle: i64) -> Option<SocketHandle> {
        let idx = user_handle_to_slot(handle, MAX_TCP_HANDLES)?;
        self.tcp[idx]
    }

    pub(super) fn take_tcp_slot(&mut self, handle: i64) -> Option<SocketHandle> {
        let idx = user_handle_to_slot(handle, MAX_TCP_HANDLES)?;
        self.tcp[idx].take()
    }

    pub(super) fn set_tcp_slot(&mut self, handle: i64, socket: SocketHandle) -> bool {
        let Some(idx) = user_handle_to_slot(handle, MAX_TCP_HANDLES) else {
            return false;
        };
        self.tcp[idx] = Some(socket);
        true
    }

    pub(super) fn tcp_active_count(&self) -> u32 {
        self.tcp.iter().filter(|h| h.is_some()).count() as u32
    }

    pub(super) fn alloc_udp_slot(&mut self, handle: SocketHandle) -> Option<i64> {
        #[allow(clippy::needless_range_loop)]
        for idx in 0..MAX_UDP_HANDLES {
            if self.udp[idx].is_none() {
                self.udp[idx] = Some(handle);
                return Some(slot_to_user_handle(idx));
            }
        }
        None
    }

    pub(super) fn get_udp_slot(&self, handle: i64) -> Option<SocketHandle> {
        let idx = user_handle_to_slot(handle, MAX_UDP_HANDLES)?;
        self.udp[idx]
    }

    pub(super) fn take_udp_slot(&mut self, handle: i64) -> Option<SocketHandle> {
        let idx = user_handle_to_slot(handle, MAX_UDP_HANDLES)?;
        self.udp[idx].take()
    }

    pub(super) fn alloc_dns_query_slot(
        &mut self,
        handle: DnsQueryHandle,
        now_ms: u64,
    ) -> Option<i64> {
        #[allow(clippy::needless_range_loop)]
        for idx in 0..MAX_DNS_QUERIES {
            if self.dns[idx].is_none() {
                self.dns[idx] = Some(handle);
                self.dns_starts[idx] = now_ms;
                return Some(slot_to_user_handle(idx));
            }
        }
        None
    }

    pub(super) fn get_dns_query_slot(&self, handle: i64) -> Option<DnsQueryHandle> {
        let idx = user_handle_to_slot(handle, MAX_DNS_QUERIES)?;
        self.dns[idx]
    }

    pub(super) fn clear_dns_query_slot(&mut self, handle: i64) {
        if let Some(idx) = user_handle_to_slot(handle, MAX_DNS_QUERIES) {
            self.dns[idx] = None;
        }
    }

    pub(super) fn take_dns_query_slot(&mut self, handle: i64) -> Option<DnsQueryHandle> {
        let idx = user_handle_to_slot(handle, MAX_DNS_QUERIES)?;
        self.dns[idx].take()
    }

    /// # Safety
    /// `name` must be a valid pointer to at least `len` readable bytes.
    pub(super) unsafe fn set_hostname(&mut self, name: *const u8, len: usize) -> i64 {
        if name.is_null() || len == 0 || len > 63 {
            return -1;
        }
        self.hostname_len = len;
        core::ptr::copy_nonoverlapping(name, self.hostname.as_mut_ptr(), len);
        self.hostname[len] = 0;
        0
    }

    pub(super) fn write_hostname_to(
        &self,
        out: &mut morpheus_kernel::syscall::handler::NetConfigInfo,
    ) {
        if self.hostname_len > 0 {
            let n = self.hostname_len.min(63);
            out.hostname[..n].copy_from_slice(&self.hostname[..n]);
            out.hostname[n] = 0;
        }
    }
}

static NET: SpinLock<NetState> = SpinLock::new(NetState::new());

pub(super) fn net() -> SpinLockGuard<'static, NetState> {
    NET.lock()
}

#[inline(always)]
pub(super) fn ip_from_nbo(ip: u32) -> Ipv4Addr {
    let [a, b, c, d] = ip.to_be_bytes();
    Ipv4Addr::new(a, b, c, d)
}

#[inline(always)]
pub(super) fn ip_to_nbo(ip: Ipv4Addr) -> u32 {
    u32::from_be_bytes(ip.octets())
}

#[inline(always)]
fn slot_to_user_handle(slot: usize) -> i64 {
    (slot as i64) + 1
}

#[inline(always)]
fn user_handle_to_slot(handle: i64, max: usize) -> Option<usize> {
    if handle <= 0 {
        return None;
    }
    let idx = (handle - 1) as usize;
    if idx < max {
        Some(idx)
    } else {
        None
    }
}

/// Reap pending DNS slots older than `DNS_QUERY_TTL_MS`, cancelling the smoltcp
/// query so a lost DNS_CANCEL can't wedge the (single) DNS query slot forever.
pub(super) fn reap_expired_dns_queries(
    stack: &mut NetInterface<UnifiedNetDevice>,
    dns: &mut [Option<DnsQueryHandle>; MAX_DNS_QUERIES],
    dns_starts: &mut [u64; MAX_DNS_QUERIES],
    now_ms: u64,
) {
    #[allow(clippy::needless_range_loop)]
    for idx in 0..MAX_DNS_QUERIES {
        if let Some(handle) = dns[idx] {
            if now_ms.saturating_sub(dns_starts[idx]) >= DNS_QUERY_TTL_MS {
                // Slot is live (see net-stack invariant), so cancel is panic-safe.
                stack.cancel_dns_query(handle);
                dns[idx] = None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_slot_roundtrip() {
        for i in 0..32usize {
            let h = slot_to_user_handle(i);
            assert_eq!(user_handle_to_slot(h, 64), Some(i));
        }
    }

    #[test]
    fn handle_slot_rejects_invalid_values() {
        assert_eq!(user_handle_to_slot(0, 8), None);
        assert_eq!(user_handle_to_slot(-5, 8), None);
        assert_eq!(user_handle_to_slot(999, 8), None);
    }

    #[test]
    fn ipv4_nbo_roundtrip() {
        let ip = Ipv4Addr::new(10, 0, 2, 15);
        let nbo = ip_to_nbo(ip);
        assert_eq!(nbo, 0x0A00_020F);
        assert_eq!(ip_from_nbo(nbo), ip);
    }
}
