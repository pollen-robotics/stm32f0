use cortex_m;
use stm32f0x2::{RCC, GPIOA, GPIOC};

pub enum Mode {
    Input = 0b00,
    Output = 0b01,
}

pub enum Pin {
    P0,
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
    P7,
    P8,
    P9,
    P10,
    P11,
    P12,
    P13,
    P14,
    P15,
}


pub fn init(_pin: &Pin, _mode: Mode) {
    cortex_m::interrupt::free(|cs| {
        let rcc = RCC.borrow(cs);

        let gpioa = GPIOA.borrow(cs);
        rcc.ahbenr.modify(|_, w| w.iopaen().enabled());
        gpioa.moder.modify(|_, w| w.moder0().bits(0b00));

        let gpioc = GPIOC.borrow(cs);
        rcc.ahbenr.modify(|_, w| w.iopcen().enabled());
        gpioc.moder.modify(|_, w| w.moder6().bits(0b01));
        gpioc.moder.modify(|_, w| w.moder7().bits(0b01));
        gpioc.moder.modify(|_, w| w.moder8().bits(0b01));
        gpioc.moder.modify(|_, w| w.moder9().bits(0b01));
    });
}

pub fn read(_pin: &Pin) -> bool {
    PA0.read()
}

pub fn write(_pin: &Pin, on: bool) {
    if on {
        PC6.high();
    } else {
        PC6.low();
    }
}


macro_rules! pin_read {
    ($PBX:ident, $idrX:ident) => {
        pub struct $PBX;

        impl $PBX {
            pub fn read(&self) -> bool {
                unsafe {
                    (*GPIOA.get()).idr.read().$idrX().bit()
                }
            }
        }
    }
}


macro_rules! pin {
    ($PBX:ident, $bsX:ident, $brX:ident) => {
        /// Digital output
        pub struct $PBX;

        impl $PBX {
            /// Sets the pin "high" (3V3)
            pub fn high(&self) {
                // NOTE(safe) atomic write
                unsafe {
                    (*GPIOC.get()).bsrr.write(|w| w.$bsX().bit(true));
                }
            }

            /// Sets the pin "low" (0V)
            pub fn low(&self) {
                // NOTE(safe) atomic write
                unsafe {
                    (*GPIOC.get()).bsrr.write(|w| w.$brX().bit(true));
                }
            }
        }
    }
}


pin_read!(PA0, idr0);

pin!(PC6, bs6, br6);
pin!(PC7, bs7, br7);
pin!(PC8, bs8, br8);
pin!(PC9, bs9, br9);
