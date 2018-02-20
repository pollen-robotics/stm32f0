#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR {
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
#[doc = "Possible values of the field `HSION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSIONR {
    #[doc = "HSI oscillator OFF"]
    DISABLED,
    #[doc = "HSI oscillator ON"]
    ENABLED,
}
impl HSIONR {
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
            HSIONR::DISABLED => false,
            HSIONR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HSIONR {
        match value {
            false => HSIONR::DISABLED,
            true => HSIONR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == HSIONR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == HSIONR::ENABLED
    }
}
#[doc = r" Value of the field"]
pub struct HSIRDYR {
    bits: bool,
}
impl HSIRDYR {
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
#[doc = r" Value of the field"]
pub struct HSITRIMR {
    bits: u8,
}
impl HSITRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HSICALR {
    bits: u8,
}
impl HSICALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `HSEON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSEONR {
    #[doc = "HSE oscillator OFF"]
    DISABLED,
    #[doc = "HSE oscillator ON"]
    ENABLED,
}
impl HSEONR {
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
            HSEONR::DISABLED => false,
            HSEONR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HSEONR {
        match value {
            false => HSEONR::DISABLED,
            true => HSEONR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == HSEONR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == HSEONR::ENABLED
    }
}
#[doc = r" Value of the field"]
pub struct HSERDYR {
    bits: bool,
}
impl HSERDYR {
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
#[doc = "Possible values of the field `HSEBYP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSEBYPR {
    #[doc = "HSE crystal oscillator not bypassed"]
    NOTBYPASSED,
    #[doc = "HSE crystal oscillator bypassed with external clock"]
    BYPASSED,
}
impl HSEBYPR {
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
            HSEBYPR::NOTBYPASSED => false,
            HSEBYPR::BYPASSED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HSEBYPR {
        match value {
            false => HSEBYPR::NOTBYPASSED,
            true => HSEBYPR::BYPASSED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTBYPASSED`"]
    #[inline]
    pub fn is_not_bypassed(&self) -> bool {
        *self == HSEBYPR::NOTBYPASSED
    }
    #[doc = "Checks if the value of the field is `BYPASSED`"]
    #[inline]
    pub fn is_bypassed(&self) -> bool {
        *self == HSEBYPR::BYPASSED
    }
}
#[doc = "Possible values of the field `CSSON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSSONR {
    #[doc = "Clock security system disabled (clock detector OFF)."]
    DISABLED,
    #[doc = "Clock security system enabled (clock detector ON if the HSE is ready, OFF if not)."]
    ENABLED,
}
impl CSSONR {
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
            CSSONR::DISABLED => false,
            CSSONR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CSSONR {
        match value {
            false => CSSONR::DISABLED,
            true => CSSONR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CSSONR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CSSONR::ENABLED
    }
}
#[doc = "Possible values of the field `PLLON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLONR {
    #[doc = "PLL Off."]
    DISABLED,
    #[doc = "PLL On."]
    ENABLED,
}
impl PLLONR {
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
            PLLONR::DISABLED => false,
            PLLONR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLLONR {
        match value {
            false => PLLONR::DISABLED,
            true => PLLONR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PLLONR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PLLONR::ENABLED
    }
}
#[doc = r" Value of the field"]
pub struct PLLRDYR {
    bits: bool,
}
impl PLLRDYR {
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
#[doc = "Values that can be written to the field `HSION`"]
pub enum HSIONW {
    #[doc = "HSI oscillator OFF"]
    DISABLED,
    #[doc = "HSI oscillator ON"]
    ENABLED,
}
impl HSIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HSIONW::DISABLED => false,
            HSIONW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HSIONW<'a> {
    w: &'a mut W,
}
impl<'a> _HSIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HSIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HSI oscillator OFF"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HSIONW::DISABLED)
    }
    #[doc = "HSI oscillator ON"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HSIONW::ENABLED)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HSITRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _HSITRIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HSEON`"]
pub enum HSEONW {
    #[doc = "HSE oscillator OFF"]
    DISABLED,
    #[doc = "HSE oscillator ON"]
    ENABLED,
}
impl HSEONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HSEONW::DISABLED => false,
            HSEONW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HSEONW<'a> {
    w: &'a mut W,
}
impl<'a> _HSEONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HSEONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HSE oscillator OFF"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HSEONW::DISABLED)
    }
    #[doc = "HSE oscillator ON"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HSEONW::ENABLED)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HSEBYP`"]
pub enum HSEBYPW {
    #[doc = "HSE crystal oscillator not bypassed"]
    NOTBYPASSED,
    #[doc = "HSE crystal oscillator bypassed with external clock"]
    BYPASSED,
}
impl HSEBYPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HSEBYPW::NOTBYPASSED => false,
            HSEBYPW::BYPASSED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HSEBYPW<'a> {
    w: &'a mut W,
}
impl<'a> _HSEBYPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HSEBYPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HSE crystal oscillator not bypassed"]
    #[inline]
    pub fn not_bypassed(self) -> &'a mut W {
        self.variant(HSEBYPW::NOTBYPASSED)
    }
    #[doc = "HSE crystal oscillator bypassed with external clock"]
    #[inline]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(HSEBYPW::BYPASSED)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CSSON`"]
pub enum CSSONW {
    #[doc = "Clock security system disabled (clock detector OFF)."]
    DISABLED,
    #[doc = "Clock security system enabled (clock detector ON if the HSE is ready, OFF if not)."]
    ENABLED,
}
impl CSSONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CSSONW::DISABLED => false,
            CSSONW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSSONW<'a> {
    w: &'a mut W,
}
impl<'a> _CSSONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSSONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock security system disabled (clock detector OFF)."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CSSONW::DISABLED)
    }
    #[doc = "Clock security system enabled (clock detector ON if the HSE is ready, OFF if not)."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CSSONW::ENABLED)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLLON`"]
