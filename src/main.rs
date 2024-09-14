#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(nyjako_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use nyjako_os::println;

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use nyjako_os::memory;
    use x86_64::{structures::paging::Translate, VirtAddr};


    println!("Hello, from {} - {}!", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    nyjako_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe { memory::init(phys_mem_offset) };
    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }

    #[cfg(test)]
    test_main();

    println!("Yay, no crash so far!");
    nyjako_os::hlt_loop();
}

#[cfg(not(test))] // Normal mode panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    nyjako_os::hlt_loop();
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