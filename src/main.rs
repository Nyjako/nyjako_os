#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello, world!").unwrap();
    write!(vga_buffer::WRITER.lock(), "   The numbers are {}, {} and {}", 69, 420, 1.0/3.0).unwrap();

    loop {}
}
