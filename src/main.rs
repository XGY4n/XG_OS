
#![no_main]
#![no_std]
use core::panic::PanicInfo;
mod vag_buf;

//#![asm]

#[panic_handler]
fn panic(_info : &PanicInfo) -> !{
	loop{}
}



static HELLO : &[u8] = b"Hello XG_os!!!!!!";

#[no_mangle]
pub extern "C" fn _start() -> !{
		let vag_buffer = 0xb8000 as *mut u8;
		for(i , &byte) in HELLO.iter().enumerate(){
				unsafe{
						*vag_buffer.offset(i as isize * 2) = byte;
						*vag_buffer.offset(i as isize * 2 + 1) = 0x8;
				}
		}
		vag_buf::print_something();
		loop {} 
}

//fn main() {
//    println!("Hello, world!");
//}