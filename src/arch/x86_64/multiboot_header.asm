section .multiboot_header

%define magic 0xe85250d6
%define arch 0
%define header_size (header_end - header_start)

header_start:
	dd magic                ; magic number (multiboot 2)
	dd arch                         ; architecture 0 (protected mode i386)
	dd header_size ; header length
	; checksum
	dd 0x100000000 - (magic + arch + header_size)

	; insert optional multiboot tags here

	; required end tag
	dw 0    ; type
	dw 0    ; flags
	dd 8    ; size
header_end:
