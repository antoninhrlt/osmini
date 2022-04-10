// This file is part of "osmini"
// Under the MIT License
// Copyright (c) Antonin Hérault

/// Video memory start pointer
static mut CURSOR_X: usize = 0;

unsafe fn print_char(c: u8) {
    let vga: *mut u8 = (0xB8000 as *mut u8).add(2 * CURSOR_X);
    *vga = c;
    *(vga.add(1)) = 0x0E; // color
    CURSOR_X += 1; 
}

#[no_mangle]
pub unsafe fn print_str(string: &[u8]) {
    for c in string {
        print_char(*c);
    }
}    
