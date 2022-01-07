lazy_static::lazy_static! {
    pub static ref SERIAL1: spin::Mutex<uart_16550::SerialPort> = {
        let mut serial_port = unsafe { uart_16550::SerialPort::new(0x3F8) };
        serial_port.init();
        spin::Mutex::new(serial_port)
    };
}

// items used to print to host from QEMU testing

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(args).expect("Failed to print to SERIAL1");
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => { $crate::serial::_print(format_args!($($arg)*)); }
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($arg)*));
}
