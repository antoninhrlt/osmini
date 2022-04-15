; This file is part of "osmini"
; Under the MIT License
; Copyright (c) 2022 Antonin Hérault

;
; Print functions for the bootloader
;

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
