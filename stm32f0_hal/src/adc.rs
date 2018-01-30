//! # ADC module
//!
//! This module provides access to the ADC of the stm32f0 board.
//! You can:
//!
//! * configure a PIN in Input
//! * read the analog value from the PIN
//!
//! ## Examples
//!
//! ```
//! extern crate stm32f0_hal;
//!
//! use stm32f0_hal::adc;
//!
//! let p = adc::Input::setup(adc::Pin::P4);
//! let b = p.read();
//! ```

use cortex_m;
use stm32f0x2::{ADC, GPIOA, GPIOC, RCC};

/// ADC Pin
pub enum Pin {
    PA0,
    PA1,
    PC13,
    PC14,
}

/// Input Mode Pin
pub struct Analog {
    pin: Pin,
}
impl Analog {
    /// Setup a PIN in Input Mode
    pub fn setup(pin: Pin) -> Analog {
        setup_pin(&pin);
        Analog { pin }
    }
    /// Read the analog value of a PIN
    pub fn read(&self) -> u16 {
        cortex_m::interrupt::free(|cs| {
            let adc = ADC.borrow(cs);

            // ADC Pin selection
            match self.pin {
                Pin::PA0 => adc.chselr.write(|w| w.chsel0().set_bit()),
                Pin::PA1 => adc.chselr.write(|w| w.chsel1().set_bit()),
                Pin::PC13 => adc.chselr.write(|w| w.chsel13().set_bit()),
                Pin::PC14 => adc.chselr.write(|w| w.chsel14().set_bit()),
            }

            // Active ADC and Start Conversion
            adc.cr.write(|w| w.aden().set_bit().adstart().set_bit());

            // Wait end of conversation
            while adc.isr.read().eoc().bit_is_clear() {}

            // Return result
            adc.dr.read().data().bits()
        })
    }
}

fn setup_pin(pin: &Pin) {
    cortex_m::interrupt::free(|cs| {
        let rcc = RCC.borrow(cs);
        let gpioa = GPIOA.borrow(cs);
        let gpioc = GPIOC.borrow(cs);
        // Clock Activation
        match *pin {
            Pin::PA0 => rcc.ahbenr.modify(|_, w| w.iopaen().enabled()),
            Pin::PA1 => rcc.ahbenr.modify(|_, w| w.iopaen().enabled()),
            Pin::PC13 => rcc.ahbenr.modify(|_, w| w.iopcen().enabled()),
            Pin::PC14 => rcc.ahbenr.modify(|_, w| w.iopcen().enabled()),
        }

        // Clock activation ADC
        rcc.apb2enr.modify(|_, w| w.adcen().enabled());

        // PA Analog Input Pin
        match *pin {
            Pin::PA0 => gpioa.moder.modify(|_, w| w.moder0().analog()),
            Pin::PA1 => gpioa.moder.modify(|_, w| w.moder1().analog()),
            Pin::PC13 => gpioc.moder.modify(|_, w| w.moder3().analog()),
            Pin::PC14 => gpioc.moder.modify(|_, w| w.moder4().analog()),
        }
    });
}
