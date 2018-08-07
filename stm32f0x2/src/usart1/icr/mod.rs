#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ICR {
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
#[doc = "Possible values of the field `WUCF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUCFR {
    #[doc = "clears the WUF flag in the USART_ISR register."]
    CLEAR,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl WUCFR {
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
            WUCFR::CLEAR => true,
            WUCFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUCFR {
        match value {
            true => WUCFR::CLEAR,
            i => WUCFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == WUCFR::CLEAR
    }
}
#[doc = "Possible values of the field `CMCF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMCFR {
    #[doc = "clears the CMF flag in the USART_ISR register."]
    CLEAR,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CMCFR {
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
            CMCFR::CLEAR => true,
            CMCFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMCFR {
        match value {
            true => CMCFR::CLEAR,
            i => CMCFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == CMCFR::CLEAR
    }
}
#[doc = "Possible values of the field `EOBCF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOBCFR {
    #[doc = "clears the EOBF flag in the USART_ISR register."]
    CLEAR,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl EOBCFR {
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
            EOBCFR::CLEAR => true,
            EOBCFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EOBCFR {
        match value {
            true => EOBCFR::CLEAR,
            i => EOBCFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == EOBCFR::CLEAR
    }
}
#[doc = "Possible values of the field `RTOCF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTOCFR {
    #[doc = "clears the RTOF flag in the USART_ISR register."]
    CLEAR,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RTOCFR {
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
            RTOCFR::CLEAR => true,
            RTOCFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTOCFR {
        match value {
            true => RTOCFR::CLEAR,
            i => RTOCFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == RTOCFR::CLEAR
    }
}
#[doc = "Possible values of the field `CTSCF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSCFR {
    #[doc = "clears the CTSIF flag in the USART_ISR register."]
    CLEAR,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CTSCFR {
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
            CTSCFR::CLEAR => true,
            CTSCFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTSCFR {
        match value {
            true => CTSCFR::CLEAR,
            i => CTSCFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == CTSCFR::CLEAR
    }
}
#[doc = "Possible values of the field `LBDCF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBDCFR {
    #[doc = "clears the LBDF flag in the USART_ISR register."]
    CLEAR,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl LBDCFR {
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
            LBDCFR::CLEAR => true,
            LBDCFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LBDCFR {
        match value {
            true => LBDCFR::CLEAR,
            i => LBDCFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == LBDCFR::CLEAR
    }
}
#[doc = "Possible values of the field `TCCF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCCFR {
    #[doc = "clears the TC flag in the USART_ISR register."]
    CLEAR,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl TCCFR {
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
            TCCFR::CLEAR => true,
            TCCFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCCFR {
        match value {
            true => TCCFR::CLEAR,
            i => TCCFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == TCCFR::CLEAR
    }
}
#[doc = "Possible values of the field `IDLECF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLECFR {
    #[doc = "clears the IDLE flag in the USART_ISR register."]
    CLEAR,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl IDLECFR {
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
            IDLECFR::CLEAR => true,
            IDLECFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IDLECFR {
        match value {
            true => IDLECFR::CLEAR,
            i => IDLECFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == IDLECFR::CLEAR
    }
}
#[doc = "Possible values of the field `ORECF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ORECFR {
    #[doc = "clears the ORE flag in the USART_ISR register."]
    CLEAR,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ORECFR {
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
            ORECFR::CLEAR => true,
            ORECFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ORECFR {
        match value {
            true => ORECFR::CLEAR,
            i => ORECFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == ORECFR::CLEAR
    }
}
#[doc = "Possible values of the field `NCF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NCFR {
    #[doc = "clears the NF flag in the USART_ISR register."]
    CLEAR,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl NCFR {
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
            NCFR::CLEAR => true,
            NCFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NCFR {
        match value {
            true => NCFR::CLEAR,
            i => NCFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == NCFR::CLEAR
    }
}
#[doc = "Possible values of the field `FECF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FECFR {
    #[doc = "clears the FE flag in the USART_ISR register."]
    CLEAR,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl FECFR {
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
            FECFR::CLEAR => true,
            FECFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FECFR {
        match value {
            true => FECFR::CLEAR,
            i => FECFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == FECFR::CLEAR
    }
}
#[doc = "Possible values of the field `PECF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PECFR {
    #[doc = "clears the PE flag in the USART_ISR register."]
    CLEAR,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PECFR {
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
            PECFR::CLEAR => true,
            PECFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PECFR {
        match value {
            true => PECFR::CLEAR,
            i => PECFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == PECFR::CLEAR
    }
}
#[doc = "Values that can be written to the field `WUCF`"]
pub enum WUCFW {
    #[doc = "clears the WUF flag in the USART_ISR register."]
    CLEAR,
}
impl WUCFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUCFW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUCFW<'a> {
    w: &'a mut W,
}
impl<'a> _WUCFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUCFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "clears the WUF flag in the USART_ISR register."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(WUCFW::CLEAR)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CMCF`"]
pub enum CMCFW {
    #[doc = "clears the CMF flag in the USART_ISR register."]
    CLEAR,
}
impl CMCFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMCFW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMCFW<'a> {
    w: &'a mut W,
}
impl<'a> _CMCFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMCFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "clears the CMF flag in the USART_ISR register."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CMCFW::CLEAR)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EOBCF`"]
pub enum EOBCFW {
    #[doc = "clears the EOBF flag in the USART_ISR register."]
    CLEAR,
}
impl EOBCFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EOBCFW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EOBCFW<'a> {
    w: &'a mut W,
}
impl<'a> _EOBCFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EOBCFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "clears the EOBF flag in the USART_ISR register."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(EOBCFW::CLEAR)
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
#[doc = "Values that can be written to the field `RTOCF`"]
pub enum RTOCFW {
    #[doc = "clears the RTOF flag in the USART_ISR register."]
    CLEAR,
}
impl RTOCFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTOCFW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTOCFW<'a> {
    w: &'a mut W,
}
impl<'a> _RTOCFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTOCFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "clears the RTOF flag in the USART_ISR register."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(RTOCFW::CLEAR)
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
#[doc = "Values that can be written to the field `CTSCF`"]
pub enum CTSCFW {
    #[doc = "clears the CTSIF flag in the USART_ISR register."]
    CLEAR,
}
impl CTSCFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTSCFW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTSCFW<'a> {
    w: &'a mut W,
}
impl<'a> _CTSCFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTSCFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "clears the CTSIF flag in the USART_ISR register."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTSCFW::CLEAR)
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
#[doc = "Values that can be written to the field `LBDCF`"]
pub enum LBDCFW {
    #[doc = "clears the LBDF flag in the USART_ISR register."]
    CLEAR,
}
impl LBDCFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LBDCFW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LBDCFW<'a> {
    w: &'a mut W,
}
impl<'a> _LBDCFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LBDCFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "clears the LBDF flag in the USART_ISR register."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(LBDCFW::CLEAR)
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
#[doc = "Values that can be written to the field `TCCF`"]
pub enum TCCFW {
    #[doc = "clears the TC flag in the USART_ISR register."]
    CLEAR,
}
impl TCCFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCCFW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCCFW<'a> {
    w: &'a mut W,
}
impl<'a> _TCCFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCCFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "clears the TC flag in the USART_ISR register."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(TCCFW::CLEAR)
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
#[doc = "Values that can be written to the field `IDLECF`"]
pub enum IDLECFW {
    #[doc = "clears the IDLE flag in the USART_ISR register."]
    CLEAR,
}
impl IDLECFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IDLECFW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDLECFW<'a> {
    w: &'a mut W,
}
impl<'a> _IDLECFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDLECFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "clears the IDLE flag in the USART_ISR register."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(IDLECFW::CLEAR)
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
#[doc = "Values that can be written to the field `ORECF`"]
pub enum ORECFW {
    #[doc = "clears the ORE flag in the USART_ISR register."]
    CLEAR,
}
impl ORECFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ORECFW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ORECFW<'a> {
    w: &'a mut W,
}
impl<'a> _ORECFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ORECFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "clears the ORE flag in the USART_ISR register."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(ORECFW::CLEAR)
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
#[doc = "Values that can be written to the field `NCF`"]
pub enum NCFW {
    #[doc = "clears the NF flag in the USART_ISR register."]
    CLEAR,
}
impl NCFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NCFW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NCFW<'a> {
    w: &'a mut W,
}
impl<'a> _NCFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NCFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "clears the NF flag in the USART_ISR register."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(NCFW::CLEAR)
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
#[doc = "Values that can be written to the field `FECF`"]
pub enum FECFW {
    #[doc = "clears the FE flag in the USART_ISR register."]
    CLEAR,
}
impl FECFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FECFW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FECFW<'a> {
    w: &'a mut W,
}
impl<'a> _FECFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FECFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "clears the FE flag in the USART_ISR register."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(FECFW::CLEAR)
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
#[doc = "Values that can be written to the field `PECF`"]
pub enum PECFW {
    #[doc = "clears the PE flag in the USART_ISR register."]
    CLEAR,
}
impl PECFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PECFW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PECFW<'a> {
    w: &'a mut W,
}
impl<'a> _PECFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PECFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "clears the PE flag in the USART_ISR register."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(PECFW::CLEAR)
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
    #[doc = "Bit 20 - Wakeup from Stop mode clear flag"]
    #[inline]
    pub fn wucf(&self) -> WUCFR {
        WUCFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Character match clear flag"]
    #[inline]
    pub fn cmcf(&self) -> CMCFR {
        CMCFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - End of timeout clear flag"]
    #[inline]
    pub fn eobcf(&self) -> EOBCFR {
        EOBCFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Receiver timeout clear flag"]
    #[inline]
    pub fn rtocf(&self) -> RTOCFR {
        RTOCFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - CTS clear flag"]
    #[inline]
    pub fn ctscf(&self) -> CTSCFR {
        CTSCFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - LIN break detection clear flag"]
    #[inline]
    pub fn lbdcf(&self) -> LBDCFR {
        LBDCFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Transmission complete clear flag"]
    #[inline]
    pub fn tccf(&self) -> TCCFR {
        TCCFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Idle line detected clear flag"]
    #[inline]
    pub fn idlecf(&self) -> IDLECFR {
        IDLECFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Overrun error clear flag"]
    #[inline]
    pub fn orecf(&self) -> ORECFR {
        ORECFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Noise detected clear flag"]
    #[inline]
    pub fn ncf(&self) -> NCFR {
        NCFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Framing error clear flag"]
    #[inline]
    pub fn fecf(&self) -> FECFR {
        FECFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Parity error clear flag"]
    #[inline]
    pub fn pecf(&self) -> PECFR {
        PECFR::_from({
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
    #[doc = "Bit 20 - Wakeup from Stop mode clear flag"]
    #[inline]
    pub fn wucf(&mut self) -> _WUCFW {
        _WUCFW { w: self }
    }
    #[doc = "Bit 17 - Character match clear flag"]
    #[inline]
    pub fn cmcf(&mut self) -> _CMCFW {
        _CMCFW { w: self }
    }
    #[doc = "Bit 12 - End of timeout clear flag"]
    #[inline]
    pub fn eobcf(&mut self) -> _EOBCFW {
        _EOBCFW { w: self }
    }
    #[doc = "Bit 11 - Receiver timeout clear flag"]
    #[inline]
    pub fn rtocf(&mut self) -> _RTOCFW {
        _RTOCFW { w: self }
    }
    #[doc = "Bit 9 - CTS clear flag"]
    #[inline]
    pub fn ctscf(&mut self) -> _CTSCFW {
        _CTSCFW { w: self }
    }
    #[doc = "Bit 8 - LIN break detection clear flag"]
    #[inline]
    pub fn lbdcf(&mut self) -> _LBDCFW {
        _LBDCFW { w: self }
    }
    #[doc = "Bit 6 - Transmission complete clear flag"]
    #[inline]
    pub fn tccf(&mut self) -> _TCCFW {
        _TCCFW { w: self }
    }
    #[doc = "Bit 4 - Idle line detected clear flag"]
    #[inline]
    pub fn idlecf(&mut self) -> _IDLECFW {
        _IDLECFW { w: self }
    }
    #[doc = "Bit 3 - Overrun error clear flag"]
    #[inline]
    pub fn orecf(&mut self) -> _ORECFW {
        _ORECFW { w: self }
    }
    #[doc = "Bit 2 - Noise detected clear flag"]
    #[inline]
    pub fn ncf(&mut self) -> _NCFW {
        _NCFW { w: self }
    }
    #[doc = "Bit 1 - Framing error clear flag"]
    #[inline]
    pub fn fecf(&mut self) -> _FECFW {
        _FECFW { w: self }
    }
    #[doc = "Bit 0 - Parity error clear flag"]
    #[inline]
    pub fn pecf(&mut self) -> _PECFW {
        _PECFW { w: self }
    }
}
