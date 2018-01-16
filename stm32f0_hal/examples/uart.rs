#![no_std]

extern crate cortex_m;
extern crate stm32f0_hal as hal;
#[macro_use(interrupt)]
extern crate stm32f0x2;

use hal::{rcc, uart};

static mut UART1: Option<uart::Uart> = None;
static mut DATA_UART1: u8 = 0;

fn main() {
    let uart3 = uart::Uart::setup(
        uart::Uarts::Uart3,
        57600,
        uart::NBits::_8bits,
        uart::StopBits::_1b,
        uart::Parity::None,
    );
    unsafe {
        UART1 = Some(uart::Uart::setup(
            uart::Uarts::Uart1,
            57600,
            uart::NBits::_8bits,
            uart::StopBits::_1b,
            uart::Parity::None,
        ));
    }
    rcc::init(); // Full Speed 48Mhz
    loop {
        if uart3.transmit_complete() {
            uart3.send(0x55);
        }
    }
}

interrupt!(USART1, receive_uart1);

fn receive_uart1() {
    unsafe {
        if let Some(ref mut uart) = UART1 {
            DATA_UART1 = uart.read_data().unwrap();
        }
    }
}
