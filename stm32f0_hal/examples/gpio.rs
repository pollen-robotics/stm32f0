#![feature(used)]
#![no_std]

extern crate stm32f0_hal as hal;

extern crate cortex_m;
extern crate cortex_m_rt;
//extern crate cortex_m_semihosting;

//use cortex_m_semihosting::hio;
//use core::fmt::Write;
use cortex_m::asm;

use hal::gpio;

fn main() {
    //let mut stdout = hio::hstdout().unwrap();

    let p0 = gpio::Input::setup(gpio::Pin::PA0);
    //let p5 = gpio::Input::setup(gpio::Pin::PA5);
    let mut p7 = gpio::Output::setup(gpio::Pin::PC7);

    loop {
        if p0.read() {
            p7.high();
            //writeln!(stdout, "{} {}", p1.read(), p5.read());
        }else {
            p7.low();
        }
    }
}


// As we are not using interrupts, we just register a dummy catch all handler
#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 128] = [default_handler; 128];

extern "C" fn default_handler() {
    asm::bkpt();
}
