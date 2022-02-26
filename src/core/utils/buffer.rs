// This file is part of "osmini"
// Under the MIT License
// Copyright (c) Antonin Hérault

use crate::{
    utils::{
        point::Point,
    },
};

pub struct Buffer<T> {
    buffer: *mut T,
    width: usize,
    height: usize,
}

impl<T> Buffer<T> {
    /// Create a new buffer without any registered values
    pub fn new(width: usize, height: usize) -> Self {
        Self::from_addr(0, width, height)
    }

    /// Create manipulable buffer from an existing buffer
    pub fn from(buffer: *mut T, width: usize, height: usize) -> Self {
        Self {
            buffer,
            width,
            height,
        }
    }
    
    /// Create a manipulable buffer from a memory address
    pub fn from_addr(address: u32, width: usize, height: usize) -> Self {
        Self {
            buffer: address as *mut T,
            width,
            height,
        }
    }

    pub fn write(&mut self, pos: &Point, what: T) {
        let pos_1d: usize = pos.x; // TODO adapt this for pos.y
        unsafe {
            *(self.buffer.add(pos_1d)) = what;
        }
    }
}
