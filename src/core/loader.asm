; This file is part of "osmini"
; Under the MIT License
; Copyright (c) Antonin Hérault

bits 64

extern main

global _start
_start:
    call main
