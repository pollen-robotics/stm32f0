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

/// ADC Channel
pub enum Channel {
    ADC0,
    ADC1,
    ADC13,
    ADC14,
}

/// Input Mode Pin
pub struct Analog {
    pin: Channel,
}
impl Analog {
    /// Setup a PIN in Input Mode
    pub fn setup(pin: Channel) -> Analog {
        setup_pin(&pin);
        Analog { pin }
    }
    /// Read the analog value of a PIN
    pub fn read(&self) -> u16 {
        cortex_m::interrupt::free(|cs| {
            let adc = ADC.borrow(cs);

            // ADC Channel selection
            match self.pin {
                Channel::ADC0 => adc.chselr.write(|w| w.chsel0().set_bit()),
                Channel::ADC1 => adc.chselr.write(|w| w.chsel1().set_bit()),
                Channel::ADC13 => adc.chselr.write(|w| w.chsel13().set_bit()),
                Channel::ADC14 => adc.chselr.write(|w| w.chsel14().set_bit()),
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

fn setup_pin(pin: &Channel) {
    cortex_m::interrupt::free(|cs| {
        let rcc = RCC.borrow(cs);
        let gpioc = GPIOC.borrow(cs);
        let gpioa = GPIOA.borrow(cs);
        // Clock Activation
        match *pin {
            Channel::ADC0 => rcc.ahbenr.modify(|_, w| w.iopaen().enabled()),
            Channel::ADC1 => rcc.ahbenr.modify(|_, w| w.iopaen().enabled()),
            Channel::ADC13 => rcc.ahbenr.modify(|_, w| w.iopcen().enabled()),
            Channel::ADC14 => rcc.ahbenr.modify(|_, w| w.iopcen().enabled()),
        }

        // Clock activation ADC
        rcc.apb2enr.modify(|_, w| w.adcen().enabled());

        // PA Analog Input Channel
        match *pin {
            Channel::ADC0 => gpioa.moder.modify(|_, w| w.moder0().analog()),
            Channel::ADC1 => gpioa.moder.modify(|_, w| w.moder1().analog()),
            Channel::ADC13 => gpioc.moder.modify(|_, w| w.moder3().analog()),
            Channel::ADC14 => gpioc.moder.modify(|_, w| w.moder4().analog()),
        }
    });
}