pub enum PLLONW {
    #[doc = "PLL Off."]
    DISABLED,
    #[doc = "PLL On."]
    ENABLED,
}
impl PLLONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLLONW::DISABLED => false,
            PLLONW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLLONW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLLONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PLL Off."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLONW::DISABLED)
    }
    #[doc = "PLL On."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLONW::ENABLED)
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
        const OFFSET: u8 = 24;
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
    #[doc = "Bit 0 - Internal High Speed clock enable"]
    #[inline]
    pub fn hsion(&self) -> HSIONR {
        HSIONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Internal High Speed clock ready flag"]
    #[inline]
    pub fn hsirdy(&self) -> HSIRDYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HSIRDYR { bits }
    }
    #[doc = "Bits 3:7 - Internal High Speed clock trimming"]
    #[inline]
    pub fn hsitrim(&self) -> HSITRIMR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HSITRIMR { bits }
    }
    #[doc = "Bits 8:15 - Internal High Speed clock Calibration"]
    #[inline]
    pub fn hsical(&self) -> HSICALR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HSICALR { bits }
    }
    #[doc = "Bit 16 - External High Speed clock enable"]
    #[inline]
    pub fn hseon(&self) -> HSEONR {
        HSEONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - External High Speed clock ready flag"]
    #[inline]
    pub fn hserdy(&self) -> HSERDYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HSERDYR { bits }
    }
    #[doc = "Bit 18 - External High Speed clock Bypass"]
    #[inline]
    pub fn hsebyp(&self) -> HSEBYPR {
        HSEBYPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Clock Security System enable"]
    #[inline]
    pub fn csson(&self) -> CSSONR {
        CSSONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - PLL enable"]
    #[inline]
    pub fn pllon(&self) -> PLLONR {
        PLLONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - PLL clock ready flag"]
    #[inline]
    pub fn pllrdy(&self) -> PLLRDYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLLRDYR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 131 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Internal High Speed clock enable"]
    #[inline]
    pub fn hsion(&mut self) -> _HSIONW {
        _HSIONW { w: self }
    }
    #[doc = "Bits 3:7 - Internal High Speed clock trimming"]
    #[inline]
    pub fn hsitrim(&mut self) -> _HSITRIMW {
        _HSITRIMW { w: self }
    }
    #[doc = "Bit 16 - External High Speed clock enable"]
    #[inline]
    pub fn hseon(&mut self) -> _HSEONW {
        _HSEONW { w: self }
    }
    #[doc = "Bit 18 - External High Speed clock Bypass"]
    #[inline]
    pub fn hsebyp(&mut self) -> _HSEBYPW {
        _HSEBYPW { w: self }
    }
    #[doc = "Bit 19 - Clock Security System enable"]
    #[inline]
    pub fn csson(&mut self) -> _CSSONW {
        _CSSONW { w: self }
    }
    #[doc = "Bit 24 - PLL enable"]
    #[inline]
    pub fn pllon(&mut self) -> _PLLONW {
        _PLLONW { w: self }
    }
}
