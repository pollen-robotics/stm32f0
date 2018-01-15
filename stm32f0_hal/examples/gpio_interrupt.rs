#![no_std]

extern crate stm32f0_hal as hal;
extern crate cortex_m;
#[macro_use(interrupt)]
extern crate stm32f0x2;

use stm32f0x2::EXTI;
use hal::{gpio, rcc};

static mut P7: Option<gpio::Output> = None;

fn main() {
    let p0 = gpio::Input::setup(gpio::Pin::PA0);
    unsafe {
        P7 = Some(gpio::Output::setup(gpio::Pin::PC7));
    }

    rcc::init(); // Full Speed 48Mhz
    p0.init_interrupt();
    loop {

    }
}

interrupt!(EXTI0_1,pa0_cb);

fn pa0_cb() {
    cortex_m::interrupt::free(|cs| {
        let exti = EXTI.borrow(cs);
        exti.pr.write(|w| w.pif0().set_bit());

        unsafe {
            if let Some(ref mut p7) = P7 {
                p7.high();
            }
        }


    })
}