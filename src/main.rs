#![no_std]
#![no_main]

// testing needs
#![feature(custom_test_frameworks)]
#![test_runner(crate::run_tests)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod vga;
use vga::*;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, world!");
    println!("Welcome to MundOS v{}.{}.{}", 0, 0, 1);

    #[cfg(test)]
    test_main();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

/* Testing Functions */

#[cfg(test)]
fn print_test_status(test_passed: usize) {
    let cur_cc = SCREEN_WRITER.lock().get_color();
    if test_passed == 0{
        vga_color!(ColorCode::new(Color::Green, Color::Black));
        println!("[OK]");
    } else if test_passed == 1 {
        vga_color!(ColorCode::new(Color::Red, Color::Black));
        println!("[Fail]");
    } else {
        vga_color!(ColorCode::new(Color::Yellow, Color::Black));
        println!("[Unknown]");
    }
    vga_color!(cur_cc);
}

#[cfg(test)]
fn run_tests(tests: &[&dyn Fn()]) {
    println!("Running {} total tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assert() {
    print!("Testing trivial... ");
    assert_eq!(1, 1);
    print_test_status(0);
}
