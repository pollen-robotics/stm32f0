#![no_std]

extern crate stm32f0_hal as hal;
use hal::adc;

extern crate cortex_m_semihosting;
use cortex_m_semihosting::hio;
use core::fmt::Write;

fn main() {
    let mut stdout = hio::hstdout().unwrap();
    let channel0 = adc::Analog::setup(adc::Pin::PA0);
    let channel1 = adc::Analog::setup(adc::Pin::PA1);
    loop {
        writeln!(stdout, "{} {}", channel0.read(), channel1.read()).unwrap();
    }
}
