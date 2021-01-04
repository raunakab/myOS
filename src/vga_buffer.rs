#![allow(unused)]
#![allow(dead_code)]

// external crates
use volatile::Volatile;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

// internal modules
mod colour;
mod screen_char;
mod buffer;
pub mod print;

use colour::{
    Colour,
    ColourCode,
};
use screen_char::ScreenChar;
use buffer::{
    Buffer,
    BUFFER_HEIGHT,
    BUFFER_WIDTH,
};

const TAB_SIZE: u8 = 4u8;

const LOWER_BYTE: u8 = 0x20u8;
const UPPER_BYTE: u8 = 0x7eu8;
const START_LOCATION: usize = 0xb8000usize;

const BLANK_BYTE: u8 = b' ';
const INVALID_BYTE: u8 = 0xfeu8;

const ESCAPE_BYTE_NEWLINE: u8 = b'\n';
const ESCAPE_BYTE_TAB: u8 = b'\t';

lazy_static! {
    pub static ref STANDARD_WRITER: Mutex::<Writer> = Mutex::new(
        Writer::new(Colour::Yellow, Colour:: Black)
    );
}

pub struct Writer {
    column_position: usize,
    color_code: ColourCode,
    buffer: &'static mut Buffer,
}
impl Writer {
    pub fn new(c1: Colour, c2: Colour) -> Self {
        return Writer {
            column_position: 0,
            color_code: colour::ColourCode::new(c1, c2),
            buffer: unsafe {
                /*
                From Phillip Opperman's Notes:
                    First, we cast the integer 0xb8000 as an mutable raw pointer.
                    Then we convert it to a mutable reference by dereferencing it (through *)
                    and immediately borrowing it again (through &mut).

                    This conversion requires an unsafe block, since the compiler
                    can't guarantee that the raw pointer is valid.
                */
                &mut *(START_LOCATION as *mut Buffer)
            },
        };
    }

    /*
        General methods
        note: No checks performed for index out of range errors for all methods in this category! Use carefully.
    */
    #[inline]
    fn write_byte_at_position(self: &mut Self, byte: u8, rpos: usize, cpos: usize) -> () {
        let colour_code: ColourCode = (*self).color_code;
        let position: &mut Volatile::<ScreenChar> = &mut (* (*self).buffer).chars[rpos][cpos];
        let screen_char: ScreenChar = ScreenChar::new(byte, colour_code);

        (*position).write(screen_char);
    }
    // #[inline]
    // fn write_byte_at_row_slice(self: &mut Self, byte: u8, rpos: usize, start_inclusive: usize, end_inclusive: usize) -> () {
    //     for col in start_inclusive..=end_inclusive {
    //         (*self).write_byte_at_position(byte, rpos, col);
    //     }
    // }
    // #[inline]
    // fn write_byte_at_row(self: &mut Self, byte: u8, rpos: usize) -> () {
    //     (*self).write_byte_at_row_slice(byte, rpos, 0, BUFFER_WIDTH - 1usize);
    // }
    #[inline]
    pub fn read_byte_at_position(self: &mut Self, rpos: usize, cpos: usize) -> u8 {
        return (* (*self).buffer).chars[rpos][cpos].read().ascii_character;
    }
    // #[inline]
    // fn read_bytes_at_row_slice(self: &mut Self, rpos: usize, start: usize, end: usize) -> u8 { todo!(); }
    // #[inline]
    // fn read_bytes_at_row(self: &mut Self, rpos: usize) -> u8 { todo!(); }

    /*
        Specialized methods
    */
    // #[inline]
    // fn write_blank_byte_at_row(self: &mut Self, rpos: usize) -> () {
    //     // (*self).write_byte_at_row(BLANK_BYTE, rpos);
    // }
    // #[inline]
    // fn clear_bottom_row(self: &mut Self) -> () {
    //     // (*self).write_blank_byte_at_row(BUFFER_HEIGHT - 1usize);
    //     for col in 0..BUFFER_WIDTH {
    //         (*self).write_byte_at_position(BLANK_BYTE, BUFFER_HEIGHT - 1usize, col);
    //     }
    // }
    #[inline]
    fn shift_up(self: &mut Self) -> () {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let c: u8 = (*self).read_byte_at_position(row, col);
                (*self).write_byte_at_position(c, row - 1usize, col);
            }
        }

        // (*self).clear_bottom_row();
        for col in 0..BUFFER_WIDTH {
            (*self).write_byte_at_position(BLANK_BYTE, BUFFER_HEIGHT - 1usize, col);
        }
        (*self).column_position = 0;
    }
    #[inline]
    fn write_byte_at_next_position(self: &mut Self, byte: u8) -> () {
        // match byte {
        //     b'\n' => (*self).new_line(),
        //     byte => {
        //         if (*self).column_position >= BUFFER_WIDTH {
        //             (*self).new_line();
        //         }

        //         // to write to the bottom
        //         let row: usize = BUFFER_HEIGHT - 1usize;
        //         let col: usize = (*self).column_position;

        //         (*self).write_byte_at_position(byte, row, col);
        //         (*self).column_position += 1;
        //     }
        // }
        if (*self).column_position >= BUFFER_WIDTH {
            (*self).shift_up();
        }

        // to write to the bottom
        let row: usize = BUFFER_HEIGHT - 1usize;
        let col: usize = (*self).column_position;

        (*self).write_byte_at_position(byte, row, col);
        (*self).column_position += 1;
    }

    pub fn write_string(self: &mut Self, s: &str) -> () {
        for byte in s.bytes() {
            match byte {
                LOWER_BYTE..=UPPER_BYTE => (*self).write_byte_at_next_position(byte),
                ESCAPE_BYTE_NEWLINE => (*self).shift_up(),
                ESCAPE_BYTE_TAB => {
                    for i in 0..TAB_SIZE {
                        (*self).write_byte_at_next_position(BLANK_BYTE);
                    }
                }
                _ => (*self).write_byte_at_next_position(INVALID_BYTE),
            }
        }
    }
}
impl fmt::Write for Writer {
    fn write_str(self: &mut Self, s: &str) -> fmt::Result {
        (*self).write_string(s);
        return fmt::Result::Ok(());
    }
}
