#![no_std]

extern crate stm32f0_hal as hal;

use hal::servo;
use hal::rcc;

fn main() {
    rcc::init();
    let servo1 = servo::Servo::init(servo::Pin::PB4);
    servo1.set_position(0.0);
    loop {
        // increase position of 0.5° each 20 ms
        for i in 0..360 {
            servo1.set_position(i as f32 / 2.0);
            rcc::ms_delay(20);
        }
        // decrease position of 0.5° each 20 ms
        for i in 0..360 {
            servo1.set_position(180.0 - (i as f32 / 2.0));
            rcc::ms_delay(20);
        }
    }
}
