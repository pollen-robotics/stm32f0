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
    // and an TDK-InvenSense Motion Sensor Universal Evaluation Board (UEVB)
    // with an MPU-9250. It assumes that the UEVB is connected to the discovery
    // board in the following way:
    // - GND to GND, VIN to 3V.
    // - AD0 to GND (sets the LSB of the 7-bit address to 0)
    // - SCL to PB6, SDA to PB7

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

        // This is the address of the MPU-9250, is AD0 is connected to GND, as
        // specified above. If it is connected to VIN instead, the LSB must be
        // 1.
        let mpu9250_address = 0b1101000 << 1;

        // WHOAMI register
        if let Err(_) = i2c.write(mpu9250_address, &[117]) {
            led_error.high();
            loop {}
        }
        let mut buffer = [0u8; 1];
        if let Err(_) = i2c.read(mpu9250_address, &mut buffer) {
            led_error.high();
            loop {}
        }

        // Check received data
        // The WHOAMI register should always return 0x71.
        if buffer[0] == 0x71 {
            led_success.high();
        } else {
            led_error.high();
        }
    });
}
