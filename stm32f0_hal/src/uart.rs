use cortex_m;
use stm32f0x2::{USART1 as UART1, USART3 as UART3, GPIOA, GPIOB, NVIC, RCC};
use stm32f0x2::interrupt::Interrupt::{USART1, USART3_4};

const FREQUENCY: u32 = 48000000;

pub enum Uarts {
    Uart1,
    Uart3,
}

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

pub struct Uart {
    uart: Uarts,
}

impl Uart {
    pub fn setup(
        uart: Uarts,
        baudrate: u32,
        nbits: NBits,
        nbstopbits: StopBits,
        parity: Parity,
    ) -> Uart {
        match uart {
            Uarts::Uart1 => {
                cortex_m::interrupt::free(|cs| {
                    let rcc = RCC.borrow(cs);
                    let nvic = NVIC.borrow(cs);
                    let gpioa = GPIOA.borrow(cs);
                    let uart1 = UART1.borrow(cs);
<<<<<<< HEAD
                    // Enable GPIOA Clock into the Advanced High-performance Bus
                    rcc.ahbenr.modify(|_, w| w.iopaen().enabled());
                    // Enable USART1 Clock  into the Advanced Peripheral Bus 2
=======
                    // Enable GPIOA Clock
                    rcc.ahbenr.modify(|_, w| w.iopaen().enabled());
                    // Enable USART1 Clock
>>>>>>> 2aa95150cba752257a5ea068937f32a1a769acab
                    rcc.apb2enr.modify(|_, w| w.usart1en().enabled());

                    // Configure speed of PA9/PA10 (refer to the datasheet for the frequency, the power supply and load conditions)
                    gpioa
                        .ospeedr
                        .modify(|_, w| w.ospeedr9().high_speed().ospeedr10().high_speed());
<<<<<<< HEAD
                    // Use pull-up on PA9/10
                    gpioa
                        .pupdr
                        .modify(|_, w| w.pupdr9().pull_up().pupdr10().pull_up());
                    // Configure PA9/PA10 Alternate Function 1 -> USART1 (PA9 -> TX & PA10 -> RX )
                    gpioa.afrh.modify(|_, w| w.afrh9().af1().afrh10().af1());
                    // PA9 & PA10 as Alternate function (USART1 peripheral)
                    gpioa
                        .moder
                        .modify(|_, w| w.moder9().alternate().moder10().alternate());
                    // PA9 & PA10 push-pull configuration
=======
                    gpioa
                        .pupdr
                        .modify(|_, w| w.pupdr9().pull_up().pupdr10().pull_up());
                    gpioa.afrh.modify(|_, w| w.afrh9().af1().afrh10().af1());
                    gpioa
                        .moder
                        .modify(|_, w| w.moder9().alternate().moder10().alternate());
>>>>>>> 2aa95150cba752257a5ea068937f32a1a769acab
                    gpioa
                        .otyper
                        .modify(|_, w| w.ot9().push_pull().ot10().push_pull());

                    // Configure UART1 : Word length
                    match nbits {
                        NBits::_8bits => {
                            uart1.cr1.modify(|_, w| w.m()._8bits());
                        }
                        NBits::_9bits => {
                            uart1.cr1.modify(|_, w| w.m()._9bits());
                        }
                    }
                    // Configure UART1 : Parity
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
                    // Configure UART1 : Transfert Direction - Oversampling - RX Interrupt
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
                    // Configure UART1 : stop bit
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

<<<<<<< HEAD
                    // Configure UART1 : disable hardware flow control - Overrun interrupt
=======
                    // Configure UART : disable hardware flow control - Overrun interrupt
>>>>>>> 2aa95150cba752257a5ea068937f32a1a769acab
                    uart1.cr3.modify(|_, w| {
                        w.rtse()
                            .disabled()
                            .ctse()
                            .disabled()
                            .ctsie()
                            .disabled()
                            .ovrdis()
                            .disabled()
                    });
                    // Configure UART1 : baudrate
                    uart1.brr.modify(|_, w| {
                        w.div_fraction()
                            .bits((FREQUENCY / (baudrate / 2)) as u8 & 0x0F)
                    });
                    uart1.brr.modify(|_, w| {
                        w.div_mantissa()
                            .bits(((FREQUENCY / (baudrate / 2)) >> 4) as u16)
                    });
                    // Configure UART1 : Asynchronous mode
                    uart1
                        .cr2
                        .modify(|_, w| w.linen().disabled().clken().disabled());
                    // UART1 enabled
                    uart1.cr1.modify(|_, w| w.ue().enabled());
                    // Interrupt UART1 activated into NVIC
                    nvic.enable(USART1);
                    nvic.clear_pending(USART1);
                });
                Uart { uart }
            }
            Uarts::Uart3 => {
                cortex_m::interrupt::free(|cs| {
                    let nvic = NVIC.borrow(cs);
                    let gpiob = GPIOB.borrow(cs);
                    let uart3 = UART3.borrow(cs);
<<<<<<< HEAD
                    // Enable GPIOB Clock into the Advanced High-performance Bus
                    rcc.ahbenr.modify(|_, w| w.iopben().enabled());
                    // Enable USART3 Clock  into the Advanced Peripheral Bus 2
=======
                    let rcc = RCC.borrow(cs);
                    // Enable GPIOA Clock
                    rcc.ahbenr.modify(|_, w| w.iopben().enabled());
                    // Enable USART1 Clock
>>>>>>> 2aa95150cba752257a5ea068937f32a1a769acab
                    rcc.apb1enr.modify(|_, w| w.usart3en().enabled());

                    // Configure speed of PB10/PB11 (refer to the datasheet for the frequency, the power supply and load conditions)
                    gpiob
                        .ospeedr
                        .modify(|_, w| w.ospeedr10().high_speed().ospeedr11().high_speed());
<<<<<<< HEAD
                    // Use pull-up on PB10/11
                    gpiob
                        .pupdr
                        .modify(|_, w| w.pupdr10().pull_up().pupdr11().pull_up());
                    // Configure PB10/PB11 Alternate Function 1 -> USART3 (PB10 -> TX & PB11 -> RX )
                    gpiob.afrh.modify(|_, w| w.afrh10().af4().afrh11().af4());
                    // PA9 & PA10 as Alternate function (USART3 Peripheral)
                    gpiob
                        .moder
                        .modify(|_, w| w.moder10().alternate().moder11().alternate());
                    // PA9 & PA10 push-pull configuration
=======
                    gpiob
                        .pupdr
                        .modify(|_, w| w.pupdr10().pull_up().pupdr11().pull_up());
                    gpiob.afrh.modify(|_, w| w.afrh10().af4().afrh11().af4());
                    gpiob
                        .moder
                        .modify(|_, w| w.moder10().alternate().moder11().alternate());
>>>>>>> 2aa95150cba752257a5ea068937f32a1a769acab
                    gpiob
                        .otyper
                        .modify(|_, w| w.ot10().push_pull().ot11().push_pull());

                    // Configure UART : Word length
                    match nbits {
                        NBits::_8bits => {
                            uart3.cr1.modify(|_, w| w.m()._8bits());
                        }
                        NBits::_9bits => {
                            uart3.cr1.modify(|_, w| w.m()._9bits());
                        }
                    }
                    // Configure UART3 : Parity
                    match parity {
                        Parity::None => {
                            uart3.cr1.modify(|_, w| w.pce().disabled());
                        }
                        Parity::Even => {
                            uart3.cr1.modify(|_, w| w.pce().enabled());
                            uart3.cr1.modify(|_, w| w.ps().even());
                        }
                        Parity::Odd => {
                            uart3.cr1.modify(|_, w| w.pce().enabled());
                            uart3.cr1.modify(|_, w| w.ps().odd());
                        }
                    }
                    // Configure UART3 : Transfert Direction - Oversampling - RX Interrupt
                    uart3.cr1.modify(|_, w| {
                        w.te()
                            .enabled()
                            .re()
                            .enabled()
                            .over8()
                            .over8()
                            .rxneie()
                            .enabled()
                    });
                    // Configure UART3 : 1 stop bit
                    match nbstopbits {
                        StopBits::_0b5 => {
                            uart3.cr2.modify(|_, w| w.stop().half_stop());
                        }
                        StopBits::_1b => {
                            uart3.cr2.modify(|_, w| w.stop()._1stop());
                        }
                        StopBits::_1b5 => {
                            uart3.cr2.modify(|_, w| w.stop()._1half_stop());
                        }
                        StopBits::_2b => {
                            uart3.cr2.modify(|_, w| w.stop()._2stop());
                        }
                    }

<<<<<<< HEAD
                    // Configure UART3 : disable hardware flow control - Overrun interrupt
=======
                    // Configure UART : disable hardware flow control - Overrun interrupt
>>>>>>> 2aa95150cba752257a5ea068937f32a1a769acab
                    uart3.cr3.modify(|_, w| {
                        w.rtse()
                            .disabled()
                            .ctse()
                            .disabled()
                            .ctsie()
                            .disabled()
                            .ovrdis()
                            .disabled()
                    });
                    // Configure UART3 : baudrate
                    uart3.brr.modify(|_, w| {
                        w.div_fraction()
                            .bits((FREQUENCY / (baudrate / 2)) as u8 & 0x0F)
                    });
                    uart3.brr.modify(|_, w| {
                        w.div_mantissa()
                            .bits(((FREQUENCY / (baudrate / 2)) >> 4) as u16)
                    });
                    // Configure UART3 : Asynchronous mode
                    uart3
                        .cr2
                        .modify(|_, w| w.linen().disabled().clken().disabled());
                    // UART3 enabled
                    uart3.cr1.modify(|_, w| w.ue().enabled());
                    // Interrupt UART3 (and USART4) activated into NVIC
                    nvic.enable(USART3_4);
                    nvic.clear_pending(USART3_4);
                });
                Uart { uart }
            }
        }
    }

