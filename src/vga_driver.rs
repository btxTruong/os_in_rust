use volatile::Volatile;
use core::fmt;
use lazy_static::lazy_static;
use spin;

const SCREEN_HEIGHT: usize = 25;
const SCREEN_WIDTH: usize = 80;

// Lazy variable are initialized at compile time, in contrast to normal variables
// that are initialized at run time
lazy_static! {
    pub static ref VGA_WRITER: spin::Mutex<VgaWriter> = spin::Mutex::new(VgaWriter {
        col_pos: 0,
        row_pos: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}


pub struct VgaWriter {
    col_pos: usize,
    row_pos: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl VgaWriter {
    fn write_byte(&mut self, text_as_byte: u8) {
        match text_as_byte {
            b'\n' => self.new_line(),
            text_as_byte => {
                if self.col_pos >= SCREEN_WIDTH {
                    self.new_line();
                }

                let row = self.row_pos;
                let col = self.col_pos;


                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_char: text_as_byte,
                    color_code: self.color_code,
                });

                self.col_pos += 1;
            }
        }
    }

    pub fn write_string(&mut self, string: &str) {
        for byte in string.bytes() {
            //     VGA text only support ascii, rust string are utf-8
            //     so they might contain bytes that are not supported by VGA buffer
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // handle byte not ascii
                _ => self.write_byte(0xfe)
            }
        }
    }

    fn new_line(&mut self) {
        self.row_pos += 1;
        self.col_pos = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_char: b' ',
            color_code: self.color_code,
        };

        for col in 0..SCREEN_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl fmt::Write for VgaWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

// repr transparent make Buffer ins has same memory layout as chars field

// 1. A 2D array is stored in the computer's memory one row following another.
//
// 2. The address of the first byte of memory is considered as the memory location of the entire 2D array.
//
// 3. Knowing the address of the first byte of memory, the compiler can easily compute
// to find the memory location of any other elements in the 2D array provided the number of columns in the array is known.
//
// 4. If each data value of the array requires B bytes of memory, and if the array has C columns,
// then the memory location of an element such as score[m][n] is (m*c+n)*B from the address of the first byte.
//
// 5. Note that to find the memory location of any element, there is no need to know the total number of rows in the array,
// i.e. the size of the first dimension. Of course the size of the first dimension is needed to prevent reading or storing data that is out of bounds.
//
// 6. Again one should not think of a 2D array as just an array with two indexes.
// You should think of it as an array of arrays.
//
// 7. Higher dimensional arrays should be similarly interpreted.
// For example a 3D array should be thought of as an array of arrays of arrays.
// To find the memory location of any element in the array relative to the address of the first byte,
// the sizes of all dimensions other than the first must be known.
//
// 8. Knowledge of how multidimensional arrays are stored in memory
// helps one understand how they can be initialized, and how they can be passed as function arguments.
/// ```
/// let s1 = String::from("hello")
///
///         s1                                memory
/// |   name  | value |                 | index | value |
/// |   ptr   |   --------------------->|   0   |   h   |
/// |   len   |   5   |                 |   1   |   e   |
/// |capacity |   5   |                 |   2   |   l   |
///                                     |   3   |   l   |
///                                     |   4   |   o   |
///
/// ptr s1 point to first memory
/// ```
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; SCREEN_WIDTH]; SCREEN_HEIGHT]
}

// represent ScreenChar memory layout same in C
// first byte: character
// second byte: how the character is displayed
// Copy trait will copy ins when move
// Clone trait will move ins
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_char: u8,
    color_code: ColorCode,
}

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

// fmt:Arguments represent can multiple argument
// Since the macros need to be able to call _print from outside of the module,
// the function needs to be public.
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    VGA_WRITER.lock().write_fmt(args).unwrap();
}

// The #[macro_export] attribute makes the macro available to the whole crate (not just the module it is defined) and external crates
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_driver::_print(format_args!($($arg)*)))
}

// One thing that we changed from the original println definition is that
// we prefixed the invocations of the print! macro with $crate too.
// This ensures that we don't need to have to import the print! macro too if we only want to use println.
//
// Like in the standard library, we add the #[macro_export] attribute to both macros to make them available everywhere in our crate.
// Note that this places the macros in the root namespace of the crate,
// so importing them via use crate::vga_buffer::println does not work. Instead, we have to do use crate::println.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}