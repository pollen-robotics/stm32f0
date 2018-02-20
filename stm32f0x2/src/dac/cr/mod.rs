#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR {
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
#[doc = "Possible values of the field `EN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN1R {
    #[doc = "DAC channel1 disabled"]
    DISABLED,
    #[doc = "DAC channel1 enabled"]
    ENABLED,
}
impl EN1R {
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
            EN1R::DISABLED => false,
            EN1R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN1R {
        match value {
            false => EN1R::DISABLED,
            true => EN1R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == EN1R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == EN1R::ENABLED
    }
}
#[doc = "Possible values of the field `BOFF1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOFF1R {
    #[doc = "DAC channel1 output buffer enabled"]
    DISABLED,
    #[doc = "DAC channel1 output buffer disabled"]
    ENABLED,
}
impl BOFF1R {
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
            BOFF1R::DISABLED => false,
            BOFF1R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BOFF1R {
        match value {
            false => BOFF1R::DISABLED,
            true => BOFF1R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == BOFF1R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == BOFF1R::ENABLED
    }
}
#[doc = "Possible values of the field `TEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEN1R {# [ doc = "DAC channel1 trigger disabled and data written into the DAC_DHRx register are\ntransferred one APB clock cycle later to the DAC_DOR1 register" ] DISABLED , # [ doc = "DAC channel1 trigger enabled and data from the DAC_DHRx register are transferred three APB clock cycles later to the DAC_DOR1 register" ] ENABLED ,}
impl TEN1R {
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
            TEN1R::DISABLED => false,
            TEN1R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TEN1R {
        match value {
            false => TEN1R::DISABLED,
            true => TEN1R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TEN1R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TEN1R::ENABLED
    }
}
#[doc = "Possible values of the field `TSEL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSEL1R {
    #[doc = "Timer 6 TRGO event"]
    TIMER6TRIG,
    #[doc = "Timer 3 TRGO event"]
    TIMER3TRIG,
    #[doc = "Timer 7 TRGO event"]
    TIMER7TRIG,
    #[doc = "Timer 15 TRGO event"]
    TIMER15TRIG,
    #[doc = "Timer 2 TRGO event"]
    TIMER2TRIG,
    #[doc = "EXTI line9"]
    EXTI9_TRIG,
    #[doc = "Software trigger"]
    SOFT,
}
impl TSEL1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TSEL1R::TIMER6TRIG => 0,
            TSEL1R::TIMER3TRIG => 1,
            TSEL1R::TIMER7TRIG => 2,
            TSEL1R::TIMER15TRIG => 3,
            TSEL1R::TIMER2TRIG => 4,
            TSEL1R::EXTI9_TRIG => 6,
            TSEL1R::SOFT => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TSEL1R {
        match value {
            0 => TSEL1R::TIMER6TRIG,
            1 => TSEL1R::TIMER3TRIG,
            2 => TSEL1R::TIMER7TRIG,
            3 => TSEL1R::TIMER15TRIG,
            4 => TSEL1R::TIMER2TRIG,
            6 => TSEL1R::EXTI9_TRIG,
            7 => TSEL1R::SOFT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER6TRIG`"]
    #[inline]
    pub fn is_timer6trig(&self) -> bool {
        *self == TSEL1R::TIMER6TRIG
    }
    #[doc = "Checks if the value of the field is `TIMER3TRIG`"]
    #[inline]
    pub fn is_timer3trig(&self) -> bool {
        *self == TSEL1R::TIMER3TRIG
    }
    #[doc = "Checks if the value of the field is `TIMER7TRIG`"]
    #[inline]
    pub fn is_timer7trig(&self) -> bool {
        *self == TSEL1R::TIMER7TRIG
    }
    #[doc = "Checks if the value of the field is `TIMER15TRIG`"]
    #[inline]
    pub fn is_timer15trig(&self) -> bool {
        *self == TSEL1R::TIMER15TRIG
    }
    #[doc = "Checks if the value of the field is `TIMER2TRIG`"]
    #[inline]
    pub fn is_timer2trig(&self) -> bool {
        *self == TSEL1R::TIMER2TRIG
    }
    #[doc = "Checks if the value of the field is `EXTI9_TRIG`"]
    #[inline]
    pub fn is_exti9_trig(&self) -> bool {
        *self == TSEL1R::EXTI9_TRIG
    }
    #[doc = "Checks if the value of the field is `SOFT`"]
    #[inline]
    pub fn is_soft(&self) -> bool {
        *self == TSEL1R::SOFT
    }
}
#[doc = r" Value of the field"]
pub struct WAVE1R {
    bits: u8,
}
impl WAVE1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MAMP1R {
    bits: u8,
}
impl MAMP1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DMAEN1R {
    bits: bool,
}
impl DMAEN1R {
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
pub struct DMAUDRIE1R {
    bits: bool,
}
impl DMAUDRIE1R {
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
pub struct EN2R {
    bits: bool,
}
impl EN2R {
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
pub struct BOFF2R {
    bits: bool,
}
impl BOFF2R {
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
pub struct TEN2R {
    bits: bool,
}
impl TEN2R {
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
pub struct TSEL2R {
    bits: u8,
}
impl TSEL2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WAVE2R {
    bits: u8,
}
impl WAVE2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MAMP2R {
    bits: u8,
}
impl MAMP2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DMAEN2R {
    bits: bool,
}
impl DMAEN2R {
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
pub struct DMAUDRIE2R {
    bits: bool,
}
impl DMAUDRIE2R {
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
#[doc = "Values that can be written to the field `EN1`"]
pub enum EN1W {
    #[doc = "DAC channel1 disabled"]
    DISABLED,
    #[doc = "DAC channel1 enabled"]
    ENABLED,
}
impl EN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN1W::DISABLED => false,
            EN1W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN1W<'a> {
    w: &'a mut W,
}
impl<'a> _EN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DAC channel1 disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN1W::DISABLED)
    }
    #[doc = "DAC channel1 enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN1W::ENABLED)
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
#[doc = "Values that can be written to the field `BOFF1`"]
pub enum BOFF1W {
    #[doc = "DAC channel1 output buffer enabled"]
    DISABLED,
    #[doc = "DAC channel1 output buffer disabled"]
    ENABLED,
}
impl BOFF1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BOFF1W::DISABLED => false,
            BOFF1W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BOFF1W<'a> {
    w: &'a mut W,
}
impl<'a> _BOFF1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BOFF1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DAC channel1 output buffer enabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BOFF1W::DISABLED)
    }
    #[doc = "DAC channel1 output buffer disabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BOFF1W::ENABLED)
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
#[doc = "Values that can be written to the field `TEN1`"]
pub enum TEN1W {# [ doc = "DAC channel1 trigger disabled and data written into the DAC_DHRx register are\ntransferred one APB clock cycle later to the DAC_DOR1 register" ] DISABLED , # [ doc = "DAC channel1 trigger enabled and data from the DAC_DHRx register are transferred three APB clock cycles later to the DAC_DOR1 register" ] ENABLED ,}
impl TEN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TEN1W::DISABLED => false,
            TEN1W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _TEN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TEN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    # [ doc = "DAC channel1 trigger disabled and data written into the DAC_DHRx register are transferred one APB clock cycle later to the DAC_DOR1 register" ] # [ inline ]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEN1W::DISABLED)
    }
    # [ doc = "DAC channel1 trigger enabled and data from the DAC_DHRx register are transferred three APB clock cycles later to the DAC_DOR1 register" ] # [ inline ]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEN1W::ENABLED)
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
#[doc = "Values that can be written to the field `TSEL1`"]
pub enum TSEL1W {
    #[doc = "Timer 6 TRGO event"]
    TIMER6TRIG,
    #[doc = "Timer 3 TRGO event"]
    TIMER3TRIG,
    #[doc = "Timer 7 TRGO event"]
    TIMER7TRIG,
    #[doc = "Timer 15 TRGO event"]
    TIMER15TRIG,
    #[doc = "Timer 2 TRGO event"]
    TIMER2TRIG,
    #[doc = "EXTI line9"]
    EXTI9_TRIG,
    #[doc = "Software trigger"]
    SOFT,
}
impl TSEL1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TSEL1W::TIMER6TRIG => 0,
            TSEL1W::TIMER3TRIG => 1,
            TSEL1W::TIMER7TRIG => 2,
            TSEL1W::TIMER15TRIG => 3,
            TSEL1W::TIMER2TRIG => 4,
            TSEL1W::EXTI9_TRIG => 6,
            TSEL1W::SOFT => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSEL1W<'a> {
    w: &'a mut W,
}
impl<'a> _TSEL1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSEL1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Timer 6 TRGO event"]
    #[inline]
    pub fn timer6trig(self) -> &'a mut W {
        self.variant(TSEL1W::TIMER6TRIG)
    }
    #[doc = "Timer 3 TRGO event"]
    #[inline]
    pub fn timer3trig(self) -> &'a mut W {
        self.variant(TSEL1W::TIMER3TRIG)
    }
    #[doc = "Timer 7 TRGO event"]
    #[inline]
    pub fn timer7trig(self) -> &'a mut W {
        self.variant(TSEL1W::TIMER7TRIG)
    }
    #[doc = "Timer 15 TRGO event"]
    #[inline]
    pub fn timer15trig(self) -> &'a mut W {
        self.variant(TSEL1W::TIMER15TRIG)
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline]
    pub fn timer2trig(self) -> &'a mut W {
        self.variant(TSEL1W::TIMER2TRIG)
    }
    #[doc = "EXTI line9"]
    #[inline]
    pub fn exti9_trig(self) -> &'a mut W {
        self.variant(TSEL1W::EXTI9_TRIG)
    }
    #[doc = "Software trigger"]
    #[inline]
    pub fn soft(self) -> &'a mut W {
        self.variant(TSEL1W::SOFT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WAVE1W<'a> {
    w: &'a mut W,
}
impl<'a> _WAVE1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MAMP1W<'a> {
    w: &'a mut W,
}
impl<'a> _MAMP1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMAEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _DMAEN1W<'a> {
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
#[doc = r" Proxy"]
pub struct _DMAUDRIE1W<'a> {
    w: &'a mut W,
}
impl<'a> _DMAUDRIE1W<'a> {
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
#[doc = r" Proxy"]
pub struct _EN2W<'a> {
    w: &'a mut W,
}
impl<'a> _EN2W<'a> {
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
#[doc = r" Proxy"]
pub struct _BOFF2W<'a> {
    w: &'a mut W,
}
impl<'a> _BOFF2W<'a> {
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
#[doc = r" Proxy"]
pub struct _TEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _TEN2W<'a> {
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
#[doc = r" Proxy"]
pub struct _TSEL2W<'a> {
    w: &'a mut W,
}
impl<'a> _TSEL2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WAVE2W<'a> {
    w: &'a mut W,
}
impl<'a> _WAVE2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MAMP2W<'a> {
    w: &'a mut W,
}
impl<'a> _MAMP2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMAEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _DMAEN2W<'a> {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMAUDRIE2W<'a> {
    w: &'a mut W,
}
impl<'a> _DMAUDRIE2W<'a> {
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
        const OFFSET: u8 = 29;
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
    #[doc = "Bit 0 - DAC channel1 enable"]
    #[inline]
    pub fn en1(&self) -> EN1R {
        EN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - DAC channel1 output buffer disable"]
    #[inline]
    pub fn boff1(&self) -> BOFF1R {
        BOFF1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - DAC channel1 trigger enable"]
    #[inline]
    pub fn ten1(&self) -> TEN1R {
        TEN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 3:5 - DAC channel1 trigger selection"]
    #[inline]
    pub fn tsel1(&self) -> TSEL1R {
        TSEL1R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - DAC channel1 noise/triangle wave generation enable"]
    #[inline]
    pub fn wave1(&self) -> WAVE1R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WAVE1R { bits }
    }
    #[doc = "Bits 8:11 - DAC channel1 mask/amplitude selector"]
    #[inline]
    pub fn mamp1(&self) -> MAMP1R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAMP1R { bits }
    }
    #[doc = "Bit 12 - DAC channel1 DMA enable"]
    #[inline]
    pub fn dmaen1(&self) -> DMAEN1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMAEN1R { bits }
    }
    #[doc = "Bit 13 - DAC channel1 DMA Underrun Interrupt enable"]
    #[inline]
    pub fn dmaudrie1(&self) -> DMAUDRIE1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMAUDRIE1R { bits }
    }
    #[doc = "Bit 16 - DAC channel2 enable"]
    #[inline]
    pub fn en2(&self) -> EN2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EN2R { bits }
    }
    #[doc = "Bit 17 - DAC channel2 output buffer disable"]
    #[inline]
    pub fn boff2(&self) -> BOFF2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BOFF2R { bits }
    }
    #[doc = "Bit 18 - DAC channel2 trigger enable"]
    #[inline]
    pub fn ten2(&self) -> TEN2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TEN2R { bits }
    }
    #[doc = "Bits 19:21 - DAC channel2 trigger selection"]
    #[inline]
    pub fn tsel2(&self) -> TSEL2R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TSEL2R { bits }
    }
    #[doc = "Bits 22:23 - DAC channel2 noise/triangle wave generation enable"]
    #[inline]
    pub fn wave2(&self) -> WAVE2R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WAVE2R { bits }
    }
    #[doc = "Bits 24:27 - DAC channel2 mask/amplitude selector"]
    #[inline]
    pub fn mamp2(&self) -> MAMP2R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAMP2R { bits }
    }
    #[doc = "Bit 28 - DAC channel2 DMA enable"]
    #[inline]
    pub fn dmaen2(&self) -> DMAEN2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMAEN2R { bits }
    }
    #[doc = "Bit 29 - DAC channel2 DMA underrun interrupt enable"]
    #[inline]
    pub fn dmaudrie2(&self) -> DMAUDRIE2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMAUDRIE2R { bits }
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
    #[doc = "Bit 0 - DAC channel1 enable"]
    #[inline]
    pub fn en1(&mut self) -> _EN1W {
        _EN1W { w: self }
    }
    #[doc = "Bit 1 - DAC channel1 output buffer disable"]
    #[inline]
    pub fn boff1(&mut self) -> _BOFF1W {
        _BOFF1W { w: self }
    }
    #[doc = "Bit 2 - DAC channel1 trigger enable"]
    #[inline]
    pub fn ten1(&mut self) -> _TEN1W {
        _TEN1W { w: self }
    }
    #[doc = "Bits 3:5 - DAC channel1 trigger selection"]
    #[inline]
    pub fn tsel1(&mut self) -> _TSEL1W {
        _TSEL1W { w: self }
    }
    #[doc = "Bits 6:7 - DAC channel1 noise/triangle wave generation enable"]
    #[inline]
    pub fn wave1(&mut self) -> _WAVE1W {
        _WAVE1W { w: self }
    }
    #[doc = "Bits 8:11 - DAC channel1 mask/amplitude selector"]
    #[inline]
    pub fn mamp1(&mut self) -> _MAMP1W {
        _MAMP1W { w: self }
    }
    #[doc = "Bit 12 - DAC channel1 DMA enable"]
    #[inline]
    pub fn dmaen1(&mut self) -> _DMAEN1W {
        _DMAEN1W { w: self }
    }
    #[doc = "Bit 13 - DAC channel1 DMA Underrun Interrupt enable"]
    #[inline]
    pub fn dmaudrie1(&mut self) -> _DMAUDRIE1W {
        _DMAUDRIE1W { w: self }
    }
    #[doc = "Bit 16 - DAC channel2 enable"]
    #[inline]
    pub fn en2(&mut self) -> _EN2W {
        _EN2W { w: self }
    }
    #[doc = "Bit 17 - DAC channel2 output buffer disable"]
    #[inline]
    pub fn boff2(&mut self) -> _BOFF2W {
        _BOFF2W { w: self }
    }
    #[doc = "Bit 18 - DAC channel2 trigger enable"]
    #[inline]
    pub fn ten2(&mut self) -> _TEN2W {
        _TEN2W { w: self }
    }
    #[doc = "Bits 19:21 - DAC channel2 trigger selection"]
    #[inline]
    pub fn tsel2(&mut self) -> _TSEL2W {
        _TSEL2W { w: self }
    }
    #[doc = "Bits 22:23 - DAC channel2 noise/triangle wave generation enable"]
    #[inline]
    pub fn wave2(&mut self) -> _WAVE2W {
        _WAVE2W { w: self }
    }
    #[doc = "Bits 24:27 - DAC channel2 mask/amplitude selector"]
    #[inline]
    pub fn mamp2(&mut self) -> _MAMP2W {
        _MAMP2W { w: self }
    }
    #[doc = "Bit 28 - DAC channel2 DMA enable"]
    #[inline]
    pub fn dmaen2(&mut self) -> _DMAEN2W {
        _DMAEN2W { w: self }
    }
    #[doc = "Bit 29 - DAC channel2 DMA underrun interrupt enable"]
    #[inline]
    pub fn dmaudrie2(&mut self) -> _DMAUDRIE2W {
        _DMAUDRIE2W { w: self }
    }
}
