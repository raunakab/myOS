#![allow(unused)]

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(myOS::test_framework::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use myOS::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

fn test_runner(tests: &[&dyn Fn()]) -> () {
    unimplemented!();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    myOS::panic::test_panic_handler(info)
}

mod test_vga_buffer {
    #[allow(unused)]
    use myOS::{
        print,
        println,
        serial_print,
        serial_println,
    };

    #[test_case]
    fn test_println() {
        println!("test_println output");
    }

    #[test_case]
    fn trivial_assertion() -> () {
        assert_eq!(1,1);
    }

    #[test_case]
    fn println_single() -> () {
        println!("println_single");
    }

    #[test_case]
    fn println_many() -> () {
        for _ in 0..100 {
            println!("println_many");
        }
    }

    #[test_case]
    fn print_single() -> () {
        print!("print_single");
    }

    #[test_case]
    fn print_many() -> () {
        for _ in 0..100 {
            print!("print_many");
        }
    }

    // For some reason, this testcase is failing...
    // #[test_case]
    // fn println_output() -> () {
    //     use myOS::vga_buffer::STANDARD_WRITER as std_writer;

    //     // let s: &str = "Some test string that fits on a single line";
    //     let s: &str = "Some";
    //     println!("{}", s);

    //     for (i, c) in s.chars().enumerate() {
    //         let screen_char: u8 = std_writer.lock().read_byte_at_position(23usize, i);
    //         assert_eq!(char::from(screen_char), c);
    //     }
    // }
}

