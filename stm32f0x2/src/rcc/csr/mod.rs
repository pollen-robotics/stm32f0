#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CSR {
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
#[doc = "Possible values of the field `LSION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSIONR {
    #[doc = "LSI oscillator OFF."]
    DISABLED,
    #[doc = "LSI oscillator ON."]
    ENABLED,
}
impl LSIONR {
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
            LSIONR::DISABLED => false,
            LSIONR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LSIONR {
        match value {
            false => LSIONR::DISABLED,
            true => LSIONR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == LSIONR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == LSIONR::ENABLED
    }
}
#[doc = r" Value of the field"]
pub struct LSIRDYR {
    bits: bool,
}
impl LSIRDYR {
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
#[doc = "Possible values of the field `RMVF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMVFR {
    #[doc = "Clear the reset flags."]
    CLEARED,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RMVFR {
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
            RMVFR::CLEARED => true,
            RMVFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RMVFR {
        match value {
            true => RMVFR::CLEARED,
            i => RMVFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline]
    pub fn is_cleared(&self) -> bool {
        *self == RMVFR::CLEARED
    }
}
#[doc = r" Value of the field"]
pub struct OBLRSTFR {
    bits: bool,
}
impl OBLRSTFR {
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
pub struct PINRSTFR {
    bits: bool,
}
impl PINRSTFR {
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
pub struct PORRSTFR {
    bits: bool,
}
impl PORRSTFR {
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
pub struct SFTRSTFR {
    bits: bool,
}
impl SFTRSTFR {
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
pub struct IWDGRSTFR {
    bits: bool,
}
impl IWDGRSTFR {
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
pub struct WWDGRSTFR {
    bits: bool,
}
impl WWDGRSTFR {
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
pub struct LPWRRSTFR {
    bits: bool,
}
impl LPWRRSTFR {
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
#[doc = "Values that can be written to the field `LSION`"]
pub enum LSIONW {
    #[doc = "LSI oscillator OFF."]
    DISABLED,
    #[doc = "LSI oscillator ON."]
    ENABLED,
}
impl LSIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LSIONW::DISABLED => false,
            LSIONW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LSIONW<'a> {
    w: &'a mut W,
}
impl<'a> _LSIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LSIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LSI oscillator OFF."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSIONW::DISABLED)
    }
    #[doc = "LSI oscillator ON."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LSIONW::ENABLED)
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
#[doc = "Values that can be written to the field `RMVF`"]
pub enum RMVFW {
    #[doc = "Clear the reset flags."]
    CLEARED,
}
impl RMVFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RMVFW::CLEARED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RMVFW<'a> {
    w: &'a mut W,
}
impl<'a> _RMVFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RMVFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear the reset flags."]
    #[inline]
    pub fn cleared(self) -> &'a mut W {
        self.variant(RMVFW::CLEARED)
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
#[doc = r" Proxy"]
pub struct _PINRSTFW<'a> {
    w: &'a mut W,
}
impl<'a> _PINRSTFW<'a> {
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
        const OFFSET: u8 = 26;
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
    #[doc = "Bit 0 - Internal low speed oscillator enable"]
    #[inline]
    pub fn lsion(&self) -> LSIONR {
        LSIONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Internal low speed oscillator ready"]
    #[inline]
    pub fn lsirdy(&self) -> LSIRDYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LSIRDYR { bits }
    }
    #[doc = "Bit 24 - Remove reset flag"]
    #[inline]
    pub fn rmvf(&self) -> RMVFR {
        RMVFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Option byte loader reset flag"]
    #[inline]
    pub fn oblrstf(&self) -> OBLRSTFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OBLRSTFR { bits }
    }
    #[doc = "Bit 26 - PIN reset flag"]
    #[inline]
    pub fn pinrstf(&self) -> PINRSTFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINRSTFR { bits }
    }
    #[doc = "Bit 27 - POR/PDR reset flag"]
    #[inline]
    pub fn porrstf(&self) -> PORRSTFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PORRSTFR { bits }
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline]
    pub fn sftrstf(&self) -> SFTRSTFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SFTRSTFR { bits }
    }
    #[doc = "Bit 29 - Independent watchdog reset flag"]
    #[inline]
    pub fn iwdgrstf(&self) -> IWDGRSTFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IWDGRSTFR { bits }
    }
    #[doc = "Bit 30 - Window watchdog reset flag"]
    #[inline]
    pub fn wwdgrstf(&self) -> WWDGRSTFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WWDGRSTFR { bits }
    }
    #[doc = "Bit 31 - Low-power reset flag"]
    #[inline]
    pub fn lpwrrstf(&self) -> LPWRRSTFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LPWRRSTFR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 201326592 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Internal low speed oscillator enable"]
    #[inline]
    pub fn lsion(&mut self) -> _LSIONW {
        _LSIONW { w: self }
    }
    #[doc = "Bit 24 - Remove reset flag"]
    #[inline]
    pub fn rmvf(&mut self) -> _RMVFW {
        _RMVFW { w: self }
    }
    #[doc = "Bit 26 - PIN reset flag"]
    #[inline]
    pub fn pinrstf(&mut self) -> _PINRSTFW {
        _PINRSTFW { w: self }
    }
}
