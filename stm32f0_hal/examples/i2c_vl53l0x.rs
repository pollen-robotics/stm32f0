#![no_std]

extern crate cortex_m;
extern crate embedded_hal;
extern crate stm32f0_hal;
extern crate stm32f0x2;

use embedded_hal::prelude::*;
use stm32f0x2::{I2C1, GPIOB, RCC};
use stm32f0_hal::gpio;
use stm32f0_hal::i2c::I2C;

fn main() {
    // This example was developed and tested with the STM32F072B Discovery board
    // and the VL53L0X sattelite board. It assumes that the VL53L0X satellite
    // board is connected to the discovery board in the following way:
    // - GND to GND, VDD to 3V.
    // - XSDN_I to VDD, via a 10kOhm pull-up resistor.
    // - SCL_I to PB6, SDA_I to PB7.

    cortex_m::interrupt::free(|cs| {
        let rcc = RCC.borrow(cs);
        let gpiob = GPIOB.borrow(cs);
        let i2c = I2C1.borrow(cs);

        let mut i2c = I2C::init(rcc, gpiob, i2c);

        // Set up LEDs that we can later use to signal error or success
        //
        // These LEDs are marked as LD3, LD4, and LD5 on the STM32F0 Discovery
        // board. Please note that the board's documentation that matches those
        // names to PC6, PC8, and PC9 is not correct. It also mixes up the
        // colors of the LEDs. At least that's the case with the board I have in
        // front of me.
        let mut led_init = gpio::Output::setup(gpio::Pin::PC8); // orange
        let mut led_success = gpio::Output::setup(gpio::Pin::PC9); // green
        let mut led_error = gpio::Output::setup(gpio::Pin::PC6); // red

        led_init.high();
        led_success.low();
        led_error.low();

        let vl53l0x_address = 0x52;

        if let Err(_) = i2c.write(vl53l0x_address, &[0xC0]) {
            led_error.high();
            loop {}
        }
        let mut buffer = [0u8; 1];
        if let Err(_) = i2c.read(vl53l0x_address, &mut buffer) {
            led_error.high();
            loop {}
        }

        // Check received data
        // We expect that it's the hardcoded value from the reference register
        // we were reading.
        if buffer[0] == 0xEE {
            led_success.high();
        } else {
            led_error.high();
        }
    });
}
