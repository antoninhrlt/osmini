// This file is part of "osmini"
// Under the MIT License
// Copyright (c) Antonin Hérault

use crate::vga::{
    VGA_BUFFER_WIDTH,
    VGA_BUFFER_HEIGHT,
};

/// VGA buffer transparent structure without functions
#[repr(transparent)] // like a real type
pub struct Buffer {
    chars: [[u8; VGA_BUFFER_WIDTH]; VGA_BUFFER_HEIGHT],
}
