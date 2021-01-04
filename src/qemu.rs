#![allow(unused)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10u32,
    Failed = 0x11u32,
}

pub fn exit_qemu(exit_code: QemuExitCode) -> () {
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port: Port::<u32> = Port::new(0xf4u16);
        port.write(exit_code as u32);
    }
}
