#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SR {
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
#[doc = "Possible values of the field `CC4OF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC4OFR {
    #[doc = "Clear overcapture flag."]
    CLEARED,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CC4OFR {
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
            CC4OFR::CLEARED => false,
            CC4OFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CC4OFR {
        match value {
            false => CC4OFR::CLEARED,
            i => CC4OFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline]
    pub fn is_cleared(&self) -> bool {
        *self == CC4OFR::CLEARED
    }
}
#[doc = "Possible values of the field `CC3OF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC3OFR {
    #[doc = "Clear overcapture flag."]
    CLEARED,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CC3OFR {
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
            CC3OFR::CLEARED => false,
            CC3OFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CC3OFR {
        match value {
            false => CC3OFR::CLEARED,
            i => CC3OFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline]
    pub fn is_cleared(&self) -> bool {
        *self == CC3OFR::CLEARED
    }
}
#[doc = "Possible values of the field `CC2OF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC2OFR {
    #[doc = "Clear overcapture flag."]
    CLEARED,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CC2OFR {
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
            CC2OFR::CLEARED => false,
            CC2OFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CC2OFR {
        match value {
            false => CC2OFR::CLEARED,
            i => CC2OFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline]
    pub fn is_cleared(&self) -> bool {
        *self == CC2OFR::CLEARED
    }
}
#[doc = "Possible values of the field `CC1OF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1OFR {
    #[doc = "Clear overcapture flag."]
    CLEARED,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CC1OFR {
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
            CC1OFR::CLEARED => false,
            CC1OFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CC1OFR {
        match value {
            false => CC1OFR::CLEARED,
            i => CC1OFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline]
    pub fn is_cleared(&self) -> bool {
        *self == CC1OFR::CLEARED
    }
}
#[doc = "Possible values of the field `BIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIFR {
    #[doc = "Clear overcapture flag."]
    CLEARED,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl BIFR {
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
            BIFR::CLEARED => false,
            BIFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BIFR {
        match value {
            false => BIFR::CLEARED,
            i => BIFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline]
    pub fn is_cleared(&self) -> bool {
        *self == BIFR::CLEARED
    }
}
#[doc = "Possible values of the field `TIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIFR {
    #[doc = "Clear overcapture flag."]
    CLEARED,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl TIFR {
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
            TIFR::CLEARED => false,
            TIFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIFR {
        match value {
            false => TIFR::CLEARED,
            i => TIFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline]
    pub fn is_cleared(&self) -> bool {
        *self == TIFR::CLEARED
    }
}
#[doc = "Possible values of the field `COMIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMIFR {
    #[doc = "Clear overcapture flag."]
    CLEARED,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl COMIFR {
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
            COMIFR::CLEARED => false,
            COMIFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMIFR {
        match value {
            false => COMIFR::CLEARED,
            i => COMIFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline]
    pub fn is_cleared(&self) -> bool {
        *self == COMIFR::CLEARED
    }
}
#[doc = "Possible values of the field `CC4IF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC4IFR {
    #[doc = "Clear overcapture flag."]
    CLEARED,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CC4IFR {
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
            CC4IFR::CLEARED => false,
            CC4IFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CC4IFR {
        match value {
            false => CC4IFR::CLEARED,
            i => CC4IFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline]
    pub fn is_cleared(&self) -> bool {
        *self == CC4IFR::CLEARED
    }
}
#[doc = "Possible values of the field `CC3IF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC3IFR {
    #[doc = "Clear overcapture flag."]
    CLEARED,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CC3IFR {
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
            CC3IFR::CLEARED => false,
            CC3IFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CC3IFR {
        match value {
            false => CC3IFR::CLEARED,
            i => CC3IFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline]
    pub fn is_cleared(&self) -> bool {
        *self == CC3IFR::CLEARED
    }
}
#[doc = "Possible values of the field `CC2IF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC2IFR {
    #[doc = "Clear overcapture flag."]
    CLEARED,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CC2IFR {
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
            CC2IFR::CLEARED => false,
            CC2IFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CC2IFR {
        match value {
            false => CC2IFR::CLEARED,
            i => CC2IFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline]
    pub fn is_cleared(&self) -> bool {
        *self == CC2IFR::CLEARED
    }
}
#[doc = "Possible values of the field `CC1IF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1IFR {
    #[doc = "Clear overcapture flag."]
    CLEARED,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CC1IFR {
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
            CC1IFR::CLEARED => false,
            CC1IFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CC1IFR {
        match value {
            false => CC1IFR::CLEARED,
            i => CC1IFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline]
    pub fn is_cleared(&self) -> bool {
        *self == CC1IFR::CLEARED
    }
}
#[doc = "Possible values of the field `UIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UIFR {
    #[doc = "No update occurred"]
    NOUPDATE,
    #[doc = "Update interrupt pending"]
    PENDING,
}
impl UIFR {
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
            UIFR::NOUPDATE => false,
            UIFR::PENDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UIFR {
        match value {
            false => UIFR::NOUPDATE,
            true => UIFR::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOUPDATE`"]
    #[inline]
    pub fn is_no_update(&self) -> bool {
        *self == UIFR::NOUPDATE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline]
    pub fn is_pending(&self) -> bool {
        *self == UIFR::PENDING
    }
}
#[doc = "Values that can be written to the field `CC4OF`"]
pub enum CC4OFW {
    #[doc = "Clear overcapture flag."]
    CLEARED,
}
impl CC4OFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CC4OFW::CLEARED => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC4OFW<'a> {
    w: &'a mut W,
}
impl<'a> _CC4OFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC4OFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear overcapture flag."]
    #[inline]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CC4OFW::CLEARED)
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
#[doc = "Values that can be written to the field `CC3OF`"]
pub enum CC3OFW {
    #[doc = "Clear overcapture flag."]
    CLEARED,
}
impl CC3OFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CC3OFW::CLEARED => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC3OFW<'a> {
    w: &'a mut W,
}
impl<'a> _CC3OFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC3OFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear overcapture flag."]
    #[inline]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CC3OFW::CLEARED)
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
#[doc = "Values that can be written to the field `CC2OF`"]
pub enum CC2OFW {
    #[doc = "Clear overcapture flag."]
    CLEARED,
}
impl CC2OFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CC2OFW::CLEARED => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC2OFW<'a> {
    w: &'a mut W,
}
impl<'a> _CC2OFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC2OFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear overcapture flag."]
    #[inline]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CC2OFW::CLEARED)
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
#[doc = "Values that can be written to the field `CC1OF`"]
pub enum CC1OFW {
    #[doc = "Clear overcapture flag."]
    CLEARED,
}
impl CC1OFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CC1OFW::CLEARED => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC1OFW<'a> {
    w: &'a mut W,
}
impl<'a> _CC1OFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC1OFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear overcapture flag."]
    #[inline]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CC1OFW::CLEARED)
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
#[doc = "Values that can be written to the field `BIF`"]
pub enum BIFW {
    #[doc = "Clear overcapture flag."]
    CLEARED,
}
impl BIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BIFW::CLEARED => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BIFW<'a> {
    w: &'a mut W,
}
impl<'a> _BIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear overcapture flag."]
    #[inline]
    pub fn cleared(self) -> &'a mut W {
        self.variant(BIFW::CLEARED)
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
#[doc = "Values that can be written to the field `TIF`"]
pub enum TIFW {
    #[doc = "Clear overcapture flag."]
    CLEARED,
}
impl TIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIFW::CLEARED => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIFW<'a> {
    w: &'a mut W,
}
impl<'a> _TIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear overcapture flag."]
    #[inline]
    pub fn cleared(self) -> &'a mut W {
        self.variant(TIFW::CLEARED)
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
#[doc = "Values that can be written to the field `COMIF`"]
pub enum COMIFW {
    #[doc = "Clear overcapture flag."]
    CLEARED,
}
impl COMIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMIFW::CLEARED => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMIFW<'a> {
    w: &'a mut W,
}
impl<'a> _COMIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear overcapture flag."]
    #[inline]
    pub fn cleared(self) -> &'a mut W {
        self.variant(COMIFW::CLEARED)
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
#[doc = "Values that can be written to the field `CC4IF`"]
pub enum CC4IFW {
    #[doc = "Clear overcapture flag."]
    CLEARED,
}
impl CC4IFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CC4IFW::CLEARED => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC4IFW<'a> {
    w: &'a mut W,
}
impl<'a> _CC4IFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC4IFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear overcapture flag."]
    #[inline]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CC4IFW::CLEARED)
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
#[doc = "Values that can be written to the field `CC3IF`"]
pub enum CC3IFW {
    #[doc = "Clear overcapture flag."]
    CLEARED,
}
impl CC3IFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CC3IFW::CLEARED => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC3IFW<'a> {
    w: &'a mut W,
}
impl<'a> _CC3IFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC3IFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear overcapture flag."]
    #[inline]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CC3IFW::CLEARED)
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
#[doc = "Values that can be written to the field `CC2IF`"]
pub enum CC2IFW {
    #[doc = "Clear overcapture flag."]
    CLEARED,
}
impl CC2IFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CC2IFW::CLEARED => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC2IFW<'a> {
    w: &'a mut W,
}
impl<'a> _CC2IFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC2IFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear overcapture flag."]
    #[inline]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CC2IFW::CLEARED)
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
#[doc = "Values that can be written to the field `CC1IF`"]
pub enum CC1IFW {
    #[doc = "Clear overcapture flag."]
    CLEARED,
}
impl CC1IFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CC1IFW::CLEARED => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC1IFW<'a> {
    w: &'a mut W,
}
impl<'a> _CC1IFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC1IFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear overcapture flag."]
    #[inline]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CC1IFW::CLEARED)
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
#[doc = "Values that can be written to the field `UIF`"]
pub enum UIFW {
    #[doc = "Clears the update interrupt flag"]
    CLEAR,
}
impl UIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UIFW::CLEAR => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UIFW<'a> {
    w: &'a mut W,
}
impl<'a> _UIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the update interrupt flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(UIFW::CLEAR)
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
    #[doc = "Bit 12 - Capture/Compare 4 overcapture flag"]
    #[inline]
    pub fn cc4of(&self) -> CC4OFR {
        CC4OFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Capture/Compare 3 overcapture flag"]
    #[inline]
    pub fn cc3of(&self) -> CC3OFR {
        CC3OFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Capture/compare 2 overcapture flag"]
    #[inline]
    pub fn cc2of(&self) -> CC2OFR {
        CC2OFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Capture/Compare 1 overcapture flag"]
    #[inline]
    pub fn cc1of(&self) -> CC1OFR {
        CC1OFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Break interrupt flag"]
    #[inline]
    pub fn bif(&self) -> BIFR {
        BIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline]
    pub fn tif(&self) -> TIFR {
        TIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - COM interrupt flag"]
    #[inline]
    pub fn comif(&self) -> COMIFR {
        COMIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Capture/Compare 4 interrupt flag"]
    #[inline]
    pub fn cc4if(&self) -> CC4IFR {
        CC4IFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Capture/Compare 3 interrupt flag"]
    #[inline]
    pub fn cc3if(&self) -> CC3IFR {
        CC3IFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt flag"]
    #[inline]
    pub fn cc2if(&self) -> CC2IFR {
        CC2IFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Capture/compare 1 interrupt flag"]
    #[inline]
    pub fn cc1if(&self) -> CC1IFR {
        CC1IFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline]
    pub fn uif(&self) -> UIFR {
        UIFR::_from({
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
    #[doc = "Bit 12 - Capture/Compare 4 overcapture flag"]
    #[inline]
    pub fn cc4of(&mut self) -> _CC4OFW {
        _CC4OFW { w: self }
    }
    #[doc = "Bit 11 - Capture/Compare 3 overcapture flag"]
    #[inline]
    pub fn cc3of(&mut self) -> _CC3OFW {
        _CC3OFW { w: self }
    }
    #[doc = "Bit 10 - Capture/compare 2 overcapture flag"]
    #[inline]
    pub fn cc2of(&mut self) -> _CC2OFW {
        _CC2OFW { w: self }
    }
    #[doc = "Bit 9 - Capture/Compare 1 overcapture flag"]
    #[inline]
    pub fn cc1of(&mut self) -> _CC1OFW {
        _CC1OFW { w: self }
    }
    #[doc = "Bit 7 - Break interrupt flag"]
    #[inline]
    pub fn bif(&mut self) -> _BIFW {
        _BIFW { w: self }
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline]
    pub fn tif(&mut self) -> _TIFW {
        _TIFW { w: self }
    }
    #[doc = "Bit 5 - COM interrupt flag"]
    #[inline]
    pub fn comif(&mut self) -> _COMIFW {
        _COMIFW { w: self }
    }
    #[doc = "Bit 4 - Capture/Compare 4 interrupt flag"]
    #[inline]
    pub fn cc4if(&mut self) -> _CC4IFW {
        _CC4IFW { w: self }
    }
    #[doc = "Bit 3 - Capture/Compare 3 interrupt flag"]
    #[inline]
    pub fn cc3if(&mut self) -> _CC3IFW {
        _CC3IFW { w: self }
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt flag"]
    #[inline]
    pub fn cc2if(&mut self) -> _CC2IFW {
        _CC2IFW { w: self }
    }
    #[doc = "Bit 1 - Capture/compare 1 interrupt flag"]
    #[inline]
    pub fn cc1if(&mut self) -> _CC1IFW {
        _CC1IFW { w: self }
    }
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline]
    pub fn uif(&mut self) -> _UIFW {
        _UIFW { w: self }
    }
}
