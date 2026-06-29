use crate::arch::i686::isr;
use core::sync::atomic::{AtomicU32, Ordering};

const SYSTEM_FREQ: usize = 1000; // in Hz
const DIVISOR: u16 = { 
    let div = (isr::PIT_FREQ + (SYSTEM_FREQ / 2)) / SYSTEM_FREQ;
    if (div >= 65536) { 0 } else { div as u16 }
};

pub static SYSTEM_TICKS: AtomicU32 = AtomicU32::new(0);

pub struct SysTime;

impl SysTime {
    
    pub fn tick() {
        SYSTEM_TICKS.fetch_add(1, Ordering::Relaxed);
    }

    pub fn get_ticks() -> u32 {
        SYSTEM_TICKS.load(Ordering::Relaxed)
    }

    pub fn on_tick<F>(last_tick: &mut u32, mut f: F) 
    where
        F: FnMut(),
    {
        let cur_tick = SysTime::get_ticks();
        if cur_tick != *last_tick {
            *last_tick = cur_tick;
            f()
        }
    }

}