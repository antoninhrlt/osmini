# This file is part of "osmini"
# Under the MIT License
# Copyright (c) Antonin Hérault

ASM = nasm
LINKER = ld

OUT_DIR = build/out

IMG = $(OUT_DIR)/osmini.img
BOOT = $(OUT_DIR)/bootloader

build : $(IMG)

# Private function to simplify the release creation
# Should not be used by another one than the repository's owner
_release : $(IMG)
	cp $< bin/

run : $(IMG)
	qemu-system-x86_64 -boot a -drive file=$<,format=raw,index=0,media=disk

$(IMG) : $(BOOT)
	cat $^ /dev/zero | dd of=$@ bs=512 count=2880 

$(BOOT) : src/boot/main.asm
	$(ASM) -f bin -I src/boot/ -o $@ $^
