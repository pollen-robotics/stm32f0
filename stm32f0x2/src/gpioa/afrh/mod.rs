#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AFRH {
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
#[doc = "Possible values of the field `AFRH15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFRH15R {
    #[doc = "Alternate Function 0"]
    AF0,
    #[doc = "Alternate Function 1"]
    AF1,
    #[doc = "Alternate Function 2"]
    AF2,
    #[doc = "Alternate Function 3"]
    AF3,
    #[doc = "Alternate Function 4"]
    AF4,
    #[doc = "Alternate Function 5"]
    AF5,
    #[doc = "Alternate Function 6"]
    AF6,
    #[doc = "Alternate Function 7"]
    AF7,
}
impl AFRH15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AFRH15R::AF0 => 0,
            AFRH15R::AF1 => 1,
            AFRH15R::AF2 => 2,
            AFRH15R::AF3 => 3,
            AFRH15R::AF4 => 4,
            AFRH15R::AF5 => 5,
            AFRH15R::AF6 => 6,
            AFRH15R::AF7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AFRH15R {
        match value {
            0 => AFRH15R::AF0,
            1 => AFRH15R::AF1,
            2 => AFRH15R::AF2,
            3 => AFRH15R::AF3,
            4 => AFRH15R::AF4,
            5 => AFRH15R::AF5,
            6 => AFRH15R::AF6,
            7 => AFRH15R::AF7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AF0`"]
    #[inline]
    pub fn is_af0(&self) -> bool {
        *self == AFRH15R::AF0
    }
    #[doc = "Checks if the value of the field is `AF1`"]
    #[inline]
    pub fn is_af1(&self) -> bool {
        *self == AFRH15R::AF1
    }
    #[doc = "Checks if the value of the field is `AF2`"]
    #[inline]
    pub fn is_af2(&self) -> bool {
        *self == AFRH15R::AF2
    }
    #[doc = "Checks if the value of the field is `AF3`"]
    #[inline]
    pub fn is_af3(&self) -> bool {
        *self == AFRH15R::AF3
    }
    #[doc = "Checks if the value of the field is `AF4`"]
    #[inline]
    pub fn is_af4(&self) -> bool {
        *self == AFRH15R::AF4
    }
    #[doc = "Checks if the value of the field is `AF5`"]
    #[inline]
    pub fn is_af5(&self) -> bool {
        *self == AFRH15R::AF5
    }
    #[doc = "Checks if the value of the field is `AF6`"]
    #[inline]
    pub fn is_af6(&self) -> bool {
        *self == AFRH15R::AF6
    }
    #[doc = "Checks if the value of the field is `AF7`"]
    #[inline]
    pub fn is_af7(&self) -> bool {
        *self == AFRH15R::AF7
    }
}
#[doc = "Possible values of the field `AFRH14`"]
pub type AFRH14R = AFRH15R;
#[doc = "Possible values of the field `AFRH13`"]
pub type AFRH13R = AFRH15R;
#[doc = "Possible values of the field `AFRH12`"]
pub type AFRH12R = AFRH15R;
#[doc = "Possible values of the field `AFRH11`"]
pub type AFRH11R = AFRH15R;
#[doc = "Possible values of the field `AFRH10`"]
pub type AFRH10R = AFRH15R;
#[doc = "Possible values of the field `AFRH9`"]
pub type AFRH9R = AFRH15R;
#[doc = "Possible values of the field `AFRH8`"]
pub type AFRH8R = AFRH15R;
#[doc = "Values that can be written to the field `AFRH15`"]
pub enum AFRH15W {
    #[doc = "Alternate Function 0"]
    AF0,
    #[doc = "Alternate Function 1"]
    AF1,
    #[doc = "Alternate Function 2"]
    AF2,
    #[doc = "Alternate Function 3"]
    AF3,
    #[doc = "Alternate Function 4"]
    AF4,
    #[doc = "Alternate Function 5"]
    AF5,
    #[doc = "Alternate Function 6"]
    AF6,
    #[doc = "Alternate Function 7"]
    AF7,
}
impl AFRH15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AFRH15W::AF0 => 0,
            AFRH15W::AF1 => 1,
            AFRH15W::AF2 => 2,
            AFRH15W::AF3 => 3,
            AFRH15W::AF4 => 4,
            AFRH15W::AF5 => 5,
            AFRH15W::AF6 => 6,
            AFRH15W::AF7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AFRH15W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRH15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRH15W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Alternate Function 0"]
    #[inline]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFRH15W::AF0)
    }
    #[doc = "Alternate Function 1"]
    #[inline]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRH15W::AF1)
    }
    #[doc = "Alternate Function 2"]
    #[inline]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRH15W::AF2)
    }
    #[doc = "Alternate Function 3"]
    #[inline]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRH15W::AF3)
    }
    #[doc = "Alternate Function 4"]
    #[inline]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRH15W::AF4)
    }
    #[doc = "Alternate Function 5"]
    #[inline]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRH15W::AF5)
    }
    #[doc = "Alternate Function 6"]
    #[inline]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRH15W::AF6)
    }
    #[doc = "Alternate Function 7"]
    #[inline]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRH15W::AF7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AFRH14`"]
pub type AFRH14W = AFRH15W;
#[doc = r" Proxy"]
pub struct _AFRH14W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRH14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRH14W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Alternate Function 0"]
    #[inline]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFRH15W::AF0)
    }
    #[doc = "Alternate Function 1"]
    #[inline]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRH15W::AF1)
    }
    #[doc = "Alternate Function 2"]
    #[inline]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRH15W::AF2)
    }
    #[doc = "Alternate Function 3"]
    #[inline]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRH15W::AF3)
    }
    #[doc = "Alternate Function 4"]
    #[inline]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRH15W::AF4)
    }
    #[doc = "Alternate Function 5"]
    #[inline]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRH15W::AF5)
    }
    #[doc = "Alternate Function 6"]
    #[inline]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRH15W::AF6)
    }
    #[doc = "Alternate Function 7"]
    #[inline]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRH15W::AF7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AFRH13`"]
