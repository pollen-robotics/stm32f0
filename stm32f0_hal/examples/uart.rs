#![no_std]

extern crate stm32f0_hal as hal;

use hal::{rcc, uart};

fn main() {
    rcc::init(); // Full Speed 48Mhz

    uart::setup(
        6000000,
        uart::NBits::_8bits,
        uart::StopBits::_1b,
        uart::Parity::None,
        |byte| uart::send(byte),
    );

    if uart::transmit_complete() {
        uart::send(0x55);
    }

    loop {}
}
