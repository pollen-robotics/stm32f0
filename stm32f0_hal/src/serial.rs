use core::marker::PhantomData;
use core::ptr;

use cortex_m;

use stm32f0x2::{Interrupt, USART1, USART3};
use nb::{Error, Result};

use gpio::gpioa::{PA10, PA9};
use gpio::gpiob::{PB10, PB11};
use gpio::{AF1, AF4, Alternate, PushPull};
use hal;
use hal::serial::Write;
use rcc::{APB1, APB2, Clocks};

use time::Bps;

pub enum Event {
    Rxe,
}

pub trait Pins<USART> {}
impl Pins<USART1>
    for (
        PA9<Alternate<PushPull, AF1>>,
        PA10<Alternate<PushPull, AF1>>,
    ) {
}

impl Pins<USART3>
    for (
        PB10<Alternate<PushPull, AF4>>,
        PB11<Alternate<PushPull, AF4>>,
    ) {
}

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


                    Serial { _usart: usart, _pins: pins }
                }
                pub fn split(self) -> (Tx<$USARTX>, Rx<$USARTX>) {
                    (
                        Tx { _usart: PhantomData },
                        Rx { _usart: PhantomData },
                    )
                }
            }
            impl Tx<$USARTX> {
                pub fn write_str(&mut self, s: &str)  -> Result<(), !> {
                    for &b in s.as_bytes() {
                        block!(self.write(b)).ok();
                    }
                    Ok(())
                }
            }
        impl hal::serial::Write<u8> for Tx<$USARTX> {
            type Error = !;
            fn write(&mut self, byte: u8) -> Result<(), !> {
                let uart = unsafe { &(*$USARTX::ptr()) };

                if uart.isr.read().tc().bit_is_set() {
                    uart.icr.write(|w| w.tccf().clear_bit());
                    unsafe {
                        ptr::write_volatile(&uart.tdr as *const _ as *mut _, u32::from(byte))
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
        impl Rx<$USARTX> {
            fn _read(&mut self) -> Result<u8, !> {
                let uart = unsafe { &(*$USARTX::ptr()) };

                if uart.isr.read().rxne().bit_is_set() {
                    Ok(unsafe { ptr::read_volatile(&uart.rdr.read().bits() as *const u32) as u8 })
                } else {
                    Err(Error::WouldBlock)
                }
            }
        }
        impl hal::serial::Read<u8> for Rx<$USARTX> {
            type Error = !;
            fn read(&mut self) -> Result<u8, Self::Error> {
                self._read()
            }
        }
        impl hal::serial::AsyncRead<u8> for Rx<$USARTX> {
            type Error = !;

            fn async_read(&mut self) -> u8 {
                self._read().unwrap()
            }

            fn listen(&mut self) {
                let uart = unsafe { &(*$USARTX::ptr()) };
                uart.cr1.modify(|_, w| { w.rxneie().enabled() });

                // TODO: That's a really really unsafe way of accessing NVIC...
                // We probably should expose a NVIC trait as a parameter instead.
                // As we don't want to have any cortex_m object in the trait signature.
                let mut nvic = unsafe { cortex_m::Peripherals::steal().NVIC };
                nvic.enable(Interrupt::$usart_inter);
                nvic.clear_pending(Interrupt::$usart_inter);
            }
        }
        )+
    }
}

usart!(
    USART1: (usart1, APB2, usart1en, USART1),
    USART3: (usart3, APB1, usart3en, USART3_4),
);
