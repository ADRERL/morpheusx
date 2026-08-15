; TSC (Time Stamp Counter) primitives
; ABI: Microsoft x64 (RCX, RDX, R8, R9, stack)
;
; Functions:
;   - asm_tsc_read: Read TSC (~40 cycles, non-serializing)
;   - asm_tsc_read_serialized: Read TSC with CPUID serialize (~200 cycles)
;
; Reference: NETWORK_IMPL_GUIDE.md §2.2.1

section .text

global asm_tsc_read
global asm_tsc_read_serialized

; non-serializing - instructions may be reordered around RDTSC. Use for
; low-overhead timing where slight inaccuracy is acceptable.
asm_tsc_read:
    rdtsc
    shl     rdx, 32
    or      rax, rdx             ; RAX = 64-bit TSC
    ret

; CPUID serializes the instruction stream before the TSC read, ensuring
; prior instructions have completed. Use for precise measurements.
; MS x64 ABI requires preserving RBX, so it is saved/restored around CPUID.
asm_tsc_read_serialized:
    push    rbx
    xor     eax, eax             ; CPUID leaf 0
    cpuid
    rdtsc
    shl     rdx, 32
    or      rax, rdx
    pop     rbx
    ret
