use cortex_m::peripheral::SYST;
use cortex_m::peripheral::syst::SystClkSource;

use hal::blocking::delay::{DelayMs, DelayUs};
use rcc::Clocks;

/// System timer (`SysTick`) as a delay provider
pub struct Delay {
    clocks: Clocks,
    syst: SYST,
}

impl Delay {
    /// Configures the system timer (`SysTick`) as a delay provider
    pub fn new(mut syst: SYST, clocks: Clocks) -> Self {
        syst.set_clock_source(SystClkSource::Core);

        Delay { syst, clocks }
    }

    /// Releases the system timer (`SysTick`) resource
    pub fn free(self) -> SYST {
        self.syst
    }
}

impl DelayMs<u32> for Delay {
    fn delay_ms(&mut self, ms: u32) {
        self.delay_us(ms * 1_000);
    }
}

impl DelayMs<u16> for Delay {
    fn delay_ms(&mut self, ms: u16) {
        self.delay_ms(u32::from(ms));
    }
}

impl DelayMs<u8> for Delay {
    fn delay_ms(&mut self, ms: u8) {
        self.delay_ms(u32::from(ms));
    }
}

impl DelayUs<u32> for Delay {
    fn delay_us(&mut self, us: u32) {
        let rvr = us * ((self.clocks.sysclk().0 / 1_000_000) - 1);
        assert!(rvr < (1 << 24));

        self.syst.set_reload(rvr);
        self.syst.clear_current();
        self.syst.enable_counter();

        while !self.syst.has_wrapped() {}

        self.syst.disable_counter();
    }
}

impl DelayUs<u16> for Delay {
    fn delay_us(&mut self, us: u16) {
        self.delay_us(u32::from(us));
    }
}

impl DelayUs<u8> for Delay {
    fn delay_us(&mut self, us: u8) {
        self.delay_us(u32::from(us));
    }
}
