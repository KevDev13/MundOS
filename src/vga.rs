#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(fg: Color, bg: Color) -> ColorCode {
        ColorCode((bg as u8) << 4 | (fg as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_char: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct ScreenBuffer {
    chars: [[volatile::Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct ScreenWriter {
    col_pos: usize,
    color_code: ColorCode,
    buffer: &'static mut ScreenBuffer,
}

impl ScreenWriter {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.col_pos >= BUFFER_WIDTH {
                    self.new_line();
                }

                let r = BUFFER_HEIGHT - 1;
                let c = self.col_pos;

                let color_code = self.color_code;
                self.buffer.chars[r][c].write(ScreenChar {
                    ascii_char: byte,
                    color_code,
                });
                self.col_pos += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for r in 1..BUFFER_HEIGHT {
            for c in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[r][c].read();
                self.buffer.chars[r-1][c].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.col_pos = 0;
    }

    fn clear_row(&mut self, r: usize) {
        let blank = ScreenChar {
            ascii_char: b' ',
            color_code: self.color_code,
        };

        for c in 0..BUFFER_WIDTH {
            self.buffer.chars[r][c].write(blank);
        }
    }

    pub fn write_string(&mut self, string: &str) {
        for byte in string.bytes() {
            match byte {
                // something we can print, or newline character
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // something we can't print
                _ => self.write_byte(0xfe), // print a square
            }
        }
    }

    pub fn change_color(&mut self, new_color: ColorCode) {
        self.color_code = new_color;
    }
}


impl core::fmt::Write for ScreenWriter {
    fn write_str(&mut self, string: &str) -> core::fmt::Result {
        self.write_string(string);
        Ok(())
    }
}

lazy_static::lazy_static! {
    pub static ref SCREEN_WRITER: spin::Mutex<ScreenWriter> =spin::Mutex::new(ScreenWriter {
        col_pos: 0,
        color_code: ColorCode::new(Color::LightGray, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut ScreenBuffer) },
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    SCREEN_WRITER.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! vga_color {
    ($color:expr) => {
        SCREEN_WRITER.lock().change_color($color);
    }
}
