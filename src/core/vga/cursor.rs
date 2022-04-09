// This file is part of "osmini"
// Under the MIT License
// Copyright (c) Antonin Hérault

use crate::{
    vga::{
        VGA_WIDTH, VGA_HEIGHT,
    },
};

/// Simple structure to make 2D cursor \
/// Can be displayed on the VGA screen by `.display()`
pub struct Cursor {
    x: usize,
    y: usize
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0
        }
    }

    /// Change position to fit according to the VGA dimensions
    fn fit(&mut self) {
        if self.x >= VGA_WIDTH {
            // self.y += 1;   
        }

        if self.y >= VGA_HEIGHT {
            // TODO : Scroll the window
        }
    }

    /// Update the current cursor position \
    /// NOTE do not display it a new time
    pub fn update(&mut self, x: usize) {
        self.x = x;
        self.fit();
    }

    // Like an update function but increment `x`
    pub fn increment(&mut self, n: usize) {
        self.update(self.x + n);
    }

    /// Display a cursor at the current `Cursor` object position
    pub fn display(&mut self) {}

    // Getters
    pub fn x(&self) -> &usize {
        &self.x
    }

    pub fn y(&self) -> &usize {
        &self.y
    }
}
