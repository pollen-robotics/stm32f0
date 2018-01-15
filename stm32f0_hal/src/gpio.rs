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
use core;

use cortex_m;
use stm32f0x2::{GPIOA, GPIOC, RCC, EXTI, NVIC};
use stm32f0x2::interrupt::*;

enum Mode {
    Input = 0b00,
    Output = 0b01,
}

/// GPIO Pin available on PORT A
pub enum Pin {
    PA0,
    PA1,
    PA5,
    PC7,
}

/// Input Mode Pin
pub struct Input {
    pin: Pin,
}

impl Input {
    /// Setup a PIN in Input Mode
    pub fn setup(pin: Pin) -> Input{
        setup_pin(&pin, Mode::Input);
        Input { pin }
    }

    /// Read the state of a PIN
    pub fn read(&self) -> bool {
        unsafe {
            match self.pin {
                Pin::PA0 => (*GPIOA.get()).idr.read().idr0().bit(),
                Pin::PA1 => (*GPIOA.get()).idr.read().idr1().bit(),
                Pin::PA5 => (*GPIOA.get()).idr.read().idr5().bit(),
                Pin::PC7 => (*GPIOC.get()).idr.read().idr7().bit(),
            }
        }
    }

    pub fn init_interrupt(&self) {
        match self.pin {
            Pin::PA0 => {
                cortex_m::interrupt::free(|cs| {
                    let exti = EXTI.borrow(cs);
                    let nvic = NVIC.borrow(cs);
                    exti.rtsr.modify(|_, w| w.tr0().set_bit());
                    exti.imr.modify( |_,w| w.mr0().set_bit());
                    nvic.enable(Interrupt::EXTI0_1);
                })
            }
            Pin::PA1 => {
                cortex_m::interrupt::free(|cs| {
                    let exti = EXTI.borrow(cs);
                    let nvic = NVIC.borrow(cs);
                    exti.rtsr.modify(|_, w| w.tr1().set_bit());
                    exti.imr.modify( |_,w| w.mr1().set_bit());
                    nvic.enable(Interrupt::EXTI0_1);
                })
            }
            Pin::PA5 => {
                cortex_m::interrupt::free(|cs| {
                    let exti = EXTI.borrow(cs);
                    let nvic = NVIC.borrow(cs);
                    exti.rtsr.modify(|_, w| w.tr5().set_bit());
                    exti.imr.modify( |_,w| w.mr5().set_bit());
                    nvic.enable(Interrupt::EXTI4_15);
                })
            }
            Pin::PC7 => {
                cortex_m::interrupt::free(|cs| {
                    let exti = EXTI.borrow(cs);
                    let nvic = NVIC.borrow(cs);
                    exti.rtsr.modify(|_, w| w.tr7().set_bit());
                    exti.imr.modify( |_,w| w.mr7().set_bit());
                    nvic.enable(Interrupt::EXTI4_15);
                })
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
        unsafe {
            match self.pin {
                Pin::PA0 => (*GPIOA.get()).bsrr.write(|w| w.bs0().bit(true)),
                Pin::PA1 => (*GPIOA.get()).bsrr.write(|w| w.bs1().bit(true)),
                Pin::PA5 => (*GPIOA.get()).bsrr.write(|w| w.bs5().bit(true)),
                Pin::PC7 => (*GPIOC.get()).bsrr.write(|w| w.bs7().bit(true)),
            }
        }
    }
    /// Set the PIN to low
    pub fn low(&mut self) {
        unsafe {
            match self.pin {
                Pin::PA0 => (*GPIOA.get()).bsrr.write(|w| w.br0().bit(true)),
                Pin::PA1 => (*GPIOA.get()).bsrr.write(|w| w.br1().bit(true)),
                Pin::PA5 => (*GPIOA.get()).bsrr.write(|w| w.br5().bit(true)),
                Pin::PC7 => (*GPIOC.get()).bsrr.write(|w| w.br7().bit(true)),
            }
        }
    }
}

fn setup_pin(pin: &Pin, mode: Mode) {
    cortex_m::interrupt::free(|cs| {
        let rcc = RCC.borrow(cs);

        let gpioa = GPIOA.borrow(cs);
        let gpioc = GPIOC.borrow(cs);
        rcc.ahbenr
            .modify(|_, w| w.iopaen().enabled().iopcen().enabled());
        let mode = mode as u8;

        match *pin {
            Pin::PA0 => gpioa.moder.modify(|_, w| w.moder0().bits(mode)),
            Pin::PA1 => gpioa.moder.modify(|_, w| w.moder1().bits(mode)),
            Pin::PA5 => gpioa.moder.modify(|_, w| w.moder5().bits(mode)),
            Pin::PC7 => gpioc.moder.modify(|_, w| w.moder7().bits(mode)),
        }
    });
}

