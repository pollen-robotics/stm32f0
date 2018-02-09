use hal;
use rcc::{APB1, Clocks};
use time::Hertz;

use stm32f0x2::TIM3;

pub struct Pwm {
    tim: TIM3,
}

impl Pwm {
    pub fn tim3(tim: TIM3, freq: Hertz, clocks: Clocks, apb: &mut APB1) -> Pwm {
        apb.enr().modify(|_, w| w.tim3en().enabled());

        apb.rstr().modify(|_, w| w.tim3rst().set_bit());
        apb.rstr().modify(|_, w| w.tim3rst().clear_bit());

        tim.ccmr1_output
            .modify(|_, w| w.oc1pe().enabled().oc1m().pwmmode1());
        tim.ccmr1_output
            .modify(|_, w| w.oc2pe().enabled().oc2m().pwmmode1());
        tim.ccmr2_output
            .modify(|_, w| w.oc3pe().enabled().oc3m().pwmmode1());
        tim.ccmr2_output
            .modify(|_, w| w.oc4pe().enabled().oc4m().pwmmode1());

        // We *2 as we work in center_align_mode1
        let ticks = clocks.pclk().0 / (2 * freq.0);
        let psc = ticks / (1 << 16);
        tim.psc.write(|w| w.psc().bits(psc as u16));

        let arr = ticks / (psc + 1);
        tim.arr.write(|w| w.arr_h().bits((arr >> 16) as u16));
        tim.arr.write(|w| w.arr_l().bits(arr as u16));

        tim.cr1.write(|w| {
            w.cms()
                .center_align_mode1()
                .dir()
                .up()
                .opm()
                .continuous()
                .cen()
                .enabled()
        });

        tim.egr.write(|w| w.ug().rst_update());

        Pwm { tim }
    }
}

impl hal::PwmPin for Pwm {
    type Duty = u32;

    fn enable(&mut self) {
        self.tim.ccer.modify(|_, w| w.cc1e().set_bit());
        self.tim.ccer.modify(|_, w| w.cc2e().set_bit());
        self.tim.ccer.modify(|_, w| w.cc3e().set_bit());
        self.tim.ccer.modify(|_, w| w.cc4e().set_bit());
    }
    fn disable(&mut self) {
        self.tim.ccer.modify(|_, w| w.cc1e().clear_bit());
        self.tim.ccer.modify(|_, w| w.cc2e().clear_bit());
        self.tim.ccer.modify(|_, w| w.cc3e().clear_bit());
        self.tim.ccer.modify(|_, w| w.cc4e().clear_bit());
    }

    fn get_duty(&self) -> Self::Duty {
        (self.tim.ccr2.read().ccr2_h().bits() as u32) << 16
            | self.tim.ccr2.read().ccr2_l().bits() as u32
    }
    fn get_max_duty(&self) -> Self::Duty {
        (self.tim.arr.read().arr_h().bits() as u32) << 16
            | self.tim.arr.read().arr_l().bits() as u32
    }
    fn set_duty(&mut self, duty: Self::Duty) {
        let h = (duty >> 16) as u16;
        let l = duty as u16;

        self.tim.ccr1.write(|w| w.ccr1_l().bits(l).ccr1_h().bits(h));
        self.tim.ccr2.write(|w| w.ccr2_l().bits(l).ccr2_h().bits(h));
        self.tim.ccr3.write(|w| w.ccr3_l().bits(l).ccr3_h().bits(h));
        self.tim.ccr4.write(|w| w.ccr4_l().bits(l).ccr4_h().bits(h));
    }
}

// pwm!(tim3: (APB1, tim3en, tim3rst, [PA6-AF0, PB4, PC6], [PA7, PB5, PC7], [PB0, PC8], [PB1, PC9]))
// C1, C2, C3, C4, C1-C2, C1-C3, C1-C4, C2-C3, C2-C4, C3-C4, C1-C2-C3, C1-C2-C4, C1-C3-C4, C2-*-*, C3-*-*, C4-*-*, C1-C2-C3-C4
// pwm!(tim15: (APB2, tim15en, tim15rst, [PA2, PB14], [PA3, PB15]))
