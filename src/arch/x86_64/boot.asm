global start

; linker.ld
extern stack_end

; check.asm
extern check_multiboot
extern check_cpuid
extern check_long_mode

section .text
bits 32

start:
    mov esp, stack_end

    call check_multiboot
    call check_cpuid

    ; print `OK` to screen
    mov dword [0xb8000], 0x2f4b2f4f
    hlt

