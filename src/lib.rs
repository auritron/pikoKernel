#![no_std]
#![allow(non_snake_case)]
#![feature(ascii_char)]

mod panichandler;
mod vga;

use vga::*;
use vga::ForegroundColor::*;

const VGA_BUFFER_ADR: *mut u8 = 0xb8000 as *mut u8;

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    let message_1 = "Hey, what's up :D!";
    let message_2 = "\nHey, what's up :D! (but it's red)\n";
    let message_3 = "I'm a ";
    let message_4 = "rust ";
    let message_5 = "femboy, uwu!";
    //let color: u8 = 0x0A;

    unsafe {
        let mut local_buffer = vga::VGABuffer::new();
        local_buffer.clear();
        local_buffer.write_plain_text_to_buf(message_1);
        local_buffer.write_fmt_text_to_buf(message_2, Red, None, None);
        local_buffer.write_fmt_text_to_buf(message_3, Magenta, None, None);
        local_buffer.write_fmt_text_to_buf(message_4, Yellow, None, None);
        local_buffer.write_fmt_text_to_buf(message_5, Magenta, None, None);

        let vga_ref = 
            &mut *(VGA_BUFFER_ADR as *mut [[u16; BUFFER_WIDTH]; BUFFER_HEIGHT]);
        local_buffer.flush(vga_ref);
    }

    loop {}
}