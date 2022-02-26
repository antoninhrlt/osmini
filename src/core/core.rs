// This file is part of "osmini"
// Under the MIT License
// Copyright (c) Antonin Hérault

#![no_std]
#![no_main]

/// Entry point of the system's core
/// "Called" by the bootloader at address 0x1000
#[no_mangle]
pub extern "C" fn _start() {
    main();
    loop {}
}

fn main() {
    let mut screen = Screen::new();
    //screen.write_char(b'B');

    unsafe {
        *(0xB8000 as *mut u8) = b'o';
        *(0xB8001 as *mut u8) = 0x02;
    };
}

pub mod utils;
pub mod vga;

use crate::{
    utils::{
        point::Point,
    },
    vga::{
        color::{Color, to_vga_color},
        screen::Screen,
    },
};

/// Panic handler because of #[no_std]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
