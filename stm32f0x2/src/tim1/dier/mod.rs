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
#[doc = "Possible values of the field `TDE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDER {
    #[doc = "Trigger DMA request disabled."]
    DISABLED,
    #[doc = "Trigger DMA request enabled."]
    ENABLED,
}
impl TDER {
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
            TDER::DISABLED => false,
            TDER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TDER {
        match value {
            false => TDER::DISABLED,
            true => TDER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TDER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TDER::ENABLED
    }
}
#[doc = "Possible values of the field `COMDE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMDER {
    #[doc = "COM DMA request disabled."]
    DISABLED,
    #[doc = "COM DMA request enabled."]
    ENABLED,
}
impl COMDER {
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
            COMDER::DISABLED => false,
            COMDER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMDER {
        match value {
            false => COMDER::DISABLED,
            true => COMDER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == COMDER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == COMDER::ENABLED
    }
}
#[doc = "Possible values of the field `CC4DE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC4DER {
    #[doc = "CC4 DMA request disabled."]
    DISABLED,
    #[doc = "CC4 DMA request enabled."]
    ENABLED,
}
impl CC4DER {
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
            CC4DER::DISABLED => false,
            CC4DER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CC4DER {
        match value {
            false => CC4DER::DISABLED,
            true => CC4DER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CC4DER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CC4DER::ENABLED
    }
}
#[doc = "Possible values of the field `CC3DE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC3DER {
    #[doc = "CC3 DMA request disabled."]
    DISABLED,
    #[doc = "CC3 DMA request enabled."]
    ENABLED,
}
impl CC3DER {
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
            CC3DER::DISABLED => false,
            CC3DER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CC3DER {
        match value {
            false => CC3DER::DISABLED,
            true => CC3DER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CC3DER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CC3DER::ENABLED
    }
}
#[doc = "Possible values of the field `CC2DE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC2DER {
    #[doc = "CC2 DMA request disabled."]
    DISABLED,
    #[doc = "CC2 DMA request enabled."]
    ENABLED,
}
impl CC2DER {
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
            CC2DER::DISABLED => false,
            CC2DER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CC2DER {
        match value {
            false => CC2DER::DISABLED,
            true => CC2DER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CC2DER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CC2DER::ENABLED
    }
}
#[doc = "Possible values of the field `CC1DE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1DER {
    #[doc = "CC1 DMA request disabled."]
    DISABLED,
    #[doc = "CC1 DMA request enabled."]
    ENABLED,
}
impl CC1DER {
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
            CC1DER::DISABLED => false,
            CC1DER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CC1DER {
        match value {
            false => CC1DER::DISABLED,
            true => CC1DER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CC1DER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CC1DER::ENABLED
    }
}
#[doc = "Possible values of the field `UDE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UDER {
    #[doc = "Update DMA request disabled."]
    DISABLED,
    #[doc = "Update DMA request enabled."]
    ENABLED,
}
impl UDER {
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
            UDER::DISABLED => false,
            UDER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UDER {
        match value {
            false => UDER::DISABLED,
            true => UDER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == UDER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == UDER::ENABLED
    }
}
#[doc = "Possible values of the field `BIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIER {
    #[doc = "Break interrupt disabled."]
    DISABLED,
    #[doc = "Break interrupt enabled."]
    ENABLED,
}
impl BIER {
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
            BIER::DISABLED => false,
            BIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BIER {
        match value {
            false => BIER::DISABLED,
            true => BIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == BIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == BIER::ENABLED
    }
}
#[doc = "Possible values of the field `TIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIER {
    #[doc = "Trigger interrupt disabled."]
    DISABLED,
    #[doc = "Trigger interrupt enabled."]
    ENABLED,
}
impl TIER {
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
            TIER::DISABLED => false,
            TIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIER {
        match value {
            false => TIER::DISABLED,
            true => TIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TIER::ENABLED
    }
}
#[doc = "Possible values of the field `COMIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMIER {
    #[doc = "COM interrupt disabled."]
    DISABLED,
    #[doc = "COM interrupt enabled."]
    ENABLED,
}
impl COMIER {
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
            COMIER::DISABLED => false,
            COMIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMIER {
        match value {
            false => COMIER::DISABLED,
            true => COMIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == COMIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == COMIER::ENABLED
    }
}
#[doc = "Possible values of the field `CC4IE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC4IER {
    #[doc = "CC4 interrupt disabled."]
    DISABLED,
    #[doc = "CC4 interrupt enabled."]
    ENABLED,
}
impl CC4IER {
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
            CC4IER::DISABLED => false,
            CC4IER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CC4IER {
        match value {
            false => CC4IER::DISABLED,
            true => CC4IER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CC4IER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CC4IER::ENABLED
    }
}
#[doc = "Possible values of the field `CC3IE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC3IER {
    #[doc = "CC3 interrupt disabled."]
    DISABLED,
    #[doc = "CC3 interrupt enabled."]
    ENABLED,
}
impl CC3IER {
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
            CC3IER::DISABLED => false,
            CC3IER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CC3IER {
        match value {
            false => CC3IER::DISABLED,
            true => CC3IER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CC3IER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CC3IER::ENABLED
    }
}
#[doc = "Possible values of the field `CC2IE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC2IER {
    #[doc = "CC2 interrupt disabled."]
    DISABLED,
    #[doc = "CC2 interrupt enabled."]
    ENABLED,
}
impl CC2IER {
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
            CC2IER::DISABLED => false,
            CC2IER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CC2IER {
        match value {
            false => CC2IER::DISABLED,
            true => CC2IER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CC2IER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CC2IER::ENABLED
    }
}
#[doc = "Possible values of the field `CC1IE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1IER {
    #[doc = "CC1 interrupt disabled."]
    DISABLED,
    #[doc = "CC1 interrupt enabled."]
    ENABLED,
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
    #[doc = "Update interrupt disabled."]
    DISABLED,
    #[doc = "Update interrupt enabled."]
    ENABLED,
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
#[doc = "Values that can be written to the field `TDE`"]
pub enum TDEW {
    #[doc = "Trigger DMA request disabled."]
    DISABLED,
    #[doc = "Trigger DMA request enabled."]
    ENABLED,
}
impl TDEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TDEW::DISABLED => false,
            TDEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TDEW<'a> {
    w: &'a mut W,
}
impl<'a> _TDEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TDEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trigger DMA request disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TDEW::DISABLED)
    }
    #[doc = "Trigger DMA request enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TDEW::ENABLED)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `COMDE`"]
