#![no_std]

extern crate cortex_m;
extern crate stm32f0_hal as hal;

use hal::{rcc, uart};

fn main() {
    rcc::init();

    unsafe {
        hal::panic::log_on_serial(uart::Uarts::Uart4);
    }
    panic!("This will be sent on TX before abort!");
}
