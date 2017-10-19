use core::fmt;
use core::fmt::Write;

use volatile::Volatile;
use spin::Mutex;

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
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

use core::ptr::Unique;

pub struct Writer {
    row_position: usize,
    column_position: usize,
    color_code: ColorCode,
    buffer: Unique<Buffer>,
}

impl Writer {
    const fn new() -> Writer {
        Writer {
            row_position: BUFFER_HEIGHT - 1, // TODO: set 0 to support scrolling?
            column_position: 0,
            color_code: ColorCode::new(Color::LightGreen, Color::Black),
            buffer: unsafe { Unique::new_unchecked(0xb8000 as *mut _) },
        }
    }

    fn write_byte(&mut self, byte: u8) {
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
        self.buffer().chars[row][col].write(ScreenChar {
            ascii_character: byte,
            color_code: color,
        });
        self.column_position += 1;
    }

    fn buffer(&mut self) -> &mut Buffer {
        unsafe { self.buffer.as_mut() }
    }

    fn new_line(&mut self) {
        (1..BUFFER_HEIGHT).for_each(|row| {
            (0..BUFFER_WIDTH).for_each(|col| {
                let buffer = self.buffer();
                let character = buffer.chars[row][col].read();
                buffer.chars[row - 1][col].write(character);
            })
        });
        let mut row = self.row_position;
        if row < BUFFER_HEIGHT - 1 {
            row += 1;
        }
        self.clear_row(row);
        self.row_position = row;
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        (0..BUFFER_WIDTH).for_each(|col| self.buffer().chars[row][col].write(blank));
    }
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        s.bytes().for_each(|x| self.write_byte(x));
        Ok(())
    }
}

pub static WRITER: Mutex<Writer> = Mutex::new(Writer::new());

pub fn print_something() {
    let mut writer = Writer::new();
    writer.write_byte(b'H');
    writer.write_str("ello").unwrap();
    write!(writer, " World\n{}\n", 3.1415926).unwrap();
    WRITER.lock().write_str("static Hello\n").unwrap();
    write!(WRITER.lock(), "{}\n", 3.1415926).unwrap();
}