pub enum COMDEW {
    #[doc = "COM DMA request disabled."]
    DISABLED,
    #[doc = "COM DMA request enabled."]
    ENABLED,
}
impl COMDEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMDEW::DISABLED => false,
            COMDEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMDEW<'a> {
    w: &'a mut W,
}
impl<'a> _COMDEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMDEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "COM DMA request disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMDEW::DISABLED)
    }
    #[doc = "COM DMA request enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMDEW::ENABLED)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CC4DE`"]
pub enum CC4DEW {
    #[doc = "CC4 DMA request disabled."]
    DISABLED,
    #[doc = "CC4 DMA request enabled."]
    ENABLED,
}
impl CC4DEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CC4DEW::DISABLED => false,
            CC4DEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC4DEW<'a> {
    w: &'a mut W,
}
impl<'a> _CC4DEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC4DEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CC4 DMA request disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC4DEW::DISABLED)
    }
    #[doc = "CC4 DMA request enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC4DEW::ENABLED)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CC3DE`"]
pub enum CC3DEW {
    #[doc = "CC3 DMA request disabled."]
    DISABLED,
    #[doc = "CC3 DMA request enabled."]
    ENABLED,
}
impl CC3DEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CC3DEW::DISABLED => false,
            CC3DEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC3DEW<'a> {
    w: &'a mut W,
}
impl<'a> _CC3DEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC3DEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CC3 DMA request disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC3DEW::DISABLED)
    }
    #[doc = "CC3 DMA request enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC3DEW::ENABLED)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CC2DE`"]
