#![allow(non_snake_case)]
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_framework::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
mod panic;
mod serial;
mod qemu;
mod test_framework;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("{}, {}{}", "Hello", "world", "!");
    let s: &str = "Some";
    println!("{}", s);

    #[cfg(test)]
    test_main();

    loop {}
}

// mod test_vga_buffer {
//     #[allow(unused)]
//     use crate::{
//         print,
//         println,
//         serial_print,
//         serial_println,
//     };

//     #[test_case]
//     fn trivial_assertion() -> () {
//         assert_eq!(1,1);
//     }

//     #[test_case]
//     fn println_single() -> () {
//         println!("println_single");
//     }

//     #[test_case]
//     fn println_many() -> () {
//         for _ in 0..100 {
//             println!("println_many");
//         }
//     }

//     #[test_case]
//     fn print_single() -> () {
//         print!("print_single");
//     }

//     #[test_case]
//     fn print_many() -> () {
//         for _ in 0..100 {
//             print!("print_many");
//         }
//     }

//     #[test_case]
//     fn println_output() -> () {
//         use crate::vga_buffer::STANDARD_WRITER as std_writer;

//         // let s: &str = "Some test string that fits on a single line";
//         let s: &str = "Some";
//         println!("{}", s);

//         for (i, c) in s.chars().enumerate() {
//             let screen_char: u8 = std_writer.lock().read_byte_at_position(23usize, i);
//             assert_eq!(char::from(screen_char), c);
//         }
//     }
// }
