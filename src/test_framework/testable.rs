#[allow(unused)]

use crate::{
    print,
    println,
    serial_print,
    serial_println,
};

pub trait Testable {
    fn run(self: &Self) -> ();
}
impl<T> Testable for T
where T: Fn(), {
    fn run(self: &Self) -> () {
        serial_print!("{}:\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}
