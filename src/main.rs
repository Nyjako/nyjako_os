#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(nyjako_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use nyjako_os::println;

#[cfg(not(test))] // Normal mode panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    nyjako_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, from {} - {}!", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("The numbers are {}, {} and {}", 69, 420, 1.0/3.0);

    #[cfg(test)]
    test_main();

    loop { }
}
