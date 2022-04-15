# osmini
Mini operating system with a graphical interface, for x64 platforms, in Rust and Assembly 

## Build
Don't forget to install the [dependencies](#requirements).

This project is delivered with its own project builder : the file "build.py" at
the repository's root. \
To build : `python3 build.py build` \
Build and run onto "qemu" : `python3 build.py run` [*](#other-dependencies)

> NOTE : If you run "build.py" without arguments, it will do nothing

## Dependencies
Project to build on a Linux platform
- [rust](https://www.rust-lang.org/) (and associated tools)
- ld (in `binutils` for Ubuntu)
- gcc
- nasm
- python3 (for build file)
- ### Other dependencies
  - [qemu](https://www.qemu.org/download/)
  - qemu-system (for package `qemu-system-x86_64`)
