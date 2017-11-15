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
extern crate stm32f0x2;
extern crate cortex_m_rt;

pub mod gpio;
pub mod adc;
pub mod pwm;
pub mod rcc;
