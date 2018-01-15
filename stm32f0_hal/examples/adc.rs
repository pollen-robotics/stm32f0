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
    let Channel0 = adc::Analog::setup(adc::Channel::ADC0);
    let Channel1 = adc::Analog::setup(adc::Channel::ADC1);
    //let p3 = adc::Input::setup(adc::Pin::P3);
    //let p4 = adc::Input::setup(adc::Pin::P4);

    loop {
        writeln!(stdout, "{} {}", Channel0.read(), Channel1.read());
    }
}


// As we are not using interrupts, we just register a dummy catch all handler
#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 128] = [default_handler; 128];

extern "C" fn default_handler() {
    asm::bkpt();
}
