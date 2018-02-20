#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DCR {
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
#[doc = "Possible values of the field `DBL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBLR {
    #[doc = "1 transfert."]
    _1TRANSFERT,
    #[doc = "2 transferts."]
    _2TRANSFERTS,
    #[doc = "3 transferts."]
    _3TRANSFERTS,
    #[doc = "4 transferts."]
    _4TRANSFERTS,
    #[doc = "5 transferts."]
    _5TRANSFERTS,
    #[doc = "6 transferts."]
    _6TRANSFERTS,
    #[doc = "7 transferts."]
    _7TRANSFERTS,
    #[doc = "8 transferts."]
    _8TRANSFERTS,
    #[doc = "9 transferts."]
    _9TRANSFERTS,
    #[doc = "10 transferts."]
    _10TRANSFERTS,
    #[doc = "11 transferts."]
    _11TRANSFERTS,
    #[doc = "12 transferts."]
    _12TRANSFERTS,
    #[doc = "13 transferts."]
    _13TRANSFERTS,
    #[doc = "14 transferts."]
    _14TRANSFERTS,
    #[doc = "15 transferts."]
    _15TRANSFERTS,
    #[doc = "16 transferts."]
    _16TRANSFERTS,
    #[doc = "17 transferts."]
    _17TRANSFERTS,
    #[doc = "18 transferts."]
    _18TRANSFERTS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DBLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DBLR::_1TRANSFERT => 0,
            DBLR::_2TRANSFERTS => 1,
            DBLR::_3TRANSFERTS => 2,
            DBLR::_4TRANSFERTS => 3,
            DBLR::_5TRANSFERTS => 4,
            DBLR::_6TRANSFERTS => 5,
            DBLR::_7TRANSFERTS => 6,
            DBLR::_8TRANSFERTS => 7,
            DBLR::_9TRANSFERTS => 8,
            DBLR::_10TRANSFERTS => 9,
            DBLR::_11TRANSFERTS => 10,
            DBLR::_12TRANSFERTS => 11,
            DBLR::_13TRANSFERTS => 12,
            DBLR::_14TRANSFERTS => 13,
            DBLR::_15TRANSFERTS => 14,
            DBLR::_16TRANSFERTS => 15,
            DBLR::_17TRANSFERTS => 16,
            DBLR::_18TRANSFERTS => 17,
            DBLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DBLR {
        match value {
            0 => DBLR::_1TRANSFERT,
            1 => DBLR::_2TRANSFERTS,
            2 => DBLR::_3TRANSFERTS,
            3 => DBLR::_4TRANSFERTS,
            4 => DBLR::_5TRANSFERTS,
            5 => DBLR::_6TRANSFERTS,
            6 => DBLR::_7TRANSFERTS,
            7 => DBLR::_8TRANSFERTS,
            8 => DBLR::_9TRANSFERTS,
            9 => DBLR::_10TRANSFERTS,
            10 => DBLR::_11TRANSFERTS,
            11 => DBLR::_12TRANSFERTS,
            12 => DBLR::_13TRANSFERTS,
            13 => DBLR::_14TRANSFERTS,
            14 => DBLR::_15TRANSFERTS,
            15 => DBLR::_16TRANSFERTS,
            16 => DBLR::_17TRANSFERTS,
            17 => DBLR::_18TRANSFERTS,
            i => DBLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1TRANSFERT`"]
    #[inline]
    pub fn is_1transfert(&self) -> bool {
        *self == DBLR::_1TRANSFERT
    }
    #[doc = "Checks if the value of the field is `_2TRANSFERTS`"]
    #[inline]
    pub fn is_2transferts(&self) -> bool {
        *self == DBLR::_2TRANSFERTS
    }
    #[doc = "Checks if the value of the field is `_3TRANSFERTS`"]
    #[inline]
    pub fn is_3transferts(&self) -> bool {
        *self == DBLR::_3TRANSFERTS
    }
    #[doc = "Checks if the value of the field is `_4TRANSFERTS`"]
    #[inline]
    pub fn is_4transferts(&self) -> bool {
        *self == DBLR::_4TRANSFERTS
    }
    #[doc = "Checks if the value of the field is `_5TRANSFERTS`"]
    #[inline]
    pub fn is_5transferts(&self) -> bool {
        *self == DBLR::_5TRANSFERTS
    }
    #[doc = "Checks if the value of the field is `_6TRANSFERTS`"]
    #[inline]
    pub fn is_6transferts(&self) -> bool {
        *self == DBLR::_6TRANSFERTS
    }
    #[doc = "Checks if the value of the field is `_7TRANSFERTS`"]
    #[inline]
    pub fn is_7transferts(&self) -> bool {
        *self == DBLR::_7TRANSFERTS
    }
    #[doc = "Checks if the value of the field is `_8TRANSFERTS`"]
    #[inline]
    pub fn is_8transferts(&self) -> bool {
        *self == DBLR::_8TRANSFERTS
    }
    #[doc = "Checks if the value of the field is `_9TRANSFERTS`"]
    #[inline]
    pub fn is_9transferts(&self) -> bool {
        *self == DBLR::_9TRANSFERTS
    }
    #[doc = "Checks if the value of the field is `_10TRANSFERTS`"]
    #[inline]
    pub fn is_10transferts(&self) -> bool {
        *self == DBLR::_10TRANSFERTS
    }
    #[doc = "Checks if the value of the field is `_11TRANSFERTS`"]
    #[inline]
    pub fn is_11transferts(&self) -> bool {
        *self == DBLR::_11TRANSFERTS
    }
    #[doc = "Checks if the value of the field is `_12TRANSFERTS`"]
    #[inline]
    pub fn is_12transferts(&self) -> bool {
        *self == DBLR::_12TRANSFERTS
    }
    #[doc = "Checks if the value of the field is `_13TRANSFERTS`"]
    #[inline]
    pub fn is_13transferts(&self) -> bool {
        *self == DBLR::_13TRANSFERTS
    }
    #[doc = "Checks if the value of the field is `_14TRANSFERTS`"]
    #[inline]
    pub fn is_14transferts(&self) -> bool {
        *self == DBLR::_14TRANSFERTS
    }
    #[doc = "Checks if the value of the field is `_15TRANSFERTS`"]
    #[inline]
    pub fn is_15transferts(&self) -> bool {
        *self == DBLR::_15TRANSFERTS
    }
    #[doc = "Checks if the value of the field is `_16TRANSFERTS`"]
    #[inline]
    pub fn is_16transferts(&self) -> bool {
        *self == DBLR::_16TRANSFERTS
    }
    #[doc = "Checks if the value of the field is `_17TRANSFERTS`"]
    #[inline]
    pub fn is_17transferts(&self) -> bool {
        *self == DBLR::_17TRANSFERTS
    }
    #[doc = "Checks if the value of the field is `_18TRANSFERTS`"]
    #[inline]
    pub fn is_18transferts(&self) -> bool {
        *self == DBLR::_18TRANSFERTS
    }
}
#[doc = r" Value of the field"]
pub struct DBAR {
    bits: u8,
}
impl DBAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `DBL`"]
pub enum DBLW {
    #[doc = "1 transfert."]
    _1TRANSFERT,
    #[doc = "2 transferts."]
    _2TRANSFERTS,
    #[doc = "3 transferts."]
    _3TRANSFERTS,
    #[doc = "4 transferts."]
    _4TRANSFERTS,
    #[doc = "5 transferts."]
    _5TRANSFERTS,
    #[doc = "6 transferts."]
    _6TRANSFERTS,
    #[doc = "7 transferts."]
    _7TRANSFERTS,
    #[doc = "8 transferts."]
    _8TRANSFERTS,
    #[doc = "9 transferts."]
    _9TRANSFERTS,
    #[doc = "10 transferts."]
    _10TRANSFERTS,
    #[doc = "11 transferts."]
    _11TRANSFERTS,
    #[doc = "12 transferts."]
    _12TRANSFERTS,
    #[doc = "13 transferts."]
    _13TRANSFERTS,
    #[doc = "14 transferts."]
    _14TRANSFERTS,
    #[doc = "15 transferts."]
    _15TRANSFERTS,
    #[doc = "16 transferts."]
    _16TRANSFERTS,
    #[doc = "17 transferts."]
    _17TRANSFERTS,
    #[doc = "18 transferts."]
    _18TRANSFERTS,
}
impl DBLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DBLW::_1TRANSFERT => 0,
            DBLW::_2TRANSFERTS => 1,
            DBLW::_3TRANSFERTS => 2,
            DBLW::_4TRANSFERTS => 3,
            DBLW::_5TRANSFERTS => 4,
            DBLW::_6TRANSFERTS => 5,
            DBLW::_7TRANSFERTS => 6,
            DBLW::_8TRANSFERTS => 7,
            DBLW::_9TRANSFERTS => 8,
            DBLW::_10TRANSFERTS => 9,
            DBLW::_11TRANSFERTS => 10,
            DBLW::_12TRANSFERTS => 11,
            DBLW::_13TRANSFERTS => 12,
            DBLW::_14TRANSFERTS => 13,
            DBLW::_15TRANSFERTS => 14,
            DBLW::_16TRANSFERTS => 15,
            DBLW::_17TRANSFERTS => 16,
            DBLW::_18TRANSFERTS => 17,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DBLW<'a> {
    w: &'a mut W,
}
impl<'a> _DBLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DBLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 transfert."]
    #[inline]
    pub fn _1transfert(self) -> &'a mut W {
        self.variant(DBLW::_1TRANSFERT)
    }
    #[doc = "2 transferts."]
    #[inline]
    pub fn _2transferts(self) -> &'a mut W {
        self.variant(DBLW::_2TRANSFERTS)
    }
    #[doc = "3 transferts."]
    #[inline]
    pub fn _3transferts(self) -> &'a mut W {
        self.variant(DBLW::_3TRANSFERTS)
    }
    #[doc = "4 transferts."]
    #[inline]
    pub fn _4transferts(self) -> &'a mut W {
        self.variant(DBLW::_4TRANSFERTS)
    }
    #[doc = "5 transferts."]
    #[inline]
    pub fn _5transferts(self) -> &'a mut W {
        self.variant(DBLW::_5TRANSFERTS)
    }
    #[doc = "6 transferts."]
    #[inline]
    pub fn _6transferts(self) -> &'a mut W {
        self.variant(DBLW::_6TRANSFERTS)
    }
    #[doc = "7 transferts."]
    #[inline]
    pub fn _7transferts(self) -> &'a mut W {
        self.variant(DBLW::_7TRANSFERTS)
    }
    #[doc = "8 transferts."]
    #[inline]
    pub fn _8transferts(self) -> &'a mut W {
        self.variant(DBLW::_8TRANSFERTS)
    }
    #[doc = "9 transferts."]
    #[inline]
    pub fn _9transferts(self) -> &'a mut W {
        self.variant(DBLW::_9TRANSFERTS)
    }
    #[doc = "10 transferts."]
    #[inline]
    pub fn _10transferts(self) -> &'a mut W {
        self.variant(DBLW::_10TRANSFERTS)
    }
    #[doc = "11 transferts."]
    #[inline]
    pub fn _11transferts(self) -> &'a mut W {
        self.variant(DBLW::_11TRANSFERTS)
    }
    #[doc = "12 transferts."]
    #[inline]
    pub fn _12transferts(self) -> &'a mut W {
        self.variant(DBLW::_12TRANSFERTS)
    }
    #[doc = "13 transferts."]
    #[inline]
    pub fn _13transferts(self) -> &'a mut W {
        self.variant(DBLW::_13TRANSFERTS)
    }
    #[doc = "14 transferts."]
    #[inline]
    pub fn _14transferts(self) -> &'a mut W {
        self.variant(DBLW::_14TRANSFERTS)
    }
    #[doc = "15 transferts."]
    #[inline]
    pub fn _15transferts(self) -> &'a mut W {
        self.variant(DBLW::_15TRANSFERTS)
    }
    #[doc = "16 transferts."]
    #[inline]
    pub fn _16transferts(self) -> &'a mut W {
        self.variant(DBLW::_16TRANSFERTS)
    }
    #[doc = "17 transferts."]
    #[inline]
    pub fn _17transferts(self) -> &'a mut W {
        self.variant(DBLW::_17TRANSFERTS)
    }
    #[doc = "18 transferts."]
    #[inline]
    pub fn _18transferts(self) -> &'a mut W {
        self.variant(DBLW::_18TRANSFERTS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DBAW<'a> {
    w: &'a mut W,
}
impl<'a> _DBAW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 8:12 - DMA burst length"]
    #[inline]
    pub fn dbl(&self) -> DBLR {
        DBLR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:4 - DMA base address"]
    #[inline]
    pub fn dba(&self) -> DBAR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DBAR { bits }
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
    #[doc = "Bits 8:12 - DMA burst length"]
    #[inline]
    pub fn dbl(&mut self) -> _DBLW {
        _DBLW { w: self }
    }
    #[doc = "Bits 0:4 - DMA base address"]
    #[inline]
    pub fn dba(&mut self) -> _DBAW {
        _DBAW { w: self }
    }
}
