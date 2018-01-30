#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DIER {
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
#[doc = "Possible values of the field `CC1IE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1IER {
    #[doc = "CC1 interrupt disabled."] DISABLED,
    #[doc = "CC1 interrupt enabled."] ENABLED,
}
impl CC1IER {
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
            CC1IER::DISABLED => false,
            CC1IER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CC1IER {
        match value {
            false => CC1IER::DISABLED,
            true => CC1IER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CC1IER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CC1IER::ENABLED
    }
}
#[doc = "Possible values of the field `UIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UIER {
    #[doc = "Update interrupt disabled."] DISABLED,
    #[doc = "Update interrupt enabled."] ENABLED,
}
impl UIER {
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
            UIER::DISABLED => false,
            UIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UIER {
        match value {
            false => UIER::DISABLED,
            true => UIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == UIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == UIER::ENABLED
    }
}
#[doc = "Values that can be written to the field `CC1IE`"]
pub enum CC1IEW {
    #[doc = "CC1 interrupt disabled."] DISABLED,
    #[doc = "CC1 interrupt enabled."] ENABLED,
}
impl CC1IEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CC1IEW::DISABLED => false,
            CC1IEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC1IEW<'a> {
    w: &'a mut W,
}
impl<'a> _CC1IEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC1IEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CC1 interrupt disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC1IEW::DISABLED)
    }
    #[doc = "CC1 interrupt enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC1IEW::ENABLED)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UIE`"]
pub enum UIEW {
    #[doc = "Update interrupt disabled."] DISABLED,
    #[doc = "Update interrupt enabled."] ENABLED,
}
impl UIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UIEW::DISABLED => false,
            UIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UIEW<'a> {
    w: &'a mut W,
}
impl<'a> _UIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Update interrupt disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UIEW::DISABLED)
    }
    #[doc = "Update interrupt enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UIEW::ENABLED)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - Capture/Compare 1 interrupt enable"]
    #[inline]
    pub fn cc1ie(&self) -> CC1IER {
        CC1IER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline]
    pub fn uie(&self) -> UIER {
        UIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
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
    #[doc = "Bit 1 - Capture/Compare 1 interrupt enable"]
    #[inline]
    pub fn cc1ie(&mut self) -> _CC1IEW {
        _CC1IEW { w: self }
    }
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline]
    pub fn uie(&mut self) -> _UIEW {
        _UIEW { w: self }
    }
}
