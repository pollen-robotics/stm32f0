#![no_std]

extern crate stm32f0_hal as hal;
use hal::prelude::*;

fn main() {
    hal::allocator::setup(5000);

    let p = hal::stm32f0x2::Peripherals::take().unwrap();
    let mut flash = p.FLASH.constrain();

    let mut rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpiob = p.GPIOB.split(&mut rcc.ahb);

    let tx = gpiob
        .pb10
        .into_alternate_push_pull(&mut gpiob.moder, &mut gpiob.afr, hal::gpio::AF4);
    let rx = gpiob
        .pb11
        .into_alternate_push_pull(&mut gpiob.moder, &mut gpiob.afr, hal::gpio::AF4);

    let serial =
        hal::serial::Serial::usart3(p.USART3, (tx, rx), 57_600_u32.bps(), clocks, &mut rcc.apb1);
    let (tx, _) = serial.split();

    hal::panic::log_on_serial(tx);
    panic!("This will be sent on TX before abort!");
}
