#![no_std]
#![no_main]

// Required by the compiler (memcpy, memset, memcmp)
extern crate rlibc;

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";
const LIGHT_CYAN: u8 = 0xb;
const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *VGA_BUFFER.offset(i as isize * 2) = byte;
            *VGA_BUFFER.offset(i as isize * 2 + 1) = LIGHT_CYAN;
        }
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
