use volatile::Volatile;
use spin::Mutex;

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::vga_buffer::print(format_args!($($arg)*));
    });
}

pub fn print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

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
            row_position: 0,
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
        } else if byte == b'\t' {
            self.column_position += 4;
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
        let mut row = self.row_position;

        if row == BUFFER_HEIGHT - 1 {
            (1..BUFFER_HEIGHT).for_each(|row| {
                (0..BUFFER_WIDTH).for_each(|col| {
                    let buffer = self.buffer();
                    let character = buffer.chars[row][col].read();
                    buffer.chars[row - 1][col].write(character);
                })
            });
        } else {
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

impl ::core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        s.bytes().for_each(|x| self.write_byte(x));
        Ok(())
    }
}

pub static WRITER: Mutex<Writer> = Mutex::new(Writer::new());

pub fn clear_screen() {
    (0..BUFFER_HEIGHT).for_each(|row| WRITER.lock().clear_row(row));
}
