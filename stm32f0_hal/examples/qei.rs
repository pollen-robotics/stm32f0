#![feature(used)]
#![no_std]
#![feature(global_allocator)]

#[macro_use]
extern crate stm32f0_hal as hal;

extern crate cortex_m;
extern crate cortex_m_rt;

extern crate alloc_cortex_m0;
use alloc_cortex_m0::CortexM0Heap;
#[global_allocator]
static ALLOCATOR: CortexM0Heap = CortexM0Heap::empty();
const STACK_SIZE: usize = 5000;

// These symbols come from a linker script
extern "C" {
    static mut _sheap: u32;
}

use core::fmt::Write;

use hal::{rcc,qei};

fn main() {
    let heap_start = unsafe { &mut _sheap as *mut u32 as usize };
    unsafe { ALLOCATOR.init(heap_start, STACK_SIZE) }
    rcc::init();
    qei::setup_debug(57600);
    qei::init();
    qei::dt_setup(10000);
    qei::dt_resume();
    loop {
        log!("Angle Parcouru : {}° - Speed : {}°/s",qei::counter(), qei::get_speed()*100 as f32);
        rcc::ms_delay(100);
    }
}


