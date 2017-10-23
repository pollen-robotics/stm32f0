use cortex_m;
use stm32f0x2::{GPIOA, RCC, ADC};

pub enum Pin {
    P4,
    P5,
}

pub struct Input {
    pin: Pin,
}
impl Input {
    pub fn setup(pin: Pin) -> Input {
        setup_pin(&pin);
        Input { pin }
    }
    pub fn read(&self) -> u16 {
        // TODO: How to choose which PIN to read?

        cortex_m::interrupt::free(|cs| {
            let adc = ADC.borrow(cs);

            // Active ADC and Start Conversion
            adc.cr.write(|w| w.aden().set_bit().adstart().set_bit());

            // Wait end of conversation
            while adc.isr.read().eoc().bit_is_clear() {}

            // Return result
            adc.dr.read().data().bits()
        })
    }
}

pub fn setup_pin(pin: &Pin) {
    cortex_m::interrupt::free(|cs| {
        let rcc = RCC.borrow(cs);
        let gpioa = GPIOA.borrow(cs);
        let adc = ADC.borrow(cs);

        // Clock Activation PORTA
        rcc.ahbenr.modify(|_, w| w.iopaen().enabled());
        // Clock activation ADC
        rcc.apb2enr.modify(|_, w| w.adcen().enabled());

        // PA Analog Input Channel
        match *pin {
            Pin::P4 => gpioa.moder.modify(|_, w| w.moder4().analog()),
            Pin::P5 => gpioa.moder.modify(|_, w| w.moder5().analog()),
        }

        // ADC Channel selection
        match *pin {
            Pin::P4 => adc.chselr.modify(|_, w| w.chsel4().set_bit()),
            Pin::P5 => adc.chselr.modify(|_, w| w.chsel5().set_bit()),
        }
    });
}