pub type AFRH13W = AFRH15W;
#[doc = r" Proxy"]
pub struct _AFRH13W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRH13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRH13W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Alternate Function 0"]
    #[inline]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFRH15W::AF0)
    }
    #[doc = "Alternate Function 1"]
    #[inline]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRH15W::AF1)
    }
    #[doc = "Alternate Function 2"]
    #[inline]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRH15W::AF2)
    }
    #[doc = "Alternate Function 3"]
    #[inline]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRH15W::AF3)
    }
    #[doc = "Alternate Function 4"]
    #[inline]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRH15W::AF4)
    }
    #[doc = "Alternate Function 5"]
    #[inline]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRH15W::AF5)
    }
    #[doc = "Alternate Function 6"]
    #[inline]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRH15W::AF6)
    }
    #[doc = "Alternate Function 7"]
    #[inline]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRH15W::AF7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AFRH12`"]
pub type AFRH12W = AFRH15W;
#[doc = r" Proxy"]
pub struct _AFRH12W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRH12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRH12W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Alternate Function 0"]
    #[inline]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFRH15W::AF0)
    }
    #[doc = "Alternate Function 1"]
    #[inline]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRH15W::AF1)
    }
    #[doc = "Alternate Function 2"]
    #[inline]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRH15W::AF2)
    }
    #[doc = "Alternate Function 3"]
    #[inline]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRH15W::AF3)
    }
    #[doc = "Alternate Function 4"]
    #[inline]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRH15W::AF4)
    }
    #[doc = "Alternate Function 5"]
    #[inline]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRH15W::AF5)
    }
    #[doc = "Alternate Function 6"]
    #[inline]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRH15W::AF6)
    }
    #[doc = "Alternate Function 7"]
    #[inline]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRH15W::AF7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AFRH11`"]
pub type AFRH11W = AFRH15W;
#[doc = r" Proxy"]
pub struct _AFRH11W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRH11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRH11W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Alternate Function 0"]
    #[inline]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFRH15W::AF0)
    }
    #[doc = "Alternate Function 1"]
    #[inline]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRH15W::AF1)
    }
    #[doc = "Alternate Function 2"]
    #[inline]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRH15W::AF2)
    }
    #[doc = "Alternate Function 3"]
    #[inline]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRH15W::AF3)
    }
    #[doc = "Alternate Function 4"]
    #[inline]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRH15W::AF4)
    }
    #[doc = "Alternate Function 5"]
    #[inline]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRH15W::AF5)
    }
    #[doc = "Alternate Function 6"]
    #[inline]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRH15W::AF6)
    }
    #[doc = "Alternate Function 7"]
    #[inline]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRH15W::AF7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AFRH10`"]
