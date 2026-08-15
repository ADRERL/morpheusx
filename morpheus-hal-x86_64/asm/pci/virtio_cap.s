; VirtIO PCI Capability Parser
; ABI: Microsoft x64 (RCX, RDX, R8, R9, stack)
;
; Functions:
;   - asm_virtio_pci_parse_cap: Parse a VirtIO PCI capability
;   - asm_virtio_pci_find_all_caps: Find and parse all VirtIO caps
;   - asm_virtio_pci_read_bar: Read BAR value (with memory/IO detection)
;
; VirtIO PCI Capability Structure (at cap_offset):
;   +0x00: cap_vndr (u8)     = 0x09 (vendor-specific)
;   +0x01: cap_next (u8)     = offset to next capability
;   +0x02: cap_len (u8)      = length of this capability
;   +0x03: cfg_type (u8)     = 1=common, 2=notify, 3=isr, 4=device, 5=pci_cfg
;   +0x04: bar (u8)          = which BAR (0-5)
;   +0x05: padding[3]        = reserved
;   +0x08: offset (u32)      = offset within BAR
;   +0x0C: length (u32)      = length of region
;
; For NOTIFY capability (cfg_type=2), additional field:
;   +0x10: notify_off_multiplier (u32)
;
; Reference: VirtIO Spec 1.2 §4.1.4

section .data
    ; VirtIO PCI capability offsets (within capability)
    VIRTIO_CAP_VNDR         equ 0x00
    VIRTIO_CAP_NEXT         equ 0x01
    VIRTIO_CAP_LEN          equ 0x02
    VIRTIO_CAP_CFG_TYPE     equ 0x03
    VIRTIO_CAP_BAR          equ 0x04
    VIRTIO_CAP_OFFSET       equ 0x08
    VIRTIO_CAP_LENGTH       equ 0x0C
    VIRTIO_CAP_NOTIFY_MULT  equ 0x10    ; Only for notify cap

    ; PCI BAR offsets in config space
    PCI_BAR0                equ 0x10
    PCI_BAR1                equ 0x14
    PCI_BAR2                equ 0x18
    PCI_BAR3                equ 0x1C
    PCI_BAR4                equ 0x20
    PCI_BAR5                equ 0x24

    ; BAR type bits
    BAR_TYPE_MEM            equ 0       ; Memory BAR (bit 0 = 0)
    BAR_TYPE_IO             equ 1       ; I/O BAR (bit 0 = 1)
    BAR_MEM_TYPE_64         equ 0x04    ; 64-bit memory BAR (bits 2:1 = 10)

section .text

extern asm_pci_cfg_read8
extern asm_pci_cfg_read16
extern asm_pci_cfg_read32
extern asm_pci_find_virtio_cap

global asm_virtio_pci_parse_cap
global asm_virtio_pci_read_bar
global asm_virtio_pci_probe_caps

; VirtioCapInfo structure (output from asm_virtio_pci_parse_cap)
; Size: 24 bytes, must match Rust #[repr(C)] struct
;
; struct VirtioCapInfo {
;     cfg_type: u8,      // +0x00
;     bar: u8,           // +0x01
;     _pad: [u8; 2],     // +0x02
;     offset: u32,       // +0x04
;     length: u32,       // +0x08
;     notify_mult: u32,  // +0x0C (only valid for notify cap)
;     cap_offset: u8,    // +0x10 (PCI config space offset of this cap)
;     _pad2: [u8; 7],    // +0x11
; }

; CL = bus, DL = device, R8B = function, R9B = cap_offset in PCI config
; space, [RSP+40] = pointer to VirtioCapInfo output struct (5th param).
; Returns EAX = 1 on success, 0 on error.
asm_virtio_pci_parse_cap:
    push    rbp
    mov     rbp, rsp
    push    rbx
    push    r12
    push    r13
    push    r14
    push    r15

    movzx   r12d, cl             ; bus
    movzx   r13d, dl             ; device
    movzx   r14d, r8b            ; function
    movzx   r15d, r9b            ; cap_offset

    mov     rbx, [rbp + 48]      ; output struct pointer (5th param: rbp+16+32)
    test    rbx, rbx
    jz      .error

    mov     byte [rbx + 0x10], r15b ; cap_offset

    ; cfg_type (cap+3)
    mov     cl, r12b
    mov     dl, r13b
    mov     r8b, r14b
    mov     r9b, r15b
    add     r9b, VIRTIO_CAP_CFG_TYPE
    call    asm_pci_cfg_read8
    mov     byte [rbx + 0x00], al

    ; bar (cap+4)
    mov     cl, r12b
    mov     dl, r13b
    mov     r8b, r14b
    mov     r9b, r15b
    add     r9b, VIRTIO_CAP_BAR
    call    asm_pci_cfg_read8
    mov     byte [rbx + 0x01], al

    mov     word [rbx + 0x02], 0 ; padding

    ; offset (cap+8, 32-bit)
    mov     cl, r12b
    mov     dl, r13b
    mov     r8b, r14b
    mov     r9b, r15b
    add     r9b, VIRTIO_CAP_OFFSET
    call    asm_pci_cfg_read32
    mov     dword [rbx + 0x04], eax

    ; length (cap+12, 32-bit)
    mov     cl, r12b
    mov     dl, r13b
    mov     r8b, r14b
    mov     r9b, r15b
    add     r9b, VIRTIO_CAP_LENGTH
    call    asm_pci_cfg_read32
    mov     dword [rbx + 0x08], eax

    cmp     byte [rbx + 0x00], 2 ; notify cap?
    jne     .skip_notify_mult

    ; notify_off_multiplier (cap+16)
    mov     cl, r12b
    mov     dl, r13b
    mov     r8b, r14b
    mov     r9b, r15b
    add     r9b, VIRTIO_CAP_NOTIFY_MULT
    call    asm_pci_cfg_read32
    mov     dword [rbx + 0x0C], eax
    jmp     .done_mult

