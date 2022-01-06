#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga;
use vga::*;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, world!");
    println!("Welcome to MundOS v{}.{}.{}", 0, 0, 1);
    vga_color!(ColorCode::new(Color::Yellow, Color::Black));
    println!("testing 123...");
    loop {}
}