pub type AFRH10W = AFRH15W;
#[doc = r" Proxy"]
pub struct _AFRH10W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRH10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRH10W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Alternate Function 0"]
    #[inline]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFRH15W::AF0)
    }
    #[doc = "Alternate Function 1"]
    #[inline]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRH15W::AF1)
    }
    #[doc = "Alternate Function 2"]
    #[inline]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRH15W::AF2)
    }
    #[doc = "Alternate Function 3"]
    #[inline]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRH15W::AF3)
    }
    #[doc = "Alternate Function 4"]
    #[inline]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRH15W::AF4)
    }
    #[doc = "Alternate Function 5"]
    #[inline]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRH15W::AF5)
    }
    #[doc = "Alternate Function 6"]
    #[inline]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRH15W::AF6)
    }
    #[doc = "Alternate Function 7"]
    #[inline]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRH15W::AF7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AFRH9`"]
pub type AFRH9W = AFRH15W;
#[doc = r" Proxy"]
pub struct _AFRH9W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRH9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRH9W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Alternate Function 0"]
    #[inline]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFRH15W::AF0)
    }
    #[doc = "Alternate Function 1"]
    #[inline]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRH15W::AF1)
    }
    #[doc = "Alternate Function 2"]
    #[inline]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRH15W::AF2)
    }
    #[doc = "Alternate Function 3"]
    #[inline]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRH15W::AF3)
    }
    #[doc = "Alternate Function 4"]
    #[inline]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRH15W::AF4)
    }
    #[doc = "Alternate Function 5"]
    #[inline]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRH15W::AF5)
    }
    #[doc = "Alternate Function 6"]
    #[inline]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRH15W::AF6)
    }
    #[doc = "Alternate Function 7"]
    #[inline]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRH15W::AF7)
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
#[doc = "Values that can be written to the field `AFRH8`"]
pub type AFRH8W = AFRH15W;
#[doc = r" Proxy"]
pub struct _AFRH8W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRH8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRH8W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Alternate Function 0"]
    #[inline]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFRH15W::AF0)
    }
    #[doc = "Alternate Function 1"]
    #[inline]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRH15W::AF1)
    }
    #[doc = "Alternate Function 2"]
    #[inline]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRH15W::AF2)
    }
    #[doc = "Alternate Function 3"]
    #[inline]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRH15W::AF3)
    }
    #[doc = "Alternate Function 4"]
    #[inline]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRH15W::AF4)
    }
    #[doc = "Alternate Function 5"]
    #[inline]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRH15W::AF5)
    }
    #[doc = "Alternate Function 6"]
    #[inline]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRH15W::AF6)
    }
    #[doc = "Alternate Function 7"]
    #[inline]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRH15W::AF7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline]
    pub fn afrh15(&self) -> AFRH15R {
        AFRH15R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline]
    pub fn afrh14(&self) -> AFRH14R {
        AFRH14R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline]
    pub fn afrh13(&self) -> AFRH13R {
        AFRH13R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline]
    pub fn afrh12(&self) -> AFRH12R {
        AFRH12R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline]
    pub fn afrh11(&self) -> AFRH11R {
        AFRH11R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline]
    pub fn afrh10(&self) -> AFRH10R {
        AFRH10R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline]
    pub fn afrh9(&self) -> AFRH9R {
        AFRH9R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline]
    pub fn afrh8(&self) -> AFRH8R {
        AFRH8R::_from({
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
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline]
    pub fn afrh15(&mut self) -> _AFRH15W {
        _AFRH15W { w: self }
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline]
    pub fn afrh14(&mut self) -> _AFRH14W {
        _AFRH14W { w: self }
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline]
    pub fn afrh13(&mut self) -> _AFRH13W {
        _AFRH13W { w: self }
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline]
    pub fn afrh12(&mut self) -> _AFRH12W {
        _AFRH12W { w: self }
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline]
    pub fn afrh11(&mut self) -> _AFRH11W {
        _AFRH11W { w: self }
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline]
    pub fn afrh10(&mut self) -> _AFRH10W {
        _AFRH10W { w: self }
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline]
    pub fn afrh9(&mut self) -> _AFRH9W {
        _AFRH9W { w: self }
    }
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline]
    pub fn afrh8(&mut self) -> _AFRH8W {
        _AFRH8W { w: self }
    }
}