.skip_notify_mult:
    mov     dword [rbx + 0x0C], 0 ; non-notify caps: zero notify_mult

.done_mult:
    mov     qword [rbx + 0x11], 0 ; remaining padding
    and     byte [rbx + 0x17], 0

    mov     eax, 1
    jmp     .done

.error:
    xor     eax, eax

.done:
    pop     r15
    pop     r14
    pop     r13
    pop     r12
    pop     rbx
    pop     rbp
    ret

; CL = bus, DL = device, R8B = function, R9B = BAR index (0-5).
; Returns RAX = BAR base address (masked, type bits removed),
; RDX = 1 if memory BAR, 0 if I/O BAR. Handles 32-bit and 64-bit BARs.
asm_virtio_pci_read_bar:
    push    rbx
    push    r12
    push    r13
    push    r14
    push    r15

    movzx   r12d, cl              ; bus
    movzx   r13d, dl              ; device
    movzx   r14d, r8b             ; function
    movzx   r15d, r9b             ; bar_index

    mov     eax, r15d
    shl     eax, 2
    add     eax, PCI_BAR0         ; BAR register offset
    movzx   r9d, al

    mov     cl, r12b
    mov     dl, r13b
    mov     r8b, r14b
    call    asm_pci_cfg_read32
    mov     ebx, eax              ; low 32 bits

    test    ebx, BAR_TYPE_IO
    jnz     .io_bar

    mov     eax, ebx
    and     eax, 0x06             ; bits 2:1
    cmp     eax, BAR_MEM_TYPE_64
    jne     .mem32_bar

    ; 64-bit memory BAR: read high 32 bits from next BAR slot
    mov     eax, r15d
    inc     eax
    cmp     eax, 5                ; bounds check
    ja      .mem32_bar            ; treat as 32-bit if at end

    shl     eax, 2
    add     eax, PCI_BAR0

    mov     cl, r12b
    mov     dl, r13b
    mov     r8b, r14b
    mov     r9b, al
    call    asm_pci_cfg_read32

    shl     rax, 32               ; RAX = (high << 32) | (low & ~0xF)
    mov     ecx, ebx
    and     ecx, 0xFFFFFFF0
    or      rax, rcx
    mov     rdx, 1
    jmp     .done

.mem32_bar:
    mov     eax, ebx
    and     eax, 0xFFFFFFF0
    mov     rdx, 1
    jmp     .done

.io_bar:
    mov     eax, ebx
    and     eax, 0xFFFFFFFC
    xor     rdx, rdx

.done:
    pop     r15
    pop     r14
    pop     r13
    pop     r12
    pop     rbx
    ret

; CL = bus, DL = device, R8B = function, R9 = pointer to array of 5
; VirtioCapInfo structs (24 bytes each): [0]=common, [1]=notify, [2]=isr,
; [3]=device, [4]=pci_cfg.
; Returns EAX = bitmask of found capabilities (bit 0=common, 1=notify, ...).
asm_virtio_pci_probe_caps:
    push    rbp
    mov     rbp, rsp
    sub     rsp, 32              ; shadow space
    push    rbx
    push    r12
    push    r13
    push    r14
    push    r15

    movzx   r12d, cl             ; bus
    movzx   r13d, dl             ; device
    movzx   r14d, r8b            ; function
    mov     r15, r9              ; output array
    xor     ebx, ebx             ; found bitmask

    mov     ecx, 1               ; cfg_type 1 (common)

.probe_loop:
    cmp     ecx, 6
    jge     .done

    push    rcx

    mov     cl, r12b
    mov     dl, r13b
    mov     r8b, r14b
    pop     r9                   ; cfg_type to find
    push    r9
    call    asm_pci_find_virtio_cap

    pop     rcx
    test    eax, eax
    jz      .next_type           ; not found

    ; found at offset EAX; output struct = r15 + (cfg_type-1) * 24
    push    rcx
    push    rax                  ; cap offset

    mov     eax, ecx
    dec     eax
    imul    eax, 24
    add     rax, r15
    mov     r10, rax             ; output ptr

    pop     rax                  ; cap offset

    push    rcx
    sub     rsp, 48              ; shadow space + 5th param
    mov     [rsp + 32], r10      ; 5th param at Win64 ABI offset
    mov     cl, r12b
    mov     dl, r13b
    mov     r8b, r14b
    mov     r9b, al              ; cap offset
    call    asm_virtio_pci_parse_cap
    add     rsp, 48
    pop     rcx

    mov     eax, 1
    push    rcx
    dec     cl
    shl     eax, cl
    pop     rcx
    or      ebx, eax

    pop     rcx

.next_type:
    inc     ecx
    jmp     .probe_loop

.done:
    mov     eax, ebx             ; bitmask

    pop     r15
    pop     r14
    pop     r13
    pop     r12
    pop     rbx
    add     rsp, 32
    pop     rbp
    ret
