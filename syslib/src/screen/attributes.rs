// This file is part of "osmini"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    White = 7,

    DarkGray = 8,
    BrightBlue = 9,
    BrightGreen = 10,
    BrightCyan = 11,
    Pink = 12,
    BrightMagenta = 13,
    Yellow = 14,
    BrightWhite = 15,
}

pub fn color_code_to_u8(fg: Color, bg: Color) -> u8 {
    (bg as u8) << 4 | (fg as u8)
}
