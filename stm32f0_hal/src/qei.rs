use cortex_m;
use stm32f0x2::{TIM3 as TIMER3, TIM7 as TIMER7, RCC, GPIOB, GPIOC,  NVIC, USART3 as UART3};
use stm32f0x2::interrupt::*;
use core::ptr;

use alloc;
use core;

const FREQUENCY : u32 = 48000000;
const STEP : u16 = 2249;
const DEGREEBYSTEP : f32 = (360 as f32 /STEP as f32);

static mut DEGREE: f32 = 0.0;
static mut PREVIOUS_DEGREE : f32 = 0.0;
static mut DELTA_DEGREE : f32 = 0.0;

pub fn init() {
    cortex_m::interrupt::free(|cs| {
        let gpio = GPIOB.borrow(cs);
        let rcc = RCC.borrow(cs);
        let timer = TIMER3.borrow(cs);
        let nvic = NVIC.borrow(cs);

        // QEI on PTB4 (TIM3_CH1) and PTB5 (TIM3_CH2)
        rcc.ahbenr.modify(|_, w| w.iopben().enabled());
        rcc.apb1enr.modify(|_, w| w.tim3en().enabled());
        gpio.pupdr.write(|w| w
            .pupdr4().pull_up()
            .pupdr5().pull_up());
        gpio.afrl.write(|w| w
            .afrl4().af1()
            .afrl5().af1());
        gpio.moder.modify(|_, w| w
            .moder4().alternate()
            .moder5().alternate());
        // QEI Mode
        timer.smcr.write(|w| w.sms().encoder_ti1ti2());

        timer.ccmr1_output.write(|w| w.cc1s().ic1mapped_ti1());
        timer.ccmr1_output.write(|w| w.cc2s().ic2mapped_ti1());

        // Quadrature encoder max. step
        timer.arr.write(|w| w.arr_h().bits(0));
        timer.arr.write(|w| w.arr_l().bits(1));

        timer.cr1.modify(|_,w| w.arpe().buffered());
        timer.cr1.write(|w| w.cen().enabled());

        // Enable interrupt
        timer.dier.modify(|_, w| w.uie().enabled());
        // Interrupt activated
        nvic.enable(Interrupt::TIM3);
        nvic.clear_pending(Interrupt::TIM3);
    });
}

