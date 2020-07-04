#![no_std]
#![no_main]

use core::panic::PanicInfo;

// will be called when panic eg. exit, break ...
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {

    }
}

// No mangle disable name mangling to ensure that the Rust compiler really output a function with
// the name _start
static TEXT: &[u8] = b"The first time";
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in TEXT.iter().enumerate() {
        // every ascii char take 1 byte
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {

    }
}
