// #![allow(unused)]

// use core::panic::PanicInfo;
// use crate::{
//     print,
//     println,
//     serial_print,
//     serial_println,
// };
// use crate::qemu::{
//     QemuExitCode,
//     exit_qemu,
// };

// panic handler in run mode
// #[cfg(not(test))]
// #[panic_handler]
// fn panic(info: &PanicInfo) -> ! {
//     println!("{}", info);
//     loop {}
// }

// // panic handler in test mode
// #[cfg(test)]
// #[panic_handler]
// fn panic(info: &PanicInfo) -> ! {
//     test_panic_handler(info);
// }

// pub fn test_panic_handler(info: &PanicInfo) -> ! {
//     serial_println!("[failed]\n");
//     serial_println!("Error: {}\n", info);
//     exit_qemu(QemuExitCode::Failed);

//     loop {}
// }
