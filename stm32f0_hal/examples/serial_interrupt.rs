#![no_std]

extern crate stm32f0_hal as hal;
use hal::prelude::*;

use hal::stm32f0x2::USART1;

extern crate embedded_hal;
use embedded_hal::prelude::*;

extern crate cortex_m;

#[macro_use(block)]
extern crate nb;

#[macro_use]
extern crate stm32f0x2;

#[macro_use(as_static)]
extern crate as_static;

as_static!(RX: hal::serial::Rx<USART1>);
as_static!(TX: hal::serial::Tx<USART1>);

fn main() {
    let p = hal::stm32f0x2::Peripherals::take().unwrap();
    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let cp = cortex_m::Peripherals::take().unwrap();
    let mut nvic = cp.NVIC;

    let mut gpioa = p.GPIOA.split(&mut rcc.ahb);
    let tx = gpioa
        .pa9
        .into_alternate_push_pull(&mut gpioa.moder, &mut gpioa.afr, hal::gpio::AF1);
    let rx = gpioa
        .pa10
        .into_alternate_push_pull(&mut gpioa.moder, &mut gpioa.afr, hal::gpio::AF1);
    let serial =
        hal::serial::Serial::usart1(p.USART1, (tx, rx), 57_600_u32.bps(), clocks, &mut rcc.apb2);

    let (tx, mut rx) = serial.split();
    rx.listen_interrupt(&mut nvic);

    unsafe {
        RX.lazy_init(rx);
        TX.lazy_init(tx);
    }
    unsafe {
        block!(TX.write(b'X')).ok();
    }
    loop {}
}

interrupt!(USART1, serial_reception);
fn serial_reception() {
    unsafe {
        let b = RX.read().unwrap();
        assert_eq!(b, b'X');
        TX.write(b).unwrap();
    }
}
