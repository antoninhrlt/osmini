// This file is part of "osmini"
// Under the MIT License
// Copyright (c) Antonin Hérault

use crate::{
    vga::{
        VGA_ADDRESS,
        color::{
            Color, 
            to_vga_color
        },
        cursor::Cursor,
    },
};

/// Structure to manage the VGA screen
pub struct Screen {
    vga: *mut u8,
    cursor: Cursor,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            vga: VGA_ADDRESS as *mut u8,
            cursor: Cursor::new(),
        }
    }

    /// Draw a char onto the screen at the chosen position
    pub unsafe fn write_char(&mut self, what: u8) {
        // Write the character at the current cursor position
        *(self.vga.add(0)) = b'A';
        self.cursor.increment(1);

        // Write the character's colors
        *(self.vga.add(1)) = 0x02; //to_vga_color(Color::White, Color::Black);
        self.cursor.increment(1);
    }
}
