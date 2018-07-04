#![feature(panic_implementation)] // required for defining the panic handler
#![feature(abi_x86_interrupt)] // required for x86-interrupt calling convention
#![no_std] // don't link the Rust standard library
#![cfg_attr(not(test), no_main)] // disable all Rust-level entry points
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#[macro_use]
extern crate rustos;
extern crate x86_64;
#[macro_use]
extern crate lazy_static;

use core::panic::PanicInfo;

/// This function is the entry point, since the linker looks for a function
/// named `_start` by default.
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    rustos::gdt::init();
    init_idt();

    fn stack_overflow() {
        stack_overflow();
    }

    stack_overflow();
    
    println!("It did not crash");

    loop {}
}

#[cfg(not(test))]
#[panic_implementation]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

use x86_64::structures::idt::{InterruptDescriptorTable, ExceptionStackFrame};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(rustos::gdt::DOUBLE_FAULT_IST_INDEX);
        }
        
        idt
    };
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: &mut ExceptionStackFrame, _error_code: u64) {
    println!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    loop {}
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

pub fn init_idt() {
    IDT.load();
}

