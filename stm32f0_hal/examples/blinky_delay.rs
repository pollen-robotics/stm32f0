#![no_std]

extern crate stm32f0_hal;
use stm32f0_hal::{gpio, rcc};

fn main() {
    let mut led = gpio::Output::setup(gpio::Pin::PC7);
    rcc::init();

    loop {
        led.high();
        rcc::ms_delay(1000);
        led.low();
        rcc::ms_delay(1000);
    }
}
