; MMIO (Memory-Mapped I/O) primitives
; ABI: Microsoft x64 (RCX, RDX, R8, R9, stack)
;
; Functions:
;   - asm_mmio_read32: Read 32-bit from MMIO address
;   - asm_mmio_write32: Write 32-bit to MMIO address
;   - asm_mmio_read16: Read 16-bit from MMIO address
;   - asm_mmio_write16: Write 16-bit to MMIO address
;   - asm_mmio_read8: Read 8-bit from MMIO address
;   - asm_mmio_write8: Write 8-bit to MMIO address
;
; CRITICAL: These are simple loads/stores. The caller is responsible for
; appropriate barriers before/after. The standalone ASM call acts as a
; compiler barrier (compiler cannot reorder across function call).
;
; SAFETY: caller must ensure the address is a valid MMIO address, aligned
; to the access width.
;
; Reference: NETWORK_IMPL_GUIDE.md §2.2.1

section .text

global asm_mmio_read32
global asm_mmio_write32
global asm_mmio_read16
global asm_mmio_write16
global asm_mmio_read8
global asm_mmio_write8

; RCX = address, returns RAX
asm_mmio_read32:
    mov     eax, [rcx]
    ret

; RCX = address, RDX = value
asm_mmio_write32:
    mov     [rcx], edx
    ret

; RCX = address, returns RAX
asm_mmio_read16:
    xor     eax, eax
    mov     ax, [rcx]
    ret

; RCX = address, RDX = value
asm_mmio_write16:
    mov     [rcx], dx
    ret

; RCX = address, returns RAX
asm_mmio_read8:
    xor     eax, eax
    mov     al, [rcx]
    ret

; RCX = address, RDX = value
asm_mmio_write8:
    mov     [rcx], dl
    ret
