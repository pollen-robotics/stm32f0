#![no_std]

extern crate cortex_m;
extern crate embedded_hal;

extern crate stm32f0_hal;
use stm32f0_hal::stm32f0x2;

use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::OutputPin;

use stm32f0_hal::delay::Delay;
use stm32f0_hal::flash::FlashExt;
use stm32f0_hal::gpio::GpioExt;
use stm32f0_hal::rcc::RccExt;

fn main() {
    let persist: u16 = 250;

    let p = stm32f0x2::Peripherals::take().unwrap();
    let mut rcc = p.RCC.constrain();

    let mut gpioc = p.GPIOC.split(&mut rcc.ahb);

    let mut red = gpioc
        .pc6
        .into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper);
    let mut blue = gpioc
        .pc7
        .into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper);
    let mut orange = gpioc
        .pc8
        .into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper);
    let mut green = gpioc
        .pc9
        .into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper);

    let mut flash = p.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let cp = cortex_m::Peripherals::take().unwrap();
    let mut delay = Delay::new(cp.SYST, clocks);

    loop {
        green.set_low();
        red.set_high();
        orange.set_high();
        delay.delay_ms(persist);
        red.set_low();
        blue.set_high();
        delay.delay_ms(persist);
        orange.set_low();
        green.set_high();
        delay.delay_ms(persist);
        blue.set_low();
        red.set_high();
        delay.delay_ms(persist);
    }
}
