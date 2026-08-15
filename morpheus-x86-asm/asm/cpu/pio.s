; Port I/O primitives
; ABI: Microsoft x64 (RCX, RDX, R8, R9, stack)
;
; Functions:
;   - asm_pio_read8: Read 8-bit from I/O port
;   - asm_pio_write8: Write 8-bit to I/O port
;   - asm_pio_read16: Read 16-bit from I/O port
;   - asm_pio_write16: Write 16-bit to I/O port
;   - asm_pio_read32: Read 32-bit from I/O port
;   - asm_pio_write32: Write 32-bit to I/O port
;
; Port I/O is used for PCI Legacy configuration space access (CF8/CFC)
; and some older hardware. The IN/OUT instructions are inherently serializing.
;
; Reference: ARCHITECTURE_V3.md - PIO layer

section .text

global asm_pio_read8
global asm_pio_write8
global asm_pio_read16
global asm_pio_write16
global asm_pio_read32
global asm_pio_write32

; RCX = port (low 16 bits), returns RAX
asm_pio_read8:
    mov     dx, cx
    xor     eax, eax
    in      al, dx
    ret

; RCX = port (low 16 bits), RDX = value (low 8 bits)
asm_pio_write8:
    mov     al, dl
    mov     dx, cx
    out     dx, al
    ret

; RCX = port (low 16 bits), returns RAX
asm_pio_read16:
    mov     dx, cx
    xor     eax, eax
    in      ax, dx
    ret

; RCX = port (low 16 bits), RDX = value (low 16 bits)
asm_pio_write16:
    mov     ax, dx               ; low 16 bits of RDX
    mov     dx, cx
    out     dx, ax
    ret

; RCX = port (low 16 bits), returns RAX
asm_pio_read32:
    mov     dx, cx
    in      eax, dx
    ret

; RCX = port (low 16 bits), RDX = value (low 32 bits)
asm_pio_write32:
    mov     eax, edx
    mov     dx, cx
    out     dx, eax
    ret
