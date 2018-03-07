#![no_std]

extern crate cortex_m;
extern crate stm32f0_hal as hal;

use hal::{rcc, uart};

fn main() {
    rcc::init();

    unsafe {
        hal::debug::panic_on_serial(uart::Uarts::Uart4);
    }
    unsafe {
        hal::debug::trace("This will be sent on TX.");
    }
    panic!("Then, this will also be sent on TX before abort!");
}
