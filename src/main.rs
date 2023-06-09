
#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(xg_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use xg_os::println;

//#![asm]


#[no_mangle]
pub extern "C" fn _start() -> ! {
	println!("initing....");
	xg_os::init();
    //println!("Hello xg_os{}", 1/0);
	x86_64::instructions::interrupts::int3();
	println!("Hello xg_os{}", "!");
    #[cfg(test)]
    test_main();

    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    xg_os::test_panic_handler(info)
}




