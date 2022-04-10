// This file is part of "osmini"
// Under the MIT License
// Copyright (c) Antonin Hérault

/// A VGA attribute (its color) is coded like that:
/// ```
/// 7___________________________________0
/// | cli | background | i | foreground |
/// ```
/// With colors as binary:
/// - black: 000
/// - blue: 001
/// - green: 010
/// - cyan: 011
/// - red: 100
/// - purple: 101
/// - yellow: 110
/// - gray: 111
#[allow(unused)]
#[derive(Copy, Clone)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Purple = 5,
    Yellow = 6,
    White = 7,
}

/// Transform two colors (foreground and background) to one hexadecimal value \
/// 0xXY, where `X` = background, and `Y` = foreground \
/// Example: 0x02, where the background is black and foreground is green
pub fn to_vga_color(fg: Color, bg: Color) -> u8 {
    0x02 // todo!()
}
