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
