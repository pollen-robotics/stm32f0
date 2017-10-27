#![feature(used)]
#![no_std]

extern crate stm32f0_hal as hal;

extern crate cortex_m_semihosting;
extern crate cortex_m_rt;
extern crate cortex_m;

use cortex_m_semihosting::hio;
use core::fmt::Write;
use cortex_m::asm;

use hal::adc;

fn main() {
    let mut stdout = hio::hstdout().unwrap();

    let p4 = adc::Input::setup(adc::Pin::P4);
    let p5 = adc::Input::setup(adc::Pin::P5);

    loop {
        writeln!(stdout, "{} {}", p4.read(), p5.read());
    }
}


// As we are not using interrupts, we just register a dummy catch all handler
#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
