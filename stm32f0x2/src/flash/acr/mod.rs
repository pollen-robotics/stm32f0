#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ACR {
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
#[doc = "Possible values of the field `LATENCY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LATENCYR {
    #[doc = "Zero wait state, if SYSCLK is below or equal to 24 MHz."]
    _0WAITSTATE,
    #[doc = "One wait state, if SYSCLK is greater 24MHz and below or equal to 48 MHz."]
    _1WAITSTATE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LATENCYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LATENCYR::_0WAITSTATE => 0,
            LATENCYR::_1WAITSTATE => 1,
            LATENCYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LATENCYR {
        match value {
            0 => LATENCYR::_0WAITSTATE,
            1 => LATENCYR::_1WAITSTATE,
            i => LATENCYR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0WAITSTATE`"]
    #[inline]
    pub fn is_0wait_state(&self) -> bool {
        *self == LATENCYR::_0WAITSTATE
    }
    #[doc = "Checks if the value of the field is `_1WAITSTATE`"]
    #[inline]
    pub fn is_1wait_state(&self) -> bool {
        *self == LATENCYR::_1WAITSTATE
    }
}
#[doc = "Possible values of the field `PRFTBE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRFTBER {
    #[doc = "Prefetch is disabled."]
    DISABLED,
    #[doc = "Prefetch is enabled."]
    ENABLED,
}
impl PRFTBER {
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
            PRFTBER::DISABLED => false,
            PRFTBER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRFTBER {
        match value {
            false => PRFTBER::DISABLED,
            true => PRFTBER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PRFTBER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PRFTBER::ENABLED
    }
}
#[doc = r" Value of the field"]
pub struct PRFTBSR {
    bits: bool,
}
impl PRFTBSR {
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
#[doc = "Values that can be written to the field `LATENCY`"]
pub enum LATENCYW {
    #[doc = "Zero wait state, if SYSCLK is below or equal to 24 MHz."]
    _0WAITSTATE,
    #[doc = "One wait state, if SYSCLK is greater 24MHz and below or equal to 48 MHz."]
    _1WAITSTATE,
}
impl LATENCYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LATENCYW::_0WAITSTATE => 0,
            LATENCYW::_1WAITSTATE => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LATENCYW<'a> {
    w: &'a mut W,
}
impl<'a> _LATENCYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LATENCYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Zero wait state, if SYSCLK is below or equal to 24 MHz."]
    #[inline]
    pub fn _0wait_state(self) -> &'a mut W {
        self.variant(LATENCYW::_0WAITSTATE)
    }
    #[doc = "One wait state, if SYSCLK is greater 24MHz and below or equal to 48 MHz."]
    #[inline]
    pub fn _1wait_state(self) -> &'a mut W {
        self.variant(LATENCYW::_1WAITSTATE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRFTBE`"]
pub enum PRFTBEW {
    #[doc = "Prefetch is disabled."]
    DISABLED,
    #[doc = "Prefetch is enabled."]
    ENABLED,
}
impl PRFTBEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PRFTBEW::DISABLED => false,
            PRFTBEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRFTBEW<'a> {
    w: &'a mut W,
}
impl<'a> _PRFTBEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRFTBEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Prefetch is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PRFTBEW::DISABLED)
    }
    #[doc = "Prefetch is enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PRFTBEW::ENABLED)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - LATENCY"]
    #[inline]
    pub fn latency(&self) -> LATENCYR {
        LATENCYR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - PRFTBE"]
    #[inline]
    pub fn prftbe(&self) -> PRFTBER {
        PRFTBER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - PRFTBS"]
    #[inline]
    pub fn prftbs(&self) -> PRFTBSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PRFTBSR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 48 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - LATENCY"]
    #[inline]
    pub fn latency(&mut self) -> _LATENCYW {
        _LATENCYW { w: self }
    }
    #[doc = "Bit 4 - PRFTBE"]
    #[inline]
    pub fn prftbe(&mut self) -> _PRFTBEW {
        _PRFTBEW { w: self }
    }
}
