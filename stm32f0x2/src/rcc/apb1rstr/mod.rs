#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::APB1RSTR {
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
#[doc = "Possible values of the field `TIM2RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM2RSTR {
    #[doc = "Reset TIM2 timer."] RESET,
    #[doc = r" Reserved"] _Reserved(bool),
}
impl TIM2RSTR {
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
            TIM2RSTR::RESET => true,
            TIM2RSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIM2RSTR {
        match value {
            true => TIM2RSTR::RESET,
            i => TIM2RSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == TIM2RSTR::RESET
    }
}
#[doc = "Possible values of the field `TIM3RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM3RSTR {
    #[doc = "Reset TIM3 timer."] RESET,
    #[doc = r" Reserved"] _Reserved(bool),
}
impl TIM3RSTR {
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
            TIM3RSTR::RESET => true,
            TIM3RSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIM3RSTR {
        match value {
            true => TIM3RSTR::RESET,
            i => TIM3RSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == TIM3RSTR::RESET
    }
}
#[doc = "Possible values of the field `TIM6RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM6RSTR {
    #[doc = "Reset TIM6 timer."] RESET,
    #[doc = r" Reserved"] _Reserved(bool),
}
impl TIM6RSTR {
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
            TIM6RSTR::RESET => true,
            TIM6RSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIM6RSTR {
        match value {
            true => TIM6RSTR::RESET,
            i => TIM6RSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == TIM6RSTR::RESET
    }
}
#[doc = "Possible values of the field `TIM7RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM7RSTR {
    #[doc = "Reset TIM7 timer."] RESET,
    #[doc = r" Reserved"] _Reserved(bool),
}
impl TIM7RSTR {
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
            TIM7RSTR::RESET => true,
            TIM7RSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIM7RSTR {
        match value {
            true => TIM7RSTR::RESET,
            i => TIM7RSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == TIM7RSTR::RESET
    }
}
#[doc = "Possible values of the field `TIM14RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM14RSTR {
    #[doc = "Reset TIM14 timer."] RESET,
    #[doc = r" Reserved"] _Reserved(bool),
}
impl TIM14RSTR {
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
            TIM14RSTR::RESET => true,
            TIM14RSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIM14RSTR {
        match value {
            true => TIM14RSTR::RESET,
            i => TIM14RSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == TIM14RSTR::RESET
    }
}
#[doc = "Possible values of the field `WWDGRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WWDGRSTR {
    #[doc = "Reset window watchdog."] RESET,
    #[doc = r" Reserved"] _Reserved(bool),
}
impl WWDGRSTR {
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
            WWDGRSTR::RESET => true,
            WWDGRSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WWDGRSTR {
        match value {
            true => WWDGRSTR::RESET,
            i => WWDGRSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == WWDGRSTR::RESET
    }
}
#[doc = "Possible values of the field `SPI2RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI2RSTR {
    #[doc = "Reset SPI2."] RESET,
    #[doc = r" Reserved"] _Reserved(bool),
}
impl SPI2RSTR {
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
            SPI2RSTR::RESET => true,
            SPI2RSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPI2RSTR {
        match value {
            true => SPI2RSTR::RESET,
            i => SPI2RSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == SPI2RSTR::RESET
    }
}
#[doc = "Possible values of the field `USART2RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART2RSTR {
    #[doc = "Reset USART2."] RESET,
    #[doc = r" Reserved"] _Reserved(bool),
}
impl USART2RSTR {
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
            USART2RSTR::RESET => true,
            USART2RSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USART2RSTR {
        match value {
            true => USART2RSTR::RESET,
            i => USART2RSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == USART2RSTR::RESET
    }
}
#[doc = "Possible values of the field `USART3RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART3RSTR {
    #[doc = "Reset USART3."] RESET,
    #[doc = r" Reserved"] _Reserved(bool),
}
impl USART3RSTR {
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
            USART3RSTR::RESET => true,
            USART3RSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USART3RSTR {
        match value {
            true => USART3RSTR::RESET,
            i => USART3RSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == USART3RSTR::RESET
    }
}
#[doc = "Possible values of the field `USART4RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART4RSTR {
    #[doc = "Reset USART4."] RESET,
    #[doc = r" Reserved"] _Reserved(bool),
}
impl USART4RSTR {
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
            USART4RSTR::RESET => true,
            USART4RSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USART4RSTR {
        match value {
            true => USART4RSTR::RESET,
            i => USART4RSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == USART4RSTR::RESET
    }
}
#[doc = "Possible values of the field `I2C1RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C1RSTR {
    #[doc = "Reset I2C1."] RESET,
    #[doc = r" Reserved"] _Reserved(bool),
}
impl I2C1RSTR {
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
            I2C1RSTR::RESET => true,
            I2C1RSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> I2C1RSTR {
        match value {
            true => I2C1RSTR::RESET,
            i => I2C1RSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == I2C1RSTR::RESET
    }
}
#[doc = "Possible values of the field `I2C2RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C2RSTR {
    #[doc = "Reset I2C2."] RESET,
    #[doc = r" Reserved"] _Reserved(bool),
}
impl I2C2RSTR {
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
            I2C2RSTR::RESET => true,
            I2C2RSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> I2C2RSTR {
        match value {
            true => I2C2RSTR::RESET,
            i => I2C2RSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == I2C2RSTR::RESET
    }
}
#[doc = "Possible values of the field `USBRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBRSTR {
    #[doc = "Reset USB interface."] RESET,
    #[doc = r" Reserved"] _Reserved(bool),
}
impl USBRSTR {
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
            USBRSTR::RESET => true,
            USBRSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USBRSTR {
        match value {
            true => USBRSTR::RESET,
            i => USBRSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == USBRSTR::RESET
    }
}
#[doc = "Possible values of the field `CANRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CANRSTR {
    #[doc = "Reset CAN interface."] RESET,
    #[doc = r" Reserved"] _Reserved(bool),
}
impl CANRSTR {
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
            CANRSTR::RESET => true,
            CANRSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CANRSTR {
        match value {
            true => CANRSTR::RESET,
            i => CANRSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == CANRSTR::RESET
    }
}
#[doc = "Possible values of the field `CRSRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRSRSTR {
    #[doc = "Reset CRS interface."] RESET,
    #[doc = r" Reserved"] _Reserved(bool),
}
impl CRSRSTR {
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
            CRSRSTR::RESET => true,
            CRSRSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRSRSTR {
        match value {
            true => CRSRSTR::RESET,
            i => CRSRSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == CRSRSTR::RESET
    }
}
#[doc = "Possible values of the field `PWRRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRRSTR {
    #[doc = "Reset power interface."] RESET,
    #[doc = r" Reserved"] _Reserved(bool),
}
impl PWRRSTR {
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
            PWRRSTR::RESET => true,
            PWRRSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWRRSTR {
        match value {
            true => PWRRSTR::RESET,
            i => PWRRSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == PWRRSTR::RESET
    }
}
#[doc = "Possible values of the field `DACRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACRSTR {
    #[doc = "Reset DAC."] RESET,
    #[doc = r" Reserved"] _Reserved(bool),
}
impl DACRSTR {
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
            DACRSTR::RESET => true,
            DACRSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DACRSTR {
        match value {
            true => DACRSTR::RESET,
            i => DACRSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == DACRSTR::RESET
    }
}
#[doc = "Possible values of the field `CECRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CECRSTR {
    #[doc = "Reset HDMI CEC."] RESET,
    #[doc = r" Reserved"] _Reserved(bool),
}
impl CECRSTR {
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
            CECRSTR::RESET => true,
            CECRSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CECRSTR {
        match value {
            true => CECRSTR::RESET,
            i => CECRSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == CECRSTR::RESET
    }
}
#[doc = "Values that can be written to the field `TIM2RST`"]
pub enum TIM2RSTW {
    #[doc = "Reset TIM2 timer."] RESET,
}
impl TIM2RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIM2RSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIM2RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM2RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM2RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset TIM2 timer."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RSTW::RESET)
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
#[doc = "Values that can be written to the field `TIM3RST`"]
pub enum TIM3RSTW {
    #[doc = "Reset TIM3 timer."] RESET,
}
impl TIM3RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIM3RSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIM3RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM3RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM3RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset TIM3 timer."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM3RSTW::RESET)
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
#[doc = "Values that can be written to the field `TIM6RST`"]
pub enum TIM6RSTW {
    #[doc = "Reset TIM6 timer."] RESET,
}
impl TIM6RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIM6RSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIM6RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM6RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM6RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset TIM6 timer."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM6RSTW::RESET)
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
#[doc = "Values that can be written to the field `TIM7RST`"]
pub enum TIM7RSTW {
    #[doc = "Reset TIM7 timer."] RESET,
}
impl TIM7RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIM7RSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIM7RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM7RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM7RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset TIM7 timer."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM7RSTW::RESET)
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
#[doc = "Values that can be written to the field `TIM14RST`"]
pub enum TIM14RSTW {
    #[doc = "Reset TIM14 timer."] RESET,
}
impl TIM14RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIM14RSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIM14RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM14RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM14RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset TIM14 timer."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM14RSTW::RESET)
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
#[doc = "Values that can be written to the field `WWDGRST`"]
pub enum WWDGRSTW {
    #[doc = "Reset window watchdog."] RESET,
}
impl WWDGRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WWDGRSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WWDGRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _WWDGRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WWDGRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset window watchdog."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(WWDGRSTW::RESET)
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
#[doc = "Values that can be written to the field `SPI2RST`"]
pub enum SPI2RSTW {
    #[doc = "Reset SPI2."] RESET,
}
impl SPI2RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPI2RSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPI2RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI2RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI2RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset SPI2."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(SPI2RSTW::RESET)
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
#[doc = "Values that can be written to the field `USART2RST`"]
pub enum USART2RSTW {
    #[doc = "Reset USART2."] RESET,
}
impl USART2RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USART2RSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USART2RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _USART2RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USART2RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset USART2."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(USART2RSTW::RESET)
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
#[doc = "Values that can be written to the field `USART3RST`"]
pub enum USART3RSTW {
    #[doc = "Reset USART3."] RESET,
}
impl USART3RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USART3RSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USART3RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _USART3RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USART3RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset USART3."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(USART3RSTW::RESET)
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
#[doc = "Values that can be written to the field `USART4RST`"]
pub enum USART4RSTW {
    #[doc = "Reset USART4."] RESET,
}
impl USART4RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USART4RSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USART4RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _USART4RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USART4RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset USART4."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(USART4RSTW::RESET)
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
#[doc = "Values that can be written to the field `I2C1RST`"]
pub enum I2C1RSTW {
    #[doc = "Reset I2C1."] RESET,
}
impl I2C1RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            I2C1RSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2C1RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C1RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2C1RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset I2C1."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(I2C1RSTW::RESET)
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
#[doc = "Values that can be written to the field `I2C2RST`"]
pub enum I2C2RSTW {
    #[doc = "Reset I2C2."] RESET,
}
impl I2C2RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            I2C2RSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2C2RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C2RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2C2RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset I2C2."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(I2C2RSTW::RESET)
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
#[doc = "Values that can be written to the field `USBRST`"]
pub enum USBRSTW {
    #[doc = "Reset USB interface."] RESET,
}
impl USBRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USBRSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _USBRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset USB interface."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(USBRSTW::RESET)
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
#[doc = "Values that can be written to the field `CANRST`"]
pub enum CANRSTW {
    #[doc = "Reset CAN interface."] RESET,
}
impl CANRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CANRSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CANRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _CANRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CANRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset CAN interface."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(CANRSTW::RESET)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CRSRST`"]
pub enum CRSRSTW {
    #[doc = "Reset CRS interface."] RESET,
}
impl CRSRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRSRSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRSRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _CRSRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRSRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset CRS interface."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(CRSRSTW::RESET)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PWRRST`"]
pub enum PWRRSTW {
    #[doc = "Reset power interface."] RESET,
}
impl PWRRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWRRSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWRRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _PWRRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWRRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset power interface."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(PWRRSTW::RESET)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DACRST`"]
pub enum DACRSTW {
    #[doc = "Reset DAC."] RESET,
}
impl DACRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DACRSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DACRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _DACRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DACRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset DAC."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(DACRSTW::RESET)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CECRST`"]
pub enum CECRSTW {
    #[doc = "Reset HDMI CEC."] RESET,
}
impl CECRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CECRSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CECRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _CECRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CECRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset HDMI CEC."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(CECRSTW::RESET)
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
        const OFFSET: u8 = 30;
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
    #[doc = "Bit 0 - Timer 2 reset"]
    #[inline]
    pub fn tim2rst(&self) -> TIM2RSTR {
        TIM2RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Timer 3 reset"]
    #[inline]
    pub fn tim3rst(&self) -> TIM3RSTR {
        TIM3RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Timer 6 reset"]
    #[inline]
    pub fn tim6rst(&self) -> TIM6RSTR {
        TIM6RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - TIM7 timer reset"]
    #[inline]
    pub fn tim7rst(&self) -> TIM7RSTR {
        TIM7RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Timer 14 reset"]
    #[inline]
    pub fn tim14rst(&self) -> TIM14RSTR {
        TIM14RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Window watchdog reset"]
    #[inline]
    pub fn wwdgrst(&self) -> WWDGRSTR {
        WWDGRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - SPI2 reset"]
    #[inline]
    pub fn spi2rst(&self) -> SPI2RSTR {
        SPI2RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - USART 2 reset"]
    #[inline]
    pub fn usart2rst(&self) -> USART2RSTR {
        USART2RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - USART3 reset"]
    #[inline]
    pub fn usart3rst(&self) -> USART3RSTR {
        USART3RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - USART4 reset"]
    #[inline]
    pub fn usart4rst(&self) -> USART4RSTR {
        USART4RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline]
    pub fn i2c1rst(&self) -> I2C1RSTR {
        I2C1RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - I2C2 reset"]
    #[inline]
    pub fn i2c2rst(&self) -> I2C2RSTR {
        I2C2RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - USB interface reset"]
    #[inline]
    pub fn usbrst(&self) -> USBRSTR {
        USBRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - CAN interface reset"]
    #[inline]
    pub fn canrst(&self) -> CANRSTR {
        CANRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Clock Recovery System interface reset"]
    #[inline]
    pub fn crsrst(&self) -> CRSRSTR {
        CRSRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline]
    pub fn pwrrst(&self) -> PWRRSTR {
        PWRRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - DAC interface reset"]
    #[inline]
    pub fn dacrst(&self) -> DACRSTR {
        DACRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - HDMI CEC reset"]
    #[inline]
    pub fn cecrst(&self) -> CECRSTR {
        CECRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
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
    #[doc = "Bit 0 - Timer 2 reset"]
    #[inline]
    pub fn tim2rst(&mut self) -> _TIM2RSTW {
        _TIM2RSTW { w: self }
    }
    #[doc = "Bit 1 - Timer 3 reset"]
    #[inline]
    pub fn tim3rst(&mut self) -> _TIM3RSTW {
        _TIM3RSTW { w: self }
    }
    #[doc = "Bit 4 - Timer 6 reset"]
    #[inline]
    pub fn tim6rst(&mut self) -> _TIM6RSTW {
        _TIM6RSTW { w: self }
    }
    #[doc = "Bit 5 - TIM7 timer reset"]
    #[inline]
    pub fn tim7rst(&mut self) -> _TIM7RSTW {
        _TIM7RSTW { w: self }
    }
    #[doc = "Bit 8 - Timer 14 reset"]
    #[inline]
    pub fn tim14rst(&mut self) -> _TIM14RSTW {
        _TIM14RSTW { w: self }
    }
    #[doc = "Bit 11 - Window watchdog reset"]
    #[inline]
    pub fn wwdgrst(&mut self) -> _WWDGRSTW {
        _WWDGRSTW { w: self }
    }
    #[doc = "Bit 14 - SPI2 reset"]
    #[inline]
    pub fn spi2rst(&mut self) -> _SPI2RSTW {
        _SPI2RSTW { w: self }
    }
    #[doc = "Bit 17 - USART 2 reset"]
    #[inline]
    pub fn usart2rst(&mut self) -> _USART2RSTW {
        _USART2RSTW { w: self }
    }
    #[doc = "Bit 18 - USART3 reset"]
    #[inline]
    pub fn usart3rst(&mut self) -> _USART3RSTW {
        _USART3RSTW { w: self }
    }
    #[doc = "Bit 19 - USART4 reset"]
    #[inline]
    pub fn usart4rst(&mut self) -> _USART4RSTW {
        _USART4RSTW { w: self }
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline]
    pub fn i2c1rst(&mut self) -> _I2C1RSTW {
        _I2C1RSTW { w: self }
    }
    #[doc = "Bit 22 - I2C2 reset"]
    #[inline]
    pub fn i2c2rst(&mut self) -> _I2C2RSTW {
        _I2C2RSTW { w: self }
    }
    #[doc = "Bit 23 - USB interface reset"]
    #[inline]
    pub fn usbrst(&mut self) -> _USBRSTW {
        _USBRSTW { w: self }
    }
    #[doc = "Bit 25 - CAN interface reset"]
    #[inline]
    pub fn canrst(&mut self) -> _CANRSTW {
        _CANRSTW { w: self }
    }
    #[doc = "Bit 27 - Clock Recovery System interface reset"]
    #[inline]
    pub fn crsrst(&mut self) -> _CRSRSTW {
        _CRSRSTW { w: self }
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline]
    pub fn pwrrst(&mut self) -> _PWRRSTW {
        _PWRRSTW { w: self }
    }
    #[doc = "Bit 29 - DAC interface reset"]
    #[inline]
    pub fn dacrst(&mut self) -> _DACRSTW {
        _DACRSTW { w: self }
    }
    #[doc = "Bit 30 - HDMI CEC reset"]
    #[inline]
    pub fn cecrst(&mut self) -> _CECRSTW {
        _CECRSTW { w: self }
    }
}
