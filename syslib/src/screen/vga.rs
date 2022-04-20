// This file is part of "osmini"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

//! Elements from this module should be used as `vga::<identifier>` by 
//! considering the importation like this : `use syslib::screen::vga;`

pub const ADDRESS: u32 = 0xB8000;
pub const HEIGHT: usize = 25;
pub const WIDTH: usize = 80;

pub unsafe fn write_at(offset: isize, what: u8) {
    *(ADDRESS as *mut u8).offset(offset) = what;
}
