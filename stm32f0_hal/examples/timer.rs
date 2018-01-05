#![no_std]

extern crate stm32f0_hal as hal;

use hal::{rcc, timer};

fn main() {
    rcc::init(); // Full Speed 48Mhz
    timer::setup(1000);
    timer::resume();

    loop {}
}
