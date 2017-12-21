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
    qei::setup_debug(115200);
    qei::init_qei1();
    qei::init_qei2();
    qei::setup_pwm1(10);       // Set PWM frequency = 100 Khz
    qei::setup_pwm2(10);       // Set PWM frequency = 100 Khz
    qei::set_motor1(350,true);  //
    qei::set_motor2(350,true);
    qei::pwm_enable1();
    qei::pwm_enable2();
    qei::dt_setup(1000);        // Speed per second - interval = 1 sec
    qei::dt_resume();
    loop {
        log!("Moteur 1 : Angle Parcouru : {}° - Speed : {}°/s Moteur 2 : Angle Parcouru : {}° - Speed : {}°/s ",qei::counter_motor1(), qei::get_speed_motor1() as f32, qei::counter_motor2(), qei::get_speed_motor2() as f32);
        rcc::ms_delay(1000);
    }
}