pub enum CC2DEW {
    #[doc = "CC2 DMA request disabled."]
    DISABLED,
    #[doc = "CC2 DMA request enabled."]
    ENABLED,
}
impl CC2DEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CC2DEW::DISABLED => false,
            CC2DEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC2DEW<'a> {
    w: &'a mut W,
}
impl<'a> _CC2DEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC2DEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CC2 DMA request disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC2DEW::DISABLED)
    }
    #[doc = "CC2 DMA request enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC2DEW::ENABLED)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CC1DE`"]
pub enum CC1DEW {
    #[doc = "CC1 DMA request disabled."]
    DISABLED,
    #[doc = "CC1 DMA request enabled."]
    ENABLED,
}
impl CC1DEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CC1DEW::DISABLED => false,
            CC1DEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC1DEW<'a> {
    w: &'a mut W,
}
impl<'a> _CC1DEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC1DEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CC1 DMA request disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC1DEW::DISABLED)
    }
    #[doc = "CC1 DMA request enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC1DEW::ENABLED)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UDE`"]
pub enum UDEW {
    #[doc = "Update DMA request disabled."]
    DISABLED,
    #[doc = "Update DMA request enabled."]
    ENABLED,
}
impl UDEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UDEW::DISABLED => false,
            UDEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UDEW<'a> {
    w: &'a mut W,
}
impl<'a> _UDEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UDEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Update DMA request disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UDEW::DISABLED)
    }
    #[doc = "Update DMA request enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UDEW::ENABLED)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BIE`"]
pub enum BIEW {
    #[doc = "Break interrupt disabled."]
    DISABLED,
    #[doc = "Break interrupt enabled."]
    ENABLED,
}
impl BIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BIEW::DISABLED => false,
            BIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BIEW<'a> {
    w: &'a mut W,
}
impl<'a> _BIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Break interrupt disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BIEW::DISABLED)
    }
    #[doc = "Break interrupt enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BIEW::ENABLED)
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
#[doc = "Values that can be written to the field `TIE`"]
pub enum TIEW {
    #[doc = "Trigger interrupt disabled."]
    DISABLED,
    #[doc = "Trigger interrupt enabled."]
    ENABLED,
}
impl TIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIEW::DISABLED => false,
            TIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trigger interrupt disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIEW::DISABLED)
    }
    #[doc = "Trigger interrupt enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIEW::ENABLED)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `COMIE`"]
pub enum COMIEW {
    #[doc = "COM interrupt disabled."]
    DISABLED,
    #[doc = "COM interrupt enabled."]
    ENABLED,
}
impl COMIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMIEW::DISABLED => false,
            COMIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMIEW<'a> {
    w: &'a mut W,
}
impl<'a> _COMIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "COM interrupt disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMIEW::DISABLED)
    }
    #[doc = "COM interrupt enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMIEW::ENABLED)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CC4IE`"]
pub enum CC4IEW {
    #[doc = "CC4 interrupt disabled."]
    DISABLED,
    #[doc = "CC4 interrupt enabled."]
    ENABLED,
}
impl CC4IEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CC4IEW::DISABLED => false,
            CC4IEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC4IEW<'a> {
    w: &'a mut W,
}
impl<'a> _CC4IEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC4IEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CC4 interrupt disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC4IEW::DISABLED)
    }
    #[doc = "CC4 interrupt enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC4IEW::ENABLED)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CC3IE`"]
pub enum CC3IEW {
    #[doc = "CC3 interrupt disabled."]
    DISABLED,
    #[doc = "CC3 interrupt enabled."]
    ENABLED,
}
impl CC3IEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CC3IEW::DISABLED => false,
            CC3IEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC3IEW<'a> {
    w: &'a mut W,
}
impl<'a> _CC3IEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC3IEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CC3 interrupt disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC3IEW::DISABLED)
    }
    #[doc = "CC3 interrupt enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC3IEW::ENABLED)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CC2IE`"]
