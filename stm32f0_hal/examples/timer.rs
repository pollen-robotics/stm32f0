#![feature(used)]
#![no_std]
extern crate cortex_m;
extern crate stm32f0_hal as hal;

use hal::rcc;
use hal::timer;


fn main() {
    rcc::init(); // Full Speed 48Mhz
    timer::setup(1000);
    timer::resume();
    loop {}
}

