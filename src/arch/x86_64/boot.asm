global start

; check.asm
extern check_multiboot
extern check_cpuid
extern check_long_mode
extern set_up_page_tables
extern enable_paging

section .text
bits 32

start:
    mov esp, stack_top

    call check_multiboot
    call check_cpuid
    call check_long_mode

    call set_up_page_tables
    call enable_paging

    ; print `OK` to screen
    mov dword [0xb8000], 0x2f4b2f4f
    hlt

section .bss
stack_bottom:
    resb 64
stack_top:
