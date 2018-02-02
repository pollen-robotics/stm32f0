#![no_std]

extern crate cortex_m;

extern crate stm32f0_hal;
use stm32f0_hal::stm32f0x2;
use stm32f0_hal::timer::Timer;

use stm32f0_hal::rcc::RccExt;
use stm32f0_hal::time::U32Ext;

fn main() {
    let p = stm32f0x2::Peripherals::take().unwrap();
    let mut rcc = p.RCC.constrain();

    let timer = Timer::tim7(p.TIM7, 1000_u32.hz(), &mut rcc.apb1);
    // allumer une led quand on appuie sur un bouton
    // + timer qui va l'eteindre au bout de 1000 ms

    loop {}
}
