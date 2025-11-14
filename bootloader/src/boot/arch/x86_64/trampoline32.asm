; 32-bit protected mode trampoline
; Handles 64-bit to 32-bit CPU mode transition

BITS 64

SECTION .text
GLOBAL drop_to_protected_mode_asm

drop_to_protected_mode_asm:
    ; Args: RDI = entry_point (u32), RSI = boot_params (u32)
    ; Save 32-bit arguments in high registers
    mov r14d, edi
    mov r15d, esi
    
    ; Disable interrupts
    cli
    cld
    
    ; Disable paging: CR0.PG = 0
    mov rax, cr0
    and eax, 0x7fffffff
    mov cr0, rax
    
    ; Disable long mode: EFER.LME = 0
    mov ecx, 0xc0000080
    rdmsr
    and eax, 0xfffffeff
    wrmsr
    
    ; Build GDT descriptor on stack with absolute address
    lea rax, [rel gdt32]        ; Get absolute address of GDT
    push rax                     ; Push base address (8 bytes)
    mov ax, 23                   ; GDT limit (3 entries * 8 bytes - 1)
    push ax                      ; Push limit (2 bytes)
    lgdt [rsp]                   ; Load GDT from stack
    add rsp, 10                  ; Clean up stack (2 + 8 bytes)
    
    ; Move arguments to stack (survives mode switch)
    push r15    ; boot_params
    push r14    ; entry_point
    
    ; Far jump to 32-bit code
    ; For retfq, stack must be: [RSP]=offset, [RSP+8]=selector
    lea rax, [rel pm32]
    push rax            ; Push offset FIRST
    push 0x08           ; Push selector SECOND
    retfq

BITS 32
pm32:
    ; Set up 32-bit segments
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    
    ; Pop arguments (4 bytes each in 32-bit mode)
    pop edi         ; entry_point
    add esp, 4      ; skip high 32 bits
    pop esi         ; boot_params
    add esp, 4      ; skip high 32 bits
    
    ; Zero registers
    xor eax, eax
    xor ebx, ebx
    xor ecx, ecx
    xor edx, edx
    xor ebp, ebp
    
    ; Jump to kernel
    jmp edi

ALIGN 16
gdt32:
    dq 0x0000000000000000  ; Null descriptor
    dq 0x00cf9a000000ffff  ; Code segment (base=0, limit=4GB, 32-bit)
    dq 0x00cf92000000ffff  ; Data segment (base=0, limit=4GB, 32-bit)
