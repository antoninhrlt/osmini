// This file is part of "osmini"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

#![no_std]

extern "C" {
    fn _bootloader() -> !;
}

// Panic handler need when `#![no_std]`
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
