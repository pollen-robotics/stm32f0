#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFGR3 {
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
        R { bits: self.register.get() }
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
#[doc = "Possible values of the field `USART1SW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART1SWR {
    #[doc = "PCLK selected as USART3 clock source (default)."]
    PCLK,
    #[doc = "System clock (SYSCLK) selected as USART3 clock."]
    SYSTEMCLOCK,
    #[doc = "LSE clock selected as USART3 clock."]
    LSE,
    #[doc = "HSI clock selected as USART3 clock."]
    HSI,
}
impl USART1SWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            USART1SWR::PCLK => 0,
            USART1SWR::SYSTEMCLOCK => 1,
            USART1SWR::LSE => 2,
            USART1SWR::HSI => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> USART1SWR {
        match value {
            0 => USART1SWR::PCLK,
            1 => USART1SWR::SYSTEMCLOCK,
            2 => USART1SWR::LSE,
            3 => USART1SWR::HSI,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PCLK`"]
    #[inline]
    pub fn is_pclk(&self) -> bool {
        *self == USART1SWR::PCLK
    }
    #[doc = "Checks if the value of the field is `SYSTEMCLOCK`"]
    #[inline]
    pub fn is_system_clock(&self) -> bool {
        *self == USART1SWR::SYSTEMCLOCK
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline]
    pub fn is_lse(&self) -> bool {
        *self == USART1SWR::LSE
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline]
    pub fn is_hsi(&self) -> bool {
        *self == USART1SWR::HSI
    }
}
#[doc = "Possible values of the field `I2C1SW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C1SWR {
    #[doc = "HSI clock selected as I2C1 clock source (default)."]
    HSI,
    #[doc = "System clock (SYSCLK) selected as I2C1 clock."]
    SYSCLK,
}
impl I2C1SWR {
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
            I2C1SWR::HSI => false,
            I2C1SWR::SYSCLK => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> I2C1SWR {
        match value {
            false => I2C1SWR::HSI,
            true => I2C1SWR::SYSCLK,
        }
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline]
    pub fn is_hsi(&self) -> bool {
        *self == I2C1SWR::HSI
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline]
    pub fn is_sysclk(&self) -> bool {
        *self == I2C1SWR::SYSCLK
    }
}
#[doc = "Possible values of the field `CECSW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CECSWR {
    #[doc = "HSI clock, divided by 244, selected as CEC clock (default)."]
    HSI,
    #[doc = "LSE clock selected as CEC clock."]
    LSE,
}
impl CECSWR {
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
            CECSWR::HSI => false,
            CECSWR::LSE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CECSWR {
        match value {
            false => CECSWR::HSI,
            true => CECSWR::LSE,
        }
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline]
    pub fn is_hsi(&self) -> bool {
        *self == CECSWR::HSI
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline]
    pub fn is_lse(&self) -> bool {
        *self == CECSWR::LSE
    }
}
#[doc = "Possible values of the field `USBSW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBSWR {
    #[doc = "HSI48 clock selected as USB clock source (default)."]
    HSI48,
    #[doc = "PLL clock (PLLCLK) selected as USB clock."]
    PLLCLOCK,
}
impl USBSWR {
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
            USBSWR::HSI48 => false,
            USBSWR::PLLCLOCK => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USBSWR {
        match value {
            false => USBSWR::HSI48,
            true => USBSWR::PLLCLOCK,
        }
    }
    #[doc = "Checks if the value of the field is `HSI48`"]
    #[inline]
    pub fn is_hsi48(&self) -> bool {
        *self == USBSWR::HSI48
    }
    #[doc = "Checks if the value of the field is `PLLCLOCK`"]
    #[inline]
    pub fn is_pllclock(&self) -> bool {
        *self == USBSWR::PLLCLOCK
    }
}
#[doc = r" Value of the field"]
pub struct ADCSWR {
    bits: bool,
}
impl ADCSWR {
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
#[doc = "Possible values of the field `USART2SW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART2SWR {
    #[doc = "PCLK selected as USART3 clock source (default)."]
    PCLK,
    #[doc = "System clock (SYSCLK) selected as USART3 clock."]
    SYSTEMCLOCK,
    #[doc = "LSE clock selected as USART3 clock."]
    LSE,
    #[doc = "HSI clock selected as USART3 clock."]
    HSI,
}
impl USART2SWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            USART2SWR::PCLK => 0,
            USART2SWR::SYSTEMCLOCK => 1,
            USART2SWR::LSE => 2,
            USART2SWR::HSI => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> USART2SWR {
        match value {
            0 => USART2SWR::PCLK,
            1 => USART2SWR::SYSTEMCLOCK,
            2 => USART2SWR::LSE,
            3 => USART2SWR::HSI,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PCLK`"]
    #[inline]
    pub fn is_pclk(&self) -> bool {
        *self == USART2SWR::PCLK
    }
    #[doc = "Checks if the value of the field is `SYSTEMCLOCK`"]
    #[inline]
    pub fn is_system_clock(&self) -> bool {
        *self == USART2SWR::SYSTEMCLOCK
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline]
    pub fn is_lse(&self) -> bool {
        *self == USART2SWR::LSE
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline]
    pub fn is_hsi(&self) -> bool {
        *self == USART2SWR::HSI
    }
}
#[doc = "Values that can be written to the field `USART1SW`"]
pub enum USART1SWW {
    #[doc = "PCLK selected as USART3 clock source (default)."]
    PCLK,
    #[doc = "System clock (SYSCLK) selected as USART3 clock."]
    SYSTEMCLOCK,
    #[doc = "LSE clock selected as USART3 clock."]
    LSE,
    #[doc = "HSI clock selected as USART3 clock."]
    HSI,
}
impl USART1SWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            USART1SWW::PCLK => 0,
            USART1SWW::SYSTEMCLOCK => 1,
            USART1SWW::LSE => 2,
            USART1SWW::HSI => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USART1SWW<'a> {
    w: &'a mut W,
}
impl<'a> _USART1SWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USART1SWW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "PCLK selected as USART3 clock source (default)."]
    #[inline]
    pub fn pclk(self) -> &'a mut W {
        self.variant(USART1SWW::PCLK)
    }
    #[doc = "System clock (SYSCLK) selected as USART3 clock."]
    #[inline]
    pub fn system_clock(self) -> &'a mut W {
        self.variant(USART1SWW::SYSTEMCLOCK)
    }
    #[doc = "LSE clock selected as USART3 clock."]
    #[inline]
    pub fn lse(self) -> &'a mut W {
        self.variant(USART1SWW::LSE)
    }
    #[doc = "HSI clock selected as USART3 clock."]
    #[inline]
    pub fn hsi(self) -> &'a mut W {
        self.variant(USART1SWW::HSI)
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
#[doc = "Values that can be written to the field `I2C1SW`"]
pub enum I2C1SWW {
    #[doc = "HSI clock selected as I2C1 clock source (default)."]
    HSI,
    #[doc = "System clock (SYSCLK) selected as I2C1 clock."]
    SYSCLK,
}
impl I2C1SWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            I2C1SWW::HSI => false,
            I2C1SWW::SYSCLK => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2C1SWW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C1SWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2C1SWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HSI clock selected as I2C1 clock source (default)."]
    #[inline]
    pub fn hsi(self) -> &'a mut W {
        self.variant(I2C1SWW::HSI)
    }
    #[doc = "System clock (SYSCLK) selected as I2C1 clock."]
    #[inline]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(I2C1SWW::SYSCLK)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CECSW`"]
pub enum CECSWW {
    #[doc = "HSI clock, divided by 244, selected as CEC clock (default)."]
    HSI,
    #[doc = "LSE clock selected as CEC clock."]
    LSE,
}
impl CECSWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CECSWW::HSI => false,
            CECSWW::LSE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CECSWW<'a> {
    w: &'a mut W,
}
impl<'a> _CECSWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CECSWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HSI clock, divided by 244, selected as CEC clock (default)."]
    #[inline]
    pub fn hsi(self) -> &'a mut W {
        self.variant(CECSWW::HSI)
    }
    #[doc = "LSE clock selected as CEC clock."]
    #[inline]
    pub fn lse(self) -> &'a mut W {
        self.variant(CECSWW::LSE)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USBSW`"]
pub enum USBSWW {
    #[doc = "HSI48 clock selected as USB clock source (default)."]
    HSI48,
    #[doc = "PLL clock (PLLCLK) selected as USB clock."]
    PLLCLOCK,
}
impl USBSWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USBSWW::HSI48 => false,
            USBSWW::PLLCLOCK => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBSWW<'a> {
    w: &'a mut W,
}
impl<'a> _USBSWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBSWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HSI48 clock selected as USB clock source (default)."]
    #[inline]
    pub fn hsi48(self) -> &'a mut W {
        self.variant(USBSWW::HSI48)
    }
    #[doc = "PLL clock (PLLCLK) selected as USB clock."]
    #[inline]
    pub fn pllclock(self) -> &'a mut W {
        self.variant(USBSWW::PLLCLOCK)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ADCSWW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCSWW<'a> {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USART2SW`"]
