#![no_std]

extern crate stm32f0_hal as hal;
use hal::adc;

extern crate cortex_m_semihosting;
use cortex_m_semihosting::hio;
use core::fmt::Write;

fn main() {
    let mut stdout = hio::hstdout().unwrap();

    let p3 = adc::Input::setup(adc::Pin::P3);
    let p4 = adc::Input::setup(adc::Pin::P4);

    loop {
        writeln!(stdout, "{} {}", p3.read(), p4.read()).unwrap();
    }
}
