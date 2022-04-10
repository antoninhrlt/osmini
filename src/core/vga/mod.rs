// This file is part of "osmini"
// Under the MIT License
// Copyright (c) Antonin Hérault

/// Video memory start pointer
pub const VGA_ADDRESS: u32 = 0xB8000;

pub const VGA_BUFFER_WIDTH: usize = 80;
pub const VGA_BUFFER_HEIGHT: usize = 25;

pub mod buffer;
pub mod colors;
pub mod print;