pub fn setup_debug(baudrate: u32) {
    cortex_m::interrupt::free(|cs| {
        let rcc = RCC.borrow(cs);
        let gpiob = GPIOB.borrow(cs);
        let uart = UART3.borrow(cs);

        // Enable GPIOB Clock
        rcc.ahbenr.modify(|_, w| w.iopben().enabled());
        // Enable USART3 Clock
        rcc.apb1enr.modify(|_, w| w.usart3en().enabled());
        // Configure PB10/PB11 Alternate Function 1 -> USART3
        gpiob
            .ospeedr
            .modify(|_, w| w.ospeedr10().high_speed().ospeedr11().high_speed());
        gpiob
            .pupdr
            .modify(|_, w| w.pupdr10().pull_up().pupdr11().pull_up());
        gpiob.afrh.modify(|_, w| w.afrh10().af4().afrh11().af4());
        gpiob
            .moder
            .modify(|_, w| w.moder10().alternate().moder11().alternate());
        gpiob
            .otyper
            .modify(|_, w| w.ot10().push_pull().ot11().push_pull());

        // Configure UART : Word length
        uart.cr1.modify(|_, w| w.m()._8bits());
        // Configure UART : Parity
        uart.cr1.modify(|_, w| w.pce().disabled());
        // Configure UART : Transfert Direction - Oversampling - RX Interrupt
        uart.cr1.modify(|_, w| {
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
        uart.cr2.modify(|_, w| w.stop()._1stop());

        // Configure UART : disable hardware flow control - Overrun interrupt
        uart.cr3.write(|w| {
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
        uart.brr.write(|w| {
            w.div_fraction()
                .bits((FREQUENCY / (baudrate / 2)) as u8 & 0x0F)
        });
        uart.brr.write(|w| {
            w.div_mantissa()
                .bits(((FREQUENCY / (baudrate / 2)) >> 4) as u16)
        });
        // Configure UART3 : Asynchronous mode
        uart.cr2
            .modify(|_, w| w.linen().disabled().clken().disabled());
        // UART3 enabled
        uart.cr1.modify(|_, w| w.ue().enabled());
    });
}

pub fn dt_setup(interval_us: u16) {
    cortex_m::interrupt::free(|cs| {
        let rcc = RCC.borrow(cs);
        let timer = TIMER7.borrow(cs);
        let nvic = NVIC.borrow(cs);
        let gpio = GPIOC.borrow(cs);

        // LED Test
        rcc.ahbenr.modify(|_, w| w.iopcen().enabled());
        gpio.moder.modify(|_, w| w.moder7().output());

        //Enable TIM7 clock
        rcc.apb1enr.modify(|_, w| w.tim7en().enabled());

        // configure Time Out
        // Set Prescaler Register - 16 bits
        timer.psc.modify(|_, w| w.psc().bits(47));
        // Set Auto-Reload register - 16 bits
        timer.arr.modify(|_, w| w.arr().bits(interval_us - 1));

        timer.cr1.modify(|_, w| w.opm().continuous());
        // Enable interrupt
        timer.dier.modify(|_, w| w.uie().enabled());
        // Interrupt activated
        nvic.enable(Interrupt::TIM7);
        nvic.clear_pending(Interrupt::TIM7);
    });
}

pub fn dt_resume() {
    cortex_m::interrupt::free(|cs| {
        let timer = TIMER7.borrow(cs);
        // Enable counter
        timer.cr1.modify(|_, w| w.cen().enabled());
    });
}

interrupt!(TIM3, step);
interrupt!(TIM7, speed);

// INTERRUPT CALL BACK
fn speed(){
    cortex_m::interrupt::free(|cs| {
        let timer = TIMER7.borrow(cs);
        timer.sr.write(| w| w.uif().clear_bit());
        unsafe {
            DELTA_DEGREE = DEGREE - PREVIOUS_DEGREE;
            PREVIOUS_DEGREE = DEGREE;
        }
    });
}

fn step(){
    cortex_m::interrupt::free(|cs| {
        let timer = TIMER3.borrow(cs);
        timer.sr.write(|w| w.uif().clear_bit());
        if timer.cr1.read().dir().bit_is_clear(){
            unsafe {DEGREE=DEGREE-DEGREEBYSTEP;}
        } else {
            unsafe {DEGREE=DEGREE+DEGREEBYSTEP;}
        }
    });
}

// DEGREE
pub fn counter() -> f32{
    return unsafe { ptr::read_volatile(&DEGREE) };
}
// SPEED
pub fn get_speed() -> f32 {return unsafe { ptr::read_volatile(&DELTA_DEGREE) }; }

/// Send a byte to the UART when it's ready.
///
/// *Beware, this function will block until the UART is ready to send.*
///
/// # Arguments
///
/// * `byte` - The u8 byte to send.
pub fn debug_send_when_ready(byte: u8) {
    cortex_m::interrupt::free(|cs| {
        let uart3 = UART3.borrow(cs);
        while !debug_transmit_complete(cs) {}
        uart3.tdr.write(|w| w.tdr().bits(byte as u16));
    })
}

fn debug_transmit_complete(cs: &cortex_m::interrupt::CriticalSection) -> bool {
    let uart3 = UART3.borrow(cs);
    if uart3.isr.read().tc().bit_is_set() {
        uart3.icr.write(|w| w.tccf().clear_bit());
        true
    } else {
        false
    }
}


/// Uart Logger implementation
///
/// *It should only be used through the `log` macro.*
/// TODO: Could we find a way to hide it?
pub static mut LOGGER: UartLogger = UartLogger {};

pub struct UartLogger {}
impl core::fmt::Write for UartLogger {
    fn write_str(&mut self, s: &str) -> Result<(), alloc::fmt::Error> {
        for &b in s.as_bytes() {
            debug_send_when_ready(b);
        }
        Ok(())
    }
}

#[macro_export]
/// Log macro that sends a fmt to the debug UART.
///
/// *You must called `robus::init()` before using the macro!*
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate robus;
///
/// use std::fmt::Write;
///
/// fn main() {
///    robus::init();
///
///    let x = 42;
///    log!("x: {:?}", x);
/// }
/// ```
macro_rules! log {
    ($fmt: expr) => ({
        let mut w = unsafe { &mut $crate::qei::LOGGER };
        writeln!(&mut w, $fmt).unwrap();
    });
    ($fmt: expr, $($arg: tt)*) => ({
        let mut w = unsafe { &mut $crate::qei::LOGGER };
        writeln!(&mut w, $fmt, $($arg)*).unwrap();
    });
}