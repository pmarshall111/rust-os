#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use rust_os::println;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // this function is the entry point
    // _start is called before main and usually does things like:
    // - setting up the stack
    // - 
    rust_os::init();

    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3(); // new

    // Invoke a page fault exception which will cause a double fault due to there
    // being no page fault handler set in GDT
    // Double faults will stop execution, so let's comment this out.
    // #[cfg(not(test))]
    // unsafe {
    //     let bad_addr_ptr: *mut u8 = 0xdeadbeef as *mut _;
    //     *bad_addr_ptr = 42;
    // }

    println!("hi\n\nWhatz up dawg");

    #[cfg(test)]
    test_main();

    rust_os::hlt_loop();
}


#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rust_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info);
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}