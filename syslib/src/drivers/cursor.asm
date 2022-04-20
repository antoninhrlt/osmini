; This file is part of "osmini"
; Under the MIT License
; Copyright (c) 2022 Antonin Hérault

	global _enable
_enable:
	ret

    global _disable
_disable:
	mov dx, 0x3D4
	mov al, 0xA
	out dx, al
 
	inc dx ; dx = 0x3D5
	mov al, 0x20
	out dx, al

	ret
