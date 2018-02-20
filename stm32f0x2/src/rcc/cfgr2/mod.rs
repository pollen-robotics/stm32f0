#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFGR2 {
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
#[doc = "Possible values of the field `PREDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREDIVR {
    #[doc = "PREDIV input clock not divided."]
    DIV1,
    #[doc = "PREDIV input clock divided by 2."]
    DIV2,
    #[doc = "PREDIV input clock divided by 3."]
    DIV3,
    #[doc = "PREDIV input clock divided by 4."]
    DIV4,
    #[doc = "PREDIV input clock divided by 5."]
    DIV5,
    #[doc = "PREDIV input clock divided by 6."]
    DIV6,
    #[doc = "PREDIV input clock divided by 7."]
    DIV7,
    #[doc = "PREDIV input clock divided by 8."]
    DIV8,
    #[doc = "PREDIV input clock divided by 9."]
    DIV9,
    #[doc = "PREDIV input clock divided by 10."]
    DIV10,
    #[doc = "PREDIV input clock divided by 11."]
    DIV11,
    #[doc = "PREDIV input clock divided by 12."]
    DIV12,
    #[doc = "PREDIV input clock divided by 13."]
    DIV13,
    #[doc = "PREDIV input clock divided by 14."]
    DIV14,
    #[doc = "PREDIV input clock divided by 15."]
    DIV15,
    #[doc = "PREDIV input clock divided by 16."]
    DIV16,
}
impl PREDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PREDIVR::DIV1 => 0,
            PREDIVR::DIV2 => 1,
            PREDIVR::DIV3 => 2,
            PREDIVR::DIV4 => 3,
            PREDIVR::DIV5 => 4,
            PREDIVR::DIV6 => 5,
            PREDIVR::DIV7 => 6,
            PREDIVR::DIV8 => 7,
            PREDIVR::DIV9 => 8,
            PREDIVR::DIV10 => 9,
            PREDIVR::DIV11 => 10,
            PREDIVR::DIV12 => 11,
            PREDIVR::DIV13 => 12,
            PREDIVR::DIV14 => 13,
            PREDIVR::DIV15 => 14,
            PREDIVR::DIV16 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PREDIVR {
        match value {
            0 => PREDIVR::DIV1,
            1 => PREDIVR::DIV2,
            2 => PREDIVR::DIV3,
            3 => PREDIVR::DIV4,
            4 => PREDIVR::DIV5,
            5 => PREDIVR::DIV6,
            6 => PREDIVR::DIV7,
            7 => PREDIVR::DIV8,
            8 => PREDIVR::DIV9,
            9 => PREDIVR::DIV10,
            10 => PREDIVR::DIV11,
            11 => PREDIVR::DIV12,
            12 => PREDIVR::DIV13,
            13 => PREDIVR::DIV14,
            14 => PREDIVR::DIV15,
            15 => PREDIVR::DIV16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == PREDIVR::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == PREDIVR::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV3`"]
    #[inline]
    pub fn is_div3(&self) -> bool {
        *self == PREDIVR::DIV3
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == PREDIVR::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV5`"]
    #[inline]
    pub fn is_div5(&self) -> bool {
        *self == PREDIVR::DIV5
    }
    #[doc = "Checks if the value of the field is `DIV6`"]
    #[inline]
    pub fn is_div6(&self) -> bool {
        *self == PREDIVR::DIV6
    }
    #[doc = "Checks if the value of the field is `DIV7`"]
    #[inline]
    pub fn is_div7(&self) -> bool {
        *self == PREDIVR::DIV7
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == PREDIVR::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV9`"]
    #[inline]
    pub fn is_div9(&self) -> bool {
        *self == PREDIVR::DIV9
    }
    #[doc = "Checks if the value of the field is `DIV10`"]
    #[inline]
    pub fn is_div10(&self) -> bool {
        *self == PREDIVR::DIV10
    }
    #[doc = "Checks if the value of the field is `DIV11`"]
    #[inline]
    pub fn is_div11(&self) -> bool {
        *self == PREDIVR::DIV11
    }
    #[doc = "Checks if the value of the field is `DIV12`"]
    #[inline]
    pub fn is_div12(&self) -> bool {
        *self == PREDIVR::DIV12
    }
    #[doc = "Checks if the value of the field is `DIV13`"]
    #[inline]
    pub fn is_div13(&self) -> bool {
        *self == PREDIVR::DIV13
    }
    #[doc = "Checks if the value of the field is `DIV14`"]
    #[inline]
    pub fn is_div14(&self) -> bool {
        *self == PREDIVR::DIV14
    }
    #[doc = "Checks if the value of the field is `DIV15`"]
    #[inline]
    pub fn is_div15(&self) -> bool {
        *self == PREDIVR::DIV15
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == PREDIVR::DIV16
    }
}
#[doc = "Values that can be written to the field `PREDIV`"]
pub enum PREDIVW {
    #[doc = "PREDIV input clock not divided."]
    DIV1,
    #[doc = "PREDIV input clock divided by 2."]
    DIV2,
    #[doc = "PREDIV input clock divided by 3."]
    DIV3,
    #[doc = "PREDIV input clock divided by 4."]
    DIV4,
    #[doc = "PREDIV input clock divided by 5."]
    DIV5,
    #[doc = "PREDIV input clock divided by 6."]
    DIV6,
    #[doc = "PREDIV input clock divided by 7."]
    DIV7,
    #[doc = "PREDIV input clock divided by 8."]
    DIV8,
    #[doc = "PREDIV input clock divided by 9."]
    DIV9,
    #[doc = "PREDIV input clock divided by 10."]
    DIV10,
    #[doc = "PREDIV input clock divided by 11."]
    DIV11,
    #[doc = "PREDIV input clock divided by 12."]
    DIV12,
    #[doc = "PREDIV input clock divided by 13."]
    DIV13,
    #[doc = "PREDIV input clock divided by 14."]
    DIV14,
    #[doc = "PREDIV input clock divided by 15."]
    DIV15,
    #[doc = "PREDIV input clock divided by 16."]
    DIV16,
}
impl PREDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PREDIVW::DIV1 => 0,
            PREDIVW::DIV2 => 1,
            PREDIVW::DIV3 => 2,
            PREDIVW::DIV4 => 3,
            PREDIVW::DIV5 => 4,
            PREDIVW::DIV6 => 5,
            PREDIVW::DIV7 => 6,
            PREDIVW::DIV8 => 7,
            PREDIVW::DIV9 => 8,
            PREDIVW::DIV10 => 9,
            PREDIVW::DIV11 => 10,
            PREDIVW::DIV12 => 11,
            PREDIVW::DIV13 => 12,
            PREDIVW::DIV14 => 13,
            PREDIVW::DIV15 => 14,
            PREDIVW::DIV16 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PREDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _PREDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PREDIVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "PREDIV input clock not divided."]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(PREDIVW::DIV1)
    }
    #[doc = "PREDIV input clock divided by 2."]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(PREDIVW::DIV2)
    }
    #[doc = "PREDIV input clock divided by 3."]
    #[inline]
    pub fn div3(self) -> &'a mut W {
        self.variant(PREDIVW::DIV3)
    }
    #[doc = "PREDIV input clock divided by 4."]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(PREDIVW::DIV4)
    }
    #[doc = "PREDIV input clock divided by 5."]
    #[inline]
    pub fn div5(self) -> &'a mut W {
        self.variant(PREDIVW::DIV5)
    }
    #[doc = "PREDIV input clock divided by 6."]
    #[inline]
    pub fn div6(self) -> &'a mut W {
        self.variant(PREDIVW::DIV6)
    }
    #[doc = "PREDIV input clock divided by 7."]
    #[inline]
    pub fn div7(self) -> &'a mut W {
        self.variant(PREDIVW::DIV7)
    }
    #[doc = "PREDIV input clock divided by 8."]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(PREDIVW::DIV8)
    }
    #[doc = "PREDIV input clock divided by 9."]
    #[inline]
    pub fn div9(self) -> &'a mut W {
        self.variant(PREDIVW::DIV9)
    }
    #[doc = "PREDIV input clock divided by 10."]
    #[inline]
    pub fn div10(self) -> &'a mut W {
        self.variant(PREDIVW::DIV10)
    }
    #[doc = "PREDIV input clock divided by 11."]
    #[inline]
    pub fn div11(self) -> &'a mut W {
        self.variant(PREDIVW::DIV11)
    }
    #[doc = "PREDIV input clock divided by 12."]
    #[inline]
    pub fn div12(self) -> &'a mut W {
        self.variant(PREDIVW::DIV12)
    }
    #[doc = "PREDIV input clock divided by 13."]
    #[inline]
    pub fn div13(self) -> &'a mut W {
        self.variant(PREDIVW::DIV13)
    }
    #[doc = "PREDIV input clock divided by 14."]
    #[inline]
    pub fn div14(self) -> &'a mut W {
        self.variant(PREDIVW::DIV14)
    }
    #[doc = "PREDIV input clock divided by 15."]
    #[inline]
    pub fn div15(self) -> &'a mut W {
        self.variant(PREDIVW::DIV15)
    }
    #[doc = "PREDIV input clock divided by 16."]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(PREDIVW::DIV16)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bits 0:3 - PREDIV division factor"]
    #[inline]
    pub fn prediv(&self) -> PREDIVR {
        PREDIVR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:3 - PREDIV division factor"]
    #[inline]
    pub fn prediv(&mut self) -> _PREDIVW {
        _PREDIVW { w: self }
    }
}
