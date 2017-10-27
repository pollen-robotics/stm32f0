#![feature(used)]
#![no_std]

extern crate stm32f0_hal as hal;

extern crate cortex_m_rt;
extern crate cortex_m;

use cortex_m::asm;

use hal::pwm;

fn main() {
    pwm::init(10000);
    pwm::set_duty(750);
    pwm::enable();

    loop {}
}


// As we are not using interrupts, we just register a dummy catch all handler
#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
