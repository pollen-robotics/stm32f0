#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFGR {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `SW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWR {
    #[doc = "HSI selected as system clock"] HSI,
    #[doc = "HSE selected as system clock"] HSE,
    #[doc = "PLL selected as system clock"] PLL,
    #[doc = "HSI48 selected as system clock (when available)"] HSI48,
}
impl SWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SWR::HSI => 0,
            SWR::HSE => 1,
            SWR::PLL => 2,
            SWR::HSI48 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SWR {
        match value {
            0 => SWR::HSI,
            1 => SWR::HSE,
            2 => SWR::PLL,
            3 => SWR::HSI48,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline]
    pub fn is_hsi(&self) -> bool {
        *self == SWR::HSI
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline]
    pub fn is_hse(&self) -> bool {
        *self == SWR::HSE
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline]
    pub fn is_pll(&self) -> bool {
        *self == SWR::PLL
    }
    #[doc = "Checks if the value of the field is `HSI48`"]
    #[inline]
    pub fn is_hsi48(&self) -> bool {
        *self == SWR::HSI48
    }
}
#[doc = r" Value of the field"]
pub struct SWSR {
    bits: u8,
}
impl SWSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `HPRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPRER {
    #[doc = "SYSCLK not divided"] DIV1,
    #[doc = "SYSCLK divided by 2"] DIV2,
    #[doc = "SYSCLK divided by 4"] DIV4,
    #[doc = "SYSCLK divided by 8"] DIV8,
    #[doc = "SYSCLK divided by 16"] DIV16,
    #[doc = "SYSCLK divided by 64"] DIV64,
    #[doc = "SYSCLK divided by 128"] DIV128,
    #[doc = "SYSCLK divided by 256"] DIV256,
    #[doc = "SYSCLK divided by 512"] DIV512,
    #[doc = r" Reserved"] _Reserved(u8),
}
impl HPRER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HPRER::DIV1 => 0,
            HPRER::DIV2 => 8,
            HPRER::DIV4 => 9,
            HPRER::DIV8 => 10,
            HPRER::DIV16 => 11,
            HPRER::DIV64 => 12,
            HPRER::DIV128 => 13,
            HPRER::DIV256 => 14,
            HPRER::DIV512 => 15,
            HPRER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HPRER {
        match value {
            0 => HPRER::DIV1,
            8 => HPRER::DIV2,
            9 => HPRER::DIV4,
            10 => HPRER::DIV8,
            11 => HPRER::DIV16,
            12 => HPRER::DIV64,
            13 => HPRER::DIV128,
            14 => HPRER::DIV256,
            15 => HPRER::DIV512,
            i => HPRER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == HPRER::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == HPRER::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == HPRER::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == HPRER::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == HPRER::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == HPRER::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline]
    pub fn is_div128(&self) -> bool {
        *self == HPRER::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline]
    pub fn is_div256(&self) -> bool {
        *self == HPRER::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline]
    pub fn is_div512(&self) -> bool {
        *self == HPRER::DIV512
    }
}
#[doc = "Possible values of the field `PPRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPRER {
    #[doc = "HCLK not divided"] DIV1,
    #[doc = "HCLK divided by 2"] DIV2,
    #[doc = "HCLK divided by 4"] DIV4,
    #[doc = "HCLK divided by 8"] DIV8,
    #[doc = "HCLK divided by 16"] DIV16,
    #[doc = r" Reserved"] _Reserved(u8),
}
impl PPRER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PPRER::DIV1 => 0,
            PPRER::DIV2 => 4,
            PPRER::DIV4 => 5,
            PPRER::DIV8 => 6,
            PPRER::DIV16 => 7,
            PPRER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PPRER {
        match value {
            0 => PPRER::DIV1,
            4 => PPRER::DIV2,
            5 => PPRER::DIV4,
            6 => PPRER::DIV8,
            7 => PPRER::DIV16,
            i => PPRER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == PPRER::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == PPRER::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == PPRER::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == PPRER::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == PPRER::DIV16
    }
}
#[doc = r" Value of the field"]
pub struct ADCPRER {
    bits: bool,
}
impl ADCPRER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `PLLSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLSRCR {# [ doc = "HSI/2 selected as PLL input clock (PREDIV forced to divide by 2 on STM32F04x, STM32F07x and STM32F09x devices)" ] HSIDIV2 , # [ doc = "HSI/PREDIV selected as PLL input clock" ] HSI , # [ doc = "HSE/PREDIV selected as PLL input clock" ] HSE , # [ doc = "HSI48/PREDIV selected as PLL input clock" ] HSI48 ,}
impl PLLSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PLLSRCR::HSIDIV2 => 0,
            PLLSRCR::HSI => 1,
            PLLSRCR::HSE => 2,
            PLLSRCR::HSI48 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PLLSRCR {
        match value {
            0 => PLLSRCR::HSIDIV2,
            1 => PLLSRCR::HSI,
            2 => PLLSRCR::HSE,
            3 => PLLSRCR::HSI48,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HSIDIV2`"]
    #[inline]
    pub fn is_hsidiv2(&self) -> bool {
        *self == PLLSRCR::HSIDIV2
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline]
    pub fn is_hsi(&self) -> bool {
        *self == PLLSRCR::HSI
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline]
    pub fn is_hse(&self) -> bool {
        *self == PLLSRCR::HSE
    }
    #[doc = "Checks if the value of the field is `HSI48`"]
    #[inline]
    pub fn is_hsi48(&self) -> bool {
        *self == PLLSRCR::HSI48
    }
}
#[doc = "Possible values of the field `PLLXTPRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLXTPRER {
    #[doc = "HSE not divided"] DIV1,
    #[doc = "HSE divided by 2"] DIV2,
}
impl PLLXTPRER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PLLXTPRER::DIV1 => false,
            PLLXTPRER::DIV2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLLXTPRER {
        match value {
            false => PLLXTPRER::DIV1,
            true => PLLXTPRER::DIV2,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == PLLXTPRER::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == PLLXTPRER::DIV2
    }
}
#[doc = "Possible values of the field `PLLMUL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLMULR {
    #[doc = "PLL input clock x 2"] MUL2,
    #[doc = "PLL input clock x 3"] MUL3,
    #[doc = "PLL input clock x 4"] MUL4,
    #[doc = "PLL input clock x 5"] MUL5,
    #[doc = "PLL input clock x 6"] MUL6,
    #[doc = "PLL input clock x 7"] MUL7,
    #[doc = "PLL input clock x 8"] MUL8,
    #[doc = "PLL input clock x 9"] MUL9,
    #[doc = "PLL input clock x 10"] MUL10,
    #[doc = "PLL input clock x 11"] MUL11,
    #[doc = "PLL input clock x 12"] MUL12,
    #[doc = "PLL input clock x 13"] MUL13,
    #[doc = "PLL input clock x 14"] MUL14,
    #[doc = "PLL input clock x 15"] MUL15,
    #[doc = "PLL input clock x 16"] MUL16,
    #[doc = r" Reserved"] _Reserved(u8),
}
impl PLLMULR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PLLMULR::MUL2 => 0,
            PLLMULR::MUL3 => 1,
            PLLMULR::MUL4 => 2,
            PLLMULR::MUL5 => 3,
            PLLMULR::MUL6 => 4,
            PLLMULR::MUL7 => 5,
            PLLMULR::MUL8 => 6,
            PLLMULR::MUL9 => 7,
            PLLMULR::MUL10 => 8,
            PLLMULR::MUL11 => 9,
            PLLMULR::MUL12 => 10,
            PLLMULR::MUL13 => 11,
            PLLMULR::MUL14 => 12,
            PLLMULR::MUL15 => 13,
            PLLMULR::MUL16 => 14,
            PLLMULR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PLLMULR {
        match value {
            0 => PLLMULR::MUL2,
            1 => PLLMULR::MUL3,
            2 => PLLMULR::MUL4,
            3 => PLLMULR::MUL5,
            4 => PLLMULR::MUL6,
            5 => PLLMULR::MUL7,
            6 => PLLMULR::MUL8,
            7 => PLLMULR::MUL9,
            8 => PLLMULR::MUL10,
            9 => PLLMULR::MUL11,
            10 => PLLMULR::MUL12,
            11 => PLLMULR::MUL13,
            12 => PLLMULR::MUL14,
            13 => PLLMULR::MUL15,
            14 => PLLMULR::MUL16,
            i => PLLMULR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MUL2`"]
    #[inline]
    pub fn is_mul2(&self) -> bool {
        *self == PLLMULR::MUL2
    }
    #[doc = "Checks if the value of the field is `MUL3`"]
    #[inline]
    pub fn is_mul3(&self) -> bool {
        *self == PLLMULR::MUL3
    }
    #[doc = "Checks if the value of the field is `MUL4`"]
    #[inline]
    pub fn is_mul4(&self) -> bool {
        *self == PLLMULR::MUL4
    }
    #[doc = "Checks if the value of the field is `MUL5`"]
    #[inline]
    pub fn is_mul5(&self) -> bool {
        *self == PLLMULR::MUL5
    }
    #[doc = "Checks if the value of the field is `MUL6`"]
    #[inline]
    pub fn is_mul6(&self) -> bool {
        *self == PLLMULR::MUL6
    }
    #[doc = "Checks if the value of the field is `MUL7`"]
    #[inline]
    pub fn is_mul7(&self) -> bool {
        *self == PLLMULR::MUL7
    }
    #[doc = "Checks if the value of the field is `MUL8`"]
    #[inline]
    pub fn is_mul8(&self) -> bool {
        *self == PLLMULR::MUL8
    }
    #[doc = "Checks if the value of the field is `MUL9`"]
    #[inline]
    pub fn is_mul9(&self) -> bool {
        *self == PLLMULR::MUL9
    }
    #[doc = "Checks if the value of the field is `MUL10`"]
    #[inline]
    pub fn is_mul10(&self) -> bool {
        *self == PLLMULR::MUL10
    }
    #[doc = "Checks if the value of the field is `MUL11`"]
    #[inline]
    pub fn is_mul11(&self) -> bool {
        *self == PLLMULR::MUL11
    }
    #[doc = "Checks if the value of the field is `MUL12`"]
    #[inline]
    pub fn is_mul12(&self) -> bool {
        *self == PLLMULR::MUL12
    }
    #[doc = "Checks if the value of the field is `MUL13`"]
    #[inline]
    pub fn is_mul13(&self) -> bool {
        *self == PLLMULR::MUL13
    }
    #[doc = "Checks if the value of the field is `MUL14`"]
    #[inline]
    pub fn is_mul14(&self) -> bool {
        *self == PLLMULR::MUL14
    }
    #[doc = "Checks if the value of the field is `MUL15`"]
    #[inline]
    pub fn is_mul15(&self) -> bool {
        *self == PLLMULR::MUL15
    }
    #[doc = "Checks if the value of the field is `MUL16`"]
    #[inline]
    pub fn is_mul16(&self) -> bool {
        *self == PLLMULR::MUL16
    }
}
#[doc = "Possible values of the field `MCO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCOR {
    #[doc = "MCO output disabled, no clock on MCO"] NOCLOCK,
    #[doc = "Internal RC 14 MHz (HSI14) oscillator clock selected"] INTERNALRC,
    #[doc = "Internal low speed (LSI) oscillator clock selected"] INTERNALLSI,
    #[doc = "External low speed (LSE) oscillator clock selected"] LSE,
    #[doc = "System clock select"] SYSTEM,
    #[doc = "Internal RC 8 MHz (HSI) oscillator clock selected"] INTERNALHSI,
    #[doc = "External 4-32 MHz (HSE) oscillator clock selected"] EXTERNALHSE,
    #[doc = "PLL clock selected (divided by 1 or 2, depending on PLLNODIV)"] PLL,
    #[doc = "Internal RC 48 MHz (HSI48) oscillator clock selected"] HSI48,
    #[doc = r" Reserved"] _Reserved(u8),
}
impl MCOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MCOR::NOCLOCK => 0,
            MCOR::INTERNALRC => 1,
            MCOR::INTERNALLSI => 2,
            MCOR::LSE => 3,
            MCOR::SYSTEM => 4,
            MCOR::INTERNALHSI => 5,
            MCOR::EXTERNALHSE => 6,
            MCOR::PLL => 7,
            MCOR::HSI48 => 8,
            MCOR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MCOR {
        match value {
            0 => MCOR::NOCLOCK,
            1 => MCOR::INTERNALRC,
            2 => MCOR::INTERNALLSI,
            3 => MCOR::LSE,
            4 => MCOR::SYSTEM,
            5 => MCOR::INTERNALHSI,
            6 => MCOR::EXTERNALHSE,
            7 => MCOR::PLL,
            8 => MCOR::HSI48,
            i => MCOR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOCLOCK`"]
    #[inline]
    pub fn is_no_clock(&self) -> bool {
        *self == MCOR::NOCLOCK
    }
    #[doc = "Checks if the value of the field is `INTERNALRC`"]
    #[inline]
    pub fn is_internal_rc(&self) -> bool {
        *self == MCOR::INTERNALRC
    }
    #[doc = "Checks if the value of the field is `INTERNALLSI`"]
    #[inline]
    pub fn is_internal_lsi(&self) -> bool {
        *self == MCOR::INTERNALLSI
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline]
    pub fn is_lse(&self) -> bool {
        *self == MCOR::LSE
    }
    #[doc = "Checks if the value of the field is `SYSTEM`"]
    #[inline]
    pub fn is_system(&self) -> bool {
        *self == MCOR::SYSTEM
    }
    #[doc = "Checks if the value of the field is `INTERNALHSI`"]
    #[inline]
    pub fn is_internal_hsi(&self) -> bool {
        *self == MCOR::INTERNALHSI
    }
    #[doc = "Checks if the value of the field is `EXTERNALHSE`"]
    #[inline]
    pub fn is_external_hse(&self) -> bool {
        *self == MCOR::EXTERNALHSE
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline]
    pub fn is_pll(&self) -> bool {
        *self == MCOR::PLL
    }
    #[doc = "Checks if the value of the field is `HSI48`"]
    #[inline]
    pub fn is_hsi48(&self) -> bool {
        *self == MCOR::HSI48
    }
}
#[doc = "Possible values of the field `MCOPRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCOPRER {
    #[doc = "MCO is divided by 1"] DIV1,
    #[doc = "MCO is divided by 2"] DIV2,
    #[doc = "MCO is divided by 4"] DIV4,
    #[doc = "MCO is divided by 8"] DIV8,
    #[doc = "MCO is divided by 16"] DIV16,
    #[doc = "MCO is divided by 32"] DIV32,
    #[doc = "MCO is divided by 64"] DIV64,
    #[doc = "MCO is divided by 128"] DIV128,
}
impl MCOPRER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MCOPRER::DIV1 => 0,
            MCOPRER::DIV2 => 1,
            MCOPRER::DIV4 => 2,
            MCOPRER::DIV8 => 3,
            MCOPRER::DIV16 => 4,
            MCOPRER::DIV32 => 5,
            MCOPRER::DIV64 => 6,
            MCOPRER::DIV128 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MCOPRER {
        match value {
            0 => MCOPRER::DIV1,
            1 => MCOPRER::DIV2,
            2 => MCOPRER::DIV4,
            3 => MCOPRER::DIV8,
            4 => MCOPRER::DIV16,
            5 => MCOPRER::DIV32,
            6 => MCOPRER::DIV64,
            7 => MCOPRER::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == MCOPRER::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == MCOPRER::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == MCOPRER::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == MCOPRER::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == MCOPRER::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline]
    pub fn is_div32(&self) -> bool {
        *self == MCOPRER::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == MCOPRER::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline]
    pub fn is_div128(&self) -> bool {
        *self == MCOPRER::DIV128
    }
}
#[doc = "Possible values of the field `PLLNODIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLNODIVR {
    #[doc = "PLL is divided by 2 for MCO"] DIV2,
    #[doc = "PLL is not divided for MCO"] NODIV,
}
impl PLLNODIVR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PLLNODIVR::DIV2 => false,
            PLLNODIVR::NODIV => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLLNODIVR {
        match value {
            false => PLLNODIVR::DIV2,
            true => PLLNODIVR::NODIV,
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == PLLNODIVR::DIV2
    }
    #[doc = "Checks if the value of the field is `NODIV`"]
    #[inline]
    pub fn is_no_div(&self) -> bool {
        *self == PLLNODIVR::NODIV
    }
}
#[doc = "Values that can be written to the field `SW`"]
pub enum SWW {
    #[doc = "HSI selected as system clock"] HSI,
    #[doc = "HSE selected as system clock"] HSE,
    #[doc = "PLL selected as system clock"] PLL,
    #[doc = "HSI48 selected as system clock (when available)"] HSI48,
}
impl SWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SWW::HSI => 0,
            SWW::HSE => 1,
            SWW::PLL => 2,
            SWW::HSI48 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWW<'a> {
    w: &'a mut W,
}
impl<'a> _SWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "HSI selected as system clock"]
    #[inline]
    pub fn hsi(self) -> &'a mut W {
        self.variant(SWW::HSI)
    }
    #[doc = "HSE selected as system clock"]
    #[inline]
    pub fn hse(self) -> &'a mut W {
        self.variant(SWW::HSE)
    }
    #[doc = "PLL selected as system clock"]
    #[inline]
    pub fn pll(self) -> &'a mut W {
        self.variant(SWW::PLL)
    }
    #[doc = "HSI48 selected as system clock (when available)"]
    #[inline]
    pub fn hsi48(self) -> &'a mut W {
        self.variant(SWW::HSI48)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HPRE`"]
pub enum HPREW {
    #[doc = "SYSCLK not divided"] DIV1,
    #[doc = "SYSCLK divided by 2"] DIV2,
    #[doc = "SYSCLK divided by 4"] DIV4,
    #[doc = "SYSCLK divided by 8"] DIV8,
    #[doc = "SYSCLK divided by 16"] DIV16,
    #[doc = "SYSCLK divided by 64"] DIV64,
    #[doc = "SYSCLK divided by 128"] DIV128,
    #[doc = "SYSCLK divided by 256"] DIV256,
    #[doc = "SYSCLK divided by 512"] DIV512,
}
impl HPREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HPREW::DIV1 => 0,
            HPREW::DIV2 => 8,
            HPREW::DIV4 => 9,
            HPREW::DIV8 => 10,
            HPREW::DIV16 => 11,
            HPREW::DIV64 => 12,
            HPREW::DIV128 => 13,
            HPREW::DIV256 => 14,
            HPREW::DIV512 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HPREW<'a> {
    w: &'a mut W,
}
impl<'a> _HPREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HPREW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "SYSCLK not divided"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(HPREW::DIV1)
    }
    #[doc = "SYSCLK divided by 2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(HPREW::DIV2)
    }
    #[doc = "SYSCLK divided by 4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(HPREW::DIV4)
    }
    #[doc = "SYSCLK divided by 8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(HPREW::DIV8)
    }
    #[doc = "SYSCLK divided by 16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(HPREW::DIV16)
    }
    #[doc = "SYSCLK divided by 64"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(HPREW::DIV64)
    }
    #[doc = "SYSCLK divided by 128"]
    #[inline]
    pub fn div128(self) -> &'a mut W {
        self.variant(HPREW::DIV128)
    }
    #[doc = "SYSCLK divided by 256"]
    #[inline]
    pub fn div256(self) -> &'a mut W {
        self.variant(HPREW::DIV256)
    }
    #[doc = "SYSCLK divided by 512"]
    #[inline]
    pub fn div512(self) -> &'a mut W {
        self.variant(HPREW::DIV512)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PPRE`"]
