#![allow(non_snake_case)]
#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_framework::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

// internal modules
pub mod vga_buffer;
pub mod serial;
pub mod test_framework;
pub mod qemu;
pub mod panic;

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    panic::test_panic_handler(info)
}
