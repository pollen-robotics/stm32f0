#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LCKR {
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
#[doc = "Possible values of the field `LCKK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCKKR {
    #[doc = "Port configuration lock key not active"] INACTIVE,
    #[doc = "Port configuration lock key active"] ACTIVE,
}
impl LCKKR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        match *self {
            LCKKR::INACTIVE => false,
            LCKKR::ACTIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> LCKKR {
        match value {
            false => LCKKR::INACTIVE,
            true => LCKKR::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == LCKKR::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == LCKKR::ACTIVE
    }
}
#[doc = "Possible values of the field `LCK15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCK15R {
    #[doc = "Port configuration not locked"] UNLOCK,
    #[doc = "Port configuration locked"] LOCK,
}
impl LCK15R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        match *self {
            LCK15R::UNLOCK => false,
            LCK15R::LOCK => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> LCK15R {
        match value {
            false => LCK15R::UNLOCK,
            true => LCK15R::LOCK,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCK`"]
    #[inline(always)]
    pub fn is_unlock(&self) -> bool {
        *self == LCK15R::UNLOCK
    }
    #[doc = "Checks if the value of the field is `LOCK`"]
    #[inline(always)]
    pub fn is_lock(&self) -> bool {
        *self == LCK15R::LOCK
    }
}
#[doc = "Possible values of the field `LCK14`"]
pub type LCK14R = LCK15R;
#[doc = "Possible values of the field `LCK13`"]
pub type LCK13R = LCK15R;
#[doc = "Possible values of the field `LCK12`"]
pub type LCK12R = LCK15R;
#[doc = "Possible values of the field `LCK11`"]
pub type LCK11R = LCK15R;
#[doc = "Possible values of the field `LCK10`"]
pub type LCK10R = LCK15R;
#[doc = "Possible values of the field `LCK9`"]
pub type LCK9R = LCK15R;
#[doc = "Possible values of the field `LCK8`"]
pub type LCK8R = LCK15R;
#[doc = "Possible values of the field `LCK7`"]
pub type LCK7R = LCK15R;
#[doc = "Possible values of the field `LCK6`"]
pub type LCK6R = LCK15R;
#[doc = "Possible values of the field `LCK5`"]
pub type LCK5R = LCK15R;
#[doc = "Possible values of the field `LCK4`"]
pub type LCK4R = LCK15R;
#[doc = "Possible values of the field `LCK3`"]
pub type LCK3R = LCK15R;
#[doc = "Possible values of the field `LCK2`"]
pub type LCK2R = LCK15R;
#[doc = "Possible values of the field `LCK1`"]
pub type LCK1R = LCK15R;
#[doc = "Possible values of the field `LCK0`"]
pub type LCK0R = LCK15R;
#[doc = "Values that can be written to the field `LCKK`"]
pub enum LCKKW {
    #[doc = "Port configuration lock key not active"] INACTIVE,
    #[doc = "Port configuration lock key active"] ACTIVE,
}
impl LCKKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LCKKW::INACTIVE => false,
            LCKKW::ACTIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LCKKW<'a> {
    w: &'a mut W,
}
impl<'a> _LCKKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCKKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Port configuration lock key not active"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(LCKKW::INACTIVE)
    }
    #[doc = "Port configuration lock key active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(LCKKW::ACTIVE)
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
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LCK15`"]
pub enum LCK15W {
    #[doc = "Port configuration not locked"] UNLOCK,
    #[doc = "Port configuration locked"] LOCK,
}
impl LCK15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LCK15W::UNLOCK => false,
            LCK15W::LOCK => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LCK15W<'a> {
    w: &'a mut W,
}
impl<'a> _LCK15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCK15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut W {
        self.variant(LCK15W::UNLOCK)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(LCK15W::LOCK)
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
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LCK14`"]
pub type LCK14W = LCK15W;
#[doc = r" Proxy"]
pub struct _LCK14W<'a> {
    w: &'a mut W,
}
impl<'a> _LCK14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCK14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut W {
        self.variant(LCK15W::UNLOCK)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(LCK15W::LOCK)
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
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LCK13`"]
pub type LCK13W = LCK15W;
#[doc = r" Proxy"]
pub struct _LCK13W<'a> {
    w: &'a mut W,
}
impl<'a> _LCK13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCK13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut W {
        self.variant(LCK15W::UNLOCK)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(LCK15W::LOCK)
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
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LCK12`"]
pub type LCK12W = LCK15W;
#[doc = r" Proxy"]
pub struct _LCK12W<'a> {
    w: &'a mut W,
}
impl<'a> _LCK12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCK12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut W {
        self.variant(LCK15W::UNLOCK)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(LCK15W::LOCK)
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
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LCK11`"]
pub type LCK11W = LCK15W;
#[doc = r" Proxy"]
pub struct _LCK11W<'a> {
    w: &'a mut W,
}
impl<'a> _LCK11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCK11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut W {
        self.variant(LCK15W::UNLOCK)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(LCK15W::LOCK)
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
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LCK10`"]
pub type LCK10W = LCK15W;
#[doc = r" Proxy"]
pub struct _LCK10W<'a> {
    w: &'a mut W,
}
impl<'a> _LCK10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCK10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut W {
        self.variant(LCK15W::UNLOCK)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(LCK15W::LOCK)
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
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LCK9`"]
pub type LCK9W = LCK15W;
#[doc = r" Proxy"]
pub struct _LCK9W<'a> {
    w: &'a mut W,
}
impl<'a> _LCK9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCK9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut W {
        self.variant(LCK15W::UNLOCK)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(LCK15W::LOCK)
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
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LCK8`"]
pub type LCK8W = LCK15W;
#[doc = r" Proxy"]
pub struct _LCK8W<'a> {
    w: &'a mut W,
}
impl<'a> _LCK8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCK8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut W {
        self.variant(LCK15W::UNLOCK)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(LCK15W::LOCK)
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
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LCK7`"]
pub type LCK7W = LCK15W;
#[doc = r" Proxy"]
pub struct _LCK7W<'a> {
    w: &'a mut W,
}
impl<'a> _LCK7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCK7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut W {
        self.variant(LCK15W::UNLOCK)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(LCK15W::LOCK)
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
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LCK6`"]
pub type LCK6W = LCK15W;
#[doc = r" Proxy"]
pub struct _LCK6W<'a> {
    w: &'a mut W,
}
impl<'a> _LCK6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCK6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut W {
        self.variant(LCK15W::UNLOCK)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(LCK15W::LOCK)
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
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LCK5`"]
pub type LCK5W = LCK15W;
#[doc = r" Proxy"]
pub struct _LCK5W<'a> {
    w: &'a mut W,
}
impl<'a> _LCK5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCK5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut W {
        self.variant(LCK15W::UNLOCK)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(LCK15W::LOCK)
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
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LCK4`"]
pub type LCK4W = LCK15W;
#[doc = r" Proxy"]
pub struct _LCK4W<'a> {
    w: &'a mut W,
}
impl<'a> _LCK4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCK4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut W {
        self.variant(LCK15W::UNLOCK)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(LCK15W::LOCK)
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
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LCK3`"]
pub type LCK3W = LCK15W;
#[doc = r" Proxy"]
pub struct _LCK3W<'a> {
    w: &'a mut W,
}
impl<'a> _LCK3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCK3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut W {
        self.variant(LCK15W::UNLOCK)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(LCK15W::LOCK)
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
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LCK2`"]
pub type LCK2W = LCK15W;
#[doc = r" Proxy"]
pub struct _LCK2W<'a> {
    w: &'a mut W,
}
impl<'a> _LCK2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCK2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut W {
        self.variant(LCK15W::UNLOCK)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(LCK15W::LOCK)
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
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LCK1`"]
pub type LCK1W = LCK15W;
#[doc = r" Proxy"]
pub struct _LCK1W<'a> {
    w: &'a mut W,
}
impl<'a> _LCK1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCK1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut W {
        self.variant(LCK15W::UNLOCK)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(LCK15W::LOCK)
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
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LCK0`"]
pub type LCK0W = LCK15W;
#[doc = r" Proxy"]
pub struct _LCK0W<'a> {
    w: &'a mut W,
}
impl<'a> _LCK0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCK0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut W {
        self.variant(LCK15W::UNLOCK)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(LCK15W::LOCK)
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
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
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
    #[doc = "Bit 16 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lckk(&self) -> LCKKR {
        LCKKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck15(&self) -> LCK15R {
        LCK15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck14(&self) -> LCK14R {
        LCK14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck13(&self) -> LCK13R {
        LCK13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck12(&self) -> LCK12R {
        LCK12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck11(&self) -> LCK11R {
        LCK11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck10(&self) -> LCK10R {
        LCK10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck9(&self) -> LCK9R {
        LCK9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck8(&self) -> LCK8R {
        LCK8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck7(&self) -> LCK7R {
        LCK7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck6(&self) -> LCK6R {
        LCK6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck5(&self) -> LCK5R {
        LCK5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck4(&self) -> LCK4R {
        LCK4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck3(&self) -> LCK3R {
        LCK3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck2(&self) -> LCK2R {
        LCK2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck1(&self) -> LCK1R {
        LCK1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck0(&self) -> LCK0R {
        LCK0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bit 16 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lckk(&mut self) -> _LCKKW {
        _LCKKW { w: self }
    }
    #[doc = "Bit 15 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck15(&mut self) -> _LCK15W {
        _LCK15W { w: self }
    }
    #[doc = "Bit 14 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck14(&mut self) -> _LCK14W {
        _LCK14W { w: self }
    }
    #[doc = "Bit 13 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck13(&mut self) -> _LCK13W {
        _LCK13W { w: self }
    }
    #[doc = "Bit 12 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck12(&mut self) -> _LCK12W {
        _LCK12W { w: self }
    }
    #[doc = "Bit 11 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck11(&mut self) -> _LCK11W {
        _LCK11W { w: self }
    }
    #[doc = "Bit 10 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck10(&mut self) -> _LCK10W {
        _LCK10W { w: self }
    }
    #[doc = "Bit 9 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck9(&mut self) -> _LCK9W {
        _LCK9W { w: self }
    }
    #[doc = "Bit 8 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck8(&mut self) -> _LCK8W {
        _LCK8W { w: self }
    }
    #[doc = "Bit 7 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck7(&mut self) -> _LCK7W {
        _LCK7W { w: self }
    }
    #[doc = "Bit 6 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck6(&mut self) -> _LCK6W {
        _LCK6W { w: self }
    }
    #[doc = "Bit 5 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck5(&mut self) -> _LCK5W {
        _LCK5W { w: self }
    }
    #[doc = "Bit 4 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck4(&mut self) -> _LCK4W {
        _LCK4W { w: self }
    }
    #[doc = "Bit 3 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck3(&mut self) -> _LCK3W {
        _LCK3W { w: self }
    }
    #[doc = "Bit 2 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck2(&mut self) -> _LCK2W {
        _LCK2W { w: self }
    }
    #[doc = "Bit 1 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck1(&mut self) -> _LCK1W {
        _LCK1W { w: self }
    }
    #[doc = "Bit 0 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck0(&mut self) -> _LCK0W {
        _LCK0W { w: self }
    }
}
