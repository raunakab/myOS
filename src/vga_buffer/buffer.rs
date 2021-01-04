use volatile::Volatile;
use super::screen_char::ScreenChar;

pub(in super) const BUFFER_HEIGHT: usize = 25usize;
pub(in super) const BUFFER_WIDTH: usize = 80usize;

#[repr(transparent)]
pub(in super) struct Buffer {
    pub(in super) chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}
