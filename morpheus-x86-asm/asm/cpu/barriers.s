; Memory barrier primitives
; ABI: Microsoft x64 (RCX, RDX, R8, R9, stack)
;
; Functions:
;   - asm_bar_sfence: Store fence - orders all prior stores
;   - asm_bar_lfence: Load fence - orders all prior loads
;   - asm_bar_mfence: Full memory fence - orders all prior loads AND stores
;
; These barriers are CRITICAL for DMA correctness. The compiler cannot
; reorder across external function calls, and these instructions prevent
; CPU reordering.
;
; Reference: NETWORK_IMPL_GUIDE.md §2.2.1, §2.4

section .text

global asm_bar_sfence
global asm_bar_lfence
global asm_bar_mfence

; use before notifying device data is ready (e.g. after writing a
; descriptor, before incrementing avail.idx)
asm_bar_sfence:
    sfence
    ret

; use after reading an index from device, before reading data at that
; index (e.g. after reading used.idx, before reading the used ring entry)
asm_bar_lfence:
    lfence
    ret

; use when both load and store ordering is required, e.g. before an
; MMIO doorbell write
asm_bar_mfence:
    mfence
    ret
