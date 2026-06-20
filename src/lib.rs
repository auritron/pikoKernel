#![no_std]
#![allow(non_snake_case)]
#![feature(ascii_char)]

mod panichandler;
mod vga;

use vga::*;

const VGA_BUFFER_ADR: *mut u8 = 0xb8000 as *mut u8;

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    let message_1 = "Hey, what's up :D!";
    let message_2 = "\nHey, what's up :D! (but it's red)";
    //let color: u8 = 0x0A;

    unsafe {
        let mut local_buffer = vga::VGABuffer::new();
        local_buffer.clear();
        local_buffer.write_plain_text_to_buf(message_1);
        local_buffer.write_fmt_text_to_buf(message_2, vga::ForegroundColor::Red, BackgroundColor::default(), false);


        let vga_ref = 
            &mut *(VGA_BUFFER_ADR as *mut [[u16; BUFFER_WIDTH]; BUFFER_HEIGHT]);
        local_buffer.flush(vga_ref);
    }

    loop {}
}