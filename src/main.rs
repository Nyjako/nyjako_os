#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(nyjako_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use nyjako_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, from {} - {}!", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    
    nyjako_os::init();

    #[cfg(test)]
    test_main();

    println!("Yay, no crash so far!");
    loop { 
        use nyjako_os::print; 
        print!("-"); 
    }
}

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