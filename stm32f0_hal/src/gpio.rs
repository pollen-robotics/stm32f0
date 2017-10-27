//! # gpio module
//!
//! This module provides access to the GPIO of the stm32f0 board.
//! You can:
//!
//! * configure a PIN in Input or Output
//! * read or write the PIN depending on its configuration
//!
//! ## Examples
//!
//! ```
//! extern crate stm32f0_hal;
//!
//! use stm32f0_hal::gpio;
//!
// ! let p_in = gpio::Input::setup(gpio::Pin::PA1);
// ! let b = p_in.read();
// !
// ! let p_out = gpio::Output::setup(gpio::Pin::PA5);
// ! p_out.write(true);
//! ```

use cortex_m;
use stm32f0x2::{RCC, GPIOA};

enum Mode {
    Input = 0b00,
    Output = 0b01,
}

/// GPIO Pin available on PORT A
pub enum Pin {
    PA1,
    PA5,
}

/// Input Mode Pin
pub struct Input {
    pin: Pin,
}

impl Input {
    /// Setup a PIN in Input Mode
    pub fn setup(pin: Pin) -> Input {
        setup_pin(&pin, Mode::Input);
        Input { pin }
    }

    /// Read the state of a PIN
    pub fn read(&self) -> bool {
        unsafe {
            match self.pin {
                Pin::PA1 => (*GPIOA.get()).idr.read().idr1().bit(),
                Pin::PA5 => (*GPIOA.get()).idr.read().idr5().bit(),
            }
        }
    }
}

/// Output Mode Pin
pub struct Output {
    pin: Pin,
}

impl Output {
    /// Setup a PIN in Output Mode
    pub fn setup(pin: Pin) -> Output {
        setup_pin(&pin, Mode::Output);
        Output { pin }
    }
    /// Set the PIN to high
    pub fn high(&mut self) {
        self.set(true);
    }
    /// Set the PIN to low
    pub fn low(&mut self) {
        self.set(false);
    }
    fn set(&mut self, on: bool) {
        unsafe {
            match self.pin {
                Pin::PA1 => (*GPIOA.get()).bsrr.write(|w| w.bs1().bit(on)),
                Pin::PA5 => (*GPIOA.get()).bsrr.write(|w| w.bs5().bit(on)),
            }
        }
    }
}


fn setup_pin(pin: &Pin, mode: Mode) {
    cortex_m::interrupt::free(|cs| {
        let rcc = RCC.borrow(cs);

        let gpioa = GPIOA.borrow(cs);
        rcc.ahbenr.modify(|_, w| w.iopaen().enabled());

        let mode = mode as u8;

        match *pin {
            Pin::PA1 => gpioa.moder.modify(|_, w| w.moder1().bits(mode)),
            Pin::PA5 => gpioa.moder.modify(|_, w| w.moder5().bits(mode)),
        }
    });
}