pub enum PPREW {
    #[doc = "HCLK not divided"] DIV1,
    #[doc = "HCLK divided by 2"] DIV2,
    #[doc = "HCLK divided by 4"] DIV4,
    #[doc = "HCLK divided by 8"] DIV8,
    #[doc = "HCLK divided by 16"] DIV16,
}
impl PPREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PPREW::DIV1 => 0,
            PPREW::DIV2 => 4,
            PPREW::DIV4 => 5,
            PPREW::DIV8 => 6,
            PPREW::DIV16 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PPREW<'a> {
    w: &'a mut W,
}
impl<'a> _PPREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PPREW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "HCLK not divided"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(PPREW::DIV1)
    }
    #[doc = "HCLK divided by 2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(PPREW::DIV2)
    }
    #[doc = "HCLK divided by 4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(PPREW::DIV4)
    }
    #[doc = "HCLK divided by 8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(PPREW::DIV8)
    }
    #[doc = "HCLK divided by 16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(PPREW::DIV16)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ADCPREW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCPREW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLLSRC`"]
pub enum PLLSRCW {# [ doc = "HSI/2 selected as PLL input clock (PREDIV forced to divide by 2 on STM32F04x, STM32F07x and STM32F09x devices)" ] HSIDIV2 , # [ doc = "HSI/PREDIV selected as PLL input clock" ] HSI , # [ doc = "HSE/PREDIV selected as PLL input clock" ] HSE , # [ doc = "HSI48/PREDIV selected as PLL input clock" ] HSI48 ,}
impl PLLSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PLLSRCW::HSIDIV2 => 0,
            PLLSRCW::HSI => 1,
            PLLSRCW::HSE => 2,
            PLLSRCW::HSI48 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLLSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLLSRCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    # [ doc = "HSI/2 selected as PLL input clock (PREDIV forced to divide by 2 on STM32F04x, STM32F07x and STM32F09x devices)" ] # [ inline ]
    pub fn hsidiv2(self) -> &'a mut W {
        self.variant(PLLSRCW::HSIDIV2)
    }
    #[doc = "HSI/PREDIV selected as PLL input clock"]
    #[inline]
    pub fn hsi(self) -> &'a mut W {
        self.variant(PLLSRCW::HSI)
    }
    #[doc = "HSE/PREDIV selected as PLL input clock"]
    #[inline]
    pub fn hse(self) -> &'a mut W {
        self.variant(PLLSRCW::HSE)
    }
    #[doc = "HSI48/PREDIV selected as PLL input clock"]
    #[inline]
    pub fn hsi48(self) -> &'a mut W {
        self.variant(PLLSRCW::HSI48)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLLXTPRE`"]
pub enum PLLXTPREW {
    #[doc = "HSE not divided"] DIV1,
    #[doc = "HSE divided by 2"] DIV2,
}
impl PLLXTPREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLLXTPREW::DIV1 => false,
            PLLXTPREW::DIV2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLLXTPREW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLXTPREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLLXTPREW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HSE not divided"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(PLLXTPREW::DIV1)
    }
    #[doc = "HSE divided by 2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLXTPREW::DIV2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLLMUL`"]
