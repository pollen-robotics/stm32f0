//! # RCC Module
//!
//! This module configures the Reset and Clock Control at 48Mhz
//!
//! ## Example :
//! ```
//! extern crate stm32f0_hal;
//!
//! use stm32f0_hal::rcc;
//!
//! rcc::init();
//! ```

use stm32f0x2::{FLASH, RCC};
use cortex_m;
use core::ptr;

use cortex_m::peripheral::{SystClkSource, SYST};

static mut TICKS: u32 = 0;

pub fn init() {
    cortex_m::interrupt::free(|cs| {
        let rcc = RCC.borrow(cs);
        let flash = FLASH.borrow(cs);
        let systick = SYST.borrow(cs);

        flash.acr.write(|w| w.latency()._1wait_state());
        // Select HSI as main system clock
        rcc.cr.modify(|_, w| w.hsion().set_bit());
        // Waiting HSI enabled
        while rcc.cr.read().hsirdy().bit_is_clear() {}
        // Set HSI Calibration trimming - 8MHz
        rcc.cr.write(|w| w.hsitrim().bits(15));

        // To be sure : Disable PLL
        rcc.cr.modify(|_, w| w.pllon().clear_bit());
        // Waiting PLL Off
        while rcc.cr.read().pllrdy().bit_is_set() {}
        // Configure PLL source clock predivider to 1
        rcc.cfgr2.write(|w| w.prediv().div1());
        // Configure PLL source clock to HSI (8MHz) and multiplier to 6
        rcc.cfgr.write(|w| w.pllsrc().hsi().pllmul().mul6());
        // PLL On
        rcc.cr.modify(|_, w| w.pllon().set_bit());
        // Waiting PLL On
        while rcc.cr.read().pllrdy().bit_is_clear() {}
        // Set AHB & APB divider to 1 -> 48MHz
        rcc.cfgr.modify(|_, w| w.hpre().div1().ppre().div1());
        // Select PLL output as system clock
        rcc.cfgr.modify(|_, w| w.sw().pll());
        // Waiting system clock ready
        while rcc.cfgr.read().sws().bits() != 2 {}
        // Configure Systick source and base time 1ms
        systick.set_clock_source(SystClkSource::Core);
        systick.set_reload((48000000 / 1000) - 1);
        systick.enable_counter();
        systick.enable_interrupt();
    });
}

pub fn ms_delay(delay: u32) {
    let start = systick();
    while (systick() - start) < delay {}
}


pub fn systick() -> u32 {
    return unsafe { ptr::read_volatile(&TICKS) };
}


exception!(SYS_TICK, tick);

fn tick() {
    unsafe {
        TICKS += 1;
    }
}
