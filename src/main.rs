#![feature(panic_implementation)]
#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#[cfg(test)]
extern crate std;
#[cfg(test)]
extern crate array_init;

extern crate volatile;
#[macro_use]
extern crate lazy_static;
extern crate spin;

#[macro_use]
mod vga_buffer;
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_implementation]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(not(test))]
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {

    print!("Hello again!");
    println!(" some numbers: {} {}", 42, 1.337);
    panic!("Some panic message");
    loop {}
}