pub enum USART2SWW {
    #[doc = "PCLK selected as USART3 clock source (default)."]
    PCLK,
    #[doc = "System clock (SYSCLK) selected as USART3 clock."]
    SYSTEMCLOCK,
    #[doc = "LSE clock selected as USART3 clock."]
    LSE,
    #[doc = "HSI clock selected as USART3 clock."]
    HSI,
}
impl USART2SWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            USART2SWW::PCLK => 0,
            USART2SWW::SYSTEMCLOCK => 1,
            USART2SWW::LSE => 2,
            USART2SWW::HSI => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USART2SWW<'a> {
    w: &'a mut W,
}
impl<'a> _USART2SWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USART2SWW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "PCLK selected as USART3 clock source (default)."]
    #[inline]
    pub fn pclk(self) -> &'a mut W {
        self.variant(USART2SWW::PCLK)
    }
    #[doc = "System clock (SYSCLK) selected as USART3 clock."]
    #[inline]
    pub fn system_clock(self) -> &'a mut W {
        self.variant(USART2SWW::SYSTEMCLOCK)
    }
    #[doc = "LSE clock selected as USART3 clock."]
    #[inline]
    pub fn lse(self) -> &'a mut W {
        self.variant(USART2SWW::LSE)
    }
    #[doc = "HSI clock selected as USART3 clock."]
    #[inline]
    pub fn hsi(self) -> &'a mut W {
        self.variant(USART2SWW::HSI)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline]
    pub fn usart1sw(&self) -> USART1SWR {
        USART1SWR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - I2C1 clock source selection"]
    #[inline]
    pub fn i2c1sw(&self) -> I2C1SWR {
        I2C1SWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - HDMI CEC clock source selection"]
    #[inline]
    pub fn cecsw(&self) -> CECSWR {
        CECSWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - USB clock source selection"]
    #[inline]
    pub fn usbsw(&self) -> USBSWR {
        USBSWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - ADC clock source selection"]
    #[inline]
    pub fn adcsw(&self) -> ADCSWR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ADCSWR { bits }
    }
    #[doc = "Bits 16:17 - USART2 clock source selection"]
    #[inline]
    pub fn usart2sw(&self) -> USART2SWR {
        USART2SWR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline]
    pub fn usart1sw(&mut self) -> _USART1SWW {
        _USART1SWW { w: self }
    }
    #[doc = "Bit 4 - I2C1 clock source selection"]
    #[inline]
    pub fn i2c1sw(&mut self) -> _I2C1SWW {
        _I2C1SWW { w: self }
    }
    #[doc = "Bit 6 - HDMI CEC clock source selection"]
    #[inline]
    pub fn cecsw(&mut self) -> _CECSWW {
        _CECSWW { w: self }
    }
    #[doc = "Bit 7 - USB clock source selection"]
    #[inline]
    pub fn usbsw(&mut self) -> _USBSWW {
        _USBSWW { w: self }
    }
    #[doc = "Bit 8 - ADC clock source selection"]
    #[inline]
    pub fn adcsw(&mut self) -> _ADCSWW {
        _ADCSWW { w: self }
    }
    #[doc = "Bits 16:17 - USART2 clock source selection"]
    #[inline]
    pub fn usart2sw(&mut self) -> _USART2SWW {
        _USART2SWW { w: self }
    }
}
