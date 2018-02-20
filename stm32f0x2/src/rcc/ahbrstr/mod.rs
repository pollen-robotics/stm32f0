#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AHBRSTR {
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
#[doc = "Possible values of the field `IOPARST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOPARSTR {
    #[doc = "Reset I/O port A."]
    RESET,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl IOPARSTR {
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
            IOPARSTR::RESET => true,
            IOPARSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IOPARSTR {
        match value {
            true => IOPARSTR::RESET,
            i => IOPARSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == IOPARSTR::RESET
    }
}
#[doc = "Possible values of the field `IOPBRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOPBRSTR {
    #[doc = "Reset I/O port B."]
    RESET,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl IOPBRSTR {
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
            IOPBRSTR::RESET => true,
            IOPBRSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IOPBRSTR {
        match value {
            true => IOPBRSTR::RESET,
            i => IOPBRSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == IOPBRSTR::RESET
    }
}
#[doc = "Possible values of the field `IOPCRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOPCRSTR {
    #[doc = "Reset I/O port C."]
    RESET,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl IOPCRSTR {
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
            IOPCRSTR::RESET => true,
            IOPCRSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IOPCRSTR {
        match value {
            true => IOPCRSTR::RESET,
            i => IOPCRSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == IOPCRSTR::RESET
    }
}
#[doc = "Possible values of the field `IOPDRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOPDRSTR {
    #[doc = "Reset I/O port D."]
    RESET,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl IOPDRSTR {
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
            IOPDRSTR::RESET => true,
            IOPDRSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IOPDRSTR {
        match value {
            true => IOPDRSTR::RESET,
            i => IOPDRSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == IOPDRSTR::RESET
    }
}
#[doc = "Possible values of the field `IOPFRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOPFRSTR {
    #[doc = "Reset I/O port F."]
    RESET,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl IOPFRSTR {
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
            IOPFRSTR::RESET => true,
            IOPFRSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IOPFRSTR {
        match value {
            true => IOPFRSTR::RESET,
            i => IOPFRSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == IOPFRSTR::RESET
    }
}
#[doc = "Possible values of the field `TSCRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSCRSTR {
    #[doc = "Reset TSC."]
    RESET,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl TSCRSTR {
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
            TSCRSTR::RESET => true,
            TSCRSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSCRSTR {
        match value {
            true => TSCRSTR::RESET,
            i => TSCRSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == TSCRSTR::RESET
    }
}
#[doc = "Values that can be written to the field `IOPARST`"]
pub enum IOPARSTW {
    #[doc = "Reset I/O port A."]
    RESET,
}
impl IOPARSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IOPARSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IOPARSTW<'a> {
    w: &'a mut W,
}
impl<'a> _IOPARSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOPARSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset I/O port A."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(IOPARSTW::RESET)
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
#[doc = "Values that can be written to the field `IOPBRST`"]
pub enum IOPBRSTW {
    #[doc = "Reset I/O port B."]
    RESET,
}
impl IOPBRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IOPBRSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IOPBRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _IOPBRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOPBRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset I/O port B."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(IOPBRSTW::RESET)
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
#[doc = "Values that can be written to the field `IOPCRST`"]
pub enum IOPCRSTW {
    #[doc = "Reset I/O port C."]
    RESET,
}
impl IOPCRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IOPCRSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IOPCRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _IOPCRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOPCRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset I/O port C."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(IOPCRSTW::RESET)
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
#[doc = "Values that can be written to the field `IOPDRST`"]
pub enum IOPDRSTW {
    #[doc = "Reset I/O port D."]
    RESET,
}
impl IOPDRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IOPDRSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IOPDRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _IOPDRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOPDRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset I/O port D."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(IOPDRSTW::RESET)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IOPFRST`"]
pub enum IOPFRSTW {
    #[doc = "Reset I/O port F."]
    RESET,
}
impl IOPFRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IOPFRSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IOPFRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _IOPFRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOPFRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset I/O port F."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(IOPFRSTW::RESET)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TSCRST`"]
pub enum TSCRSTW {
    #[doc = "Reset TSC."]
    RESET,
}
impl TSCRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSCRSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSCRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TSCRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSCRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset TSC."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TSCRSTW::RESET)
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
    #[doc = "Bit 17 - I/O port A reset"]
    #[inline]
    pub fn ioparst(&self) -> IOPARSTR {
        IOPARSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - I/O port B reset"]
    #[inline]
    pub fn iopbrst(&self) -> IOPBRSTR {
        IOPBRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - I/O port C reset"]
    #[inline]
    pub fn iopcrst(&self) -> IOPCRSTR {
        IOPCRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - I/O port D reset"]
    #[inline]
    pub fn iopdrst(&self) -> IOPDRSTR {
        IOPDRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - I/O port F reset"]
    #[inline]
    pub fn iopfrst(&self) -> IOPFRSTR {
        IOPFRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Touch sensing controller reset"]
    #[inline]
    pub fn tscrst(&self) -> TSCRSTR {
        TSCRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
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
    #[doc = "Bit 17 - I/O port A reset"]
    #[inline]
    pub fn ioparst(&mut self) -> _IOPARSTW {
        _IOPARSTW { w: self }
    }
    #[doc = "Bit 18 - I/O port B reset"]
    #[inline]
    pub fn iopbrst(&mut self) -> _IOPBRSTW {
        _IOPBRSTW { w: self }
    }
    #[doc = "Bit 19 - I/O port C reset"]
    #[inline]
    pub fn iopcrst(&mut self) -> _IOPCRSTW {
        _IOPCRSTW { w: self }
    }
    #[doc = "Bit 20 - I/O port D reset"]
    #[inline]
    pub fn iopdrst(&mut self) -> _IOPDRSTW {
        _IOPDRSTW { w: self }
    }
    #[doc = "Bit 22 - I/O port F reset"]
    #[inline]
    pub fn iopfrst(&mut self) -> _IOPFRSTW {
        _IOPFRSTW { w: self }
    }
    #[doc = "Bit 24 - Touch sensing controller reset"]
    #[inline]
    pub fn tscrst(&mut self) -> _TSCRSTW {
        _TSCRSTW { w: self }
    }
}
