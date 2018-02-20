#![no_std]
#![deny(unsafe_code)]

extern crate cortex_m;

extern crate embedded_hal;
use embedded_hal::prelude::*;

extern crate stm32f0_hal as hal;
use hal::stm32f0x2;
use hal::prelude::*;

fn main() {
    let p = stm32f0x2::Peripherals::take().unwrap();

    let mut rcc = p.RCC.constrain();

    let mut gpioa = p.GPIOA.split(&mut rcc.ahb);
    let mut gpioc = p.GPIOC.split(&mut rcc.ahb);

    let mut led = gpioc
        .pc7
        .into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper);

    let button = gpioa
        .pa0
        .into_floating_input(&mut gpioa.moder, &mut gpioa.pupdr);

    loop {
        if button.is_high() {
            led.set_high();
        } else {
            led.set_low();
        }
    }
}
