# This file is part of "osmini"
# Under the MIT License
# Copyright (c) 2022 Antonin Hérault

OUT_DIR = target/x64osmini/debug/

BOOTLOADER_OUT = $(OUT_DIR)/bootloader.o
SYSCORE_OUT = $(OUT_DIR)/libsyscore.a
SYSLIB_OUT = $(OUT_DIR)/libsyslib.a

SYSTEM_OUT = $(OUT_DIR)/system.bin
IMAGE_OUT = $(OUT_DIR)/osmini.img

build : 
	cargo build
	ld -o $(SYSTEM_OUT) --oformat binary -Ttext 1000 $(SYSCORE_OUT) $(SYSLIB_OUT)
	cat $(BOOTLOADER_OUT) $(SYSTEM_OUT) /dev/zero | dd of=$(IMAGE_OUT) bs=512 count=2880

run : build
	qemu-system-x86_64 -boot a -drive file=$(IMAGE_OUT),format=raw

clean :
	rm -rf target/

