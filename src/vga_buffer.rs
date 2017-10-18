#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
struct ColorCode(u8);

impl ColorCode {
    const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | foreground as u8)
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

use core::ptr::Unique;

pub struct Writer {
    row_position: usize,
    column_position: usize,
    color_code: ColorCode,
    buffer: Unique<Buffer>,
}

impl Writer {
    fn new() -> Writer {
        Writer {
            row_position: BUFFER_HEIGHT - 1,
            column_position: 0,
            color_code: ColorCode::new(Color::Blue, Color::White),
            buffer: unsafe { Unique::new_unchecked(0xb8000 as *mut _) },
        }
    }
    pub fn write_byte(&mut self, byte: u8) {
        if self.column_position >= BUFFER_WIDTH {
            self.new_line();
        }

        if byte == b'\n' {
            self.new_line();
            return;
        }

        let row = self.row_position;
        let col = self.column_position;
        let color = self.color_code;
        self.buffer().chars[row][col] = ScreenChar {
            ascii_character: byte,
            color_code: color,
        };
        self.column_position += 1;
    }

    fn buffer(&mut self) -> &mut Buffer {
        unsafe { self.buffer.as_mut() }
    }

    fn new_line(&mut self) {
        unimplemented!();
    }
}

pub fn print_something() {
    let mut writer = Writer::new();
    writer.write_byte(b'a');
}
