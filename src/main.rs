#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod vga_buffer;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, world{}", '!');
    println!("The numbers are {}, {} and {}", 69, 420, 1.0/3.0);

    panic!("Oh no. Code panicked!");

    loop {}
}
