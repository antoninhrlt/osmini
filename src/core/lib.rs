// This file is part of "osmini"
// Under the MIT License
// Copyright (c) Antonin Hérault

#![no_std]
#![no_main]

pub mod main;

use core::panic::PanicInfo;

/// Called when `panic!()`
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() {
    crate::main::core();
}
