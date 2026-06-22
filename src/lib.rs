#![no_std]
#![allow(non_snake_case)]
#![feature(ascii_char)]
#![feature(ascii_char_variants)]

pub mod kernel;
pub mod panichandler;
pub mod arch;
pub mod drivers;
pub mod time;

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    kernel::main()
}
