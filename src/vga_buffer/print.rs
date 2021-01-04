#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::print::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    use crate::vga_buffer::STANDARD_WRITER as std_writer;

    std_writer.lock().write_fmt(args).unwrap();
}
