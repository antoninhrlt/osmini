# This file is part of "osmini"
# Under the MIT License
# Copyright (c) 2022 Antonin Hérault

#!/bin/python3

import sys
import subprocess


OUT_DIR = "target/debug"

BOOTLOADER_OUT = OUT_DIR + "/bootloader.o"
SYSCORE_OUT = OUT_DIR + "/libsyscore.a"
SYSLIB_OUT = OUT_DIR + "/libsyslib.a"

SYSTEM_OUT = OUT_DIR + "/system.bin"
IMAGE_OUT = OUT_DIR + "/osmini.img"


def build():
    subprocess.run(["cargo", "build"])

    subprocess.run([
        "ld",
        "-o", SYSTEM_OUT,
        "--oformat", "binary",
        "-Ttext", "1000",
        SYSCORE_OUT,
        SYSLIB_OUT
    ])

    cat = subprocess.Popen([
        "cat",
        BOOTLOADER_OUT,
        SYSTEM_OUT,
        "/dev/zero",
    ], stdout=subprocess.PIPE)

    dd = subprocess.Popen([
        "dd",
        f"of={IMAGE_OUT}",
        "bs=512",
        "count=2880"
    ], stdin=cat.stdout, stdout=subprocess.PIPE)

    cat.stdout.close()
    
    dd.communicate()[0]


def run():
    subprocess.run([
        "qemu-system-x86_64",
        "-boot", "a",
        "-drive", f"file={IMAGE_OUT},format=raw"
    ])


if "build" in sys.argv:
    build()

if "run" in sys.argv:
    build()
    run()
