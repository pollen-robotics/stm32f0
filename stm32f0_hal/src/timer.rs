use cortex_m;
use hal::timer::{CountDown, Event, Timeout};
use nb::{Error, Result};

use time::Hertz;
use stm32f0x2::{Interrupt, TIM6, TIM7};
use rcc::{APB1, Clocks};

pub struct Timer<TIM> {
    tim: TIM,
    clocks: Clocks,
}

macro_rules! timer {
    ($($TIMX:ident: ($timx:ident, $APBX:ident, $timxen:ident, $timxrst:ident, $TIMX_INTERRUPT:ident), )+) => {
        $(
        impl Timer<$TIMX> {
            pub fn $timx(tim: $TIMX, clocks: Clocks, apb: &mut $APBX) -> Self
            {
                apb.enr().modify(|_, w| w.$timxen().enabled());
                apb.rstr().modify(|_, w| w.$timxrst().set_bit());
                apb.rstr().modify(|_, w| w.$timxrst().clear_bit());

                Timer { clocks, tim }
            }
            pub fn _start(&mut self, frequency: Hertz) {
                self.tim.cr1.modify(|_, w| w.cen().disabled());
                self.tim.cnt.reset();

                let ticks = self.clocks.pclk().0 / frequency.0;

                let psc = (ticks - 1) / (1 << 16);
                self.tim.psc.write(|w| w.psc().bits(psc as u16));

                let arr = ticks / (psc + 1);
                self.tim.arr.write(|w| w.arr().bits(arr as u16));

                self.tim.cr1.modify(|_, w| w.cen().enabled());
            }
            pub unsafe fn enable_interrupt(&mut self) {
                // TODO: That's a really really unsafe way of accessing NVIC...
                // We probably should expose a NVIC trait as a parameter instead.
                // As we don't want to have any cortex_m object in the trait signature.
                let mut nvic = cortex_m::Peripherals::steal().NVIC;

                nvic.enable(Interrupt::$TIMX_INTERRUPT);
                nvic.clear_pending(Interrupt::$TIMX_INTERRUPT);
            }
        }
        impl Timeout for Timer<$TIMX> {
            type Time = u32;

            fn start(&mut self, timeout: u32) {
                self._start(Hertz(timeout));
            }
            fn listen(&mut self, event: Event) {

                match event {
                    Event::Fired => {
                        self.tim.dier.modify(|_, w| w.uie().enabled());
                        unsafe {
                            self.enable_interrupt();
                        }
                    },
                }
            }
            fn unlisten(&mut self, event: Event) {
                match event {
                    Event::Fired => {
                        self.tim.dier.modify(|_, w| w.uie().disabled());
                    }
                }
            }
        }
        impl CountDown for Timer<$TIMX> {
            type Time = Hertz;
            fn start<T>(&mut self, timeout: T)
            where
                T: Into<Hertz> {
                self._start(timeout.into());
            }
            fn wait(&mut self) -> Result<(), !> {
                if self.tim.sr.read().uif().bit_is_clear() {
                    Err(Error::WouldBlock)
                } else {
                    // Clear event flag
                    self.tim.sr.modify(|_, w| w.uif().clear());
                    Ok(())
                }
            }
        }
        )+
    }
}

timer!(TIM6: (tim6, APB1, tim6en, tim6rst, TIM6_DAC),);
timer!(TIM7: (tim7, APB1, tim7en, tim7rst, TIM7),);
