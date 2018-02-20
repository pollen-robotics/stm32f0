#![no_std]

extern crate stm32f0_hal as hal;
use hal::prelude::*;

extern crate embedded_hal;
use embedded_hal::prelude::*;

fn main() {
    let p = hal::stm32f0x2::Peripherals::take().unwrap();

    let mut rcc = p.RCC.constrain();
    let mut flash = p.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioc = p.GPIOC.split(&mut rcc.ahb);
    let pc6 = gpioc
        .pc6
        .into_alternate_push_pull(&mut gpioc.moder, &mut gpioc.afr, hal::gpio::AF0);
    let pc9 = gpioc
        .pc9
        .into_alternate_push_pull(&mut gpioc.moder, &mut gpioc.afr, hal::gpio::AF0);

    let mut pwm = hal::pwm::Pwm::tim3(p.TIM3, (pc6, pc9), 50.hz(), clocks, &mut rcc.apb1);
    let duty = 9 * pwm.get_max_duty() / 10;
    pwm.set_duty(duty);
    pwm.enable();

    loop {}
}
