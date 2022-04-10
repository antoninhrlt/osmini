; This file is part of "osmini"
; Under the MIT License
; Copyright (c) Antonin Hérault

bits 64

    global test_hi

; Print "Hi" on the screen
; Test function
test_hi:
    mov dword [0xB8000], 0x07690748
    ret
