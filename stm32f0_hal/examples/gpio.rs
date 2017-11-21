#![feature(used)]
#![no_std]

extern crate stm32f0_hal as hal;

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;

use cortex_m_semihosting::hio;
use core::fmt::Write;
use cortex_m::asm;

use hal::gpio;

fn main() {
    let mut stdout = hio::hstdout().unwrap();

    let p1 = gpio::Input::setup(gpio::Pin::PA1);
    let p5 = gpio::Input::setup(gpio::Pin::PA5);

    loop {
        writeln!(stdout, "{} {}", p1.read(), p5.read());
    }
}


// As we are not using interrupts, we just register a dummy catch all handler
#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 128] = [default_handler; 128];

extern "C" fn default_handler() {
    asm::bkpt();
}
