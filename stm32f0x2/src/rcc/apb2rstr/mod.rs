#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::APB2RSTR {
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
#[doc = "Possible values of the field `SYSCFGRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCFGRSTR {
    #[doc = "Reset SYSCFG."] RESET,
    #[doc = r" Reserved"] _Reserved(bool),
}
impl SYSCFGRSTR {
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
            SYSCFGRSTR::RESET => true,
            SYSCFGRSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> SYSCFGRSTR {
        match value {
            true => SYSCFGRSTR::RESET,
            i => SYSCFGRSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SYSCFGRSTR::RESET
    }
}
#[doc = "Possible values of the field `ADCRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCRSTR {
    #[doc = "Reset ADC."] RESET,
    #[doc = r" Reserved"] _Reserved(bool),
}
impl ADCRSTR {
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
            ADCRSTR::RESET => true,
            ADCRSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> ADCRSTR {
        match value {
            true => ADCRSTR::RESET,
            i => ADCRSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == ADCRSTR::RESET
    }
}
#[doc = "Possible values of the field `TIM1RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM1RSTR {
    #[doc = "Reset TIM1 timer."] RESET,
    #[doc = r" Reserved"] _Reserved(bool),
}
impl TIM1RSTR {
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
            TIM1RSTR::RESET => true,
            TIM1RSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> TIM1RSTR {
        match value {
            true => TIM1RSTR::RESET,
            i => TIM1RSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TIM1RSTR::RESET
    }
}
#[doc = "Possible values of the field `SPI1RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI1RSTR {
    #[doc = "Reset SPI1."] RESET,
    #[doc = r" Reserved"] _Reserved(bool),
}
impl SPI1RSTR {
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
            SPI1RSTR::RESET => true,
            SPI1RSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> SPI1RSTR {
        match value {
            true => SPI1RSTR::RESET,
            i => SPI1RSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SPI1RSTR::RESET
    }
}
#[doc = "Possible values of the field `USART1RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART1RSTR {
    #[doc = "Reset USART1."] RESET,
    #[doc = r" Reserved"] _Reserved(bool),
}
impl USART1RSTR {
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
            USART1RSTR::RESET => true,
            USART1RSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> USART1RSTR {
        match value {
            true => USART1RSTR::RESET,
            i => USART1RSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == USART1RSTR::RESET
    }
}
#[doc = "Possible values of the field `TIM15RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM15RSTR {
    #[doc = "Reset TIM15 timer."] RESET,
    #[doc = r" Reserved"] _Reserved(bool),
}
impl TIM15RSTR {
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
            TIM15RSTR::RESET => true,
            TIM15RSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> TIM15RSTR {
        match value {
            true => TIM15RSTR::RESET,
            i => TIM15RSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TIM15RSTR::RESET
    }
}
#[doc = "Possible values of the field `TIM16RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM16RSTR {
    #[doc = "Reset TIM16 timer."] RESET,
    #[doc = r" Reserved"] _Reserved(bool),
}
impl TIM16RSTR {
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
            TIM16RSTR::RESET => true,
            TIM16RSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> TIM16RSTR {
        match value {
            true => TIM16RSTR::RESET,
            i => TIM16RSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TIM16RSTR::RESET
    }
}
#[doc = "Possible values of the field `TIM17RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM17RSTR {
    #[doc = "Reset TIM17 timer."] RESET,
    #[doc = r" Reserved"] _Reserved(bool),
}
impl TIM17RSTR {
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
            TIM17RSTR::RESET => true,
            TIM17RSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> TIM17RSTR {
        match value {
            true => TIM17RSTR::RESET,
            i => TIM17RSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TIM17RSTR::RESET
    }
}
#[doc = "Possible values of the field `DBGMCURST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGMCURSTR {
    #[doc = "Reset Debug MCU."] RESET,
    #[doc = r" Reserved"] _Reserved(bool),
}
impl DBGMCURSTR {
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
            DBGMCURSTR::RESET => true,
            DBGMCURSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> DBGMCURSTR {
        match value {
            true => DBGMCURSTR::RESET,
            i => DBGMCURSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DBGMCURSTR::RESET
    }
}
#[doc = "Values that can be written to the field `SYSCFGRST`"]
pub enum SYSCFGRSTW {
    #[doc = "Reset SYSCFG."] RESET,
}
impl SYSCFGRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SYSCFGRSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYSCFGRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCFGRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCFGRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset SYSCFG."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SYSCFGRSTW::RESET)
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
#[doc = "Values that can be written to the field `ADCRST`"]
pub enum ADCRSTW {
    #[doc = "Reset ADC."] RESET,
}
impl ADCRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            ADCRSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset ADC."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(ADCRSTW::RESET)
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
#[doc = "Values that can be written to the field `TIM1RST`"]
pub enum TIM1RSTW {
    #[doc = "Reset TIM1 timer."] RESET,
}
impl TIM1RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            TIM1RSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIM1RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM1RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM1RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset TIM1 timer."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM1RSTW::RESET)
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
#[doc = "Values that can be written to the field `SPI1RST`"]
pub enum SPI1RSTW {
    #[doc = "Reset SPI1."] RESET,
}
impl SPI1RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SPI1RSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPI1RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI1RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI1RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset SPI1."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SPI1RSTW::RESET)
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
#[doc = "Values that can be written to the field `USART1RST`"]
pub enum USART1RSTW {
    #[doc = "Reset USART1."] RESET,
}
impl USART1RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            USART1RSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USART1RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _USART1RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART1RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset USART1."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(USART1RSTW::RESET)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIM15RST`"]
pub enum TIM15RSTW {
    #[doc = "Reset TIM15 timer."] RESET,
}
impl TIM15RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            TIM15RSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIM15RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM15RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM15RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset TIM15 timer."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM15RSTW::RESET)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIM16RST`"]
pub enum TIM16RSTW {
    #[doc = "Reset TIM16 timer."] RESET,
}
impl TIM16RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            TIM16RSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIM16RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM16RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM16RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset TIM16 timer."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM16RSTW::RESET)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIM17RST`"]
pub enum TIM17RSTW {
    #[doc = "Reset TIM17 timer."] RESET,
}
impl TIM17RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            TIM17RSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIM17RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM17RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM17RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset TIM17 timer."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM17RSTW::RESET)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DBGMCURST`"]
pub enum DBGMCURSTW {
    #[doc = "Reset Debug MCU."] RESET,
}
impl DBGMCURSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            DBGMCURSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DBGMCURSTW<'a> {
    w: &'a mut W,
}
impl<'a> _DBGMCURSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBGMCURSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset Debug MCU."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DBGMCURSTW::RESET)
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
        const OFFSET: u8 = 22;
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
    #[doc = "Bit 0 - SYSCFG and COMP reset"]
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRSTR {
        SYSCFGRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - ADC interface reset"]
    #[inline(always)]
    pub fn adcrst(&self) -> ADCRSTR {
        ADCRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - TIM1 timer reset"]
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RSTR {
        TIM1RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - SPI 1 reset"]
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RSTR {
        SPI1RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RSTR {
        USART1RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - TIM15 timer reset"]
    #[inline(always)]
    pub fn tim15rst(&self) -> TIM15RSTR {
        TIM15RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - TIM16 timer reset"]
    #[inline(always)]
    pub fn tim16rst(&self) -> TIM16RSTR {
        TIM16RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - TIM17 timer reset"]
    #[inline(always)]
    pub fn tim17rst(&self) -> TIM17RSTR {
        TIM17RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Debug MCU reset"]
    #[inline(always)]
    pub fn dbgmcurst(&self) -> DBGMCURSTR {
        DBGMCURSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
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
    #[doc = "Bit 0 - SYSCFG and COMP reset"]
    #[inline(always)]
    pub fn syscfgrst(&mut self) -> _SYSCFGRSTW {
        _SYSCFGRSTW { w: self }
    }
    #[doc = "Bit 9 - ADC interface reset"]
    #[inline(always)]
    pub fn adcrst(&mut self) -> _ADCRSTW {
        _ADCRSTW { w: self }
    }
    #[doc = "Bit 11 - TIM1 timer reset"]
    #[inline(always)]
    pub fn tim1rst(&mut self) -> _TIM1RSTW {
        _TIM1RSTW { w: self }
    }
    #[doc = "Bit 12 - SPI 1 reset"]
    #[inline(always)]
    pub fn spi1rst(&mut self) -> _SPI1RSTW {
        _SPI1RSTW { w: self }
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    pub fn usart1rst(&mut self) -> _USART1RSTW {
        _USART1RSTW { w: self }
    }
    #[doc = "Bit 16 - TIM15 timer reset"]
    #[inline(always)]
    pub fn tim15rst(&mut self) -> _TIM15RSTW {
        _TIM15RSTW { w: self }
    }
    #[doc = "Bit 17 - TIM16 timer reset"]
    #[inline(always)]
    pub fn tim16rst(&mut self) -> _TIM16RSTW {
        _TIM16RSTW { w: self }
    }
    #[doc = "Bit 18 - TIM17 timer reset"]
    #[inline(always)]
    pub fn tim17rst(&mut self) -> _TIM17RSTW {
        _TIM17RSTW { w: self }
    }
    #[doc = "Bit 22 - Debug MCU reset"]
    #[inline(always)]
    pub fn dbgmcurst(&mut self) -> _DBGMCURSTW {
        _DBGMCURSTW { w: self }
    }
}
