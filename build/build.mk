# This file is part of "osmini"
# Under the MIT License
# Copyright (c) Antonin Hérault

ASM = nasm

OUT_DIR = build/out
EXT = osmini

IMG = $(OUT_DIR)/osmini.img
BOOT = $(OUT_DIR)/bootloader.$(EXT)
CORE = $(OUT_DIR)/core.$(EXT)

BOOT_SRC = $(wildcard src/boot/*.asm)

build : $(IMG)

run : $(IMG)
	qemu-system-x86_64 -boot a -drive file=$<,format=raw,index=0,media=disk

$(IMG) : $(BOOT) $(CORE)
	cat $^ /dev/zero | dd of=$@ bs=512 count=2880 

$(BOOT) : $(BOOT_SRC)
	$(ASM) -f bin -o $@ $^

$(CORE) : 
