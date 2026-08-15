; PCI Legacy Configuration Space Access (CF8/CFC)
; ABI: Microsoft x64 (RCX, RDX, R8, R9, stack)
;
; Functions:
;   - asm_pci_cfg_read32: Read 32-bit from PCI config space
;   - asm_pci_cfg_write32: Write 32-bit to PCI config space
;   - asm_pci_cfg_read16: Read 16-bit from PCI config space
;   - asm_pci_cfg_write16: Write 16-bit to PCI config space
;   - asm_pci_cfg_read8: Read 8-bit from PCI config space
;   - asm_pci_cfg_write8: Write 8-bit to PCI config space
;
; PCI Config Address Format (port CF8h):
;   Bit 31:    Enable bit (must be 1)
;   Bits 30-24: Reserved (0)
;   Bits 23-16: Bus number (0-255)
;   Bits 15-11: Device number (0-31)
;   Bits 10-8:  Function number (0-7)
;   Bits 7-2:   Register number (dword aligned)
;   Bits 1-0:   Must be 00 (dword alignment)
;
; Port CFCh: Data port (read/write config data)
;
; Reference: PCI Local Bus Spec 3.0, ARCHITECTURE_V3.md

section .text

global asm_pci_cfg_read32
global asm_pci_cfg_write32
global asm_pci_cfg_read16
global asm_pci_cfg_write16
global asm_pci_cfg_read8
global asm_pci_cfg_write8
global asm_pci_make_addr

; PCI configuration ports
%define PCI_CONFIG_ADDR 0x0CF8
%define PCI_CONFIG_DATA 0x0CFC

; CL = bus, DL = device, R8B = function, R9B = register offset (aligned
; to dword). Returns EAX = config address with enable bit set.
asm_pci_make_addr:
    ; 0x80000000 | (bus << 16) | (dev << 11) | (func << 8) | (reg & 0xFC)
    movzx   eax, cl
    shl     eax, 16

    movzx   r10d, dl
    shl     r10d, 11
    or      eax, r10d

    movzx   r10d, r8b
    shl     r10d, 8
    or      eax, r10d

    movzx   r10d, r9b
    and     r10d, 0xFC           ; align to dword boundary
    or      eax, r10d

    or      eax, 0x80000000      ; enable bit
    ret

; CL = bus, DL = device, R8B = function, R9B = register offset (dword
; aligned). Returns EAX = 32-bit config value.
asm_pci_cfg_read32:
    push    rcx
    push    rdx
    push    r8
    push    r9

    call    asm_pci_make_addr

    mov     dx, PCI_CONFIG_ADDR
    out     dx, eax
    mov     dx, PCI_CONFIG_DATA
    in      eax, dx

    pop     r9
    pop     r8
    pop     rdx
    pop     rcx
    ret

; CL = bus, DL = device, R8B = function, R9B = register offset (dword
; aligned), [RSP+40] = value (5th param, on stack per MS x64 ABI).
asm_pci_cfg_write32:
    push    rcx
    push    r8
    push    r9
    push    rbx                  ; scratch across the call, callee-saved

    ; 5th param is at RSP+40 relative to entry; after 4 pushes (32 bytes): RSP+72
    mov     ebx, [rsp + 40 + 32] ; value to write

    call    asm_pci_make_addr

    mov     dx, PCI_CONFIG_ADDR
    out     dx, eax
    mov     dx, PCI_CONFIG_DATA
    mov     eax, ebx
    out     dx, eax

    pop     rbx
    pop     r9
    pop     r8
    pop     rcx
    ret

; CL = bus, DL = device, R8B = function, R9B = register offset (word
; aligned). Returns AX = 16-bit config value (zero-extended in EAX).
asm_pci_cfg_read16:
    push    rcx
    push    rdx
    push    r8
    push    r9

    movzx   r10d, r9b
    and     r10d, 2              ; offset 0 or 2 within dword
    push    r10

    call    asm_pci_make_addr

    mov     dx, PCI_CONFIG_ADDR
    out     dx, eax
    mov     dx, PCI_CONFIG_DATA
    in      eax, dx

    pop     r10
    test    r10d, r10d
    jz      .low_word
    shr     eax, 16              ; high word
