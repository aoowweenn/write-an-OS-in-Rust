global long_mode_start

section .text

reset_data_seg_regs:
bits 32
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

long_mode_start:
bits 64
    ; print "OKAY"
    mov rax, 0x2F592F412F4B2F4F
    mov qword [0xb8000] , rax
    hlt
