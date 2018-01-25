#![no_std]

extern crate stm32f0_hal;
use stm32f0_hal::gpio;
use stm32f0_hal::rcc;

fn main() {
    let persist = 250;
    let mut orange = gpio::Output::setup(gpio::Pin::PC8);
    let mut green = gpio::Output::setup(gpio::Pin::PC9);
    let mut red = gpio::Output::setup(gpio::Pin::PC6);
    let mut blue = gpio::Output::setup(gpio::Pin::PC7);
    rcc::init();

    loop {
        green.low();
        red.high();
        orange.high();
        rcc::ms_delay(persist);
        red.low();
        blue.high();
        rcc::ms_delay(persist);
        orange.low();
        green.high();
        rcc::ms_delay(persist);
        blue.low();
        red.high();
        rcc::ms_delay(persist);
    }
}
