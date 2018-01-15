#![no_std]

extern crate stm32f0_hal as hal;

use hal::pwm;

fn main() {
    pwm::init(10000);
    pwm::set_duty(750);
    pwm::enable();

    loop {}
}
