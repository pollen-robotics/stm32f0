#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AFRL {
    #[doc = r" Modifies the contents of the register"]
    #[inline(always)]
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
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `AFRL7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFRL7R {
    #[doc = "Alternate Function 0"] AF0,
    #[doc = "Alternate Function 1"] AF1,
    #[doc = "Alternate Function 2"] AF2,
    #[doc = "Alternate Function 3"] AF3,
    #[doc = "Alternate Function 4"] AF4,
    #[doc = "Alternate Function 5"] AF5,
    #[doc = "Alternate Function 6"] AF6,
    #[doc = "Alternate Function 7"] AF7,
}
impl AFRL7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            AFRL7R::AF0 => 0,
            AFRL7R::AF1 => 1,
            AFRL7R::AF2 => 2,
            AFRL7R::AF3 => 3,
            AFRL7R::AF4 => 4,
            AFRL7R::AF5 => 5,
            AFRL7R::AF6 => 6,
            AFRL7R::AF7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> AFRL7R {
        match value {
            0 => AFRL7R::AF0,
            1 => AFRL7R::AF1,
            2 => AFRL7R::AF2,
            3 => AFRL7R::AF3,
            4 => AFRL7R::AF4,
            5 => AFRL7R::AF5,
            6 => AFRL7R::AF6,
            7 => AFRL7R::AF7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AF0`"]
    #[inline(always)]
    pub fn is_af0(&self) -> bool {
        *self == AFRL7R::AF0
    }
    #[doc = "Checks if the value of the field is `AF1`"]
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == AFRL7R::AF1
    }
    #[doc = "Checks if the value of the field is `AF2`"]
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == AFRL7R::AF2
    }
    #[doc = "Checks if the value of the field is `AF3`"]
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == AFRL7R::AF3
    }
    #[doc = "Checks if the value of the field is `AF4`"]
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == AFRL7R::AF4
    }
    #[doc = "Checks if the value of the field is `AF5`"]
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == AFRL7R::AF5
    }
    #[doc = "Checks if the value of the field is `AF6`"]
    #[inline(always)]
    pub fn is_af6(&self) -> bool {
        *self == AFRL7R::AF6
    }
    #[doc = "Checks if the value of the field is `AF7`"]
    #[inline(always)]
    pub fn is_af7(&self) -> bool {
        *self == AFRL7R::AF7
    }
}
#[doc = "Possible values of the field `AFRL6`"]
pub type AFRL6R = AFRL7R;
#[doc = "Possible values of the field `AFRL5`"]
pub type AFRL5R = AFRL7R;
#[doc = "Possible values of the field `AFRL4`"]
pub type AFRL4R = AFRL7R;
#[doc = "Possible values of the field `AFRL3`"]
pub type AFRL3R = AFRL7R;
#[doc = "Possible values of the field `AFRL2`"]
pub type AFRL2R = AFRL7R;
#[doc = "Possible values of the field `AFRL1`"]
pub type AFRL1R = AFRL7R;
#[doc = "Possible values of the field `AFRL0`"]
pub type AFRL0R = AFRL7R;
#[doc = "Values that can be written to the field `AFRL7`"]
pub enum AFRL7W {
    #[doc = "Alternate Function 0"] AF0,
    #[doc = "Alternate Function 1"] AF1,
    #[doc = "Alternate Function 2"] AF2,
    #[doc = "Alternate Function 3"] AF3,
    #[doc = "Alternate Function 4"] AF4,
    #[doc = "Alternate Function 5"] AF5,
    #[doc = "Alternate Function 6"] AF6,
    #[doc = "Alternate Function 7"] AF7,
}
impl AFRL7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            AFRL7W::AF0 => 0,
            AFRL7W::AF1 => 1,
            AFRL7W::AF2 => 2,
            AFRL7W::AF3 => 3,
            AFRL7W::AF4 => 4,
            AFRL7W::AF5 => 5,
            AFRL7W::AF6 => 6,
            AFRL7W::AF7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AFRL7W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRL7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AFRL7W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Alternate Function 0"]
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFRL7W::AF0)
    }
    #[doc = "Alternate Function 1"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRL7W::AF1)
    }
    #[doc = "Alternate Function 2"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRL7W::AF2)
    }
    #[doc = "Alternate Function 3"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRL7W::AF3)
    }
    #[doc = "Alternate Function 4"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRL7W::AF4)
    }
    #[doc = "Alternate Function 5"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRL7W::AF5)
    }
    #[doc = "Alternate Function 6"]
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRL7W::AF6)
    }
    #[doc = "Alternate Function 7"]
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRL7W::AF7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AFRL6`"]
pub type AFRL6W = AFRL7W;
#[doc = r" Proxy"]
pub struct _AFRL6W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRL6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AFRL6W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Alternate Function 0"]
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFRL7W::AF0)
    }
    #[doc = "Alternate Function 1"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRL7W::AF1)
    }
    #[doc = "Alternate Function 2"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRL7W::AF2)
    }
    #[doc = "Alternate Function 3"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRL7W::AF3)
    }
    #[doc = "Alternate Function 4"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRL7W::AF4)
    }
    #[doc = "Alternate Function 5"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRL7W::AF5)
    }
    #[doc = "Alternate Function 6"]
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRL7W::AF6)
    }
    #[doc = "Alternate Function 7"]
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRL7W::AF7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AFRL5`"]
pub type AFRL5W = AFRL7W;
#[doc = r" Proxy"]
pub struct _AFRL5W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRL5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AFRL5W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Alternate Function 0"]
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFRL7W::AF0)
    }
    #[doc = "Alternate Function 1"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRL7W::AF1)
    }
    #[doc = "Alternate Function 2"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRL7W::AF2)
    }
    #[doc = "Alternate Function 3"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRL7W::AF3)
    }
    #[doc = "Alternate Function 4"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRL7W::AF4)
    }
    #[doc = "Alternate Function 5"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRL7W::AF5)
    }
    #[doc = "Alternate Function 6"]
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRL7W::AF6)
    }
    #[doc = "Alternate Function 7"]
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRL7W::AF7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AFRL4`"]
pub type AFRL4W = AFRL7W;
#[doc = r" Proxy"]
pub struct _AFRL4W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRL4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AFRL4W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Alternate Function 0"]
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFRL7W::AF0)
    }
    #[doc = "Alternate Function 1"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRL7W::AF1)
    }
    #[doc = "Alternate Function 2"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRL7W::AF2)
    }
    #[doc = "Alternate Function 3"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRL7W::AF3)
    }
    #[doc = "Alternate Function 4"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRL7W::AF4)
    }
    #[doc = "Alternate Function 5"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRL7W::AF5)
    }
    #[doc = "Alternate Function 6"]
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRL7W::AF6)
    }
    #[doc = "Alternate Function 7"]
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRL7W::AF7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AFRL3`"]
pub type AFRL3W = AFRL7W;
#[doc = r" Proxy"]
pub struct _AFRL3W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRL3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AFRL3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Alternate Function 0"]
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFRL7W::AF0)
    }
    #[doc = "Alternate Function 1"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRL7W::AF1)
    }
    #[doc = "Alternate Function 2"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRL7W::AF2)
    }
    #[doc = "Alternate Function 3"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRL7W::AF3)
    }
    #[doc = "Alternate Function 4"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRL7W::AF4)
    }
    #[doc = "Alternate Function 5"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRL7W::AF5)
    }
    #[doc = "Alternate Function 6"]
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRL7W::AF6)
    }
    #[doc = "Alternate Function 7"]
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRL7W::AF7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AFRL2`"]
pub type AFRL2W = AFRL7W;
#[doc = r" Proxy"]
pub struct _AFRL2W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRL2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AFRL2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Alternate Function 0"]
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFRL7W::AF0)
    }
    #[doc = "Alternate Function 1"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRL7W::AF1)
    }
    #[doc = "Alternate Function 2"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRL7W::AF2)
    }
    #[doc = "Alternate Function 3"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRL7W::AF3)
    }
    #[doc = "Alternate Function 4"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRL7W::AF4)
    }
    #[doc = "Alternate Function 5"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRL7W::AF5)
    }
    #[doc = "Alternate Function 6"]
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRL7W::AF6)
    }
    #[doc = "Alternate Function 7"]
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRL7W::AF7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AFRL1`"]
pub type AFRL1W = AFRL7W;
#[doc = r" Proxy"]
pub struct _AFRL1W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRL1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AFRL1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Alternate Function 0"]
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFRL7W::AF0)
    }
    #[doc = "Alternate Function 1"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRL7W::AF1)
    }
    #[doc = "Alternate Function 2"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRL7W::AF2)
    }
    #[doc = "Alternate Function 3"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRL7W::AF3)
    }
    #[doc = "Alternate Function 4"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRL7W::AF4)
    }
    #[doc = "Alternate Function 5"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRL7W::AF5)
    }
    #[doc = "Alternate Function 6"]
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRL7W::AF6)
    }
    #[doc = "Alternate Function 7"]
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRL7W::AF7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AFRL0`"]
pub type AFRL0W = AFRL7W;
#[doc = r" Proxy"]
pub struct _AFRL0W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRL0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AFRL0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Alternate Function 0"]
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFRL7W::AF0)
    }
    #[doc = "Alternate Function 1"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRL7W::AF1)
    }
    #[doc = "Alternate Function 2"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRL7W::AF2)
    }
    #[doc = "Alternate Function 3"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRL7W::AF3)
    }
    #[doc = "Alternate Function 4"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRL7W::AF4)
    }
    #[doc = "Alternate Function 5"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRL7W::AF5)
    }
    #[doc = "Alternate Function 6"]
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRL7W::AF6)
    }
    #[doc = "Alternate Function 7"]
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRL7W::AF7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
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
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl7(&self) -> AFRL7R {
        AFRL7R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl6(&self) -> AFRL6R {
        AFRL6R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl5(&self) -> AFRL5R {
        AFRL5R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl4(&self) -> AFRL4R {
        AFRL4R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl3(&self) -> AFRL3R {
        AFRL3R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl2(&self) -> AFRL2R {
        AFRL2R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl1(&self) -> AFRL1R {
        AFRL1R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl0(&self) -> AFRL0R {
        AFRL0R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline(always)]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl7(&mut self) -> _AFRL7W {
        _AFRL7W { w: self }
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl6(&mut self) -> _AFRL6W {
        _AFRL6W { w: self }
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl5(&mut self) -> _AFRL5W {
        _AFRL5W { w: self }
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl4(&mut self) -> _AFRL4W {
        _AFRL4W { w: self }
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl3(&mut self) -> _AFRL3W {
        _AFRL3W { w: self }
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl2(&mut self) -> _AFRL2W {
        _AFRL2W { w: self }
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl1(&mut self) -> _AFRL1W {
        _AFRL1W { w: self }
    }
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl0(&mut self) -> _AFRL0W {
        _AFRL0W { w: self }
    }
}
