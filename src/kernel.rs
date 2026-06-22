use crate::drivers::display::*;
use crate::drivers::display::ForegroundColor as FGColor;
use crate::drivers::display::BackgroundColor as BGColor;
use crate::arch::i686::vga;
use crate::time;

pub fn main() -> ! {
    
    let message_1 = "Hey, what's up :D!";
    let message_2 = "Hey, what's up :D! (but it's red)";
    let message_3 = "I'm a ";
    let message_4 = "Rustacean";
    let message_5 = ", what's up?";

    unsafe {
        vga::arch_i686_enable_cursor(14, 15);

        let mut local_buffer = VGABuffer::new(Some(vga::arch_i686_update_cursor));
        let vga_ref = 
            &mut *(vga::VGA_BUFFER_ADR as *mut [[u16; BUFFER_WIDTH]; BUFFER_HEIGHT]);
        
        println!(local_buffer, vga_ref, message_1);
        println!(local_buffer, vga_ref, message_2, FGColor::Red);
        print!(local_buffer, vga_ref, message_3, FGColor::Magenta);
        print!(local_buffer, vga_ref, message_4, FGColor::Yellow);
        println!(local_buffer, vga_ref, message_5, FGColor::Magenta);

        time::delay_seconds(2);
        local_buffer.clear_screen(vga_ref);
    }

    loop {}
}
