global set_up_page_tables
global enable_paging
global gdt64
global gdt64.code
global gdt64.pointer

%define PRESENT     (1)
%define WRITABLE    (1 << 1)
%define HUGE        (1 << 7)

section .text
bits 32

set_up_page_tables:

    ; [p4_table] -> p3_table
    mov eax, p3_table
    or eax, PRESENT | WRITABLE
    mov [p4_table], eax

    ; [p3_table] -> p2_table
    mov eax, p2_table
    or eax, PRESENT | WRITABLE
    mov [p3_table], eax

    ; [p2_table/+8/+16/...] -> 2MB page
    mov ecx, 0
    mov ebx, 0
    mov eax, 0
.map_p2_table:
    ;mov eax, 0x200000 ; 2MB
    ;mul ecx
    or eax, PRESENT | WRITABLE | HUGE
    mov [p2_table + ecx * 8], eax
    add ebx, 0x200000 ; 2MB and
    mov eax, ebx ; these two lines eliminate the  mul instruction
    inc ecx
    cmp ecx, 512
    jne .map_p2_table

    ret

enable_paging:
    mov eax, p4_table
    mov cr3, eax
    ; enable PAE
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    ; enable long mode
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    ; enable paging
    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax
    
    ret
    

section .bss
align 4096
p4_table:       ; Page-Map Level-4 (PML4)
    resb 4096
p3_table:       ; Page-Directory Pointer (PDP)
    resb 4096
p2_table:       ; Page-Directory (PD)
    resb 4096
p1_table:       ; Page Table (PT)
    resb 4096

section .rodata
gdt64: ; initialize GDT with code segment configuration
; Although we have paging mechanism, this setup is necessary for long mode.
    dq 0 ; zeroth entry
.code equ $ - gdt64
    dq (1 << 43) | (1 << 44) | (1 << 47) | (1 << 53)
.pointer:
    dw $ - gdt64 - 1 ; sizeof(GDT) - 1
    dq gdt64
