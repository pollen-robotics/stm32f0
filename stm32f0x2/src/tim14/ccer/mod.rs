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
#[doc = "Possible values of the field `CC1NP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1NPR {
    #[doc = "active high"] HIGH,
    #[doc = "active low"] LOW,
}
impl CC1NPR {
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
            CC1NPR::HIGH => false,
            CC1NPR::LOW => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CC1NPR {
        match value {
            false => CC1NPR::HIGH,
            true => CC1NPR::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == CC1NPR::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == CC1NPR::LOW
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
            CC1PR::HIGH => false,
            CC1PR::LOW => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CC1PR {
        match value {
            false => CC1PR::HIGH,
            true => CC1PR::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == CC1PR::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == CC1PR::LOW
    }
}
#[doc = "Possible values of the field `CC1E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1ER {# [ doc = "Output : OC1 is not active - Input : Capture disabled" ] INACTIVE , # [ doc = "Output : OC1 signal is output on the corresponding output pin - Input : Capture enabled" ] ACTIVE ,}
impl CC1ER {
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
            CC1ER::INACTIVE => false,
            CC1ER::ACTIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CC1ER {
        match value {
            false => CC1ER::INACTIVE,
            true => CC1ER::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline]
    pub fn is_inactive(&self) -> bool {
        *self == CC1ER::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline]
    pub fn is_active(&self) -> bool {
        *self == CC1ER::ACTIVE
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
    #[inline]
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
    #[inline]
    pub fn variant(self, variant: CC1NPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "active high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(CC1NPW::HIGH)
    }
    #[doc = "active low"]
    #[inline]
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
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
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
    #[inline]
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
    #[inline]
    pub fn variant(self, variant: CC1PW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "active high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(CC1PW::HIGH)
    }
    #[doc = "active low"]
    #[inline]
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
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CC1E`"]
pub enum CC1EW {# [ doc = "Output : OC1 is not active - Input : Capture disabled" ] INACTIVE , # [ doc = "Output : OC1 signal is output on the corresponding output pin - Input : Capture enabled" ] ACTIVE ,}
impl CC1EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
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
    #[inline]
    pub fn variant(self, variant: CC1EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Output : OC1 is not active - Input : Capture disabled"]
    #[inline]
    pub fn inactive(self) -> &'a mut W {
        self.variant(CC1EW::INACTIVE)
    }
    # [ doc = "Output : OC1 signal is output on the corresponding output pin - Input : Capture enabled" ] # [ inline ]
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
    #[doc = "Bit 3 - Capture/Compare 1 output Polarity"]
    #[inline]
    pub fn cc1np(&self) -> CC1NPR {
        CC1NPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Capture/Compare 1 output Polarity"]
    #[inline]
    pub fn cc1p(&self) -> CC1PR {
        CC1PR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Capture/Compare 1 output enable"]
    #[inline]
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
    #[doc = "Bit 3 - Capture/Compare 1 output Polarity"]
    #[inline]
    pub fn cc1np(&mut self) -> _CC1NPW {
        _CC1NPW { w: self }
    }
    #[doc = "Bit 1 - Capture/Compare 1 output Polarity"]
    #[inline]
    pub fn cc1p(&mut self) -> _CC1PW {
        _CC1PW { w: self }
    }
    #[doc = "Bit 0 - Capture/Compare 1 output enable"]
    #[inline]
    pub fn cc1e(&mut self) -> _CC1EW {
        _CC1EW { w: self }
    }
}
