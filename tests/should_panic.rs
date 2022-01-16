#![no_std]
#![no_main]

use core::panic::PanicInfo;
use mundos::{
    QemuExitCode,
    qemu_exit,
    serial_println,
    serial_print
};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    qemu_exit(QemuExitCode::Success);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("[test did not panic]");
    qemu_exit(QemuExitCode::Failure);
    loop {}
}

fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}
