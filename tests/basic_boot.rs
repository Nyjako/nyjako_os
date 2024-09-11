#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(nyjako_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    nyjako_os::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

use nyjako_os::println;

#[test_case]
fn test_println() {
    println!("test_println output");
}