#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os_in_rust::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use os_in_rust::println;

// will be called when panic eg. exit, break ...
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {

    }
}

// No mangle disable name mangling to ensure that the Rust compiler really output a function with
// the name _start
/// ```
/// Old:
/// static TEXT: &[u8] = b"The first time";
/// let vga_buffer = 0xb8000 as *mut u8;
///
///     for (i, &byte) in TEXT.iter().enumerate() {
///         //  every ascii char take 1 byte
///         unsafe {
///             *vga_buffer.offset(i as isize * 2) = byte;
///             *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
///         }
///     }
///
/// We should to minimize the use of unsafe as much as possible
/// ```
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello new version {}", 2);

    #[cfg(test)]
    test_main();
    loop {

    }
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os_in_rust::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
