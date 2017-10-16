#![feature(used)]
#![no_std]

extern crate cortex_m_rt;
extern crate cortex_m;
use cortex_m::asm;

extern crate stm32f0_hal;
use stm32f0_hal::gpio;

const BTN_PIN: gpio::Pin = gpio::Pin::P6;
const LED_PIN: gpio::Pin = gpio::Pin::P9;

fn main() {
    gpio::init(&BTN_PIN, &gpio::Mode::Input);
    gpio::init(&LED_PIN, &gpio::Mode::Output);

    loop {
        gpio::write(&LED_PIN, gpio::read(&BTN_PIN));
    }
}

// As we are not using interrupts, we just register a dummy catch all handler
#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
