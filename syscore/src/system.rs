// This file is part of "osmini"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

use syslib::drivers;
use syslib::screen::vga;

static HELLO_WORLD: &[u8] = b"Bonjour";

pub unsafe fn main() {
    drivers::cursor::disable();

    vga::write_at(2 * 2, HELLO_WORLD[0]);
    vga::write_at(2 * 2 + 1, 0x0F);
}
