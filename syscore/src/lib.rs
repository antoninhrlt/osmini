// This file is part of "osmini"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

//! x64 system core started at address 0x1000, that means :
//! `org 0x1000` (see the `ld` command in "Makefile")

#![no_std]
#![feature(lang_items)]

/// Simply call the main system main function and do a loop on itself to keep
/// the OS as "on". \
/// The `_start` function need to be the first element, that's why module
/// declarations are done below
#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        system::main();
    }
    loop {}
}

pub mod system;

// Panic handler needed when `#![no_std]`
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
