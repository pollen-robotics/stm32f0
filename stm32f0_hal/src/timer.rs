use hal::timer::CountDown;
use nb::{Error, Result};

use time::Hertz;
use stm32f0x2::{TIM6, TIM7};

use rcc::{APB1, Clocks};

pub struct Timer<TIM> {
    tim: TIM,
    clocks: Clocks,
}

macro_rules! timer {
    ($($TIMX:ident: ($timx:ident, $APBX:ident, $timxen:ident, $timxrst:ident), )+) => {
        $(
        impl Timer<$TIMX> {
            pub fn $timx(tim: $TIMX, clocks: Clocks, apb: &mut $APBX) -> Self
            {
                apb.enr().modify(|_, w| w.$timxen().enabled());
                apb.rstr().modify(|_, w| w.$timxrst().set_bit());
                apb.rstr().modify(|_, w| w.$timxrst().clear_bit());

                Timer { clocks, tim }
            }
        }
        impl CountDown for Timer<$TIMX> {
            type Time = Hertz;
            fn start<T>(&mut self, timeout: T)
            where
                T: Into<Hertz> {
                self.tim.cr1.modify(|_, w| w.cen().disabled());
                self.tim.cnt.reset();

                let frequency = timeout.into().0;
                let ticks = self.clocks.pclk().0 / frequency;

                let psc = (ticks - 1) / (1 << 16);
                self.tim.psc.write(|w| w.psc().bits(psc as u16));

                let arr = ticks / (psc + 1);
                self.tim.arr.write(|w| w.arr().bits(arr as u16));

                self.tim.cr1.modify(|_, w| w.cen().enabled());
            }
            fn wait(&mut self) -> Result<(), !> {
                if self.tim.sr.read().uif().bit_is_clear() {
                    Err(Error::WouldBlock)
                } else {
                    // Clean event flat
                    self.tim.sr.modify(|_, w| w.uif().clear());
                    Ok(())
                }
            }
        }
        )+
    }
}

timer!(TIM6: (tim6, APB1, tim6en, tim6rst),);
timer!(TIM7: (tim7, APB1, tim7en, tim7rst),);
