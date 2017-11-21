#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCER {
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
#[doc = "Possible values of the field `CC4P`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC4PR {
    #[doc = "active high"] HIGH,
    #[doc = "active low"] LOW,
}
impl CC4PR {
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
            CC4PR::HIGH => false,
            CC4PR::LOW => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> CC4PR {
        match value {
            false => CC4PR::HIGH,
            true => CC4PR::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CC4PR::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CC4PR::LOW
    }
}
#[doc = "Possible values of the field `CC4E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC4ER {
    #[doc = "Output : OC1 is not active - Input : Capture disabled"] INACTIVE,
    #[doc = "Output : OC1 signal is output on the corresponding output pin - Input : Capture enabled"] ACTIVE,
}
impl CC4ER {
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
            CC4ER::INACTIVE => false,
            CC4ER::ACTIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> CC4ER {
        match value {
            false => CC4ER::INACTIVE,
            true => CC4ER::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == CC4ER::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == CC4ER::ACTIVE
    }
}
#[doc = "Possible values of the field `CC3NP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC3NPR {
    #[doc = "active high"] HIGH,
    #[doc = "active low"] LOW,
}
impl CC3NPR {
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
            CC3NPR::HIGH => false,
            CC3NPR::LOW => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> CC3NPR {
        match value {
            false => CC3NPR::HIGH,
            true => CC3NPR::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CC3NPR::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CC3NPR::LOW
    }
}
#[doc = "Possible values of the field `CC3NE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC3NER {
    #[doc = "Off"] DISABLED,
    #[doc = "On"] ENABLED,
}
impl CC3NER {
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
            CC3NER::DISABLED => false,
            CC3NER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> CC3NER {
        match value {
            false => CC3NER::DISABLED,
            true => CC3NER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CC3NER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CC3NER::ENABLED
    }
}
#[doc = "Possible values of the field `CC3P`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC3PR {
    #[doc = "active high"] HIGH,
    #[doc = "active low"] LOW,
}
impl CC3PR {
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
            CC3PR::HIGH => false,
            CC3PR::LOW => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> CC3PR {
        match value {
            false => CC3PR::HIGH,
            true => CC3PR::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CC3PR::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CC3PR::LOW
    }
}
#[doc = "Possible values of the field `CC3E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC3ER {
    #[doc = "Output : OC1 is not active - Input : Capture disabled"] INACTIVE,
    #[doc = "Output : OC1 signal is output on the corresponding output pin - Input : Capture enabled"] ACTIVE,
}
impl CC3ER {
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
            CC3ER::INACTIVE => false,
            CC3ER::ACTIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> CC3ER {
        match value {
            false => CC3ER::INACTIVE,
            true => CC3ER::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == CC3ER::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == CC3ER::ACTIVE
    }
}
#[doc = "Possible values of the field `CC2NP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC2NPR {
    #[doc = "active high"] HIGH,
    #[doc = "active low"] LOW,
}
impl CC2NPR {
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
            CC2NPR::HIGH => false,
            CC2NPR::LOW => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> CC2NPR {
        match value {
            false => CC2NPR::HIGH,
            true => CC2NPR::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CC2NPR::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CC2NPR::LOW
    }
}
#[doc = "Possible values of the field `CC2NE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC2NER {
    #[doc = "Off"] DISABLED,
    #[doc = "On"] ENABLED,
}
impl CC2NER {
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
            CC2NER::DISABLED => false,
            CC2NER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> CC2NER {
        match value {
            false => CC2NER::DISABLED,
            true => CC2NER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CC2NER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CC2NER::ENABLED
    }
}
#[doc = "Possible values of the field `CC2P`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC2PR {
    #[doc = "active high"] HIGH,
    #[doc = "active low"] LOW,
}
impl CC2PR {
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
            CC2PR::HIGH => false,
            CC2PR::LOW => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> CC2PR {
        match value {
            false => CC2PR::HIGH,
            true => CC2PR::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CC2PR::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CC2PR::LOW
    }
}
#[doc = "Possible values of the field `CC2E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC2ER {
    #[doc = "Output : OC1 is not active - Input : Capture disabled"] INACTIVE,
    #[doc = "Output : OC1 signal is output on the corresponding output pin - Input : Capture enabled"] ACTIVE,
}
impl CC2ER {
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
            CC2ER::INACTIVE => false,
            CC2ER::ACTIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> CC2ER {
        match value {
            false => CC2ER::INACTIVE,
            true => CC2ER::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == CC2ER::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == CC2ER::ACTIVE
    }
}
#[doc = "Possible values of the field `CC1NP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1NPR {
    #[doc = "active high"] HIGH,
    #[doc = "active low"] LOW,
}
impl CC1NPR {
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
            CC1NPR::HIGH => false,
            CC1NPR::LOW => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> CC1NPR {
        match value {
            false => CC1NPR::HIGH,
            true => CC1NPR::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CC1NPR::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CC1NPR::LOW
    }
}
#[doc = "Possible values of the field `CC1NE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1NER {
    #[doc = "Off"] DISABLED,
    #[doc = "On"] ENABLED,
}
impl CC1NER {
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
            CC1NER::DISABLED => false,
            CC1NER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> CC1NER {
        match value {
            false => CC1NER::DISABLED,
            true => CC1NER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CC1NER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CC1NER::ENABLED
    }
}
#[doc = "Possible values of the field `CC1P`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1PR {
    #[doc = "active high"] HIGH,
    #[doc = "active low"] LOW,
}
impl CC1PR {
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
            CC1PR::HIGH => false,
            CC1PR::LOW => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> CC1PR {
        match value {
            false => CC1PR::HIGH,
            true => CC1PR::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CC1PR::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CC1PR::LOW
    }
}
#[doc = "Possible values of the field `CC1E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1ER {
    #[doc = "Output : OC1 is not active - Input : Capture disabled"] INACTIVE,
    #[doc = "Output : OC1 signal is output on the corresponding output pin - Input : Capture enabled"] ACTIVE,
}
impl CC1ER {
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
            CC1ER::INACTIVE => false,
            CC1ER::ACTIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> CC1ER {
        match value {
            false => CC1ER::INACTIVE,
            true => CC1ER::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == CC1ER::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == CC1ER::ACTIVE
    }
}
#[doc = "Values that can be written to the field `CC4P`"]
pub enum CC4PW {
    #[doc = "active high"] HIGH,
    #[doc = "active low"] LOW,
}
impl CC4PW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CC4PW::HIGH => false,
            CC4PW::LOW => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC4PW<'a> {
    w: &'a mut W,
}
impl<'a> _CC4PW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC4PW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "active high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CC4PW::HIGH)
    }
    #[doc = "active low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CC4PW::LOW)
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
#[doc = "Values that can be written to the field `CC4E`"]
pub enum CC4EW {
    #[doc = "Output : OC1 is not active - Input : Capture disabled"] INACTIVE,
    #[doc = "Output : OC1 signal is output on the corresponding output pin - Input : Capture enabled"] ACTIVE,
}
impl CC4EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CC4EW::INACTIVE => false,
            CC4EW::ACTIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC4EW<'a> {
    w: &'a mut W,
}
impl<'a> _CC4EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC4EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Output : OC1 is not active - Input : Capture disabled"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(CC4EW::INACTIVE)
    }
    #[doc = "Output : OC1 signal is output on the corresponding output pin - Input : Capture enabled"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(CC4EW::ACTIVE)
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
#[doc = "Values that can be written to the field `CC3NP`"]
pub enum CC3NPW {
    #[doc = "active high"] HIGH,
    #[doc = "active low"] LOW,
}
impl CC3NPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CC3NPW::HIGH => false,
            CC3NPW::LOW => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC3NPW<'a> {
    w: &'a mut W,
}
impl<'a> _CC3NPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC3NPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "active high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CC3NPW::HIGH)
    }
    #[doc = "active low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CC3NPW::LOW)
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
#[doc = "Values that can be written to the field `CC3NE`"]
pub enum CC3NEW {
    #[doc = "Off"] DISABLED,
    #[doc = "On"] ENABLED,
}
impl CC3NEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CC3NEW::DISABLED => false,
            CC3NEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC3NEW<'a> {
    w: &'a mut W,
}
impl<'a> _CC3NEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC3NEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC3NEW::DISABLED)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC3NEW::ENABLED)
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
#[doc = "Values that can be written to the field `CC3P`"]
pub enum CC3PW {
    #[doc = "active high"] HIGH,
    #[doc = "active low"] LOW,
}
impl CC3PW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CC3PW::HIGH => false,
            CC3PW::LOW => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC3PW<'a> {
    w: &'a mut W,
}
impl<'a> _CC3PW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC3PW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "active high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CC3PW::HIGH)
    }
    #[doc = "active low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CC3PW::LOW)
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
#[doc = "Values that can be written to the field `CC3E`"]
pub enum CC3EW {
    #[doc = "Output : OC1 is not active - Input : Capture disabled"] INACTIVE,
    #[doc = "Output : OC1 signal is output on the corresponding output pin - Input : Capture enabled"] ACTIVE,
}
impl CC3EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CC3EW::INACTIVE => false,
            CC3EW::ACTIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC3EW<'a> {
    w: &'a mut W,
}
impl<'a> _CC3EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC3EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Output : OC1 is not active - Input : Capture disabled"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(CC3EW::INACTIVE)
    }
    #[doc = "Output : OC1 signal is output on the corresponding output pin - Input : Capture enabled"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(CC3EW::ACTIVE)
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
#[doc = "Values that can be written to the field `CC2NP`"]
pub enum CC2NPW {
    #[doc = "active high"] HIGH,
    #[doc = "active low"] LOW,
}
impl CC2NPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CC2NPW::HIGH => false,
            CC2NPW::LOW => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC2NPW<'a> {
    w: &'a mut W,
}
impl<'a> _CC2NPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC2NPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "active high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CC2NPW::HIGH)
    }
    #[doc = "active low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CC2NPW::LOW)
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
#[doc = "Values that can be written to the field `CC2NE`"]
pub enum CC2NEW {
    #[doc = "Off"] DISABLED,
    #[doc = "On"] ENABLED,
}
impl CC2NEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CC2NEW::DISABLED => false,
            CC2NEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC2NEW<'a> {
    w: &'a mut W,
}
impl<'a> _CC2NEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC2NEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC2NEW::DISABLED)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC2NEW::ENABLED)
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
#[doc = "Values that can be written to the field `CC2P`"]
pub enum CC2PW {
    #[doc = "active high"] HIGH,
    #[doc = "active low"] LOW,
}
impl CC2PW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CC2PW::HIGH => false,
            CC2PW::LOW => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC2PW<'a> {
    w: &'a mut W,
}
impl<'a> _CC2PW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC2PW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "active high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CC2PW::HIGH)
    }
    #[doc = "active low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CC2PW::LOW)
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
#[doc = "Values that can be written to the field `CC2E`"]
pub enum CC2EW {
    #[doc = "Output : OC1 is not active - Input : Capture disabled"] INACTIVE,
    #[doc = "Output : OC1 signal is output on the corresponding output pin - Input : Capture enabled"] ACTIVE,
}
impl CC2EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CC2EW::INACTIVE => false,
            CC2EW::ACTIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC2EW<'a> {
    w: &'a mut W,
}
impl<'a> _CC2EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC2EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Output : OC1 is not active - Input : Capture disabled"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(CC2EW::INACTIVE)
    }
    #[doc = "Output : OC1 signal is output on the corresponding output pin - Input : Capture enabled"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(CC2EW::ACTIVE)
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
#[doc = "Values that can be written to the field `CC1NP`"]
pub enum CC1NPW {
    #[doc = "active high"] HIGH,
    #[doc = "active low"] LOW,
}
impl CC1NPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CC1NPW::HIGH => false,
            CC1NPW::LOW => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC1NPW<'a> {
    w: &'a mut W,
}
impl<'a> _CC1NPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC1NPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "active high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CC1NPW::HIGH)
    }
    #[doc = "active low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CC1NPW::LOW)
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
#[doc = "Values that can be written to the field `CC1NE`"]
pub enum CC1NEW {
    #[doc = "Off"] DISABLED,
    #[doc = "On"] ENABLED,
}
impl CC1NEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CC1NEW::DISABLED => false,
            CC1NEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC1NEW<'a> {
    w: &'a mut W,
}
impl<'a> _CC1NEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC1NEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC1NEW::DISABLED)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC1NEW::ENABLED)
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
#[doc = "Values that can be written to the field `CC1P`"]
pub enum CC1PW {
    #[doc = "active high"] HIGH,
    #[doc = "active low"] LOW,
}
impl CC1PW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CC1PW::HIGH => false,
            CC1PW::LOW => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC1PW<'a> {
    w: &'a mut W,
}
impl<'a> _CC1PW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC1PW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "active high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CC1PW::HIGH)
    }
    #[doc = "active low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CC1PW::LOW)
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
#[doc = "Values that can be written to the field `CC1E`"]
pub enum CC1EW {
    #[doc = "Output : OC1 is not active - Input : Capture disabled"] INACTIVE,
    #[doc = "Output : OC1 signal is output on the corresponding output pin - Input : Capture enabled"] ACTIVE,
}
impl CC1EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CC1EW::INACTIVE => false,
            CC1EW::ACTIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC1EW<'a> {
    w: &'a mut W,
}
impl<'a> _CC1EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC1EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Output : OC1 is not active - Input : Capture disabled"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(CC1EW::INACTIVE)
    }
    #[doc = "Output : OC1 signal is output on the corresponding output pin - Input : Capture enabled"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(CC1EW::ACTIVE)
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
    #[doc = "Bit 13 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn cc4p(&self) -> CC4PR {
        CC4PR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Capture/Compare 4 output enable"]
    #[inline(always)]
    pub fn cc4e(&self) -> CC4ER {
        CC4ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn cc3np(&self) -> CC3NPR {
        CC3NPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Capture/Compare 3 complementary output enable"]
    #[inline(always)]
    pub fn cc3ne(&self) -> CC3NER {
        CC3NER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn cc3p(&self) -> CC3PR {
        CC3PR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Capture/Compare 3 output enable"]
    #[inline(always)]
    pub fn cc3e(&self) -> CC3ER {
        CC3ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Capture/Compare 2 output Polarity"]
    #[inline(always)]
    pub fn cc2np(&self) -> CC2NPR {
        CC2NPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Capture/Compare 2 complementary output enable"]
    #[inline(always)]
    pub fn cc2ne(&self) -> CC2NER {
        CC2NER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Capture/Compare 2 output Polarity"]
    #[inline(always)]
    pub fn cc2p(&self) -> CC2PR {
        CC2PR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Capture/Compare 2 output enable"]
    #[inline(always)]
    pub fn cc2e(&self) -> CC2ER {
        CC2ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn cc1np(&self) -> CC1NPR {
        CC1NPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Capture/Compare 1 complementary output enable"]
    #[inline(always)]
    pub fn cc1ne(&self) -> CC1NER {
        CC1NER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn cc1p(&self) -> CC1PR {
        CC1PR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Capture/Compare 1 output enable"]
    #[inline(always)]
    pub fn cc1e(&self) -> CC1ER {
        CC1ER::_from({
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
    #[doc = "Bit 13 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn cc4p(&mut self) -> _CC4PW {
        _CC4PW { w: self }
    }
    #[doc = "Bit 12 - Capture/Compare 4 output enable"]
    #[inline(always)]
    pub fn cc4e(&mut self) -> _CC4EW {
        _CC4EW { w: self }
    }
    #[doc = "Bit 11 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn cc3np(&mut self) -> _CC3NPW {
        _CC3NPW { w: self }
    }
    #[doc = "Bit 10 - Capture/Compare 3 complementary output enable"]
    #[inline(always)]
    pub fn cc3ne(&mut self) -> _CC3NEW {
        _CC3NEW { w: self }
    }
    #[doc = "Bit 9 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn cc3p(&mut self) -> _CC3PW {
        _CC3PW { w: self }
    }
    #[doc = "Bit 8 - Capture/Compare 3 output enable"]
    #[inline(always)]
    pub fn cc3e(&mut self) -> _CC3EW {
        _CC3EW { w: self }
    }
    #[doc = "Bit 7 - Capture/Compare 2 output Polarity"]
    #[inline(always)]
    pub fn cc2np(&mut self) -> _CC2NPW {
        _CC2NPW { w: self }
    }
    #[doc = "Bit 6 - Capture/Compare 2 complementary output enable"]
    #[inline(always)]
    pub fn cc2ne(&mut self) -> _CC2NEW {
        _CC2NEW { w: self }
    }
    #[doc = "Bit 5 - Capture/Compare 2 output Polarity"]
    #[inline(always)]
    pub fn cc2p(&mut self) -> _CC2PW {
        _CC2PW { w: self }
    }
    #[doc = "Bit 4 - Capture/Compare 2 output enable"]
    #[inline(always)]
    pub fn cc2e(&mut self) -> _CC2EW {
        _CC2EW { w: self }
    }
    #[doc = "Bit 3 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn cc1np(&mut self) -> _CC1NPW {
        _CC1NPW { w: self }
    }
    #[doc = "Bit 2 - Capture/Compare 1 complementary output enable"]
    #[inline(always)]
    pub fn cc1ne(&mut self) -> _CC1NEW {
        _CC1NEW { w: self }
    }
    #[doc = "Bit 1 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn cc1p(&mut self) -> _CC1PW {
        _CC1PW { w: self }
    }
    #[doc = "Bit 0 - Capture/Compare 1 output enable"]
    #[inline(always)]
    pub fn cc1e(&mut self) -> _CC1EW {
        _CC1EW { w: self }
    }
}
