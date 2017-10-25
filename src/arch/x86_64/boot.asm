global start

; check.asm
extern check_multiboot
extern check_cpuid
extern check_long_mode
extern set_up_page_tables
extern enable_paging

; paging.asm
extern gdt64
extern gdt64.pointer
extern gdt64.code

; long_mode_init.asm
extern long_mode_start

section .text
bits 32

start:
    mov esp, stack_top

    ; ebx:= Multiboot2 info ptr
    push ebx

    call check_multiboot
    call check_cpuid
    call check_long_mode

    call set_up_page_tables
    call enable_paging

    pop edi ; pass Multiboot2 info ptr to rust_main
    lgdt [gdt64.pointer]
    jmp gdt64.code:long_mode_start

    ; print `OK` to screen
    mov dword [0xb8000], 0x2f4b2f4f
    hlt

section .bss
stack_bottom:
    resb 4096 * 4
stack_top:
