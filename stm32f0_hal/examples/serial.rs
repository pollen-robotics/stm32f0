#![no_std]

extern crate stm32f0_hal as hal;
use hal::prelude::*;
use hal::serial::Serial;

extern crate cortex_m;
extern crate embedded_hal;
#[macro_use(block)]
extern crate nb;

use embedded_hal::prelude::*;

fn main() {
    let p = hal::stm32f0x2::Peripherals::take().unwrap();
    let mut flash = p.FLASH.constrain();

    let mut rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioa = p.GPIOA.split(&mut rcc.ahb);

    let tx = gpioa
        .pa9
        .into_alternate_push_pull(&mut gpioa.moder, &mut gpioa.afr, hal::gpio::AF1);
    let rx = gpioa
        .pa10
        .into_alternate_push_pull(&mut gpioa.moder, &mut gpioa.afr, hal::gpio::AF1);

    // let cp = cortex_m::Peripherals::take().unwrap();
    // let mut nvic = cp.NVIC;

    let serial = Serial::usart1(
        p.USART1,
        (tx, rx),
        57_600_u32.bps(),
        // 3_000_000_u32.bps(),
        clocks,
        &mut rcc.apb2,
        // &mut nvic,
    );
    let (mut tx, mut rx) = serial.split();

    let sent = b'U';

    loop {
        block!(tx.write(sent)).ok();
        block!(tx.complete()).ok();
        let received = block!(rx.read()).unwrap();

        assert_eq!(received, sent);
    }
}
