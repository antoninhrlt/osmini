# This file is part of "osmini"
# Under the MIT License
# Copyright (c) Antonin Hérault

ASM = nasm
LINKER = ld
RUST = rustc

OUT_DIR = build

IMG = $(OUT_DIR)/osmini.img
BOOT = $(OUT_DIR)/bootloader.out
CORE = $(OUT_DIR)/core.out

# Private function to simplify the release creation
# Should not be used by another one than the repository's owner
_release : $(IMG)
	cp $< bin/

_init :
	mkdir -p $(OUT_DIR) $(OUT_DIR)/core/

build : _init $(IMG)

run : build
	qemu-system-x86_64 -boot a -drive file=$(IMG),format=raw

clean :
	rm -rf $(OUT_DIR)

# Create the system image file
$(IMG) : $(BOOT) $(CORE)
	cat $^ /dev/zero | dd of=$@ bs=512 count=2880 

# Boot loader
$(BOOT) : src/boot/main.asm
	$(ASM) -f bin -I src/boot/ -o $@ $^

# System core
CORERS = build/osmini/debug/libcorers.a
CORE_SRC = $(wildcard src/core/*.asm) 
CORE_OBJ = $(patsubst src/core/%.asm, $(OUT_DIR)/core/%.o, $(CORE_SRC))

# ASM and Rust object files linked
$(CORE) : $(CORERS) $(CORE_OBJ)
	$(LINKER) --oformat binary -Ttext 1000 -o $@ $^

# ASM files for core
$(OUT_DIR)/core/%.o : src/core/%.asm
	$(ASM) -f elf64 -I src/core/ -o $@ $<

# Rust files for core
$(CORERS) : $(wildcard src/core/*.rs)
	cargo build

.SILENT : help
help :
	echo -e "\x1b[1m"
	echo -e "Help page for osmini" "\n"
	echo "Available commands (make) :"
	echo -e "\t" "build : build the project"
	echo -e "\t" "help : get this help page"
	echo -e "\t" "run : run the system into the qemu emulator"
	echo "Dependencies :"
	echo -e "\t" "(build) make, nasm, ld, "
	echo -e "\t" "(run) qemu-system-x86_64"
	echo -e "\x1b[0m"
