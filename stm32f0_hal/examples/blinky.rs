#![no_std]
#![deny(unsafe_code)]

extern crate cortex_m;
extern crate embedded_hal;

extern crate stm32f0_hal as hal;
use hal::delay::Delay;
use hal::prelude::*;
use hal::stm32f0x2;

use embedded_hal::prelude::*;

fn main() {
    let p = stm32f0x2::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut rcc = p.RCC.constrain();

    let mut gpioc = p.GPIOC.split(&mut rcc.ahb);

    let mut led = gpioc
        .pc7
        .into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper);

    let mut flash = p.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut delay = Delay::new(cp.SYST, clocks);

    loop {
        led.set_high();
        delay.delay_ms(200_u16);
        led.set_low();
        delay.delay_ms(200_u16);
    }
}
