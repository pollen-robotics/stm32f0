//! # STM32F0 HAL
//!
//! Provides high-level API to the STM32F0 board functionalities. At the moment the following features are available:
//!
//! * GPIO
//! * ADC
//! * PWM
//! * RCC

#![no_std]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate cortex_m;
#[macro_use(exception)]
extern crate cortex_m_rt;
#[macro_use(interrupt)]
extern crate stm32f0x2;

pub mod gpio;
pub mod adc;
pub mod pwm;
pub mod rcc;
pub mod uart;
