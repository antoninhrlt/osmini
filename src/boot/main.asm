; This file is part of "osmini"
; Under the MIT License
; Copyright (c) Antonin Hérault

bits 16 ; default mode for instruction coding and data
org 0x0 ; offset to add to each referenced address

%define BASE 0x100
%define KSIZE 50 ;  512 Bytes sectors to load

    jmp start

%include "printer.asm"

start:
    ; Initialize segments at 0x07C00
    mov ax, 0x07C0 ; the boot sector is loaded at this address by the BIOS
    mov ds, ax
    mov es, ax

    ; Stack
    mov ax, 0x8000
    mov ss, ax
    mov sp, 0xF000

    ; Retrieve the boot unit
    mov [bootdrv], dl

    ; Print message
    mov si, hello_msg
    call print_str

    ; Load the core
    xor ax, ax
    int 0x13 ; call to the BIOS

    push es
    
    mov ax, BASE
    mov es, ax
    mov bx, 0
    mov ah, 2
    mov al, KSIZE
    mov ch, 0
    mov cl, 2
    mov dh, 0
    mov dl, [bootdrv]
    int 0x13 ; call to the BIOS

    pop es

    ; Initialization of the GDT pointer

    ; GDT limit calculation
    mov ax, gdt_end
    mov bx, gdt
    sub ax, bx 
    mov word [gdt_ptr], ax

    ; GDT linear address calculation
    xor eax, eax
    xor ebx, ebx
    mov ax, ds
    mov ecx, eax
    shl ecx, 4
    mov bx, gdt
    add ecx, ebx
    mov dword [gdt_ptr + 2], ecx

    ; Protected mode
    cli
    lgdt [gdt_ptr] ; load the GDT
    mov eax, cr0
    or ax, 1
    mov cr0, eax

    ; Data segment
    mov ax, 0x10
    mov ds, ax
    mov fs, ax
    mov gs, ax
    mov es, ax
    mov ss, ax
    mov esp, 0x9F000

    ; Jump to the core
    jmp dword 0x8:0x1000 ; code segment reset

; ---
hello_msg: db "osmini % Core loading", 13, 10, 0
bootdrv: db 0

gdt:
    db 0, 0, 0, 0, 0, 0, 0, 0

gdt_cs:
    db 0xFF, 0xFF, 0x0, 0x0, 0x0, 10011011b, 11011111b, 0x0

gdt_ds:
    db 0xFF, 0xFF, 0x0, 0x0, 0x0, 10010011b, 11011111b, 0x0

gdt_end:

gdt_ptr:
    dw 0  ; limit
    dd 0  ; start

; Finalize the bootloader
    times 510 - ($ - $$) db 144 ; create a binary file of size 512 Bytes
    dw 0xAA55 ; the BIOS need to have this signature at the end
