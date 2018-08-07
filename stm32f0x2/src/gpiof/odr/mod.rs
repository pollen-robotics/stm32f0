#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ODR {
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
#[doc = "Possible values of the field `ODR15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ODR15R {
    #[doc = "Reset output value"]
    RESET,
    #[doc = "Set output value"]
    SET,
}
impl ODR15R {
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
            ODR15R::RESET => false,
            ODR15R::SET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ODR15R {
        match value {
            false => ODR15R::RESET,
            true => ODR15R::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == ODR15R::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == ODR15R::SET
    }
}
#[doc = "Possible values of the field `ODR14`"]
pub type ODR14R = ODR15R;
#[doc = "Possible values of the field `ODR13`"]
pub type ODR13R = ODR15R;
#[doc = "Possible values of the field `ODR12`"]
pub type ODR12R = ODR15R;
#[doc = "Possible values of the field `ODR11`"]
pub type ODR11R = ODR15R;
#[doc = "Possible values of the field `ODR10`"]
pub type ODR10R = ODR15R;
#[doc = "Possible values of the field `ODR9`"]
pub type ODR9R = ODR15R;
#[doc = "Possible values of the field `ODR8`"]
pub type ODR8R = ODR15R;
#[doc = "Possible values of the field `ODR7`"]
pub type ODR7R = ODR15R;
#[doc = "Possible values of the field `ODR6`"]
pub type ODR6R = ODR15R;
#[doc = "Possible values of the field `ODR5`"]
pub type ODR5R = ODR15R;
#[doc = "Possible values of the field `ODR4`"]
pub type ODR4R = ODR15R;
#[doc = "Possible values of the field `ODR3`"]
pub type ODR3R = ODR15R;
#[doc = "Possible values of the field `ODR2`"]
pub type ODR2R = ODR15R;
#[doc = "Possible values of the field `ODR1`"]
pub type ODR1R = ODR15R;
#[doc = "Possible values of the field `ODR0`"]
pub type ODR0R = ODR15R;
#[doc = "Values that can be written to the field `ODR15`"]
pub enum ODR15W {
    #[doc = "Reset output value"]
    RESET,
    #[doc = "Set output value"]
    SET,
}
impl ODR15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ODR15W::RESET => false,
            ODR15W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ODR15W<'a> {
    w: &'a mut W,
}
impl<'a> _ODR15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ODR15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset output value"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(ODR15W::RESET)
    }
    #[doc = "Set output value"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(ODR15W::SET)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ODR14`"]
