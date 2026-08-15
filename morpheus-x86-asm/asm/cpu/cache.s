; Cache management primitives
; ABI: Microsoft x64 (RCX, RDX, R8, R9, stack)
;
; Functions:
;   - asm_cache_clflush: Flush cache line (serializing)
;   - asm_cache_clflushopt: Optimized cache line flush (weakly ordered)
;   - asm_cache_flush_range: Flush a range of memory
;
; These are used for DMA coherency when memory is mapped as Write-Back (WB)
; instead of Uncached (UC) or Write-Combining (WC). In UC/WC mode, cache
; flush is not needed - hardware handles coherency.
;
; Reference: NETWORK_IMPL_GUIDE.md §3.6 - Cache coherency

section .text

global asm_cache_clflush
global asm_cache_clflushopt
global asm_cache_flush_range

; cache line size (64 bytes on all modern x86-64)
%define CACHE_LINE_SIZE 64

; RCX = address. CLFLUSH is strongly ordered wrt other CLFLUSH, stores,
; and MFENCE/SFENCE - use when ordering wrt other stores matters.
asm_cache_clflush:
    clflush [rcx]
    ret

; RCX = address. CLFLUSHOPT is weakly ordered - multiple CLFLUSHOPTs may
; execute in any order; SFENCE after a batch to ensure completion.
asm_cache_clflushopt:
    clflushopt [rcx]
    ret

; RCX = start address, RDX = length in bytes. Flushes [addr, addr+len)
; via CLFLUSHOPT, SFENCE at the end.
asm_cache_flush_range:
    test    rdx, rdx
    jz      .done

    lea     rax, [rcx + rdx]            ; end address

    and     rcx, ~(CACHE_LINE_SIZE - 1) ; align start down to cache line

.loop:
    clflushopt [rcx]
    add     rcx, CACHE_LINE_SIZE
    cmp     rcx, rax
    jb      .loop

    sfence

.done:
    ret
