#![no_std]
#![no_main]
// #![feature(const_mut_refs)]

use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, world!");
    println!("Welcome to MundOS v{}.{}.{}", 0, 0, 1);
    panic!("OH SHIT");
    // loop {}
}
