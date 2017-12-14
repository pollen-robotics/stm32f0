use cortex_m;
use stm32f0x2::{TIM2 as TIMER2, TIM3 as TIMER3, TIM7 as TIMER7, TIM16 as TIMER16, TIM17 as TIMER17, RCC, GPIOA, GPIOB, GPIOC, NVIC, USART3 as UART3};
use stm32f0x2::interrupt::*;
use core::ptr;

use alloc;
use core;

const FREQUENCY : u32 = 48000000;
const STEP1 : u16 = 2249;
const STEP2 : u16 = 2249;
const DEGREEBYSTEP1 : f32 = (360 as f32 /STEP1 as f32);
const DEGREEBYSTEP2 : f32 = (360 as f32 /STEP2 as f32);

static mut DEGREE1: f32 = 0.0;
static mut PREVIOUS_DEGREE1 : f32 = 0.0;
static mut DELTA_DEGREE1 : f32 = 0.0;

static mut DEGREE2: f32 = 0.0;
static mut PREVIOUS_DEGREE2 : f32 = 0.0;
static mut DELTA_DEGREE2 : f32 = 0.0;

pub fn setup_pwm(period : u16){
    cortex_m::interrupt::free(|cs| {
        let rcc = RCC.borrow(cs);
        let tim16 = TIMER16.borrow(cs);
        let tim17 = TIMER17.borrow(cs);
        let gpiob = GPIOB.borrow(cs);
        let gpioc = GPIOC.borrow(cs);

        // Configure AIN et BIN
        rcc.ahbenr.modify(|_,w| w.iopcen().enabled());
        gpioc.moder.modify(|_,w| w
            .moder0().output()
            .moder1().output()
            .moder2().output()
            .moder3().output()
        );


        // GPIO Clock Activated
        rcc.ahbenr.modify(|_, w| w.iopben().enabled());
        // TIMER16 Clock Activated
        rcc.apb2enr.modify(|_, w| w.tim16en().enabled());
        gpiob.moder.modify(|_, w| w.moder8().alternate());
        gpiob.afrh.modify(|_, w| w.afrh8().af2());
        gpiob.otyper.modify(|_, w| w.ot8().push_pull());

        tim16.ccmr1_output.modify(|_, w| {
            w.oc1m().pwmmode1().oc1pe().enabled()
        });
        tim16.ccer.modify(|_, w| w.cc1p().clear_bit());
        // Set Prescaler Register - 16 bits
        tim16.psc.write(|w| w.psc().bits(7));
        // Set Auto-Reload register - 16 bits
        tim16.arr.write(|w| w.arr().bits(period));
        tim16.ccer.modify(|_, w| w.cc1e().active());
        tim16.cr1.write(|w| w.cen().enabled());

        // TIMER17 Clock Activated
        rcc.apb2enr.modify(|_, w| w.tim17en().enabled());
        gpiob.moder.modify(|_, w| w.moder9().alternate());
        gpiob.afrh.modify(|_, w| w.afrh9().af2());
        gpiob.otyper.modify(|_, w| w.ot9().push_pull());

        tim17.ccmr1_output.modify(|_, w| {
            w.oc1m().pwmmode1().oc1pe().enabled()
        });
        tim17.ccer.modify(|_, w| w.cc1p().clear_bit());
        // Set Prescaler Register - 16 bits
        tim17.psc.write(|w| w.psc().bits(7));
        // Set Auto-Reload register - 16 bits
        tim17.arr.write(|w| w.arr().bits(period));
        tim17.ccer.modify(|_, w| w.cc1e().active());
        tim17.cr1.write(|w| w.cen().enabled());

        tim16.egr.write(|w| w.ug().set_bit());
        tim17.egr.write(|w| w.ug().set_bit());
    });
}

pub fn pwm_enable(){
    cortex_m::interrupt::free(|cs| {
        let tim16 = TIMER16.borrow(cs);
        let tim17 = TIMER17.borrow(cs);
        tim16.ccer.modify(|_, w| w.cc1e().set_bit());
        tim17.ccer.modify(|_, w| w.cc1e().set_bit());
    });
}

pub fn set_motor1(power : u8, dir : bool){
    cortex_m::interrupt::free(|cs| {
        let gpioc = GPIOC.borrow(cs);
        if dir {
            gpioc.bsrr.write(|w| w.bs0().set_bit());
            gpioc.bsrr.write(|w| w.br1().set_bit());
        } else {
            gpioc.bsrr.write(|w| w.br0().set_bit());
            gpioc.bsrr.write(|w| w.bs1().set_bit());
        }
    });
    set_duty_motor1(power as u16);
}

