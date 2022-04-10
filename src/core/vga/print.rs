// This file is part of "osmini"
// Under the MIT License
// Copyright (c) Antonin Hérault

use crate::vga::{
    VGA_ADDRESS,
    buffer::Buffer,
};

pub struct Printer {
    buffer: &'static mut Buffer,
    pos_x: usize,
}

impl Printer {
    pub unsafe fn new() -> Self {
        Self {
            buffer: &mut *(VGA_ADDRESS as *mut Buffer),
            pos_x: 0,
        }
    }

    pub unsafe fn print(&mut self, string: &[u8]) {
        
    }
} 