pub enum PLLMULW {
    #[doc = "PLL input clock x 2"] MUL2,
    #[doc = "PLL input clock x 3"] MUL3,
    #[doc = "PLL input clock x 4"] MUL4,
    #[doc = "PLL input clock x 5"] MUL5,
    #[doc = "PLL input clock x 6"] MUL6,
    #[doc = "PLL input clock x 7"] MUL7,
    #[doc = "PLL input clock x 8"] MUL8,
    #[doc = "PLL input clock x 9"] MUL9,
    #[doc = "PLL input clock x 10"] MUL10,
    #[doc = "PLL input clock x 11"] MUL11,
    #[doc = "PLL input clock x 12"] MUL12,
    #[doc = "PLL input clock x 13"] MUL13,
    #[doc = "PLL input clock x 14"] MUL14,
    #[doc = "PLL input clock x 15"] MUL15,
    #[doc = "PLL input clock x 16"] MUL16,
}
impl PLLMULW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PLLMULW::MUL2 => 0,
            PLLMULW::MUL3 => 1,
            PLLMULW::MUL4 => 2,
            PLLMULW::MUL5 => 3,
            PLLMULW::MUL6 => 4,
            PLLMULW::MUL7 => 5,
            PLLMULW::MUL8 => 6,
            PLLMULW::MUL9 => 7,
            PLLMULW::MUL10 => 8,
            PLLMULW::MUL11 => 9,
            PLLMULW::MUL12 => 10,
            PLLMULW::MUL13 => 11,
            PLLMULW::MUL14 => 12,
            PLLMULW::MUL15 => 13,
            PLLMULW::MUL16 => 14,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLLMULW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLMULW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLLMULW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PLL input clock x 2"]
    #[inline]
    pub fn mul2(self) -> &'a mut W {
        self.variant(PLLMULW::MUL2)
    }
    #[doc = "PLL input clock x 3"]
    #[inline]
    pub fn mul3(self) -> &'a mut W {
        self.variant(PLLMULW::MUL3)
    }
    #[doc = "PLL input clock x 4"]
    #[inline]
    pub fn mul4(self) -> &'a mut W {
        self.variant(PLLMULW::MUL4)
    }
    #[doc = "PLL input clock x 5"]
    #[inline]
    pub fn mul5(self) -> &'a mut W {
        self.variant(PLLMULW::MUL5)
    }
    #[doc = "PLL input clock x 6"]
    #[inline]
    pub fn mul6(self) -> &'a mut W {
        self.variant(PLLMULW::MUL6)
    }
    #[doc = "PLL input clock x 7"]
    #[inline]
    pub fn mul7(self) -> &'a mut W {
        self.variant(PLLMULW::MUL7)
    }
    #[doc = "PLL input clock x 8"]
    #[inline]
    pub fn mul8(self) -> &'a mut W {
        self.variant(PLLMULW::MUL8)
    }
    #[doc = "PLL input clock x 9"]
    #[inline]
    pub fn mul9(self) -> &'a mut W {
        self.variant(PLLMULW::MUL9)
    }
    #[doc = "PLL input clock x 10"]
    #[inline]
    pub fn mul10(self) -> &'a mut W {
        self.variant(PLLMULW::MUL10)
    }
    #[doc = "PLL input clock x 11"]
    #[inline]
    pub fn mul11(self) -> &'a mut W {
        self.variant(PLLMULW::MUL11)
    }
    #[doc = "PLL input clock x 12"]
    #[inline]
    pub fn mul12(self) -> &'a mut W {
        self.variant(PLLMULW::MUL12)
    }
    #[doc = "PLL input clock x 13"]
    #[inline]
    pub fn mul13(self) -> &'a mut W {
        self.variant(PLLMULW::MUL13)
    }
    #[doc = "PLL input clock x 14"]
    #[inline]
    pub fn mul14(self) -> &'a mut W {
        self.variant(PLLMULW::MUL14)
    }
    #[doc = "PLL input clock x 15"]
    #[inline]
    pub fn mul15(self) -> &'a mut W {
        self.variant(PLLMULW::MUL15)
    }
    #[doc = "PLL input clock x 16"]
    #[inline]
    pub fn mul16(self) -> &'a mut W {
        self.variant(PLLMULW::MUL16)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MCO`"]
pub enum MCOW {
    #[doc = "MCO output disabled, no clock on MCO"] NOCLOCK,
    #[doc = "Internal RC 14 MHz (HSI14) oscillator clock selected"] INTERNALRC,
    #[doc = "Internal low speed (LSI) oscillator clock selected"] INTERNALLSI,
    #[doc = "External low speed (LSE) oscillator clock selected"] LSE,
    #[doc = "System clock select"] SYSTEM,
    #[doc = "Internal RC 8 MHz (HSI) oscillator clock selected"] INTERNALHSI,
    #[doc = "External 4-32 MHz (HSE) oscillator clock selected"] EXTERNALHSE,
    #[doc = "PLL clock selected (divided by 1 or 2, depending on PLLNODIV)"] PLL,
    #[doc = "Internal RC 48 MHz (HSI48) oscillator clock selected"] HSI48,
}
impl MCOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MCOW::NOCLOCK => 0,
            MCOW::INTERNALRC => 1,
            MCOW::INTERNALLSI => 2,
            MCOW::LSE => 3,
            MCOW::SYSTEM => 4,
            MCOW::INTERNALHSI => 5,
            MCOW::EXTERNALHSE => 6,
            MCOW::PLL => 7,
            MCOW::HSI48 => 8,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCOW<'a> {
    w: &'a mut W,
}
impl<'a> _MCOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCOW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "MCO output disabled, no clock on MCO"]
    #[inline]
    pub fn no_clock(self) -> &'a mut W {
        self.variant(MCOW::NOCLOCK)
    }
    #[doc = "Internal RC 14 MHz (HSI14) oscillator clock selected"]
    #[inline]
    pub fn internal_rc(self) -> &'a mut W {
        self.variant(MCOW::INTERNALRC)
    }
    #[doc = "Internal low speed (LSI) oscillator clock selected"]
    #[inline]
    pub fn internal_lsi(self) -> &'a mut W {
        self.variant(MCOW::INTERNALLSI)
    }
    #[doc = "External low speed (LSE) oscillator clock selected"]
    #[inline]
    pub fn lse(self) -> &'a mut W {
        self.variant(MCOW::LSE)
    }
    #[doc = "System clock select"]
    #[inline]
    pub fn system(self) -> &'a mut W {
        self.variant(MCOW::SYSTEM)
    }
    #[doc = "Internal RC 8 MHz (HSI) oscillator clock selected"]
    #[inline]
    pub fn internal_hsi(self) -> &'a mut W {
        self.variant(MCOW::INTERNALHSI)
    }
    #[doc = "External 4-32 MHz (HSE) oscillator clock selected"]
    #[inline]
    pub fn external_hse(self) -> &'a mut W {
        self.variant(MCOW::EXTERNALHSE)
    }
    #[doc = "PLL clock selected (divided by 1 or 2, depending on PLLNODIV)"]
    #[inline]
    pub fn pll(self) -> &'a mut W {
        self.variant(MCOW::PLL)
    }
    #[doc = "Internal RC 48 MHz (HSI48) oscillator clock selected"]
    #[inline]
    pub fn hsi48(self) -> &'a mut W {
        self.variant(MCOW::HSI48)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MCOPRE`"]