pub enum CC2IEW {
    #[doc = "CC2 interrupt disabled."]
    DISABLED,
    #[doc = "CC2 interrupt enabled."]
    ENABLED,
}
impl CC2IEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CC2IEW::DISABLED => false,
            CC2IEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC2IEW<'a> {
    w: &'a mut W,
}
impl<'a> _CC2IEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC2IEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CC2 interrupt disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC2IEW::DISABLED)
    }
    #[doc = "CC2 interrupt enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC2IEW::ENABLED)
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
#[doc = "Values that can be written to the field `CC1IE`"]
pub enum CC1IEW {
    #[doc = "CC1 interrupt disabled."]
    DISABLED,
    #[doc = "CC1 interrupt enabled."]
    ENABLED,
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
    #[doc = "Update interrupt disabled."]
    DISABLED,
    #[doc = "Update interrupt enabled."]
    ENABLED,
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
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline]
    pub fn tde(&self) -> TDER {
        TDER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - COM DMA request enable"]
    #[inline]
    pub fn comde(&self) -> COMDER {
        COMDER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Capture/Compare 4 DMA request enable"]
    #[inline]
    pub fn cc4de(&self) -> CC4DER {
        CC4DER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Capture/Compare 3 DMA request enable"]
    #[inline]
    pub fn cc3de(&self) -> CC3DER {
        CC3DER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Capture/Compare 2 DMA request enable"]
    #[inline]
    pub fn cc2de(&self) -> CC2DER {
        CC2DER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Capture/Compare 1 DMA request enable"]
    #[inline]
    pub fn cc1de(&self) -> CC1DER {
        CC1DER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline]
    pub fn ude(&self) -> UDER {
        UDER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Break interrupt enable"]
    #[inline]
    pub fn bie(&self) -> BIER {
        BIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline]
    pub fn tie(&self) -> TIER {
        TIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - COM interrupt enable"]
    #[inline]
    pub fn comie(&self) -> COMIER {
        COMIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Capture/Compare 4 interrupt enable"]
    #[inline]
    pub fn cc4ie(&self) -> CC4IER {
        CC4IER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Capture/Compare 3 interrupt enable"]
    #[inline]
    pub fn cc3ie(&self) -> CC3IER {
        CC3IER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt enable"]
    #[inline]
    pub fn cc2ie(&self) -> CC2IER {
        CC2IER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
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
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline]
    pub fn tde(&mut self) -> _TDEW {
        _TDEW { w: self }
    }
    #[doc = "Bit 13 - COM DMA request enable"]
    #[inline]
    pub fn comde(&mut self) -> _COMDEW {
        _COMDEW { w: self }
    }
    #[doc = "Bit 12 - Capture/Compare 4 DMA request enable"]
    #[inline]
    pub fn cc4de(&mut self) -> _CC4DEW {
        _CC4DEW { w: self }
    }
    #[doc = "Bit 11 - Capture/Compare 3 DMA request enable"]
    #[inline]
    pub fn cc3de(&mut self) -> _CC3DEW {
        _CC3DEW { w: self }
    }
    #[doc = "Bit 10 - Capture/Compare 2 DMA request enable"]
    #[inline]
    pub fn cc2de(&mut self) -> _CC2DEW {
        _CC2DEW { w: self }
    }
    #[doc = "Bit 9 - Capture/Compare 1 DMA request enable"]
    #[inline]
    pub fn cc1de(&mut self) -> _CC1DEW {
        _CC1DEW { w: self }
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline]
    pub fn ude(&mut self) -> _UDEW {
        _UDEW { w: self }
    }
    #[doc = "Bit 7 - Break interrupt enable"]
    #[inline]
    pub fn bie(&mut self) -> _BIEW {
        _BIEW { w: self }
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline]
    pub fn tie(&mut self) -> _TIEW {
        _TIEW { w: self }
    }
    #[doc = "Bit 5 - COM interrupt enable"]
    #[inline]
    pub fn comie(&mut self) -> _COMIEW {
        _COMIEW { w: self }
    }
    #[doc = "Bit 4 - Capture/Compare 4 interrupt enable"]
    #[inline]
    pub fn cc4ie(&mut self) -> _CC4IEW {
        _CC4IEW { w: self }
    }
    #[doc = "Bit 3 - Capture/Compare 3 interrupt enable"]
    #[inline]
    pub fn cc3ie(&mut self) -> _CC3IEW {
        _CC3IEW { w: self }
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt enable"]
    #[inline]
    pub fn cc2ie(&mut self) -> _CC2IEW {
        _CC2IEW { w: self }
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
