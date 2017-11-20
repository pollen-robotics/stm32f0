use stm32f0x2::{TIM3, GPIOB, RCC};

use cortex_m;

// Init Timer Function without remap pin
pub fn init(period: u32) {
    cortex_m::interrupt::free(|cs| {
        let rcc = RCC.borrow(cs);
        let tim3 = TIM3.borrow(cs);
        let gpiob = GPIOB.borrow(cs);

        // GPIO Clock Activated
        rcc.ahbenr.modify(|_, w| w.iopben().enabled());
        // TIMER Clock Activated
        rcc.apb1enr.modify(|_, w| w.tim3en().enabled());

        gpiob.moder.modify(|_, w| w.moder4().alternate());
        gpiob.afrl.modify(|_, w| w.afrl4().af1());
        gpiob.otyper.modify(|_, w| w.ot4().push_pull());

        tim3.ccmr1_output.modify(|_, w| {
            w.oc1m().pwmmode1().oc1pe().enabled())
        });
        tim3.ccer.modify(|_, w| w.cc1p().clear_bit());
        set_period(period);

        tim3.ccer.modify(|_, w| w.cc1e().active());
        tim3.cr1.write(|w| {
            w.cms()
                .center_align_mode1()
                .dir()
                .up()
                .opm()
                .continuous()
                .cen()
                .enabled()
        });

        tim3.egr.write(|w| w.ug().set_bit());
    });
}

pub fn set_period(_period: u32) {
    cortex_m::interrupt::free(|cs| {
        let tim3 = TIM3.borrow(cs);

        // Set Prescaler Register - 16 bits
        tim3.psc.write(|w| w.psc().bits(7));

        // Set Auto-Reload register - 32 bits
        let arr = 10000;
        tim3.arr.write(|w| w.arr_h().bits((arr >> 16) as u16));
        tim3.arr.write(|w| w.arr_l().bits(arr as u16));
    });
}

pub fn enable() {
    cortex_m::interrupt::free(|cs| {
        let tim3 = TIM3.borrow(cs);
        tim3.ccer.modify(|_, w| w.cc1e().set_bit());
    });
}

pub fn set_duty(duty: u32) {
    cortex_m::interrupt::free(|cs| {
        let tim3 = TIM3.borrow(cs);
        tim3.ccr1.write(|w| {
            w.ccr1_l().bits((duty) as u16).ccr1_h().bits(
                (duty >> 16) as u16,
            )
        })
    });
}
