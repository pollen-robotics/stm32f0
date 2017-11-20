use cortex_m;
use stm32f0x2::{USART1 as UART1, GPIOA, NVIC, RCC};
use stm32f0x2::interrupt::*;

const FREQUENCY: u32 = 48000000;

static mut DATA_UART1: u16 = 0;

pub enum NBits {
    _8bits,
    _9bits,
}

pub enum StopBits {
    _0b5,
    _1b,
    _1b5,
    _2b,
}

pub enum Parity {
    /// No Parity
    None,
    /// Even Parity
    Even,
    /// Odd Parity
    Odd,
}

interrupt!(USART1, receive);

pub fn init(baudrate: u32, nbits: NBits, nbstopbits: StopBits, parity: Parity) {
    cortex_m::interrupt::free(|cs| {
        let rcc = RCC.borrow(cs);
        let gpioa = GPIOA.borrow(cs);
        let uart1 = UART1.borrow(cs);
        let nvic = NVIC.borrow(cs);

        // Enable GPIOA Clock
        rcc.ahbenr.write(|w| w.iopaen().enabled());
        // Enable USART1 Clock
        rcc.apb2enr.write(|w| w.usart1en().enabled());

        // Configure PA9/PA10 Alternate Function 1 -> USART1
        gpioa
            .ospeedr
            .write(|w| w.ospeedr9().high_speed().ospeedr10().high_speed());
        gpioa
            .pupdr
            .write(|w| w.pupdr9().pull_up().pupdr10().pull_up());
        gpioa.afrh.write(|w| w.afrh9().af1().afrh10().af1());
        gpioa
            .moder
            .write(|w| w.moder9().alternate().moder10().alternate());
        gpioa
            .otyper
            .write(|w| w.ot9().push_pull().ot10().push_pull());

        // Configure UART : Word length
        match nbits {
            NBits::_8bits => {
                uart1.cr1.modify(|_, w| w.m()._8bits());
            }
            NBits::_9bits => {
                uart1.cr1.modify(|_, w| w.m()._9bits());
            }
        }
        // Configure UART : Parity
        match parity {
            Parity::None => {
                uart1.cr1.modify(|_, w| w.pce().disabled());
            }
            Parity::Even => {
                uart1.cr1.modify(|_, w| w.pce().enabled());
                uart1.cr1.modify(|_, w| w.ps().even());
            }
            Parity::Odd => {
                uart1.cr1.modify(|_, w| w.pce().enabled());
                uart1.cr1.modify(|_, w| w.ps().odd());
            }
        }
        // Configure UART : Transfert Direction - Oversampling - RX Interrupt
        uart1.cr1.modify(|_, w| {
            w.te()
                .enabled()
                .re()
                .enabled()
                .over8()
                .over8()
                .rxneie()
                .enabled()
        });
        // Configure UART : 1 stop bit
        match nbstopbits {
            StopBits::_0b5 => {
                uart1.cr2.modify(|_, w| w.stop().half_stop());
            }
            StopBits::_1b => {
                uart1.cr2.modify(|_, w| w.stop()._1stop());
            }
            StopBits::_1b5 => {
                uart1.cr2.modify(|_, w| w.stop()._1half_stop());
            }
            StopBits::_2b => {
                uart1.cr2.modify(|_, w| w.stop()._2stop());
            }
        }

        // Configure UART : disable hardware flow control - Overrun interrupt
        uart1.cr3.write(|w| {
            w.rtse()
                .disabled()
                .ctse()
                .disabled()
                .ctsie()
                .disabled()
                .ovrdis()
                .disabled()
        });
        // Configure UART : baudrate
        uart1.brr.write(|w| {
            w.div_fraction()
                .bits((FREQUENCY / (baudrate / 2)) as u8 & 0x0F)
        });
        uart1.brr.write(|w| {
            w.div_mantissa()
                .bits(((FREQUENCY / (baudrate / 2)) >> 4) as u16)
        });
        // Configure UART : Asynchronous mode
        uart1
            .cr2
            .modify(|_, w| w.linen().disabled().clken().disabled());
        // UART1 enabled
        uart1.cr1.modify(|_, w| w.ue().enabled());
        nvic.enable(Interrupt::USART1);
        nvic.clear_pending(Interrupt::USART1);
    })
}

pub fn send(byte: u8) {
    cortex_m::interrupt::free(|cs| {
        let uart1 = UART1.borrow(cs);
        uart1.tdr.write(|w| w.tdr().bits(byte as u16));
    })
}

pub fn transmit_complete() -> bool {
    cortex_m::interrupt::free(|cs| {
        let uart1 = UART1.borrow(cs);
        if uart1.isr.read().tc().bit_is_set() {
            uart1.icr.write(|w| w.tccf().clear_bit());
            true
        } else {
            false
        }
    })
}

fn receive_callback() {
    cortex_m::interrupt::free(|cs| {
        let uart = UART1.borrow(cs);
        unsafe {
            DATA_UART1 = uart.rdr.read().rdr().bits();
        }
    })
}




pub fn receive() {
    cortex_m::interrupt::free(|cs| {
        let uart = UART1.borrow(cs);
        if uart.isr.read().rxne().bit_is_set() {
            receive_callback();
        }
    })
}
