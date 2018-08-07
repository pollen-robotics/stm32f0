//use core;

use cortex_m;
use stm32f0x2::{TIM7 as TIMER7, GPIOC, NVIC, RCC};
use stm32f0x2::interrupt::*;

pub fn setup(timeout: u16, gpio: GPIOC, rcc: RCC, timer: TIMER7, nvic: NVIC) {
    // LED Test
    rcc.ahbenr.modify(|_, w| w.iopcen().enabled());
    gpio.moder.modify(|_, w| w.moder7().output());

    //Enable TIM7 clock
    rcc.apb1enr.modify(|_, w| w.tim7en().enabled());

    // configure Time Out
    // Set Prescaler Register - 16 bits
    timer.psc.modify(|_, w| w.psc().bits(47));
    // Set Auto-Reload register - 32 bits
    timer.arr.modify(|_, w| w.arr().bits(timeout - 1));

    timer.cr1.modify(|_, w| w.opm().continuous());
    // Enable interrupt
    timer.dier.modify(|_, w| w.uie().enabled());
    // Interrupt activated
    nvic.enable(Interrupt::TIM7);
    nvic.clear_pending(Interrupt::TIM7);
}

pub fn pause(timer: TIMER7) {
    // Disable counter
    timer.cr1.modify(|_, w| w.cen().disabled());
}

pub fn restart(timer: TIMER7) {
    // Reset counter
    timer.cnt.write(|w| w.cnt().bits(0));
}

pub fn resume(timer: TIMER7) {
    // Enable counter
    timer.cr1.modify(|_, w| w.cen().enabled());
}
