#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(xg_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
use core::panic::PanicInfo;
use xg_os::println;
use xg_os::serial_println;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    xg_os::test_panic_handler(info)
}
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();
    test1_println();
    loop {}
}

fn test_runner(tests: &[&dyn Fn()]) {
    unimplemented!();
}



#[test_case]
fn test_println() {
    //serial_println!("test_println output");
    //println!("test output");
}


fn test1_println() {
    serial_println!("test_println output");
    println!("test output");
}