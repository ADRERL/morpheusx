; Delay/timing primitives
; ABI: Microsoft x64 (RCX, RDX, R8, R9, stack)
;
; Functions:
;   - asm_spin_hint: CPU hint for spin loop (PAUSE instruction)
;   - asm_delay_tsc: Delay for N TSC ticks
;   - asm_delay_us: Delay for N microseconds (requires TSC frequency)
;
; WARNING: Delay functions are BLOCKING! Use only during initialization
; where blocking is acceptable. Never use in the main poll loop.
;
; Reference: ARCHITECTURE_V3.md - delay primitives

section .text

global asm_spin_hint
global asm_delay_tsc
global asm_delay_us

asm_spin_hint:
    pause
    ret

; RCX = number of TSC ticks to delay. Blocking - init use only.
asm_delay_tsc:
    rdtsc
    shl     rdx, 32
    or      rax, rdx             ; RAX = start TSC (64-bit)

    add     rcx, rax             ; RCX = target TSC (start + delay)

.wait_loop:
    pause
    rdtsc
    shl     rdx, 32
    or      rax, rdx
    cmp     rax, rcx
    jb      .wait_loop

    ret

; RCX = microseconds, RDX = TSC frequency in Hz. Blocking - init use only.
; ticks = (us * freq) / 1,000,000
asm_delay_us:
    mov     rax, rcx
    mul     rdx                  ; RDX:RAX = us * freq

    mov     rcx, 1000000
    div     rcx                  ; RAX = ticks

    mov     rcx, rax

    rdtsc
    shl     rdx, 32
    or      rax, rdx             ; RAX = start TSC
    add     rcx, rax             ; RCX = target TSC

.wait_loop:
    pause
    rdtsc
    shl     rdx, 32
    or      rax, rdx
    cmp     rax, rcx
    jb      .wait_loop

    ret
