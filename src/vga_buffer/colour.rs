#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Colour {
    Black = 0u8,
    Blue = 1u8,
    Green = 2u8,
    Cyan = 3u8,
    Red = 4u8,
    Magenta = 5u8,
    Brown = 6u8,
    LightGray = 7u8,
    DarkGray = 8u8,
    LightBlue = 9u8,
    LightGreen = 10u8,
    LightCyan = 11u8,
    LightRed = 12u8,
    Pink = 13u8,
    Yellow = 14u8,
    White = 15u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub(in super) struct ColourCode (
    u8,
);
impl ColourCode {
    pub(in super) fn new(foreground: Colour, background: Colour) -> ColourCode {
        return ColourCode (
            (background as u8) << 4 | (foreground as u8),
        );
    }
}