.low_word:
    movzx   eax, ax

    pop     r9
    pop     r8
    pop     rdx
    pop     rcx
    ret

; Read-modify-write to preserve the other 16 bits of the dword.
; CL = bus, DL = device, R8B = function, R9B = register offset (word
; aligned), [RSP+40] = value (5th param, on stack).
asm_pci_cfg_write16:
    push    rcx
    push    r8
    push    r9
    push    rbx
    push    r11

    ; 5th param at RSP+40 relative to entry; after 5 pushes (40 bytes): RSP+80
    movzx   r11d, word [rsp + 80] ; value to write
    movzx   ebx, r9b
    and     ebx, 2               ; offset within dword (0 or 2)

    call    asm_pci_make_addr
    push    rax                  ; save address

    mov     dx, PCI_CONFIG_ADDR
    out     dx, eax
    mov     dx, PCI_CONFIG_DATA
    in      eax, dx              ; current dword

    test    ebx, ebx
    jz      .write_low
    and     eax, 0x0000FFFF      ; clear high word
    shl     r11d, 16
    or      eax, r11d
    jmp     .do_write
.write_low:
    and     eax, 0xFFFF0000      ; clear low word
    or      eax, r11d

.do_write:
    pop     rbx                  ; restore address to RBX
    push    rax                  ; save modified value

    mov     eax, ebx
    mov     dx, PCI_CONFIG_ADDR
    out     dx, eax

    pop     rax
    mov     dx, PCI_CONFIG_DATA
    out     dx, eax

    pop     r11
    pop     rbx
    pop     r9
    pop     r8
    pop     rcx
    ret

; CL = bus, DL = device, R8B = function, R9B = register offset (any
; alignment). Returns AL = 8-bit config value (zero-extended in EAX).
asm_pci_cfg_read8:
    push    rcx
    push    rdx
    push    r8
    push    r9

    movzx   r10d, r9b
    and     r10d, 3              ; offset 0-3 within dword
    push    r10

    call    asm_pci_make_addr

    mov     dx, PCI_CONFIG_ADDR
    out     dx, eax
    mov     dx, PCI_CONFIG_DATA
    in      eax, dx

    pop     rcx                  ; byte offset
    shl     ecx, 3               ; bit offset
    shr     eax, cl
    movzx   eax, al

    pop     r9
    pop     r8
    pop     rdx
    pop     rcx
    ret

; Read-modify-write. CL = bus, DL = device, R8B = function, R9B =
; register offset, [RSP+40] = value (5th param, on stack).
asm_pci_cfg_write8:
    push    rcx
    push    r8
    push    r9
    push    rbx
    push    r11
    push    r12

    ; 5th param at RSP+40 relative to entry; after 6 pushes (48 bytes): RSP+88
    movzx   r11d, byte [rsp + 88] ; value to write
    movzx   r12d, r9b
    and     r12d, 3              ; byte offset within dword

    call    asm_pci_make_addr
    push    rax                  ; save address

    mov     dx, PCI_CONFIG_ADDR
    out     dx, eax
    mov     dx, PCI_CONFIG_DATA
    in      eax, dx              ; current dword

    mov     ebx, 0xFF
    mov     ecx, r12d
    shl     ecx, 3               ; bit offset
    shl     ebx, cl              ; mask for byte position
    not     ebx
    and     eax, ebx             ; clear target byte
    shl     r11d, cl             ; position new value
    or      eax, r11d

    pop     rbx                  ; address
    push    rax
    mov     eax, ebx
    mov     dx, PCI_CONFIG_ADDR
    out     dx, eax
    pop     rax
    mov     dx, PCI_CONFIG_DATA
    out     dx, eax

    pop     r12
    pop     r11
    pop     rbx
    pop     r9
    pop     r8
    pop     rcx
    ret
