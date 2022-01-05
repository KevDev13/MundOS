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
struct ColorCode(u8);

impl ColorCode {
    fn new(fg: Color, bg: Color) -> ColorCode {
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
        // TODO
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
}

pub fn print_something() {
    let mut writer = ScreenWriter {
        col_pos: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut ScreenBuffer) },
    };

    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("Wörld!");
}
