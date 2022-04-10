// This file is part of "osmini"
// Under the MIT License
// Copyright (c) Antonin Hérault

#![no_main]
#![no_std]

/// Nothing should be put before `_start` \
/// Called by the bootloader at 0x1000
#[no_mangle]
pub extern "C" fn _start() {
    unsafe {
        main();
    }
    loop {}
}

unsafe fn main() {
    // test_hi();
}

/// ASM functions
extern "C" {
    fn test_hi();
} 

pub mod vga;

/// Called when `panic!()` \
/// NOTE `PanicInfo` is from Rust's core module
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
