; This file is part of "osmini"
; Under the MIT License
; Copyright (c) Antonin Hérault

bits 16 ; default mode for instruction coding and data
org 0x0 ; offset to add to each referenced address

; Initialize segments at 0x07C00
    mov ax, 0x07C0 ; the boot sector is loaded at this address by the BIOS
    mov ds, ax
    mov es, ax

    ; stack
    mov ax, 0x8000
    mov ss, ax
    mov sp, 0xF000

; Print message
    mov si, hello_msg
    call print_str

; ---
hello_msg: db "osmini % System startup", 13, 10, 0


; Print a string of char to the screen
;   si : pointer on the string to print 
print_str:
    push ax
    push bx

    ; Loop to print each char of the string
    ; When the character is '\0', the loop ends
    .print_char:
        lodsb
        cmp al, 0 ; ? end of string ('\0' == 0x0)
            jz .end
            
        mov ah, 0x0E ; service 0x0E, int 0x10 of BIOS
        mov bx, 0x07
        int 0x10 ; call the BIOS

        jmp .print_char

    .end:
        pop bx
        pop ax
        ret

; ---
    times 510 - ($ - $$) db 144 ; create a binary file of size 512 Bytes
    dw 0xAA55 ; the BIOS need to have this signature at the end
