#![no_std]
#![feature(alloc)]

extern crate cortex_m;
extern crate stm32f0_hal as hal;

use hal::gpio::{InputPin, GpioExt};
use hal::rcc::RccExt;

extern crate alloc;
use alloc::Vec;

fn main() {
    hal::allocator::setup(5000);

    let mut v = Vec::new();

    let p = hal::stm32f0x2::Peripherals::take().unwrap();
    let mut rcc = p.RCC.constrain();
    let mut gpioa = p.GPIOA.split(&mut rcc.ahb);
    let button = gpioa
        .pa0
        .into_floating_input(&mut gpioa.moder, &mut gpioa.pupdr);

    loop {
        // Warning: you should add a breakpoint here
        // to check that it works correctly
        // Otherwise, you may explode the heap pretty quickly :-)
        if button.is_high() {
            let l = v.len();
            v.push(l);
        }
    }
}
