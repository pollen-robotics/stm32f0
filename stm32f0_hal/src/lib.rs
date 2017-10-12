#![no_std]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate cortex_m;
extern crate stm32f0x2;

pub mod gpio;
