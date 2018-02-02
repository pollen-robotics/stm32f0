use hal::timer::CountDown;
use nb::Result;

use time::Hertz;
use stm32f0x2::TIM7;

use rcc::APB1;

pub struct Timer<TIM> {
    tim: TIM,
}

macro_rules! timer {
    ($($TIMX:ident: ($timx:ident, $APBX:ident, $timxen:ident, $timxrst:ident), )+) => {
        $(
        impl Timer<$TIMX> {
            pub fn $timx<T>(tim: $TIMX, timeout: T, apb: &mut $APBX) -> Self
            where T: Into<Hertz> {
                apb.enr().modify(|_, w| w.$timxen().enabled());
                apb.rstr().modify(|_, w| w.$timxrst().set_bit());
                apb.rstr().modify(|_, w| w.$timxrst().clear_bit());

                let mut timer = Timer { tim };
                timer.configure(timeout);
                timer
            }
            fn configure<T>(&mut self, _timeout: T)
            where T: Into<Hertz> {
                self.tim.cr1.modify(|_, w| w.cen().disabled());
                // TODO: WIP
            }
        }
        impl CountDown for Timer<$TIMX> {
            type Time = Hertz;
            fn start<T>(&mut self, timeout: T)
            where
                T: Into<Hertz> {
                self.configure(timeout);
                // TODO: WIP
            }
            fn wait(&mut self) -> Result<(), !> {
                // TODO: WIP
                Ok(())
            }
        }
        )+
    }
}

timer!(TIM7: (tim7, APB1, tim7en, tim7rst),);
