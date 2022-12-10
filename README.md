# osmini
Mini operating system with a graphical interface, for x64 platforms, in Rust and Assembly 

## Build
Don't forget to install the [dependencies](#dependencies).

Run the following command to build the project : `make build`

## Dependencies
Project to build on a Linux platform
- [rust](https://www.rust-lang.org/) (and associated tools)
- ld (in `binutils` for Ubuntu)
- gcc
- nasm
- make

## System emulator
When running `make run`, the [qemu](https://www.qemu.org/download/) program starts. Be sure it is installed.

You have to install : `qemu-system` (for package `qemu-system-x86_64`)
