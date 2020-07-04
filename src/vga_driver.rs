const SCREEN_HEIGHT: usize = 25;
const SCREEN_WIDTH: usize = 80;


pub struct VgaWriter {
    col_pos: usize,
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

                let row = 1;
                let col = self.col_pos;


                self.buffer.chars[row][col] = ScreenChar {
                    ascii_char: text_as_byte,
                    color_code: self.color_code,
                };

                self.col_pos += 1;
            }
        }
    }

    fn write_string(&mut self, string: &str) {
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

    fn new_line(&mut self) {}
}

pub fn print_sample() {
    let mut vga_writer = VgaWriter {
        col_pos: 1,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        // buffer point to 0xb8000 address instead array in Buffer
        // ref in Rust point to the address of the first byte of memory
        buffer: unsafe {
            &mut *(0xb8000 as *mut Buffer)
        },
    };

    vga_writer.write_byte(b'T');
    vga_writer.write_string("he second");
    vga_writer.write_string(" Special character: ö")
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
    chars: [[ScreenChar; SCREEN_WIDTH]; SCREEN_HEIGHT]
}

// represent ScreenChar memory layout same in C
// first byte: character
// second byte: how the character is displayed
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