#![no_std]

extern crate stm32f0_hal as hal;
use hal::prelude::*;

extern crate cortex_m_semihosting;
use cortex_m_semihosting::hio;
use core::fmt::Write;

#[macro_use(block)]
extern crate nb;

fn main() {
    let mut stdout = hio::hstdout().unwrap();

    let p = hal::stm32f0x2::Peripherals::take().unwrap();
    let mut rcc = p.RCC.constrain();

    let mut gpioa = p.GPIOA.split(&mut rcc.ahb);
    let pa0 = gpioa.pa0.into_analog(&mut gpioa.moder);

    let mut gpioc = p.GPIOC.split(&mut rcc.ahb);
    let pc3 = gpioc.pc3.into_analog(&mut gpioc.moder);

    loop {
        pa0.start(&mut rcc.apb2);
        let v0 = block!(pa0.read()).unwrap();

        pc3.start(&mut rcc.apb2);
        let v3 = block!(pc3.read()).unwrap();

        writeln!(stdout, "{} {}", v0, v3).unwrap();
    }
}
