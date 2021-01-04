use super::colour::ColourCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub(in super) struct ScreenChar {
    pub(in super) ascii_character: u8,
    pub(in super) color_code: ColourCode,
}
impl ScreenChar {
    pub(in super) fn new(ascii_character: u8, color_code: ColourCode) -> Self {
        return ScreenChar {
            ascii_character: ascii_character,
            color_code: color_code,
        }
    }
}
