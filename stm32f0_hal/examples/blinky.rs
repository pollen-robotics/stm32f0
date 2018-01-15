#![no_std]

extern crate stm32f0_hal;
use stm32f0_hal::gpio;

fn main() {
    let button = gpio::Input::setup(gpio::Pin::PA1);
    let mut led = gpio::Output::setup(gpio::Pin::PA5);

    loop {
        if button.read() {
            led.high();
        } else {
            led.low();
        }
    }
}
