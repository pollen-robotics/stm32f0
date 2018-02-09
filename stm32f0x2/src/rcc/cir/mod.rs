#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CIR {
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
#[doc = r" Value of the field"]
pub struct LSIRDYFR {
    bits: bool,
}
impl LSIRDYFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct LSERDYFR {
    bits: bool,
}
impl LSERDYFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct HSIRDYFR {
    bits: bool,
}
impl HSIRDYFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct HSERDYFR {
    bits: bool,
}
impl HSERDYFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PLLRDYFR {
    bits: bool,
}
impl PLLRDYFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct HSI14RDYFR {
    bits: bool,
}
impl HSI14RDYFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct HSI48RDYFR {
    bits: bool,
}
impl HSI48RDYFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct CSSFR {
    bits: bool,
}
impl CSSFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `LSIRDYIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSIRDYIER {
    #[doc = "LSI ready interrupt disabled."]
    DISABLED,
    #[doc = "LSI ready interrupt enabled."]
    ENABLED,
}
impl LSIRDYIER {
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
            LSIRDYIER::DISABLED => false,
            LSIRDYIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LSIRDYIER {
        match value {
            false => LSIRDYIER::DISABLED,
            true => LSIRDYIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == LSIRDYIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == LSIRDYIER::ENABLED
    }
}
#[doc = "Possible values of the field `LSERDYIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSERDYIER {
    #[doc = "LSE ready interrupt disabled."]
    DISABLED,
    #[doc = "LSE ready interrupt enabled."]
    ENABLED,
}
impl LSERDYIER {
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
            LSERDYIER::DISABLED => false,
            LSERDYIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LSERDYIER {
        match value {
            false => LSERDYIER::DISABLED,
            true => LSERDYIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == LSERDYIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == LSERDYIER::ENABLED
    }
}
#[doc = "Possible values of the field `HSIRDYIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSIRDYIER {
    #[doc = "HSI ready interrupt disabled."]
    DISABLED,
    #[doc = "HSI ready interrupt enabled."]
    ENABLED,
}
impl HSIRDYIER {
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
            HSIRDYIER::DISABLED => false,
            HSIRDYIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HSIRDYIER {
        match value {
            false => HSIRDYIER::DISABLED,
            true => HSIRDYIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == HSIRDYIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == HSIRDYIER::ENABLED
    }
}
#[doc = "Possible values of the field `HSERDYIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSERDYIER {
    #[doc = "HSE ready interrupt disabled."]
    DISABLED,
    #[doc = "HSE ready interrupt enabled."]
    ENABLED,
}
impl HSERDYIER {
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
            HSERDYIER::DISABLED => false,
            HSERDYIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HSERDYIER {
        match value {
            false => HSERDYIER::DISABLED,
            true => HSERDYIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == HSERDYIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == HSERDYIER::ENABLED
    }
}
#[doc = "Possible values of the field `PLLRDYIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLRDYIER {
    #[doc = "PLL lock interrupt disabled."]
    DISABLED,
    #[doc = "PLL lock interrupt enabled."]
    ENABLED,
}
impl PLLRDYIER {
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
            PLLRDYIER::DISABLED => false,
            PLLRDYIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLLRDYIER {
        match value {
            false => PLLRDYIER::DISABLED,
            true => PLLRDYIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PLLRDYIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PLLRDYIER::ENABLED
    }
}
#[doc = "Possible values of the field `HSI14RDYE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSI14RDYER {
    #[doc = "HSI14 ready interrupt disabled."]
    DISABLED,
    #[doc = "HSI14 ready interrupt enabled."]
    ENABLED,
}
impl HSI14RDYER {
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
            HSI14RDYER::DISABLED => false,
            HSI14RDYER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HSI14RDYER {
        match value {
            false => HSI14RDYER::DISABLED,
            true => HSI14RDYER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == HSI14RDYER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == HSI14RDYER::ENABLED
    }
}
#[doc = "Possible values of the field `HSI48RDYIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSI48RDYIER {
    #[doc = "HSI48 ready interrupt disabled."]
    DISABLED,
    #[doc = "HSI48 ready interrupt enabled."]
    ENABLED,
}
impl HSI48RDYIER {
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
            HSI48RDYIER::DISABLED => false,
            HSI48RDYIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HSI48RDYIER {
        match value {
            false => HSI48RDYIER::DISABLED,
            true => HSI48RDYIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == HSI48RDYIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == HSI48RDYIER::ENABLED
    }
}
#[doc = "Values that can be written to the field `LSIRDYIE`"]
pub enum LSIRDYIEW {
    #[doc = "LSI ready interrupt disabled."]
    DISABLED,
    #[doc = "LSI ready interrupt enabled."]
    ENABLED,
}
impl LSIRDYIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LSIRDYIEW::DISABLED => false,
            LSIRDYIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LSIRDYIEW<'a> {
    w: &'a mut W,
}
impl<'a> _LSIRDYIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LSIRDYIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LSI ready interrupt disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSIRDYIEW::DISABLED)
    }
    #[doc = "LSI ready interrupt enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LSIRDYIEW::ENABLED)
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
#[doc = "Values that can be written to the field `LSERDYIE`"]
pub enum LSERDYIEW {
    #[doc = "LSE ready interrupt disabled."]
    DISABLED,
    #[doc = "LSE ready interrupt enabled."]
    ENABLED,
}
impl LSERDYIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LSERDYIEW::DISABLED => false,
            LSERDYIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LSERDYIEW<'a> {
    w: &'a mut W,
}
impl<'a> _LSERDYIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LSERDYIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LSE ready interrupt disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSERDYIEW::DISABLED)
    }
    #[doc = "LSE ready interrupt enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LSERDYIEW::ENABLED)
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
#[doc = "Values that can be written to the field `HSIRDYIE`"]
pub enum HSIRDYIEW {
    #[doc = "HSI ready interrupt disabled."]
    DISABLED,
    #[doc = "HSI ready interrupt enabled."]
    ENABLED,
}
impl HSIRDYIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HSIRDYIEW::DISABLED => false,
            HSIRDYIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HSIRDYIEW<'a> {
    w: &'a mut W,
}
impl<'a> _HSIRDYIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HSIRDYIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HSI ready interrupt disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HSIRDYIEW::DISABLED)
    }
    #[doc = "HSI ready interrupt enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HSIRDYIEW::ENABLED)
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
#[doc = "Values that can be written to the field `HSERDYIE`"]
pub enum HSERDYIEW {
    #[doc = "HSE ready interrupt disabled."]
    DISABLED,
    #[doc = "HSE ready interrupt enabled."]
    ENABLED,
}
impl HSERDYIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HSERDYIEW::DISABLED => false,
            HSERDYIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HSERDYIEW<'a> {
    w: &'a mut W,
}
impl<'a> _HSERDYIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HSERDYIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HSE ready interrupt disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HSERDYIEW::DISABLED)
    }
    #[doc = "HSE ready interrupt enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HSERDYIEW::ENABLED)
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
#[doc = "Values that can be written to the field `PLLRDYIE`"]
pub enum PLLRDYIEW {
    #[doc = "PLL lock interrupt disabled."]
    DISABLED,
    #[doc = "PLL lock interrupt enabled."]
    ENABLED,
}
impl PLLRDYIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLLRDYIEW::DISABLED => false,
            PLLRDYIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLLRDYIEW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLRDYIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLLRDYIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PLL lock interrupt disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLRDYIEW::DISABLED)
    }
    #[doc = "PLL lock interrupt enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLRDYIEW::ENABLED)
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
#[doc = "Values that can be written to the field `HSI14RDYE`"]
pub enum HSI14RDYEW {
    #[doc = "HSI14 ready interrupt disabled."]
    DISABLED,
    #[doc = "HSI14 ready interrupt enabled."]
    ENABLED,
}
impl HSI14RDYEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HSI14RDYEW::DISABLED => false,
            HSI14RDYEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HSI14RDYEW<'a> {
    w: &'a mut W,
}
impl<'a> _HSI14RDYEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HSI14RDYEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HSI14 ready interrupt disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HSI14RDYEW::DISABLED)
    }
    #[doc = "HSI14 ready interrupt enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HSI14RDYEW::ENABLED)
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
#[doc = "Values that can be written to the field `HSI48RDYIE`"]
pub enum HSI48RDYIEW {
    #[doc = "HSI48 ready interrupt disabled."]
    DISABLED,
    #[doc = "HSI48 ready interrupt enabled."]
    ENABLED,
}
impl HSI48RDYIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HSI48RDYIEW::DISABLED => false,
            HSI48RDYIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HSI48RDYIEW<'a> {
    w: &'a mut W,
}
impl<'a> _HSI48RDYIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HSI48RDYIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HSI48 ready interrupt disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HSI48RDYIEW::DISABLED)
    }
    #[doc = "HSI48 ready interrupt enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HSI48RDYIEW::ENABLED)
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
#[doc = "Values that can be written to the field `LSIRDYC`"]
pub enum LSIRDYCW {
    #[doc = "LSIRDYF Cleared."]
    RESET,
}
impl LSIRDYCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LSIRDYCW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LSIRDYCW<'a> {
    w: &'a mut W,
}
impl<'a> _LSIRDYCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LSIRDYCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LSIRDYF Cleared."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(LSIRDYCW::RESET)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LSERDYC`"]
pub enum LSERDYCW {
    #[doc = "LSERDYF Cleared."]
    RESET,
}
impl LSERDYCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LSERDYCW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LSERDYCW<'a> {
    w: &'a mut W,
}
impl<'a> _LSERDYCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LSERDYCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LSERDYF Cleared."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(LSERDYCW::RESET)
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
#[doc = "Values that can be written to the field `HSIRDYC`"]
pub enum HSIRDYCW {
    #[doc = "HSIRDYF Cleared."]
    RESET,
}
impl HSIRDYCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HSIRDYCW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HSIRDYCW<'a> {
    w: &'a mut W,
}
impl<'a> _HSIRDYCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HSIRDYCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HSIRDYF Cleared."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(HSIRDYCW::RESET)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HSERDYC`"]
pub enum HSERDYCW {
    #[doc = "HSERDYF Cleared."]
    RESET,
}
impl HSERDYCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HSERDYCW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HSERDYCW<'a> {
    w: &'a mut W,
}
impl<'a> _HSERDYCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HSERDYCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HSERDYF Cleared."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(HSERDYCW::RESET)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLLRDYC`"]
pub enum PLLRDYCW {
    #[doc = "PLLRDYF Cleared."]
    RESET,
}
impl PLLRDYCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLLRDYCW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLLRDYCW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLRDYCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLLRDYCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PLLRDYF Cleared."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(PLLRDYCW::RESET)
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
#[doc = "Values that can be written to the field `HSI14RDYC`"]
pub enum HSI14RDYCW {
    #[doc = "HSI14RDYF Cleared."]
    RESET,
}
impl HSI14RDYCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HSI14RDYCW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HSI14RDYCW<'a> {
    w: &'a mut W,
}
impl<'a> _HSI14RDYCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HSI14RDYCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HSI14RDYF Cleared."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(HSI14RDYCW::RESET)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HSI48RDYC`"]
pub enum HSI48RDYCW {
    #[doc = "HSI48RDYF Cleared."]
    RESET,
}
impl HSI48RDYCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HSI48RDYCW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HSI48RDYCW<'a> {
    w: &'a mut W,
}
impl<'a> _HSI48RDYCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HSI48RDYCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HSI48RDYF Cleared."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(HSI48RDYCW::RESET)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CSSC`"]
pub enum CSSCW {
    #[doc = "CSSF Cleared."]
    RESET,
}
impl CSSCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CSSCW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSSCW<'a> {
    w: &'a mut W,
}
impl<'a> _CSSCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSSCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CSSF Cleared."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(CSSCW::RESET)
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
        const OFFSET: u8 = 23;
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
    #[doc = "Bit 0 - LSI Ready Interrupt flag"]
    #[inline]
    pub fn lsirdyf(&self) -> LSIRDYFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LSIRDYFR { bits }
    }
    #[doc = "Bit 1 - LSE Ready Interrupt flag"]
    #[inline]
    pub fn lserdyf(&self) -> LSERDYFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LSERDYFR { bits }
    }
    #[doc = "Bit 2 - HSI Ready Interrupt flag"]
    #[inline]
    pub fn hsirdyf(&self) -> HSIRDYFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HSIRDYFR { bits }
    }
    #[doc = "Bit 3 - HSE Ready Interrupt flag"]
    #[inline]
    pub fn hserdyf(&self) -> HSERDYFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HSERDYFR { bits }
    }
    #[doc = "Bit 4 - PLL Ready Interrupt flag"]
    #[inline]
    pub fn pllrdyf(&self) -> PLLRDYFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLLRDYFR { bits }
    }
    #[doc = "Bit 5 - HSI14 ready interrupt flag"]
    #[inline]
    pub fn hsi14rdyf(&self) -> HSI14RDYFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HSI14RDYFR { bits }
    }
    #[doc = "Bit 6 - HSI48 ready interrupt flag"]
    #[inline]
    pub fn hsi48rdyf(&self) -> HSI48RDYFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HSI48RDYFR { bits }
    }
    #[doc = "Bit 7 - Clock Security System Interrupt flag"]
    #[inline]
    pub fn cssf(&self) -> CSSFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CSSFR { bits }
    }
    #[doc = "Bit 8 - LSI Ready Interrupt Enable"]
    #[inline]
    pub fn lsirdyie(&self) -> LSIRDYIER {
        LSIRDYIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - LSE Ready Interrupt Enable"]
    #[inline]
    pub fn lserdyie(&self) -> LSERDYIER {
        LSERDYIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - HSI Ready Interrupt Enable"]
    #[inline]
    pub fn hsirdyie(&self) -> HSIRDYIER {
        HSIRDYIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - HSE Ready Interrupt Enable"]
    #[inline]
    pub fn hserdyie(&self) -> HSERDYIER {
        HSERDYIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - PLL Ready Interrupt Enable"]
    #[inline]
    pub fn pllrdyie(&self) -> PLLRDYIER {
        PLLRDYIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - HSI14 ready interrupt enable"]
    #[inline]
    pub fn hsi14rdye(&self) -> HSI14RDYER {
        HSI14RDYER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - HSI48 ready interrupt enable"]
    #[inline]
    pub fn hsi48rdyie(&self) -> HSI48RDYIER {
        HSI48RDYIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
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
    #[doc = "Bit 8 - LSI Ready Interrupt Enable"]
    #[inline]
    pub fn lsirdyie(&mut self) -> _LSIRDYIEW {
        _LSIRDYIEW { w: self }
    }
    #[doc = "Bit 9 - LSE Ready Interrupt Enable"]
    #[inline]
    pub fn lserdyie(&mut self) -> _LSERDYIEW {
        _LSERDYIEW { w: self }
    }
    #[doc = "Bit 10 - HSI Ready Interrupt Enable"]
    #[inline]
    pub fn hsirdyie(&mut self) -> _HSIRDYIEW {
        _HSIRDYIEW { w: self }
    }
    #[doc = "Bit 11 - HSE Ready Interrupt Enable"]
    #[inline]
    pub fn hserdyie(&mut self) -> _HSERDYIEW {
        _HSERDYIEW { w: self }
    }
    #[doc = "Bit 12 - PLL Ready Interrupt Enable"]
    #[inline]
    pub fn pllrdyie(&mut self) -> _PLLRDYIEW {
        _PLLRDYIEW { w: self }
    }
    #[doc = "Bit 13 - HSI14 ready interrupt enable"]
    #[inline]
    pub fn hsi14rdye(&mut self) -> _HSI14RDYEW {
        _HSI14RDYEW { w: self }
    }
    #[doc = "Bit 14 - HSI48 ready interrupt enable"]
    #[inline]
    pub fn hsi48rdyie(&mut self) -> _HSI48RDYIEW {
        _HSI48RDYIEW { w: self }
    }
    #[doc = "Bit 16 - LSI Ready Interrupt Clear"]
    #[inline]
    pub fn lsirdyc(&mut self) -> _LSIRDYCW {
        _LSIRDYCW { w: self }
    }
    #[doc = "Bit 17 - LSE Ready Interrupt Clear"]
    #[inline]
    pub fn lserdyc(&mut self) -> _LSERDYCW {
        _LSERDYCW { w: self }
    }
    #[doc = "Bit 18 - HSI Ready Interrupt Clear"]
    #[inline]
    pub fn hsirdyc(&mut self) -> _HSIRDYCW {
        _HSIRDYCW { w: self }
    }
    #[doc = "Bit 19 - HSE Ready Interrupt Clear"]
    #[inline]
    pub fn hserdyc(&mut self) -> _HSERDYCW {
        _HSERDYCW { w: self }
    }
    #[doc = "Bit 20 - PLL Ready Interrupt Clear"]
    #[inline]
    pub fn pllrdyc(&mut self) -> _PLLRDYCW {
        _PLLRDYCW { w: self }
    }
    #[doc = "Bit 21 - HSI 14 MHz Ready Interrupt Clear"]
    #[inline]
    pub fn hsi14rdyc(&mut self) -> _HSI14RDYCW {
        _HSI14RDYCW { w: self }
    }
    #[doc = "Bit 22 - HSI48 Ready Interrupt Clear"]
    #[inline]
    pub fn hsi48rdyc(&mut self) -> _HSI48RDYCW {
        _HSI48RDYCW { w: self }
    }
    #[doc = "Bit 23 - Clock security system interrupt clear"]
    #[inline]
    pub fn cssc(&mut self) -> _CSSCW {
        _CSSCW { w: self }
    }
}
