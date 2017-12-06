//use core;

use cortex_m;
use stm32f0x2::{TIM7 as TIMER7, NVIC, RCC, GPIOC};
use stm32f0x2::interrupt::*;

pub fn setup(timeout: u16)
{
    cortex_m::interrupt::free(|cs| {
        let gpio = GPIOC.borrow(cs);
        let rcc = RCC.borrow(cs);
        let timer = TIMER7.borrow(cs);
        let nvic = NVIC.borrow(cs);

        // LED Test
        rcc.ahbenr.modify(|_, w| w.iopcen().enabled());
        gpio.moder.modify(|_, w| w.moder7().output());

        //Enable TIM7 clock
        rcc.apb1enr.modify(|_,w| w.tim7en().enabled());

        // configure Time Out
        // Set Prescaler Register - 16 bits
        timer.psc.modify(|_, w| w.psc().bits(47));
        // Set Auto-Reload register - 32 bits
        timer.arr.modify(|_, w| w.arr().bits(timeout-1));

        timer.cr1.modify(|_, w| w.opm().continuous());
        // Enable interrupt
        timer.dier.modify(|_, w| w.uie().enabled());
        // Interrupt activated
        nvic.enable(Interrupt::TIM7);
        nvic.clear_pending(Interrupt::TIM7);
    });
}

pub fn pause(){
    cortex_m::interrupt::free(|cs| {
        let timer=TIMER7.borrow(cs);
        // Disable counter
        timer.cr1.modify(|_,w| w.cen().disabled());
    });
}

pub fn restart(){
    cortex_m::interrupt::free(|cs| {
        let timer=TIMER7.borrow(cs);
        // Reset counter
        timer.cnt.write(|w| w.cnt().bits(0));
    });
}

pub fn resume(){
    cortex_m::interrupt::free(|cs| {
        let timer=TIMER7.borrow(cs);
        // Enable counter
        timer.cr1.modify(|_,w| w.cen().enabled());
    });
}


interrupt!(TIM7, led);

pub fn led(){
    cortex_m::interrupt::free(|cs| {
        let timer = TIMER7.borrow(cs);
        let gpio = GPIOC.borrow(cs);
        // Clear interrupt flag
        timer.sr.modify(|_,w| w.uif().clear_bit());
        if gpio.idr.read().idr7().bit_is_set() {
            gpio.bsrr.write(|w| w.br7().set_bit());
        } else {
            gpio.bsrr.write(|w| w.bs7().set_bit());
        }
    });
}
