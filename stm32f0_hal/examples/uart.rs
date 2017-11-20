#![feature(used)]
#![no_std]
extern crate stm32f0_hal as hal;
extern crate cortex_m;

use hal::rcc;
use hal::uart;

fn main() {
    rcc::init(); // Full Speed 48Mhz
    uart::init(
        6000000,
        uart::NBITS::_8bits,
        uart::STOPBITS::_1b,
        uart::PARITY::None,
    );
    loop {
        if uart::transmit_complete() {
            uart::send(0x55);
        }
    }
}
