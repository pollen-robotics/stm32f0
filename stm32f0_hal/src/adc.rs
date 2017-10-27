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
// ! let p = adc::Input::setup(adc::Pin::P4);
// ! let b = p.read();
//! ```

use cortex_m;
use stm32f0x2::{GPIOA, RCC, ADC};

/// ADC Pin available on PORT A
pub enum Pin {
    P4,
    P5,
}

/// Input Mode Pin
pub struct Input {
    pin: Pin,
}
impl Input {
    /// Setup a PIN in Input Mode
    pub fn setup(pin: Pin) -> Input {
        setup_pin(&pin);
        Input { pin }
    }
    /// Read the analog value of a PIN
    pub fn read(&self) -> u16 {
        cortex_m::interrupt::free(|cs| {
            let adc = ADC.borrow(cs);

            // ADC Channel selection
            match self.pin {
                Pin::P4 => adc.chselr.write(|w| w.chsel4().set_bit()),
                Pin::P5 => adc.chselr.write(|w| w.chsel5().set_bit()),
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
        let adc = ADC.borrow(cs);

        // Clock Activation PORTA
        rcc.ahbenr.modify(|_, w| w.iopaen().enabled());
        // Clock activation ADC
        rcc.apb2enr.modify(|_, w| w.adcen().enabled());

        // PA Analog Input Channel
        match *pin {
            Pin::P4 => gpioa.moder.modify(|_, w| w.moder4().analog()),
            Pin::P5 => gpioa.moder.modify(|_, w| w.moder5().analog()),
        }
    });
}
