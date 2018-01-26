use stm32f0x2::{TIM3, GPIOB, GPIOC, RCC};

use cortex_m;

static mut FREQ: u32 = 0;
static mut DUT: f32 = 0.0;

// PWM channels : Pins available
pub enum Pin {
    PB4,
    PB5,
    PC6,
    PC7,
    PC8,
    PC9,
}

pub struct Pwm {
    pin: Pin,
}

impl Pwm {
    pub fn init(pin: Pin) -> Pwm {
        cortex_m::interrupt::free(|cs| {
            let rcc = RCC.borrow(cs);
            let tim3 = TIM3.borrow(cs);
            let gpiob = GPIOB.borrow(cs);
            let gpioc = GPIOC.borrow(cs);

            // GPIOB Clock Activated
            rcc.ahbenr
                .modify(|_, w| w.iopben().enabled().iopcen().enabled());
            // TIMER Clock Activated
            rcc.apb1enr.modify(|_, w| w.tim3en().enabled());

            match pin {
                Pin::PB4 => {
                    gpiob.moder.modify(|_, w| w.moder4().alternate());
                    gpiob.afrl.modify(|_, w| w.afrl4().af1());
                    gpiob.otyper.modify(|_, w| w.ot4().push_pull());
                    tim3.ccmr1_output
                        .modify(|_, w| w.oc1m().pwmmode1().oc1pe().enabled());
                    tim3.ccer.modify(|_, w| w.cc1p().clear_bit());
                    tim3.ccer.modify(|_, w| w.cc1e().active());
                }
                Pin::PB5 => {
                    gpiob.moder.modify(|_, w| w.moder5().alternate());
                    gpiob.afrl.modify(|_, w| w.afrl5().af1());
                    gpiob.otyper.modify(|_, w| w.ot5().push_pull());
                    tim3.ccmr1_output
                        .modify(|_, w| w.oc2m().pwmmode1().oc2pe().enabled());
                    tim3.ccer.modify(|_, w| w.cc2p().clear_bit());
                    tim3.ccer.modify(|_, w| w.cc2e().active());
                }
                Pin::PC6 => {
                    gpioc.moder.modify(|_, w| w.moder6().alternate());
                    gpioc.afrl.modify(|_, w| w.afrl6().af0());
                    gpioc.otyper.modify(|_, w| w.ot6().push_pull());
                    tim3.ccmr1_output
                        .modify(|_, w| w.oc1m().pwmmode1().oc1pe().enabled());
                    tim3.ccer.modify(|_, w| w.cc1p().clear_bit());
                    tim3.ccer.modify(|_, w| w.cc1e().active());
                }
                Pin::PC7 => {
                    gpioc.moder.modify(|_, w| w.moder7().alternate());
                    gpioc.afrl.modify(|_, w| w.afrl7().af0());
                    gpioc.otyper.modify(|_, w| w.ot7().push_pull());
                    tim3.ccmr1_output
                        .modify(|_, w| w.oc2m().pwmmode1().oc2pe().enabled());
                    tim3.ccer.modify(|_, w| w.cc2p().clear_bit());
                    tim3.ccer.modify(|_, w| w.cc2e().active());
                }
                Pin::PC8 => {
                    gpioc.moder.modify(|_, w| w.moder8().alternate());
                    gpioc.afrh.modify(|_, w| w.afrh8().af0());
                    gpioc.otyper.modify(|_, w| w.ot8().push_pull());
                    tim3.ccmr2_output
                        .modify(|_, w| w.oc3m().pwmmode1().oc3pe().enabled());
                    tim3.ccer.modify(|_, w| w.cc3p().clear_bit());
                    tim3.ccer.modify(|_, w| w.cc3e().active());
                }
                Pin::PC9 => {
                    gpioc.moder.modify(|_, w| w.moder9().alternate());
                    gpioc.afrh.modify(|_, w| w.afrh9().af0());
                    gpioc.otyper.modify(|_, w| w.ot9().push_pull());
                    tim3.ccmr2_output
                        .modify(|_, w| w.oc4m().pwmmode1().oc4pe().enabled());
                    tim3.ccer.modify(|_, w| w.cc4p().clear_bit());
                    tim3.ccer.modify(|_, w| w.cc4e().active());
                }
            }

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
        Pwm { pin }
    }

    pub fn set_frequency(&self, frequency: u32) {
        cortex_m::interrupt::free(|cs| {
            let tim3 = TIM3.borrow(cs);

            // Set Prescaler Register - 16 bits
            tim3.psc.write(|w| w.psc().bits(23)); // 1 MHz // 23

            // Set Auto-Reload register - 16 bits
            let arr = 1000000 / frequency; //1000000
            tim3.arr.write(|w| w.arr_h().bits((arr >> 16) as u16));
            tim3.arr.write(|w| w.arr_l().bits(arr as u16));
            unsafe {
                FREQ = frequency;
            }
            unsafe {
                self.set_duty(DUT);
            }
        });
    }

    pub fn enable(&self) {
        cortex_m::interrupt::free(|cs| {
            let tim3 = TIM3.borrow(cs);
            match self.pin {
                Pin::PB4 => tim3.ccer.modify(|_, w| w.cc1e().set_bit()),
                Pin::PB5 => tim3.ccer.modify(|_, w| w.cc2e().set_bit()),
                Pin::PC6 => tim3.ccer.modify(|_, w| w.cc1e().set_bit()),
                Pin::PC7 => tim3.ccer.modify(|_, w| w.cc2e().set_bit()),
                Pin::PC8 => tim3.ccer.modify(|_, w| w.cc3e().set_bit()),
                Pin::PC9 => tim3.ccer.modify(|_, w| w.cc4e().set_bit()),
            }
        });
    }

    pub fn set_duty(&self, duty: f32) {
        cortex_m::interrupt::free(|cs| {
            let tim3 = TIM3.borrow(cs);
            unsafe {
                let ccr_value = (((1000000.0 / FREQ as f32) / 100.0) * duty) as u32;
                DUT = duty;
                match self.pin {
                    Pin::PB4 => tim3.ccr1.write(|w| {
                        w.ccr1_l()
                            .bits((ccr_value) as u16)
                            .ccr1_h()
                            .bits((ccr_value >> 16) as u16)
                    }),
                    Pin::PB5 => tim3.ccr2.write(|w| {
                        w.ccr2_l()
                            .bits((ccr_value) as u16)
                            .ccr2_h()
                            .bits((ccr_value >> 16) as u16)
                    }),
                    Pin::PC6 => tim3.ccr1.write(|w| {
                        w.ccr1_l()
                            .bits((ccr_value) as u16)
                            .ccr1_h()
                            .bits((ccr_value >> 16) as u16)
                    }),
                    Pin::PC7 => tim3.ccr2.write(|w| {
                        w.ccr2_l()
                            .bits((ccr_value) as u16)
                            .ccr2_h()
                            .bits((ccr_value >> 16) as u16)
                    }),
                    Pin::PC8 => tim3.ccr3.write(|w| {
                        w.ccr3_l()
                            .bits((ccr_value) as u16)
                            .ccr3_h()
                            .bits((ccr_value >> 16) as u16)
                    }),
                    Pin::PC9 => tim3.ccr4.write(|w| {
                        w.ccr4_l()
                            .bits((ccr_value) as u16)
                            .ccr4_h()
                            .bits((ccr_value >> 16) as u16)
                    }),
                }
            }
        });
    }
}
