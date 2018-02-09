use hal;
use gpio::{Alternate, PushPull};
use gpio::gpioc::{PC6, PC9};
use rcc::{APB1, APB2, Clocks};
use time::Hertz;

use stm32f0x2::{TIM15, TIM3};

pub struct Pwm<TIM> {
    tim: TIM,
    channels: (bool, bool, bool, bool),
}

macro_rules! pwm {
    ($TIMX:ident: ($timx:ident, $APBX:ident, $timxen:ident, $timxrst:ident)) => {
        impl Pwm<$TIMX> {
            pub fn $timx<PINS>(tim: $TIMX, pins: PINS, freq: Hertz, clocks: Clocks, apb: &mut $APBX) -> Pwm<$TIMX>
            where PINS: Pins<$TIMX>
            {
                apb.enr().modify(|_, w| w.$timxen().enabled());

                apb.rstr().modify(|_, w| w.$timxrst().set_bit());
                apb.rstr().modify(|_, w| w.$timxrst().clear_bit());

                let channels = pins.channels();
                if channels.0 {
                    tim.ccmr1_output
                        .modify(|_, w| w.oc1pe().enabled().oc1m().pwmmode1());
                }
                if channels.1 {
                    tim.ccmr1_output
                        .modify(|_, w| w.oc2pe().enabled().oc2m().pwmmode1());
                }
                if channels.2 {
                    tim.ccmr2_output
                        .modify(|_, w| w.oc3pe().enabled().oc3m().pwmmode1());
                }
                if channels.3 {
                    tim.ccmr2_output
                        .modify(|_, w| w.oc4pe().enabled().oc4m().pwmmode1());
                }

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

                Pwm { channels, tim }
            }
        }
        impl hal::PwmPin for Pwm<$TIMX> {
            type Duty = u32;
            fn enable(&mut self) {
                if self.channels.0 {
                    self.tim.ccer.modify(|_, w| w.cc1e().set_bit());
                }
                if self.channels.1 {
                    self.tim.ccer.modify(|_, w| w.cc2e().set_bit());
                }
                if self.channels.2 {
                    self.tim.ccer.modify(|_, w| w.cc3e().set_bit());
                }
                if self.channels.3 {
                    self.tim.ccer.modify(|_, w| w.cc4e().set_bit());
                }
            }
            fn disable(&mut self) {
                if self.channels.0 {
                    self.tim.ccer.modify(|_, w| w.cc1e().clear_bit());
                }
                if self.channels.1 {
                    self.tim.ccer.modify(|_, w| w.cc2e().clear_bit());
                }
                if self.channels.2 {
                    self.tim.ccer.modify(|_, w| w.cc3e().clear_bit());
                }
                if self.channels.3 {
                    self.tim.ccer.modify(|_, w| w.cc4e().clear_bit());
                }
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
                if self.channels.0 {
                    self.tim.ccr1.write(|w| w.ccr1_l().bits(l).ccr1_h().bits(h));
                }
                if self.channels.1 {
                    self.tim.ccr2.write(|w| w.ccr2_l().bits(l).ccr2_h().bits(h));
                }
                if self.channels.2 {
                    self.tim.ccr3.write(|w| w.ccr3_l().bits(l).ccr3_h().bits(h));
                }
                if self.channels.3 {
                    self.tim.ccr4.write(|w| w.ccr4_l().bits(l).ccr4_h().bits(h));
                }
            }
        }
    }
}

pub trait Pins<TIM> {
    fn channels(&self) -> (bool, bool, bool, bool);
}

impl Pins<TIM3> for (PC6<Alternate<PushPull>>, PC9<Alternate<PushPull>>) {
    fn channels(&self) -> (bool, bool, bool, bool) {
        (true, false, false, true)
    }
}

pwm!(TIM3: (tim3, APB1, tim3en, tim3rst));
// pwm!(TIM15: (tim15, APB2, tim15en, tim15rst));

// pwm!(tim3: (APB1, tim3en, tim3rst, [PA6-AF0, PB4, PC6], [PA7, PB5, PC7], [PB0, PC8], [PB1, PC9]))
// C1, C2, C3, C4, C1-C2, C1-C3, C1-C4, C2-C3, C2-C4, C3-C4, C1-C2-C3, C1-C2-C4, C1-C3-C4, C2-*-*, C3-*-*, C4-*-*, C1-C2-C3-C4
// pwm!(tim15: (APB2, tim15en, tim15rst, [PA2, PB14], [PA3, PB15]))
