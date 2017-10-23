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
//! gpio::init(gpio::Pin::P8, gpio::Mode::Input);
//! let b = gpio::read(gpio::Pin::P8);
//!
//! gpio::init(gpio::Pin::P9, gpio::Mode::Output);
//! gpio::write(gpio::Pin::P9, true);
//! ```

use cortex_m;
use stm32f0x2::{RCC, GPIOA};

pub enum Mode {
    Input = 0b00,
    Output = 0b01,
}

pub enum Pin {
    P8,
    P9,
    P10,
}


/// Setup a pin in Input or Output Mode.
pub fn init(pin: &Pin, mode: Mode) {
    cortex_m::interrupt::free(|cs| {
        let rcc = RCC.borrow(cs);

        let gpioa = GPIOA.borrow(cs);
        rcc.ahbenr.modify(|_, w| w.iopaen().enabled());

        let mode = mode as u8;

        match *pin {
            Pin::P8 => gpioa.moder.modify(|_, w| w.moder8().bits(mode)),
            Pin::P9 => gpioa.moder.modify(|_, w| w.moder9().bits(mode)),
            Pin::P10 => gpioa.moder.modify(|_, w| w.moder10().bits(mode)),
        }
    });
}

/// Read a pin in Input Mode.
pub fn read(pin: &Pin) -> bool {
    match *pin {
        Pin::P8 => PA8.read(),
        Pin::P9 => PA9.read(),
        Pin::P10 => PA10.read(),
    }
}

/// Write a bool to a pin in Output Mode.
pub fn write(_pin: &Pin, _on: bool) {
    // if on {
    //     PC6.high();
    // } else {
    //     PC6.low();
    // }
}


macro_rules! pin_read {
    ($PBX:ident, $idrX:ident) => {
        pub struct $PBX;

        impl $PBX {
            pub fn read(&self) -> bool {
                unsafe {
                    (*GPIOA.get()).idr.read().$idrX().bit()
                }
            }
        }
    }
}


// macro_rules! pin_write {
//     ($PBX:ident, $bsX:ident, $brX:ident) => {
//         /// Digital output
//         pub struct $PBX;
//
//         impl $PBX {
//             /// Sets the pin "high" (3V3)
//             pub fn high(&self) {
//                 // NOTE(safe) atomic write
//                 unsafe {
//                     (*GPIOC.get()).bsrr.write(|w| w.$bsX().bit(true));
//                 }
//             }
//
//             /// Sets the pin "low" (0V)
//             pub fn low(&self) {
//                 // NOTE(safe) atomic write
//                 unsafe {
//                     (*GPIOC.get()).bsrr.write(|w| w.$brX().bit(true));
//                 }
//             }
//         }
//     }
// }

pin_read!(PA8, idr8);
pin_read!(PA9, idr9);
pin_read!(PA10, idr10);
