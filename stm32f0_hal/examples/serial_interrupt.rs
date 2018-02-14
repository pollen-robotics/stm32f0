#![no_std]

extern crate stm32f0_hal as hal;
use hal::prelude::*;
use hal::serial::{Serial, Tx, Rx};


extern crate cortex_m;
extern crate embedded_hal;
#[macro_use(block)]
extern crate nb;

use embedded_hal::prelude::*;

#[macro_use(interrupt)]
extern crate stm32f0x2;
use stm32f0x2::USART1;

pub static mut TX: Option<Tx<USART1>> = None;
pub static mut RX: Option<Rx<USART1>> = None;

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

    let serial = Serial::usart1(
        p.USART1,
        (tx, rx),
        57_600_u32.bps(),
        // 3_000_000_u32.bps(),
        clocks,
        &mut rcc.apb2,
    );

    unsafe {
        let (tx, rx) = serial.split();
        TX = Some(tx);
        RX = Some(rx);
    }

    let cp = cortex_m::Peripherals::take().unwrap();
    let mut nvic = cp.NVIC;
    RX.unwrap().interrupt_enable(&mut nvic);

    let sent = b'U';
    block!(TX.unwrap().write(sent)).ok();

    loop {
        // do nothing
    }
}

interrupt!(USART1, reception);
fn reception() {

    let received = RX.unwrap().read();
    assert_eq!(received, 'U');
    TX.unwrap().write('U' as u8);
}
