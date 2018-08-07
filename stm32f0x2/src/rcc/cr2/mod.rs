#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR2 {
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
#[doc = "Possible values of the field `HSI14ON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSI14ONR {
    #[doc = "HSI14 oscillator OFF."]
    DISABLED,
    #[doc = "HSI14 oscillator ON."]
    ENABLED,
}
impl HSI14ONR {
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
            HSI14ONR::DISABLED => false,
            HSI14ONR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HSI14ONR {
        match value {
            false => HSI14ONR::DISABLED,
            true => HSI14ONR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == HSI14ONR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == HSI14ONR::ENABLED
    }
}
#[doc = r" Value of the field"]
pub struct HSI14RDYR {
    bits: bool,
}
impl HSI14RDYR {
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
#[doc = "Possible values of the field `HSI14DIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSI14DISR {
    #[doc = "ADC interface can turn on the HSI14 oscillator."]
    ENABLED,
    #[doc = "ADC interface can not turn on the HSI14 oscillator."]
    DISABLED,
}
impl HSI14DISR {
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
            HSI14DISR::ENABLED => false,
            HSI14DISR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HSI14DISR {
        match value {
            false => HSI14DISR::ENABLED,
            true => HSI14DISR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == HSI14DISR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == HSI14DISR::DISABLED
    }
}
#[doc = r" Value of the field"]
pub struct HSI14TRIMR {
    bits: u8,
}
impl HSI14TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HSI14CALR {
    bits: u8,
}
impl HSI14CALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `HSI48ON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSI48ONR {
    #[doc = "HSI48 oscillator OFF."]
    DISABLED,
    #[doc = "HSI48 oscillator ON."]
    ENABLED,
}
impl HSI48ONR {
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
            HSI48ONR::DISABLED => false,
            HSI48ONR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HSI48ONR {
        match value {
            false => HSI48ONR::DISABLED,
            true => HSI48ONR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == HSI48ONR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == HSI48ONR::ENABLED
    }
}
#[doc = r" Value of the field"]
pub struct HSI48RDYR {
    bits: bool,
}
impl HSI48RDYR {
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
pub struct HSI48CALR {
    bits: bool,
}
impl HSI48CALR {
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
#[doc = "Values that can be written to the field `HSI14ON`"]
pub enum HSI14ONW {
    #[doc = "HSI14 oscillator OFF."]
    DISABLED,
    #[doc = "HSI14 oscillator ON."]
    ENABLED,
}
impl HSI14ONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HSI14ONW::DISABLED => false,
            HSI14ONW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HSI14ONW<'a> {
    w: &'a mut W,
}
impl<'a> _HSI14ONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HSI14ONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HSI14 oscillator OFF."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HSI14ONW::DISABLED)
    }
    #[doc = "HSI14 oscillator ON."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HSI14ONW::ENABLED)
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
#[doc = "Values that can be written to the field `HSI14DIS`"]
pub enum HSI14DISW {
    #[doc = "ADC interface can turn on the HSI14 oscillator."]
    ENABLED,
    #[doc = "ADC interface can not turn on the HSI14 oscillator."]
    DISABLED,
}
impl HSI14DISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HSI14DISW::ENABLED => false,
            HSI14DISW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HSI14DISW<'a> {
    w: &'a mut W,
}
impl<'a> _HSI14DISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HSI14DISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ADC interface can turn on the HSI14 oscillator."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HSI14DISW::ENABLED)
    }
    #[doc = "ADC interface can not turn on the HSI14 oscillator."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HSI14DISW::DISABLED)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HSI14TRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _HSI14TRIMW<'a> {
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
#[doc = "Values that can be written to the field `HSI48ON`"]
pub enum HSI48ONW {
    #[doc = "HSI48 oscillator OFF."]
    DISABLED,
    #[doc = "HSI48 oscillator ON."]
    ENABLED,
}
impl HSI48ONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HSI48ONW::DISABLED => false,
            HSI48ONW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HSI48ONW<'a> {
    w: &'a mut W,
}
impl<'a> _HSI48ONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HSI48ONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HSI48 oscillator OFF."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HSI48ONW::DISABLED)
    }
    #[doc = "HSI48 oscillator ON."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HSI48ONW::ENABLED)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - HSI14 clock enable"]
    #[inline]
    pub fn hsi14on(&self) -> HSI14ONR {
        HSI14ONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - HR14 clock ready flag"]
    #[inline]
    pub fn hsi14rdy(&self) -> HSI14RDYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HSI14RDYR { bits }
    }
    #[doc = "Bit 2 - HSI14 clock request from ADC disable"]
    #[inline]
    pub fn hsi14dis(&self) -> HSI14DISR {
        HSI14DISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 3:7 - HSI14 clock trimming"]
    #[inline]
    pub fn hsi14trim(&self) -> HSI14TRIMR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HSI14TRIMR { bits }
    }
    #[doc = "Bits 8:15 - HSI14 clock calibration"]
    #[inline]
    pub fn hsi14cal(&self) -> HSI14CALR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HSI14CALR { bits }
    }
    #[doc = "Bit 16 - HSI48 clock enable"]
    #[inline]
    pub fn hsi48on(&self) -> HSI48ONR {
        HSI48ONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - HSI48 clock ready flag"]
    #[inline]
    pub fn hsi48rdy(&self) -> HSI48RDYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HSI48RDYR { bits }
    }
    #[doc = "Bit 24 - HSI48 factory clock calibration"]
    #[inline]
    pub fn hsi48cal(&self) -> HSI48CALR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HSI48CALR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 128 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - HSI14 clock enable"]
    #[inline]
    pub fn hsi14on(&mut self) -> _HSI14ONW {
        _HSI14ONW { w: self }
    }
    #[doc = "Bit 2 - HSI14 clock request from ADC disable"]
    #[inline]
    pub fn hsi14dis(&mut self) -> _HSI14DISW {
        _HSI14DISW { w: self }
    }
    #[doc = "Bits 3:7 - HSI14 clock trimming"]
    #[inline]
    pub fn hsi14trim(&mut self) -> _HSI14TRIMW {
        _HSI14TRIMW { w: self }
    }
    #[doc = "Bit 16 - HSI48 clock enable"]
    #[inline]
    pub fn hsi48on(&mut self) -> _HSI48ONW {
        _HSI48ONW { w: self }
    }
}
