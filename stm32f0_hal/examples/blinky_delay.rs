#![feature(used)]
#![no_std]

extern crate cortex_m_rt;
extern crate cortex_m;
use cortex_m::asm;

extern crate stm32f0_hal;
use stm32f0_hal::gpio;
use stm32f0_hal::rcc;

fn main() {
    let mut led = gpio::Output::setup(gpio::Pin::PA5);
    rcc::init();

    loop {
        led.high();
        rcc::ms_delay(1000);
        led.low();
        rcc::ms_delay(1000);
    }
}

// As we are not using interrupts, we just register a dummy catch all handler
#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 128] = [default_handler; 128];

extern "C" fn default_handler() {
    asm::bkpt();
}