    pub fn send(&self, byte: u8) {
        match self.uart {
            Uarts::Uart1 => cortex_m::interrupt::free(|cs| {
                let uart1 = UART1.borrow(cs);
<<<<<<< HEAD
                // the byte is stored in the UART1 transmit register
                uart1.tdr.write(|w| w.tdr().bits(byte as u16));
            }),
            Uarts::Uart3 => cortex_m::interrupt::free(|cs| {
                let uart3 = UART3.borrow(cs);
                // the byte is stored in the UART3 transmit register
                uart3.tdr.write(|w| w.tdr().bits(byte as u16));
=======
                uart1.tdr.modify(|_, w| w.tdr().bits(byte as u16));
            }),
            Uarts::Uart3 => cortex_m::interrupt::free(|cs| {
                let uart3 = UART3.borrow(cs);
                uart3.tdr.modify(|_, w| w.tdr().bits(byte as u16));
>>>>>>> 2aa95150cba752257a5ea068937f32a1a769acab
            }),
        }
    }

    pub fn transmit_complete(&self) -> bool {
        match self.uart {
            Uarts::Uart1 => cortex_m::interrupt::free(|cs| {
                let uart1 = UART1.borrow(cs);
                // Check the Transmit Completed flag status
                if uart1.isr.read().tc().bit_is_set() {
<<<<<<< HEAD
                    // if transmit completed, clear the transmit completed flag and return true
                    uart1.icr.write(|w| w.tccf().clear_bit());
=======
                    uart1.icr.modify(|_, w| w.tccf().clear_bit());
>>>>>>> 2aa95150cba752257a5ea068937f32a1a769acab
                    true
                } else {
                    false
                }
            }),
            Uarts::Uart3 => cortex_m::interrupt::free(|cs| {
                let uart3 = UART3.borrow(cs);
                // Check the Transmit Completed flag status
                if uart3.isr.read().tc().bit_is_set() {
<<<<<<< HEAD
                    // if transmit completed, clear the transmit completed flag and return true
                    uart3.icr.write(|w| w.tccf().clear_bit());
=======
                    uart3.icr.modify(|_, w| w.tccf().clear_bit());
>>>>>>> 2aa95150cba752257a5ea068937f32a1a769acab
                    true
                } else {
                    false
                }
            }),
        }
    }

    pub fn read_data(&self) -> Option<u8> {
        match self.uart {
            Uarts::Uart1 => cortex_m::interrupt::free(|cs| {
                let uart1 = UART1.borrow(cs);
                // Check the Receive Not Empty flag flag status
                if uart1.isr.read().rxne().bit_is_set() {
                    // if true, read the received data and finish the rxne flag cleared procedure
                    Some(uart1.rdr.read().rdr().bits() as u8)
                } else {
                    None
                }
            }),
            Uarts::Uart3 => cortex_m::interrupt::free(|cs| {
                let uart3 = UART3.borrow(cs);
                // Check the Receive Not Empty flag flag status
                if uart3.isr.read().rxne().bit_is_set() {
                    // if true, read the received data and finish the rxne flag cleared procedure
                    Some(uart3.rdr.read().rdr().bits() as u8)
                } else {
                    None
                }
            }),
        }
    }
}