pub fn set_motor2(power : u8, dir : bool){
    cortex_m::interrupt::free(|cs| {
        let gpioc = GPIOC.borrow(cs);
        if dir {
            gpioc.bsrr.write(|w| w.bs2().set_bit());
            gpioc.bsrr.write(|w| w.br3().set_bit());
        } else {
            gpioc.bsrr.write(|w| w.br2().set_bit());
            gpioc.bsrr.write(|w| w.bs3().set_bit());
        }
    });
    set_duty_motor2(power as u16);
}

fn set_duty_motor1(duty: u16) {
    cortex_m::interrupt::free(|cs| {
        let tim16 = TIMER16.borrow(cs);
        tim16.ccr1.write(|w| w.ccr1().bits(duty));
    });
}

fn set_duty_motor2(duty: u16) {
    cortex_m::interrupt::free(|cs| {
        let tim17 = TIMER17.borrow(cs);
        tim17.ccr1.write(|w| w.ccr1().bits(duty));
    });
}

pub fn init_qei1() {
    cortex_m::interrupt::free(|cs| {
        let gpio = GPIOA.borrow(cs);
        let rcc = RCC.borrow(cs);
        let timer = TIMER2.borrow(cs);
        let nvic = NVIC.borrow(cs);

        // QEI1 on PTA0 (TIM2_CH1) and PTA1 (TIM2_CH2)
        rcc.ahbenr.modify(|_, w| w.iopaen().enabled());
        rcc.apb1enr.modify(|_, w| w.tim2en().enabled());
        gpio.pupdr.write(|w| w
            .pupdr0().pull_up()
            .pupdr1().pull_up());
        gpio.afrl.write(|w| w
            .afrl0().af2()
            .afrl1().af2());
        gpio.moder.modify(|_, w| w
            .moder0().alternate()
            .moder1().alternate());

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
        nvic.enable(Interrupt::TIM2);
        nvic.clear_pending(Interrupt::TIM2);
    });
}



pub fn init_qei2() {
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

interrupt!(TIM2, step_motor1);
interrupt!(TIM3, step_motor2);
interrupt!(TIM7, speed_motor);

// INTERRUPT CALL BACK
fn speed_motor(){
    cortex_m::interrupt::free(|cs| {
        let timer = TIMER7.borrow(cs);
        timer.sr.write(| w| w.uif().clear_bit());
        unsafe {
            DELTA_DEGREE1 = DEGREE1 - PREVIOUS_DEGREE1;
            PREVIOUS_DEGREE1 = DEGREE1;
            DELTA_DEGREE2 = DEGREE2 - PREVIOUS_DEGREE2;
            PREVIOUS_DEGREE2 = DEGREE2;
        }
    });
}

fn step_motor1(){
    cortex_m::interrupt::free(|cs| {
        let timer = TIMER2.borrow(cs);
        timer.sr.write(|w| w.uif().clear_bit());
        if timer.cr1.read().dir().bit_is_clear(){
            unsafe {DEGREE1=DEGREE1-DEGREEBYSTEP1;}
        } else {
            unsafe {DEGREE1=DEGREE1+DEGREEBYSTEP1;}
        }
    });
}

fn step_motor2(){
    cortex_m::interrupt::free(|cs| {
        let timer = TIMER3.borrow(cs);
        timer.sr.write(|w| w.uif().clear_bit());
        if timer.cr1.read().dir().bit_is_clear(){
            unsafe {DEGREE2=DEGREE2-DEGREEBYSTEP2;}
        } else {
            unsafe {DEGREE2=DEGREE2+DEGREEBYSTEP2;}
        }
    });
}

// DEGREE
pub fn counter_motor1() -> f32{
    return unsafe { ptr::read_volatile(&DEGREE1) };
}
pub fn counter_motor2() -> f32{
    return unsafe { ptr::read_volatile(&DEGREE2) };
}
// SPEED
pub fn get_speed_motor1() -> f32 {return unsafe { ptr::read_volatile(&DELTA_DEGREE1) }; }
pub fn get_speed_motor2() -> f32 {return unsafe { ptr::read_volatile(&DELTA_DEGREE2) }; }

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