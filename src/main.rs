#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(mundos::run_tests)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use mundos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, world!");
    println!("Welcome to MundOS v{}.{}.{}", 0, 0, 1);

    mundos::init();

    unsafe {
        *(0xdead2bad as *mut u64) = 42;
    }

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    mundos::test_panic_handler(info)
}
