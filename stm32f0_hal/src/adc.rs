use gpio::{Analog, Input};
use rcc::APB2;

use stm32f0x2::ADC;
use nb::{Error, Result};

pub trait Adc {
    fn start(&self, &mut APB2);
    fn read(&self) -> Result<u16, !>;
}

macro_rules! adc {
    ($gpiox:ident:
        [$($PXi:ident : $chselx:ident)+]
    ) => {
        $(
            impl Adc for super::gpio::$gpiox::$PXi<Input<Analog>> {
                fn start(&self, apb: &mut APB2) {
                    apb.enr().modify(|_, w| w.adcen().enabled());

                    let adc = unsafe { &(*ADC::ptr()) };
                    adc.chselr.write(|w| w.$chselx().set_bit());
                    adc.cr.write(|w| w.aden().set_bit().adstart().set_bit());
                }
                fn read(&self) -> Result<u16, !> {
                    let adc = unsafe { &(*ADC::ptr()) };

                    if adc.isr.read().eoc().bit_is_set() {
                        Ok(adc.dr.read().data().bits())
                    } else {
                        Err(Error::WouldBlock)
                    }
                }
            }
        )+
    }
}

adc!(gpioa: [PA0:chsel0 PA1:chsel1]);
adc!(gpioc: [PC3:chsel13]);
