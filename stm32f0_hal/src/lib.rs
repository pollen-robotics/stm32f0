//! # STM32F0 HAL
//!
//! Provides high-level API to the STM32F0 board functionalities. At the moment the following features are available:
//!
//! * GPIO
//! * ADC
//! * PWM
//! * RCC

#![no_std]
#![cfg_attr(feature = "use_alloc", feature(global_allocator))]

extern crate cortex_m;
#[macro_use(exception)]
extern crate cortex_m_rt;
extern crate stm32f0x2;

pub mod gpio;
pub mod adc;
pub mod pwm;
pub mod rcc;
pub mod uart;
pub mod timer;

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
