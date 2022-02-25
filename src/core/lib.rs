// This file is part of "osmini"
// Under the MIT License
// Copyright (c) Antonin Hérault

#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// Called when `panic!()`
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
