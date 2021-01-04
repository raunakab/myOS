#![allow(unused)]

use core::panic::PanicInfo;
use crate::serial_println;

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    use crate::qemu::{
        QemuExitCode,
        exit_qemu,
    };

    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);

    exit_qemu(QemuExitCode::Failed);
    loop {}
}
