use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;
//static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();
use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(xg_breakponit_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

#[test_case]
fn test_breakpoint_exception() {
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
}

extern "x86-interrupt" fn xg_breakponit_handler(stack_frame: InterruptStackFrame){
    println!("EXCEPTION : BREAKPOINT\n{:#?}", stack_frame);

}

