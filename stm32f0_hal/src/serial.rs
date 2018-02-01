use core::marker::PhantomData;
use core::ptr;

// use stm32f0x2::{Interrupt, USART1};
use stm32f0x2::USART1;
// use cortex_m::peripheral::NVIC;
use nb::{Error, Result};

use gpio::gpioa::{PA10, PA9};
use gpio::{Alternate, PushPull};
use hal;
use rcc::{APB2, Clocks};

use time::Bps;

pub trait Pins<USART> {}
// TODO: toutes les configs
impl Pins<USART1> for (PA9<Alternate<PushPull>>, PA10<Alternate<PushPull>>) {}

pub struct Serial<PINS> {
    _pins: PINS,
}

// TODO: macro
// USART1 --> Macro
// usart1en --> Macro
// Interrupt::USART1 --> Macro

impl<PINS> Serial<PINS> {
    pub fn usart1(
        usart: USART1,
        pins: PINS,
        baud_rate: Bps,
        clocks: Clocks,
        apb: &mut APB2,
        // nvic: &mut NVIC,
    ) -> Self
    where
        PINS: Pins<USART1>,
    {
        apb.enr().modify(|_, w| w.usart1en().enabled());
        // TODO: configure nbits, parity, stop bit
        usart.cr1.modify(|_, w| {
            w.te().enabled().re().enabled().over8().over8()
            // Si Interrupt
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
        usart
            .cr2
            .modify(|_, w| w.linen().disabled().clken().disabled());
        usart.cr1.modify(|_, w| w.ue().enabled());

        // Si interrupt
        // nvic.enable(Interrupt::USART1);
        // nvic.clear_pending(Interrupt::USART1);

        Serial { _pins: pins }
    }
    pub fn split(self) -> (Tx<USART1>, Rx<USART1>) {
        (
            Tx {
                _usart: PhantomData,
            },
            Rx {
                _usart: PhantomData,
            },
        )
    }
}

/// Serial receiver
pub struct Rx<USART> {
    _usart: PhantomData<USART>,
}

/// Serial transmitter
pub struct Tx<USART> {
    _usart: PhantomData<USART>,
}
impl hal::serial::Write<u8> for Tx<USART1> {
    type Error = !;

    fn write(&mut self, byte: u8) -> Result<(), !> {
        let uart = unsafe { &(*USART1::ptr()) };

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
impl hal::serial::Read<u8> for Rx<USART1> {
    type Error = !;

    fn read(&mut self) -> Result<u8, Self::Error> {
        let uart = unsafe { &(*USART1::ptr()) };

        if uart.isr.read().rxne().bit_is_set() {
            Ok(unsafe { ptr::read_volatile(&uart.rdr.read().bits() as *const u32) as u8 })
        } else {
            Err(Error::WouldBlock)
        }
    }
}