pub type ODR14W = ODR15W;
#[doc = r" Proxy"]
pub struct _ODR14W<'a> {
    w: &'a mut W,
}
impl<'a> _ODR14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ODR14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset output value"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(ODR15W::RESET)
    }
    #[doc = "Set output value"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(ODR15W::SET)
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
#[doc = "Values that can be written to the field `ODR13`"]
pub type ODR13W = ODR15W;
#[doc = r" Proxy"]
pub struct _ODR13W<'a> {
    w: &'a mut W,
}
impl<'a> _ODR13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ODR13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset output value"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(ODR15W::RESET)
    }
    #[doc = "Set output value"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(ODR15W::SET)
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
#[doc = "Values that can be written to the field `ODR12`"]
pub type ODR12W = ODR15W;
#[doc = r" Proxy"]
pub struct _ODR12W<'a> {
    w: &'a mut W,
}
impl<'a> _ODR12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ODR12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset output value"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(ODR15W::RESET)
    }
    #[doc = "Set output value"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(ODR15W::SET)
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
#[doc = "Values that can be written to the field `ODR11`"]
pub type ODR11W = ODR15W;
#[doc = r" Proxy"]
pub struct _ODR11W<'a> {
    w: &'a mut W,
}
impl<'a> _ODR11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ODR11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset output value"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(ODR15W::RESET)
    }
    #[doc = "Set output value"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(ODR15W::SET)
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
#[doc = "Values that can be written to the field `ODR10`"]
pub type ODR10W = ODR15W;
#[doc = r" Proxy"]
pub struct _ODR10W<'a> {
    w: &'a mut W,
}
impl<'a> _ODR10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ODR10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset output value"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(ODR15W::RESET)
    }
    #[doc = "Set output value"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(ODR15W::SET)
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
#[doc = "Values that can be written to the field `ODR9`"]
pub type ODR9W = ODR15W;
#[doc = r" Proxy"]
pub struct _ODR9W<'a> {
    w: &'a mut W,
}
impl<'a> _ODR9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ODR9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset output value"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(ODR15W::RESET)
    }
    #[doc = "Set output value"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(ODR15W::SET)
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
#[doc = "Values that can be written to the field `ODR8`"]
pub type ODR8W = ODR15W;
#[doc = r" Proxy"]
pub struct _ODR8W<'a> {
    w: &'a mut W,
}
impl<'a> _ODR8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ODR8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset output value"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(ODR15W::RESET)
    }
    #[doc = "Set output value"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(ODR15W::SET)
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
#[doc = "Values that can be written to the field `ODR7`"]
pub type ODR7W = ODR15W;
#[doc = r" Proxy"]
pub struct _ODR7W<'a> {
    w: &'a mut W,
}
impl<'a> _ODR7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ODR7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset output value"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(ODR15W::RESET)
    }
    #[doc = "Set output value"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(ODR15W::SET)
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
#[doc = "Values that can be written to the field `ODR6`"]
pub type ODR6W = ODR15W;
#[doc = r" Proxy"]
pub struct _ODR6W<'a> {
    w: &'a mut W,
}
impl<'a> _ODR6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ODR6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset output value"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(ODR15W::RESET)
    }
    #[doc = "Set output value"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(ODR15W::SET)
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
#[doc = "Values that can be written to the field `ODR5`"]
pub type ODR5W = ODR15W;
#[doc = r" Proxy"]
pub struct _ODR5W<'a> {
    w: &'a mut W,
}
impl<'a> _ODR5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ODR5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset output value"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(ODR15W::RESET)
    }
    #[doc = "Set output value"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(ODR15W::SET)
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
#[doc = "Values that can be written to the field `ODR4`"]
pub type ODR4W = ODR15W;
#[doc = r" Proxy"]
pub struct _ODR4W<'a> {
    w: &'a mut W,
}
impl<'a> _ODR4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ODR4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset output value"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(ODR15W::RESET)
    }
    #[doc = "Set output value"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(ODR15W::SET)
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
#[doc = "Values that can be written to the field `ODR3`"]
pub type ODR3W = ODR15W;
#[doc = r" Proxy"]
pub struct _ODR3W<'a> {
    w: &'a mut W,
}
impl<'a> _ODR3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ODR3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset output value"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(ODR15W::RESET)
    }
    #[doc = "Set output value"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(ODR15W::SET)
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
#[doc = "Values that can be written to the field `ODR2`"]
pub type ODR2W = ODR15W;
#[doc = r" Proxy"]
pub struct _ODR2W<'a> {
    w: &'a mut W,
}
impl<'a> _ODR2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ODR2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset output value"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(ODR15W::RESET)
    }
    #[doc = "Set output value"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(ODR15W::SET)
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
#[doc = "Values that can be written to the field `ODR1`"]
pub type ODR1W = ODR15W;
#[doc = r" Proxy"]
pub struct _ODR1W<'a> {
    w: &'a mut W,
}
impl<'a> _ODR1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ODR1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset output value"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(ODR15W::RESET)
    }
    #[doc = "Set output value"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(ODR15W::SET)
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
#[doc = "Values that can be written to the field `ODR0`"]
pub type ODR0W = ODR15W;
#[doc = r" Proxy"]
pub struct _ODR0W<'a> {
    w: &'a mut W,
}
impl<'a> _ODR0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ODR0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset output value"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(ODR15W::RESET)
    }
    #[doc = "Set output value"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(ODR15W::SET)
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
    #[doc = "Bit 15 - Port output data (y = 0..15)"]
    #[inline]
    pub fn odr15(&self) -> ODR15R {
        ODR15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Port output data (y = 0..15)"]
    #[inline]
    pub fn odr14(&self) -> ODR14R {
        ODR14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Port output data (y = 0..15)"]
    #[inline]
    pub fn odr13(&self) -> ODR13R {
        ODR13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Port output data (y = 0..15)"]
    #[inline]
    pub fn odr12(&self) -> ODR12R {
        ODR12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Port output data (y = 0..15)"]
    #[inline]
    pub fn odr11(&self) -> ODR11R {
        ODR11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Port output data (y = 0..15)"]
    #[inline]
    pub fn odr10(&self) -> ODR10R {
        ODR10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Port output data (y = 0..15)"]
    #[inline]
    pub fn odr9(&self) -> ODR9R {
        ODR9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Port output data (y = 0..15)"]
    #[inline]
    pub fn odr8(&self) -> ODR8R {
        ODR8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Port output data (y = 0..15)"]
    #[inline]
    pub fn odr7(&self) -> ODR7R {
        ODR7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Port output data (y = 0..15)"]
    #[inline]
    pub fn odr6(&self) -> ODR6R {
        ODR6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Port output data (y = 0..15)"]
    #[inline]
    pub fn odr5(&self) -> ODR5R {
        ODR5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Port output data (y = 0..15)"]
    #[inline]
    pub fn odr4(&self) -> ODR4R {
        ODR4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Port output data (y = 0..15)"]
    #[inline]
    pub fn odr3(&self) -> ODR3R {
        ODR3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Port output data (y = 0..15)"]
    #[inline]
    pub fn odr2(&self) -> ODR2R {
        ODR2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Port output data (y = 0..15)"]
    #[inline]
    pub fn odr1(&self) -> ODR1R {
        ODR1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Port output data (y = 0..15)"]
    #[inline]
    pub fn odr0(&self) -> ODR0R {
        ODR0R::_from({
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
    #[doc = "Bit 15 - Port output data (y = 0..15)"]
    #[inline]
    pub fn odr15(&mut self) -> _ODR15W {
        _ODR15W { w: self }
    }
    #[doc = "Bit 14 - Port output data (y = 0..15)"]
    #[inline]
    pub fn odr14(&mut self) -> _ODR14W {
        _ODR14W { w: self }
    }
    #[doc = "Bit 13 - Port output data (y = 0..15)"]
    #[inline]
    pub fn odr13(&mut self) -> _ODR13W {
        _ODR13W { w: self }
    }
    #[doc = "Bit 12 - Port output data (y = 0..15)"]
    #[inline]
    pub fn odr12(&mut self) -> _ODR12W {
        _ODR12W { w: self }
    }
    #[doc = "Bit 11 - Port output data (y = 0..15)"]
    #[inline]
    pub fn odr11(&mut self) -> _ODR11W {
        _ODR11W { w: self }
    }
    #[doc = "Bit 10 - Port output data (y = 0..15)"]
    #[inline]
    pub fn odr10(&mut self) -> _ODR10W {
        _ODR10W { w: self }
    }
    #[doc = "Bit 9 - Port output data (y = 0..15)"]
    #[inline]
    pub fn odr9(&mut self) -> _ODR9W {
        _ODR9W { w: self }
    }
    #[doc = "Bit 8 - Port output data (y = 0..15)"]
    #[inline]
    pub fn odr8(&mut self) -> _ODR8W {
        _ODR8W { w: self }
    }
    #[doc = "Bit 7 - Port output data (y = 0..15)"]
    #[inline]
    pub fn odr7(&mut self) -> _ODR7W {
        _ODR7W { w: self }
    }
    #[doc = "Bit 6 - Port output data (y = 0..15)"]
    #[inline]
    pub fn odr6(&mut self) -> _ODR6W {
        _ODR6W { w: self }
    }
    #[doc = "Bit 5 - Port output data (y = 0..15)"]
    #[inline]
    pub fn odr5(&mut self) -> _ODR5W {
        _ODR5W { w: self }
    }
    #[doc = "Bit 4 - Port output data (y = 0..15)"]
    #[inline]
    pub fn odr4(&mut self) -> _ODR4W {
        _ODR4W { w: self }
    }
    #[doc = "Bit 3 - Port output data (y = 0..15)"]
    #[inline]
    pub fn odr3(&mut self) -> _ODR3W {
        _ODR3W { w: self }
    }
    #[doc = "Bit 2 - Port output data (y = 0..15)"]
    #[inline]
    pub fn odr2(&mut self) -> _ODR2W {
        _ODR2W { w: self }
    }
    #[doc = "Bit 1 - Port output data (y = 0..15)"]
    #[inline]
    pub fn odr1(&mut self) -> _ODR1W {
        _ODR1W { w: self }
    }
    #[doc = "Bit 0 - Port output data (y = 0..15)"]
    #[inline]
    pub fn odr0(&mut self) -> _ODR0W {
        _ODR0W { w: self }
    }
}
