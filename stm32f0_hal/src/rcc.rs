use stm32f0x2::{rcc, RCC};

use flash::ACR;
use time::Hertz;

/// Extension trait that constrains the `RCC` peripheral
pub trait RccExt {
    /// Constrains the `RCC` peripheral so it plays nicely with the other abstractions
    fn constrain(self) -> Rcc;
}

impl RccExt for RCC {
    fn constrain(self) -> Rcc {
        Rcc {
            ahb: AHB { _0: () },
            apb1: APB1 { _0: () },
            apb2: APB2 { _0: () },
            cfgr: CFGR {
                hclk: None,
                pclk: None,
                sysclk: None,
            },
        }
    }
}

/// Constrained RCC peripheral
pub struct Rcc {
    /// AMBA High-performance Bus (AHB) registers
    pub ahb: AHB,

    pub apb1: APB1,
    pub apb2: APB2,

    pub cfgr: CFGR,
}

/// AMBA High-performance Bus (AHB) registers
pub struct AHB {
    _0: (),
}

impl AHB {
    pub(crate) fn enr(&mut self) -> &rcc::AHBENR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).ahbenr }
    }
    pub(crate) fn rstr(&mut self) -> &rcc::AHBRSTR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).ahbrstr }
    }
}

pub struct APB1 {
    _0: (),
}

impl APB1 {
    pub(crate) fn enr(&mut self) -> &rcc::APB1ENR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).apb1enr }
    }
    pub(crate) fn rstr(&mut self) -> &rcc::APB1RSTR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).apb1rstr }
    }
}

pub struct APB2 {
    _0: (),
}

impl APB2 {
    pub(crate) fn enr(&mut self) -> &rcc::APB2ENR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).apb2enr }
    }
    // pub(crate) fn rstr(&mut self) -> &rcc::APB2RSTR {
    //     // NOTE(unsafe) this proxy grants exclusive access to this register
    //     unsafe { &(*RCC::ptr()).apb2rstr }
    // }
}

pub struct CFGR {
    hclk: Option<u32>,
    pclk: Option<u32>,
    sysclk: Option<u32>,
}

impl CFGR {
    pub fn hclk<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.hclk = Some(freq.into().0);
        self
    }

    pub fn pclk<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.pclk = Some(freq.into().0);
        self
    }
    pub fn sysclk<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.sysclk = Some(freq.into().0);
        self
    }
    pub fn freeze(self, acr: &mut ACR) -> Clocks {
        const HSI: u32 = 8_000_000; // Hz

        acr.acr().write(|w| w.latency()._1wait_state());

        let rcc = unsafe { &*RCC::ptr() };
        rcc.cr.modify(|_, w| w.hsion().enabled());

        // Waiting for HSI to be ready
        while rcc.cr.read().hsirdy().bit_is_clear() {}

        // Set HSI calibration
        // Advised trim step value according to temperature and voltage
        const TRIM_STEP: u8 = 15;
        rcc.cr.write(|w| w.hsitrim().bits(TRIM_STEP));

        // Disable PLL
        rcc.cr.modify(|_, w| w.pllon().clear_bit());
        // Waiting for PLL to be Off
        while rcc.cr.read().pllrdy().bit_is_set() {}

        // Configure PLL source clock pre-divider to 1
        rcc.cfgr2.write(|w| w.prediv().div1());

        // Configure PLL source clock to HSI and multiplier to PLLMUL
        const PLLMUL: u32 = 6;
        const SYSCLK: u32 = HSI * PLLMUL;
        rcc.cfgr.write(|w| w.pllsrc().hsi().pllmul().mul6());

        // Enable PLL
        rcc.cr.modify(|_, w| w.pllon().set_bit());
        // Waiting for PLL to be On
        while rcc.cr.read().pllrdy().bit_is_clear() {}

        // Set AHB & APB divider to 1 -> 48MHz
        const HPRE: u8 = 1;
        const HCLK: u32 = SYSCLK / HPRE as u32;
        const PPRE: u8 = 1;
        const PCLK: u32 = SYSCLK / PPRE as u32;
        rcc.cfgr.modify(|_, w| w.hpre().div1().ppre().div1());

        // Select PLL output as system clock
        rcc.cfgr.modify(|_, w| w.sw().pll());
        // Waiting for system clock to be ready
        while rcc.cfgr.read().sws().bits() != 2 {}

        // Warning: Configure SysTick is done in delay

        Clocks {
            hclk: Hertz(HCLK),
            // hpre: HPRE,
            pclk: Hertz(PCLK),
            // ppre: PPRE,
            sysclk: Hertz(SYSCLK),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Clocks {
    hclk: Hertz,
    // hpre: u8,
    pclk: Hertz,
    // ppre: u8,
    sysclk: Hertz,
}

impl Clocks {
    /// Returns the frequency of the AHB
    pub fn hclk(&self) -> Hertz {
        self.hclk
    }

    // // Returns the HCLK pre-scalar
    // pub(crate) fn hpre(&self) -> u8 {
    //     self.hpre
    // }

    /// Returns the frequency of the APB
    pub fn pclk(&self) -> Hertz {
        self.pclk
    }

    // // Returns the PCLK pre-scalar
    // pub(crate) fn ppre(&self) -> u8 {
    //     self.ppre
    // }

    /// Returns the system (core) frequency
    pub fn sysclk(&self) -> Hertz {
        self.sysclk
    }
}
