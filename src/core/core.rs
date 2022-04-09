// This file is part of "osmini"
// Under the MIT License
// Copyright (c) Antonin Hérault

#![no_main]
#![no_std]

/// Entry point of the system's core
/// "Called" by the bootloader at address 0x1000
#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        main();
    }
    loop {}
}

unsafe fn main() {
    // let mut screen = Screen::new();
    // screen.write_char(b'A');

    // unsafe {
    //     let vga: *mut u8 = 0xB8000 as *mut u8;
    //     *(vga.add(0)) = b'A';
    //     *(vga.add(1)) = 0x02;
    //     *(vga.add(2)) = b'b';
    //     *(vga.add(3)) = 0x02;
    // }
}

pub mod vga;

use crate::{
    vga::{
        color::{Color, to_vga_color},
        cursor::Cursor,
        screen::Screen,
    },
};

/// Panic handler because of #[no_std] \
/// Called when `panic!()` \
/// NOTE `PanicInfo` is from Rust's core module
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
