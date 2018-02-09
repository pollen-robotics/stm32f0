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
#[doc = "Possible values of the field `RESET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESETR {
    #[doc = "Resets the CRC calculation unit"]
    RESET,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RESETR {
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
            RESETR::RESET => true,
            RESETR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESETR {
        match value {
            true => RESETR::RESET,
            i => RESETR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == RESETR::RESET
    }
}
#[doc = "Possible values of the field `POLYSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POLYSIZER {
    #[doc = "32 bit polynomial."]
    _32BITS,
    #[doc = "16 bit polynomial."]
    _16BITS,
    #[doc = "8 bit polynomial."]
    _8BITS,
    #[doc = "7 bit polynomial."]
    _7BITS,
}
impl POLYSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            POLYSIZER::_32BITS => 0,
            POLYSIZER::_16BITS => 1,
            POLYSIZER::_8BITS => 2,
            POLYSIZER::_7BITS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> POLYSIZER {
        match value {
            0 => POLYSIZER::_32BITS,
            1 => POLYSIZER::_16BITS,
            2 => POLYSIZER::_8BITS,
            3 => POLYSIZER::_7BITS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_32BITS`"]
    #[inline]
    pub fn is_32bits(&self) -> bool {
        *self == POLYSIZER::_32BITS
    }
    #[doc = "Checks if the value of the field is `_16BITS`"]
    #[inline]
    pub fn is_16bits(&self) -> bool {
        *self == POLYSIZER::_16BITS
    }
    #[doc = "Checks if the value of the field is `_8BITS`"]
    #[inline]
    pub fn is_8bits(&self) -> bool {
        *self == POLYSIZER::_8BITS
    }
    #[doc = "Checks if the value of the field is `_7BITS`"]
    #[inline]
    pub fn is_7bits(&self) -> bool {
        *self == POLYSIZER::_7BITS
    }
}
#[doc = "Possible values of the field `REV_IN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV_INR {
    #[doc = "Bit order not affected."]
    NOREVERSE,
    #[doc = "Bit reversal done by byte."]
    BYTE,
    #[doc = "Bit reversal done by half-word."]
    HALFWORD,
    #[doc = "Bit reversal done by word."]
    _7BITS,
}
impl REV_INR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REV_INR::NOREVERSE => 0,
            REV_INR::BYTE => 1,
            REV_INR::HALFWORD => 2,
            REV_INR::_7BITS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REV_INR {
        match value {
            0 => REV_INR::NOREVERSE,
            1 => REV_INR::BYTE,
            2 => REV_INR::HALFWORD,
            3 => REV_INR::_7BITS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOREVERSE`"]
    #[inline]
    pub fn is_no_reverse(&self) -> bool {
        *self == REV_INR::NOREVERSE
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline]
    pub fn is_byte(&self) -> bool {
        *self == REV_INR::BYTE
    }
    #[doc = "Checks if the value of the field is `HALFWORD`"]
    #[inline]
    pub fn is_half_word(&self) -> bool {
        *self == REV_INR::HALFWORD
    }
    #[doc = "Checks if the value of the field is `_7BITS`"]
    #[inline]
    pub fn is_7bits(&self) -> bool {
        *self == REV_INR::_7BITS
    }
}
#[doc = "Possible values of the field `REV_OUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV_OUTR {
    #[doc = "Bit order not affected."]
    NOREVERSE,
    #[doc = "Bit-reversed output format."]
    REVERSE,
}
impl REV_OUTR {
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
            REV_OUTR::NOREVERSE => false,
            REV_OUTR::REVERSE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REV_OUTR {
        match value {
            false => REV_OUTR::NOREVERSE,
            true => REV_OUTR::REVERSE,
        }
    }
    #[doc = "Checks if the value of the field is `NOREVERSE`"]
    #[inline]
    pub fn is_no_reverse(&self) -> bool {
        *self == REV_OUTR::NOREVERSE
    }
    #[doc = "Checks if the value of the field is `REVERSE`"]
    #[inline]
    pub fn is_reverse(&self) -> bool {
        *self == REV_OUTR::REVERSE
    }
}
#[doc = "Values that can be written to the field `RESET`"]
pub enum RESETW {
    #[doc = "Resets the CRC calculation unit"]
    RESET,
}
impl RESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESETW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _RESETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Resets the CRC calculation unit"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(RESETW::RESET)
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
#[doc = "Values that can be written to the field `POLYSIZE`"]
pub enum POLYSIZEW {
    #[doc = "32 bit polynomial."]
    _32BITS,
    #[doc = "16 bit polynomial."]
    _16BITS,
    #[doc = "8 bit polynomial."]
    _8BITS,
    #[doc = "7 bit polynomial."]
    _7BITS,
}
impl POLYSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            POLYSIZEW::_32BITS => 0,
            POLYSIZEW::_16BITS => 1,
            POLYSIZEW::_8BITS => 2,
            POLYSIZEW::_7BITS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POLYSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _POLYSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POLYSIZEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "32 bit polynomial."]
    #[inline]
    pub fn _32bits(self) -> &'a mut W {
        self.variant(POLYSIZEW::_32BITS)
    }
    #[doc = "16 bit polynomial."]
    #[inline]
    pub fn _16bits(self) -> &'a mut W {
        self.variant(POLYSIZEW::_16BITS)
    }
    #[doc = "8 bit polynomial."]
    #[inline]
    pub fn _8bits(self) -> &'a mut W {
        self.variant(POLYSIZEW::_8BITS)
    }
    #[doc = "7 bit polynomial."]
    #[inline]
    pub fn _7bits(self) -> &'a mut W {
        self.variant(POLYSIZEW::_7BITS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REV_IN`"]
pub enum REV_INW {
    #[doc = "Bit order not affected."]
    NOREVERSE,
    #[doc = "Bit reversal done by byte."]
    BYTE,
    #[doc = "Bit reversal done by half-word."]
    HALFWORD,
    #[doc = "Bit reversal done by word."]
    _7BITS,
}
impl REV_INW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REV_INW::NOREVERSE => 0,
            REV_INW::BYTE => 1,
            REV_INW::HALFWORD => 2,
            REV_INW::_7BITS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REV_INW<'a> {
    w: &'a mut W,
}
impl<'a> _REV_INW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REV_INW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Bit order not affected."]
    #[inline]
    pub fn no_reverse(self) -> &'a mut W {
        self.variant(REV_INW::NOREVERSE)
    }
    #[doc = "Bit reversal done by byte."]
    #[inline]
    pub fn byte(self) -> &'a mut W {
        self.variant(REV_INW::BYTE)
    }
    #[doc = "Bit reversal done by half-word."]
    #[inline]
    pub fn half_word(self) -> &'a mut W {
        self.variant(REV_INW::HALFWORD)
    }
    #[doc = "Bit reversal done by word."]
    #[inline]
    pub fn _7bits(self) -> &'a mut W {
        self.variant(REV_INW::_7BITS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REV_OUT`"]
pub enum REV_OUTW {
    #[doc = "Bit order not affected."]
    NOREVERSE,
    #[doc = "Bit-reversed output format."]
    REVERSE,
}
impl REV_OUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REV_OUTW::NOREVERSE => false,
            REV_OUTW::REVERSE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REV_OUTW<'a> {
    w: &'a mut W,
}
impl<'a> _REV_OUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REV_OUTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bit order not affected."]
    #[inline]
    pub fn no_reverse(self) -> &'a mut W {
        self.variant(REV_OUTW::NOREVERSE)
    }
    #[doc = "Bit-reversed output format."]
    #[inline]
    pub fn reverse(self) -> &'a mut W {
        self.variant(REV_OUTW::REVERSE)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - reset bit"]
    #[inline]
    pub fn reset(&self) -> RESETR {
        RESETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 3:4 - Polynomial size"]
    #[inline]
    pub fn polysize(&self) -> POLYSIZER {
        POLYSIZER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 5:6 - Reverse input data"]
    #[inline]
    pub fn rev_in(&self) -> REV_INR {
        REV_INR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Reverse output data"]
    #[inline]
    pub fn rev_out(&self) -> REV_OUTR {
        REV_OUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
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
    #[doc = "Bit 0 - reset bit"]
    #[inline]
    pub fn reset(&mut self) -> _RESETW {
        _RESETW { w: self }
    }
    #[doc = "Bits 3:4 - Polynomial size"]
    #[inline]
    pub fn polysize(&mut self) -> _POLYSIZEW {
        _POLYSIZEW { w: self }
    }
    #[doc = "Bits 5:6 - Reverse input data"]
    #[inline]
    pub fn rev_in(&mut self) -> _REV_INW {
        _REV_INW { w: self }
    }
    #[doc = "Bit 7 - Reverse output data"]
    #[inline]
    pub fn rev_out(&mut self) -> _REV_OUTW {
        _REV_OUTW { w: self }
    }
}
