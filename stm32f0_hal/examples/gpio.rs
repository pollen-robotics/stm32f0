#![no_std]

extern crate stm32f0_hal as hal;

use hal::gpio;

fn main() {
    let p0 = gpio::Input::setup(gpio::Pin::PB10);
    let mut p7 = gpio::Output::setup(gpio::Pin::PC7);

    loop {
        if p0.read() {
            p7.high();
        } else {
            p7.low();
        }
    }
}
