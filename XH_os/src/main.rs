#![no_std] 
#![no_main] 
#![feature(custom_test_frameworks)]
#![test_runner(XH_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use XH_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}","!");
    XH_os::init();
    // let ptr = 0x2031b2 as *mut u8;
    // unsafe { *ptr = 42; }

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    XH_os::hlt_loop();
}
#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    XH_os::hlt_loop();
}
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    XH_os::test_panic_handler(info)
}
