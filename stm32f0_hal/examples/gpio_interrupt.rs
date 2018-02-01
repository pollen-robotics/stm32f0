#![no_std]

extern crate cortex_m;
extern crate stm32f0_hal as hal;
use hal::{gpio, rcc};

#[macro_use(interrupt)]
extern crate stm32f0x2;
use stm32f0x2::{EXTI, Interrupt};

use hal::gpio::GpioExt;
use hal::rcc::RccExt;

fn main() {
    let p = stm32f0x2::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut rcc = p.RCC.constrain();

    let mut gpioa = p.GPIOA.split(&mut rcc.ahb);
    let mut gpioc = p.GPIOC.split(&mut rcc.ahb);

    let mut led = gpioc
        .pc7
        .into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper);

    let button = gpioa
        .pa0
        .into_floating_input(&mut gpioa.moder, &mut gpioa.pupdr);

    // Enable interrupt on EXTI0
    let exti = p.EXTI;
    exti.rtsr.modify(|_, w| w.tr0().set_bit()); // Rising
    exti.ftsr.modify(|_, w| w.tr0().set_bit()); // Falling
    exti.imr.modify(|_, w| w.mr0().set_bit());

    let mut nvic = cp.NVIC;
    nvic.enable(Interrupt::EXTI0_1);

    loop {}
}

interrupt!(EXTI0_1, button_pushed);

fn button_pushed() {
    let exti = unsafe { &*EXTI::ptr() };
    exti.pr.write(|w| w.pif0().set_bit());
}
