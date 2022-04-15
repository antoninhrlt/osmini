# osmini
Mini operating system with a graphical interface, for x64 platforms, in Rust and Assembly 

## Build
Don't forget to install the [dependencies](#dependencies).

This project is delivered with its own project builder : the file "build.py" at
the repository's root. \
To build : `make build` \
Build and run onto "qemu" : `make run` [*](#other-dependencies)

## Dependencies
Project to build on a Linux platform
- [rust](https://www.rust-lang.org/) (and associated tools)
- ld (in `binutils` for Ubuntu)
- gcc
- nasm
- make
- ### Other dependencies
  - [qemu](https://www.qemu.org/download/)
  - qemu-system (for package `qemu-system-x86_64`)
