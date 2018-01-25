#![no_std]

extern crate stm32f0_hal as hal;

use hal::pwm;
use hal::rcc;

fn main() {
    rcc::init();
    let channel1 = pwm::Pwm::init(pwm::Pin::PC6);
    let channel2 = pwm::Pwm::init(pwm::Pin::PC7);
    let channel3 = pwm::Pwm::init(pwm::Pin::PC8);
    let channel4 = pwm::Pwm::init(pwm::Pin::PC9);
    channel1.set_frequency(49);
    channel1.set_duty(10);
    channel2.set_duty(20);
    channel3.set_duty(30);
    channel4.set_duty(40);
    channel1.enable();
    channel2.enable();
    channel3.enable();
    channel4.enable();
    loop {}
}
