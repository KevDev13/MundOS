#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(mundos::run_tests)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

fn run_tests(tests: &[&dyn Fn()]) {
    unimplemented!();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    mundos::test_panic_handler(_info)
}
