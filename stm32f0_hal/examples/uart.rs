#![feature(used)]
#![no_std]
extern crate cortex_m;
extern crate stm32f0_hal as hal;

use hal::rcc;
use hal::uart;

fn main() {
    rcc::init(); // Full Speed 48Mhz
    uart::init(
        6000000,
        uart::NBits::_8bits,
        uart::StopBits::_1b,
        uart::Parity::None,
    );
    loop {
        if uart::transmit_complete() {
            uart::send(0x55);
        }
    }
}