pub enum MCOPREW {
    #[doc = "MCO is divided by 1"] DIV1,
    #[doc = "MCO is divided by 2"] DIV2,
    #[doc = "MCO is divided by 4"] DIV4,
    #[doc = "MCO is divided by 8"] DIV8,
    #[doc = "MCO is divided by 16"] DIV16,
    #[doc = "MCO is divided by 32"] DIV32,
    #[doc = "MCO is divided by 64"] DIV64,
    #[doc = "MCO is divided by 128"] DIV128,
}
impl MCOPREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MCOPREW::DIV1 => 0,
            MCOPREW::DIV2 => 1,
            MCOPREW::DIV4 => 2,
            MCOPREW::DIV8 => 3,
            MCOPREW::DIV16 => 4,
            MCOPREW::DIV32 => 5,
            MCOPREW::DIV64 => 6,
            MCOPREW::DIV128 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCOPREW<'a> {
    w: &'a mut W,
}
impl<'a> _MCOPREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCOPREW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "MCO is divided by 1"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(MCOPREW::DIV1)
    }
    #[doc = "MCO is divided by 2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(MCOPREW::DIV2)
    }
    #[doc = "MCO is divided by 4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(MCOPREW::DIV4)
    }
    #[doc = "MCO is divided by 8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(MCOPREW::DIV8)
    }
    #[doc = "MCO is divided by 16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(MCOPREW::DIV16)
    }
    #[doc = "MCO is divided by 32"]
    #[inline]
    pub fn div32(self) -> &'a mut W {
        self.variant(MCOPREW::DIV32)
    }
    #[doc = "MCO is divided by 64"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(MCOPREW::DIV64)
    }
    #[doc = "MCO is divided by 128"]
    #[inline]
    pub fn div128(self) -> &'a mut W {
        self.variant(MCOPREW::DIV128)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLLNODIV`"]
pub enum PLLNODIVW {
    #[doc = "PLL is divided by 2 for MCO"] DIV2,
    #[doc = "PLL is not divided for MCO"] NODIV,
}
impl PLLNODIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLLNODIVW::DIV2 => false,
            PLLNODIVW::NODIV => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLLNODIVW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLNODIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLLNODIVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PLL is divided by 2 for MCO"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLNODIVW::DIV2)
    }
    #[doc = "PLL is not divided for MCO"]
    #[inline]
    pub fn no_div(self) -> &'a mut W {
        self.variant(PLLNODIVW::NODIV)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 31;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - System clock Switch"]
    #[inline]
    pub fn sw(&self) -> SWR {
        SWR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - System Clock Switch Status"]
    #[inline]
    pub fn sws(&self) -> SWSR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SWSR { bits }
    }
    #[doc = "Bits 4:7 - AHB prescaler"]
    #[inline]
    pub fn hpre(&self) -> HPRER {
        HPRER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:10 - APB Low speed prescaler (APB1)"]
    #[inline]
    pub fn ppre(&self) -> PPRER {
        PPRER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 14 - ADC prescaler"]
    #[inline]
    pub fn adcpre(&self) -> ADCPRER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ADCPRER { bits }
    }
    #[doc = "Bits 15:16 - PLL input clock source"]
    #[inline]
    pub fn pllsrc(&self) -> PLLSRCR {
        PLLSRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 17 - HSE divider for PLL entry"]
    #[inline]
    pub fn pllxtpre(&self) -> PLLXTPRER {
        PLLXTPRER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 18:21 - PLL Multiplication Factor"]
    #[inline]
    pub fn pllmul(&self) -> PLLMULR {
        PLLMULR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:26 - Microcontroller clock output"]
    #[inline]
    pub fn mco(&self) -> MCOR {
        MCOR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:30 - Microcontroller Clock Output Prescaler"]
    #[inline]
    pub fn mcopre(&self) -> MCOPRER {
        MCOPRER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 31 - PLL clock not divided for MCO"]
    #[inline]
    pub fn pllnodiv(&self) -> PLLNODIVR {
        PLLNODIVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - System clock Switch"]
    #[inline]
    pub fn sw(&mut self) -> _SWW {
        _SWW { w: self }
    }
    #[doc = "Bits 4:7 - AHB prescaler"]
    #[inline]
    pub fn hpre(&mut self) -> _HPREW {
        _HPREW { w: self }
    }
    #[doc = "Bits 8:10 - APB Low speed prescaler (APB1)"]
    #[inline]
    pub fn ppre(&mut self) -> _PPREW {
        _PPREW { w: self }
    }
    #[doc = "Bit 14 - ADC prescaler"]
    #[inline]
    pub fn adcpre(&mut self) -> _ADCPREW {
        _ADCPREW { w: self }
    }
    #[doc = "Bits 15:16 - PLL input clock source"]
    #[inline]
    pub fn pllsrc(&mut self) -> _PLLSRCW {
        _PLLSRCW { w: self }
    }
    #[doc = "Bit 17 - HSE divider for PLL entry"]
    #[inline]
    pub fn pllxtpre(&mut self) -> _PLLXTPREW {
        _PLLXTPREW { w: self }
    }
    #[doc = "Bits 18:21 - PLL Multiplication Factor"]
    #[inline]
    pub fn pllmul(&mut self) -> _PLLMULW {
        _PLLMULW { w: self }
    }
    #[doc = "Bits 24:26 - Microcontroller clock output"]
    #[inline]
    pub fn mco(&mut self) -> _MCOW {
        _MCOW { w: self }
    }
    #[doc = "Bits 28:30 - Microcontroller Clock Output Prescaler"]
    #[inline]
    pub fn mcopre(&mut self) -> _MCOPREW {
        _MCOPREW { w: self }
    }
    #[doc = "Bit 31 - PLL clock not divided for MCO"]
    #[inline]
    pub fn pllnodiv(&mut self) -> _PLLNODIVW {
        _PLLNODIVW { w: self }
    }
}
