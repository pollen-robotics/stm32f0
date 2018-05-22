//! # STM32F0 HAL
//!
//! Provides high-level API to the STM32F0 board functionalities. At the moment the following features are available:
//!
//! * GPIO
//! * ADC
//! * PWM
//! * RCC

#![no_std]
#![feature(lang_items)]
#![feature(core_intrinsics)]
#![cfg_attr(feature = "use_alloc", feature(global_allocator))]

extern crate cortex_m;
#[macro_use(exception)]
extern crate cortex_m_rt;
extern crate embedded_hal;
extern crate stm32f0x2;

pub mod adc;
pub mod debug;
pub mod gpio;
pub mod i2c;
pub mod pwm;
pub mod rcc;
pub mod uart;
pub mod timer;
pub mod servo;

#[cfg(feature = "use_alloc")]
extern crate alloc_cortex_m as heap;
#[cfg(feature = "use_alloc")]
#[global_allocator]
static ALLOCATOR: heap::CortexMHeap = heap::CortexMHeap::empty();
#[cfg(feature = "use_alloc")]
pub mod allocator {
    extern "C" {
        static mut _sheap: u32;
    }

    pub fn setup(heap_size: usize) {
        let heap_start = unsafe { &mut _sheap as *mut u32 as usize };
        unsafe {
            ::ALLOCATOR.init(heap_start, heap_size);
        }
    }
}
