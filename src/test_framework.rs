#![allow(unused)]

mod testable;

use testable::Testable;
use crate::{
    print,
    println,
    serial_print,
    serial_println,
};
use crate::qemu::{
    QemuExitCode,
    exit_qemu,
};

// #[cfg(test)]
// pub fn test_runner(tests: &[&dyn Testable]) -> () {
//     serial_println!("Running {} tests", tests.len());
//     for test in tests {
//         test.run();
//     }

    // use crate::qemu::{
    //     QemuExitCode,
    //     exit_qemu,
    // };
//     exit_qemu(QemuExitCode::Success);
// }

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}
