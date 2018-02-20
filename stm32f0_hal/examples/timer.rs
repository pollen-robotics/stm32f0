#![no_std]

extern crate cortex_m;

extern crate stm32f0_hal as hal;
use hal::stm32f0x2;
use hal::timer::Timer;
use hal::prelude::*;

extern crate embedded_hal;
use embedded_hal::prelude::*;

#[macro_use(block)]
extern crate nb;

fn main() {
    let p = stm32f0x2::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();

    let mut rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut timer = Timer::tim6(p.TIM6, clocks, &mut rcc.apb1);

    let mut gpioc = p.GPIOC.split(&mut rcc.ahb);
    let mut led = gpioc
        .pc7
        .into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper);

    let mut gpioa = p.GPIOA.split(&mut rcc.ahb);
    let button = gpioa
        .pa0
        .into_floating_input(&mut gpioa.moder, &mut gpioa.pupdr);

    loop {
        if button.is_high() {
            led.set_high();

            timer.start(1_u32.hz());
            block!(timer.wait()).ok();

            led.set_low();
        }
    }
}
