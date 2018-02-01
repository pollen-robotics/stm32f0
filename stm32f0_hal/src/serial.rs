use core::marker::PhantomData;
use core::ptr;

// use stm32f0x2::{Interrupt};
use stm32f0x2::{USART1, USART3};
// use cortex_m::peripheral::NVIC;
use nb::{Error, Result};

use gpio::gpioa::{PA10, PA9};
use gpio::{Alternate, PushPull};
use hal;
use rcc::{APB1, APB2, Clocks};

use time::Bps;

pub trait Pins<USART> {}
impl Pins<USART1> for (PA9<Alternate<PushPull>>, PA10<Alternate<PushPull>>) {}

pub struct Serial<USART, PINS> {
    _usart: USART,
    _pins: PINS,
}

/// Serial receiver
pub struct Rx<USART> {
    _usart: PhantomData<USART>,
}

/// Serial transmitter
pub struct Tx<USART> {
    _usart: PhantomData<USART>,
}

macro_rules! usart {
    ($($USARTX:ident: ($usartx:ident, $APBX:ident, $usartxen:ident, $usart_inter:ident), )+) => {
        $(
            impl<PINS> Serial<$USARTX, PINS> {
                pub fn $usartx(
                    usart: $USARTX,
                    pins: PINS,
                    baud_rate: Bps,
                    clocks: Clocks,
                    apb: &mut $APBX,
                    // nvic: &mut NVIC,
                ) -> Self
                where PINS: Pins<$USARTX> {
                    apb.enr().modify(|_, w| w.$usartxen().enabled());
                    // TODO: configure nbits, parity, stop bit
                    usart.cr1.modify(|_, w| {
                        w.te().enabled().re().enabled().over8().over8()
                        // If using Interrupt
                        // .rxneie()
                        // .enabled()
                    });
                    usart.cr3.modify(|_, w| {
                        w.rtse()
                        .disabled()
                        .ctse()
                        .disabled()
                        .ctsie()
                        .disabled()
                        .ovrdis()
                        .disabled()
                    });
                    let freq = clocks.hclk().0;
                    usart.brr.modify(|_, w| {
                        w.div_fraction()
                        .bits((freq / (baud_rate.0 / 2)) as u8 & 0x0F)
                    });
                    usart.brr.modify(|_, w| {
                        w.div_mantissa()
                        .bits((freq / (baud_rate.0 / 2) >> 4) as u16)
                    });
                    usart.cr2.modify(|_, w|
                        w.linen().disabled().clken().disabled()
                    );
                    usart.cr1.modify(|_, w| w.ue().enabled());

                    // // If interrupt
                    // nvic.enable(Interrupt::$usart_inter);
                    // nvic.clear_pending(Interrupt::$usart_inter);

                    Serial { _usart: usart, _pins: pins }
                }
                pub fn split(self) -> (Tx<$USARTX>, Rx<$USARTX>) {
                    (
                        Tx { _usart: PhantomData },
                        Rx { _usart: PhantomData },
                    )
                }
            }
        impl hal::serial::Write<u8> for Tx<$USARTX> {
            type Error = !;
            fn write(&mut self, byte: u8) -> Result<(), !> {
                let uart = unsafe { &(*$USARTX::ptr()) };

                if uart.isr.read().tc().bit_is_set() {
                    uart.icr.write(|w| w.tccf().clear_bit());
                    unsafe {
                        ptr::write_volatile(&uart.tdr as *const _ as *mut _, byte as u32);
                    }

                    Ok(())
                } else {
                    Err(Error::WouldBlock)
                }
            }
            fn flush(&mut self) -> Result<(), !> {
                Ok(())
            }
        }
        impl hal::serial::Read<u8> for Rx<$USARTX> {
            type Error = !;

            fn read(&mut self) -> Result<u8, Self::Error> {
                let uart = unsafe { &(*$USARTX::ptr()) };

                if uart.isr.read().rxne().bit_is_set() {
                    Ok(unsafe { ptr::read_volatile(&uart.rdr.read().bits() as *const u32) as u8 })
                } else {
                    Err(Error::WouldBlock)
                }
            }
        }
        )+
    }
}

usart!(
    USART1: (usart1, APB2, usart1en, USART1),
    USART3: (usart3, APB1, usart3en, USART3_4),
);
