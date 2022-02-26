// This file is part of "osmini"
// Under the MIT License
// Copyright (c) Antonin Hérault

use crate::{
    utils::{
        buffer::Buffer,
        point::Point,
    },
    vga::{
        VGA_ADDRESS, VGA_WIDTH, VGA_HEIGHT,
        color::{Color, to_vga_color},
    },
};

/// Structure to manage the VGA screen
pub struct Screen {
    vga_buffer: Buffer<u8>,
    cursor: Point,
    colors: (Color, Color),
}

impl Screen {
    pub fn new() -> Self {
        Self {
            vga_buffer: Buffer::from_addr(VGA_ADDRESS, VGA_WIDTH, VGA_HEIGHT),
            cursor: Point{x: 0, y: 0},
            colors: (Color::Green, Color::Black),
        }
    }

    /// Draw a char onto the screen at the chosen position
    pub fn write_char(&mut self, what: u8) {
        // Write the character
        self.vga_buffer.write(&Point{x: 1, y: 0}, what);
        //self.cursor.x += 1;

        // Write the character's colors
        self.vga_buffer.write(
            &Point{x: 1, y: 0}, 
            to_vga_color(self.colors.0, self.colors.1)
        );
        //self.cursor.x += 1;
    }

    pub fn write_str(&mut self, what: &[u8]) {
        // let mut pos = Point{
        //     x: start_pos.x, 
        //     y: start_pos.y
        // };

        // for c in what {
        //     pos.x += 1;
        //     if *c == b'\n' {
        //         pos.y += 1;
        //     }
        //     self.write_char(Point{x: pos.x, y: pos.y}, *c);
        // }
    }
}
