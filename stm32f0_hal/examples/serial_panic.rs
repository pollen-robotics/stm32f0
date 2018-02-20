#![no_std]

extern crate stm32f0_hal as hal;
use hal::prelude::*;

use hal::serial::Tx;
use hal::stm32f0x2::USART3;

fn main() {
    let p = hal::stm32f0x2::Peripherals::take().unwrap();
    let mut flash = p.FLASH.constrain();

    let mut rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpiob = p.GPIOB.split(&mut rcc.ahb);

    let pb10 =
        gpiob
            .pb10
            .into_alternate_push_pull(&mut gpiob.moder, &mut gpiob.afr, hal::gpio::AF4);
    let pb11 =
        gpiob
            .pb11
            .into_alternate_push_pull(&mut gpiob.moder, &mut gpiob.afr, hal::gpio::AF4);

    let serial = hal::serial::Serial::usart3(
        p.USART3,
        (pb10, pb11),
        57_600_u32.bps(),
        clocks,
        &mut rcc.apb1,
    );
    let (mut tx, _) = serial.split();

    unsafe {
        hal::panic::log_on_serial(core::mem::transmute::<
            &mut Tx<USART3>,
            &'static mut Tx<USART3>,
        >(&mut tx));
    }
    tx.write_str("Log ready!\n").ok();
    panic!("This will be sent on TX before abort!");
}
