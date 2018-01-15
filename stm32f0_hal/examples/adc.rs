#![feature(used)]
#![no_std]

extern crate stm32f0_hal as hal;

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;

use cortex_m_semihosting::hio;
use core::fmt::Write;
use cortex_m::asm;

use hal::adc;

fn main() {
    let mut stdout = hio::hstdout().unwrap();
    let channel0 = adc::Analog::setup(adc::Channel::ADC0);
    let channel1 = adc::Analog::setup(adc::Channel::ADC1);
    loop {
        writeln!(stdout, "{} {}", channel0.read(), channel1.read());
    }
}


// As we are not using interrupts, we just register a dummy catch all handler
#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 128] = [default_handler; 128];

extern "C" fn default_handler() {
    asm::bkpt();
}
