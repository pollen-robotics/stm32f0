#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR2 {
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
#[doc = r" Value of the field"]
pub struct ADD4R {
    bits: u8,
}
impl ADD4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ADD0R {
    bits: u8,
}
impl ADD0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `RTOEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTOENR {
    #[doc = "Receiver timeout feature disabled."]
    DISABLED,
    #[doc = "Receiver timeout feature enabled."]
    ENABLED,
}
impl RTOENR {
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
            RTOENR::DISABLED => false,
            RTOENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTOENR {
        match value {
            false => RTOENR::DISABLED,
            true => RTOENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RTOENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RTOENR::ENABLED
    }
}
#[doc = "Possible values of the field `ABRMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABRMODR {
    #[doc = "Measurement of the start bit is used to detect the baud rate."]
    STARTBIT,
    #[doc = "Falling edge to falling edge measurement."]
    FALLINGEDGE,
    #[doc = "0x7F frame detection.."]
    _0X7F,
    #[doc = "0x55 frame detection."]
    _0X55,
}
impl ABRMODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ABRMODR::STARTBIT => 0,
            ABRMODR::FALLINGEDGE => 1,
            ABRMODR::_0X7F => 2,
            ABRMODR::_0X55 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ABRMODR {
        match value {
            0 => ABRMODR::STARTBIT,
            1 => ABRMODR::FALLINGEDGE,
            2 => ABRMODR::_0X7F,
            3 => ABRMODR::_0X55,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STARTBIT`"]
    #[inline]
    pub fn is_start_bit(&self) -> bool {
        *self == ABRMODR::STARTBIT
    }
    #[doc = "Checks if the value of the field is `FALLINGEDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == ABRMODR::FALLINGEDGE
    }
    #[doc = "Checks if the value of the field is `_0X7F`"]
    #[inline]
    pub fn is_0x7f(&self) -> bool {
        *self == ABRMODR::_0X7F
    }
    #[doc = "Checks if the value of the field is `_0X55`"]
    #[inline]
    pub fn is_0x55(&self) -> bool {
        *self == ABRMODR::_0X55
    }
}
#[doc = "Possible values of the field `ABREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABRENR {
    #[doc = "Auto baud rate detection disabled."]
    DISABLED,
    #[doc = "Auto baud rate detection enabled."]
    ENABLED,
}
impl ABRENR {
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
            ABRENR::DISABLED => false,
            ABRENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ABRENR {
        match value {
            false => ABRENR::DISABLED,
            true => ABRENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ABRENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ABRENR::ENABLED
    }
}
#[doc = "Possible values of the field `MSBFIRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSBFIRSTR {
    #[doc = "Data is transmitted/received with data bit 0 first, following the start bit."]
    LSB,
    #[doc = "Data is transmitted/received with the MSB (bit 7/8/9) first, following the start bit."]
    MSB,
}
impl MSBFIRSTR {
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
            MSBFIRSTR::LSB => false,
            MSBFIRSTR::MSB => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSBFIRSTR {
        match value {
            false => MSBFIRSTR::LSB,
            true => MSBFIRSTR::MSB,
        }
    }
    #[doc = "Checks if the value of the field is `LSB`"]
    #[inline]
    pub fn is_lsb(&self) -> bool {
        *self == MSBFIRSTR::LSB
    }
    #[doc = "Checks if the value of the field is `MSB`"]
    #[inline]
    pub fn is_msb(&self) -> bool {
        *self == MSBFIRSTR::MSB
    }
}
#[doc = "Possible values of the field `DATAINV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATAINVR {
    #[doc = "Logical data from the data register are send/received in positive/direct logic."]
    POSITIVE,
    #[doc = "Logical data from the data register are send/received in negative/inverse logic."]
    NEGATIVE,
}
impl DATAINVR {
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
            DATAINVR::POSITIVE => false,
            DATAINVR::NEGATIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATAINVR {
        match value {
            false => DATAINVR::POSITIVE,
            true => DATAINVR::NEGATIVE,
        }
    }
    #[doc = "Checks if the value of the field is `POSITIVE`"]
    #[inline]
    pub fn is_positive(&self) -> bool {
        *self == DATAINVR::POSITIVE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE`"]
    #[inline]
    pub fn is_negative(&self) -> bool {
        *self == DATAINVR::NEGATIVE
    }
}
#[doc = "Possible values of the field `TXINV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXINVR {
    #[doc = "TX pin signal works using the standard logic levels"]
    NORMAL,
    #[doc = "TX pin signal values are inverted."]
    INVERTED,
}
impl TXINVR {
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
            TXINVR::NORMAL => false,
            TXINVR::INVERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXINVR {
        match value {
            false => TXINVR::NORMAL,
            true => TXINVR::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == TXINVR::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline]
    pub fn is_inverted(&self) -> bool {
        *self == TXINVR::INVERTED
    }
}
#[doc = "Possible values of the field `RXINV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXINVR {
    #[doc = "RX pin signal works using the standard logic levels"]
    NORMAL,
    #[doc = "RX pin signal values are inverted."]
    INVERTED,
}
impl RXINVR {
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
            RXINVR::NORMAL => false,
            RXINVR::INVERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXINVR {
        match value {
            false => RXINVR::NORMAL,
            true => RXINVR::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == RXINVR::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline]
    pub fn is_inverted(&self) -> bool {
        *self == RXINVR::INVERTED
    }
}
#[doc = "Possible values of the field `SWAP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWAPR {
    #[doc = "TX/RX pins are used as defined in standard pinout."]
    STANDARD,
    #[doc = "The TX and RX pins functions are swapped."]
    SWAPPED,
}
impl SWAPR {
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
            SWAPR::STANDARD => false,
            SWAPR::SWAPPED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWAPR {
        match value {
            false => SWAPR::STANDARD,
            true => SWAPR::SWAPPED,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline]
    pub fn is_standard(&self) -> bool {
        *self == SWAPR::STANDARD
    }
    #[doc = "Checks if the value of the field is `SWAPPED`"]
    #[inline]
    pub fn is_swapped(&self) -> bool {
        *self == SWAPR::SWAPPED
    }
}
#[doc = "Possible values of the field `LINEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINENR {
    #[doc = "LIN mode disabled."]
    DISABLED,
    #[doc = "LIN mode enabled."]
    ENABLED,
}
impl LINENR {
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
            LINENR::DISABLED => false,
            LINENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINENR {
        match value {
            false => LINENR::DISABLED,
            true => LINENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == LINENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == LINENR::ENABLED
    }
}
#[doc = "Possible values of the field `STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPR {
    #[doc = "1 stop bit."]
    _1STOP,
    #[doc = "0.5 stop bit."]
    HALFSTOP,
    #[doc = "2 stop bits."]
    _2STOP,
    #[doc = "1.5 stop bits."]
    _1HALFSTOP,
}
impl STOPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STOPR::_1STOP => 0,
            STOPR::HALFSTOP => 1,
            STOPR::_2STOP => 2,
            STOPR::_1HALFSTOP => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STOPR {
        match value {
            0 => STOPR::_1STOP,
            1 => STOPR::HALFSTOP,
            2 => STOPR::_2STOP,
            3 => STOPR::_1HALFSTOP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1STOP`"]
    #[inline]
    pub fn is_1stop(&self) -> bool {
        *self == STOPR::_1STOP
    }
    #[doc = "Checks if the value of the field is `HALFSTOP`"]
    #[inline]
    pub fn is_half_stop(&self) -> bool {
        *self == STOPR::HALFSTOP
    }
    #[doc = "Checks if the value of the field is `_2STOP`"]
    #[inline]
    pub fn is_2stop(&self) -> bool {
        *self == STOPR::_2STOP
    }
    #[doc = "Checks if the value of the field is `_1HALFSTOP`"]
    #[inline]
    pub fn is_1half_stop(&self) -> bool {
        *self == STOPR::_1HALFSTOP
    }
}
#[doc = "Possible values of the field `CLKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKENR {
    #[doc = "CK pin disabled."]
    DISABLED,
    #[doc = "CK pin enabled."]
    ENABLED,
}
impl CLKENR {
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
            CLKENR::DISABLED => false,
            CLKENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLKENR {
        match value {
            false => CLKENR::DISABLED,
            true => CLKENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CLKENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CLKENR::ENABLED
    }
}
#[doc = "Possible values of the field `CPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOLR {
    #[doc = "Steady low value on CK pin outside transmission window."]
    LOW,
    #[doc = "Steady high value on CK pin outside transmission window."]
    HIGH,
}
impl CPOLR {
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
            CPOLR::LOW => false,
            CPOLR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPOLR {
        match value {
            false => CPOLR::LOW,
            true => CPOLR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == CPOLR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == CPOLR::HIGH
    }
}
#[doc = "Possible values of the field `CPHA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHAR {
    #[doc = "The first clock transition is the first data capture edge."]
    FIRST,
    #[doc = "The second clock transition is the first data capture edge."]
    SECOND,
}
impl CPHAR {
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
            CPHAR::FIRST => false,
            CPHAR::SECOND => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPHAR {
        match value {
            false => CPHAR::FIRST,
            true => CPHAR::SECOND,
        }
    }
    #[doc = "Checks if the value of the field is `FIRST`"]
    #[inline]
    pub fn is_first(&self) -> bool {
        *self == CPHAR::FIRST
    }
    #[doc = "Checks if the value of the field is `SECOND`"]
    #[inline]
    pub fn is_second(&self) -> bool {
        *self == CPHAR::SECOND
    }
}
#[doc = "Possible values of the field `LBCL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBCLR {
    #[doc = "The clock pulse of the last data bit is not output to the CK pin."]
    NOOUTPUT,
    #[doc = "The clock pulse of the last data bit is output to the CK pin."]
    OUTPUT,
}
impl LBCLR {
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
            LBCLR::NOOUTPUT => false,
            LBCLR::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LBCLR {
        match value {
            false => LBCLR::NOOUTPUT,
            true => LBCLR::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOOUTPUT`"]
    #[inline]
    pub fn is_nooutput(&self) -> bool {
        *self == LBCLR::NOOUTPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == LBCLR::OUTPUT
    }
}
#[doc = "Possible values of the field `LBDIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBDIER {
    #[doc = "Interrupt is inhibited."]
    DISABLED,
    #[doc = "An interrupt is generated whenever LBDF=1 in the USART_ISR register."]
    ENABLED,
}
impl LBDIER {
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
            LBDIER::DISABLED => false,
            LBDIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LBDIER {
        match value {
            false => LBDIER::DISABLED,
            true => LBDIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == LBDIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == LBDIER::ENABLED
    }
}
#[doc = "Possible values of the field `LBDL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBDLR {
    #[doc = "10-bit break detection."]
    _10BITS,
    #[doc = "11-bit break detection."]
    _11BITS,
}
impl LBDLR {
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
            LBDLR::_10BITS => false,
            LBDLR::_11BITS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LBDLR {
        match value {
            false => LBDLR::_10BITS,
            true => LBDLR::_11BITS,
        }
    }
    #[doc = "Checks if the value of the field is `_10BITS`"]
    #[inline]
    pub fn is_10bits(&self) -> bool {
        *self == LBDLR::_10BITS
    }
    #[doc = "Checks if the value of the field is `_11BITS`"]
    #[inline]
    pub fn is_11bits(&self) -> bool {
        *self == LBDLR::_11BITS
    }
}
#[doc = "Possible values of the field `ADDM7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDM7R {
    #[doc = "4-bit address detection."]
    _4BITS,
    #[doc = "7-bit address detection (in 8-bit data mode)."]
    _7BITS,
}
impl ADDM7R {
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
            ADDM7R::_4BITS => false,
            ADDM7R::_7BITS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADDM7R {
        match value {
            false => ADDM7R::_4BITS,
            true => ADDM7R::_7BITS,
        }
    }
    #[doc = "Checks if the value of the field is `_4BITS`"]
    #[inline]
    pub fn is_4bits(&self) -> bool {
        *self == ADDM7R::_4BITS
    }
    #[doc = "Checks if the value of the field is `_7BITS`"]
    #[inline]
    pub fn is_7bits(&self) -> bool {
        *self == ADDM7R::_7BITS
    }
}
#[doc = r" Proxy"]
pub struct _ADD4W<'a> {
    w: &'a mut W,
}
impl<'a> _ADD4W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ADD0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADD0W<'a> {
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
#[doc = "Values that can be written to the field `RTOEN`"]
pub enum RTOENW {
    #[doc = "Receiver timeout feature disabled."]
    DISABLED,
    #[doc = "Receiver timeout feature enabled."]
    ENABLED,
}
impl RTOENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTOENW::DISABLED => false,
            RTOENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTOENW<'a> {
    w: &'a mut W,
}
impl<'a> _RTOENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTOENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receiver timeout feature disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTOENW::DISABLED)
    }
    #[doc = "Receiver timeout feature enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RTOENW::ENABLED)
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
#[doc = "Values that can be written to the field `ABRMOD`"]
pub enum ABRMODW {
    #[doc = "Measurement of the start bit is used to detect the baud rate."]
    STARTBIT,
    #[doc = "Falling edge to falling edge measurement."]
    FALLINGEDGE,
    #[doc = "0x7F frame detection.."]
    _0X7F,
    #[doc = "0x55 frame detection."]
    _0X55,
}
impl ABRMODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ABRMODW::STARTBIT => 0,
            ABRMODW::FALLINGEDGE => 1,
            ABRMODW::_0X7F => 2,
            ABRMODW::_0X55 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ABRMODW<'a> {
    w: &'a mut W,
}
impl<'a> _ABRMODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ABRMODW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Measurement of the start bit is used to detect the baud rate."]
    #[inline]
    pub fn start_bit(self) -> &'a mut W {
        self.variant(ABRMODW::STARTBIT)
    }
    #[doc = "Falling edge to falling edge measurement."]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ABRMODW::FALLINGEDGE)
    }
    #[doc = "0x7F frame detection.."]
    #[inline]
    pub fn _0x7f(self) -> &'a mut W {
        self.variant(ABRMODW::_0X7F)
    }
    #[doc = "0x55 frame detection."]
    #[inline]
    pub fn _0x55(self) -> &'a mut W {
        self.variant(ABRMODW::_0X55)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ABREN`"]
pub enum ABRENW {
    #[doc = "Auto baud rate detection disabled."]
    DISABLED,
    #[doc = "Auto baud rate detection enabled."]
    ENABLED,
}
impl ABRENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ABRENW::DISABLED => false,
            ABRENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ABRENW<'a> {
    w: &'a mut W,
}
impl<'a> _ABRENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ABRENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Auto baud rate detection disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ABRENW::DISABLED)
    }
    #[doc = "Auto baud rate detection enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ABRENW::ENABLED)
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
#[doc = "Values that can be written to the field `MSBFIRST`"]
pub enum MSBFIRSTW {
    #[doc = "Data is transmitted/received with data bit 0 first, following the start bit."]
    LSB,
    #[doc = "Data is transmitted/received with the MSB (bit 7/8/9) first, following the start bit."]
    MSB,
}
impl MSBFIRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSBFIRSTW::LSB => false,
            MSBFIRSTW::MSB => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSBFIRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _MSBFIRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSBFIRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data is transmitted/received with data bit 0 first, following the start bit."]
    #[inline]
    pub fn lsb(self) -> &'a mut W {
        self.variant(MSBFIRSTW::LSB)
    }
    #[doc = "Data is transmitted/received with the MSB (bit 7/8/9) first, following the start bit."]
    #[inline]
    pub fn msb(self) -> &'a mut W {
        self.variant(MSBFIRSTW::MSB)
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
#[doc = "Values that can be written to the field `DATAINV`"]
pub enum DATAINVW {
    #[doc = "Logical data from the data register are send/received in positive/direct logic."]
    POSITIVE,
    #[doc = "Logical data from the data register are send/received in negative/inverse logic."]
    NEGATIVE,
}
impl DATAINVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATAINVW::POSITIVE => false,
            DATAINVW::NEGATIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATAINVW<'a> {
    w: &'a mut W,
}
impl<'a> _DATAINVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATAINVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Logical data from the data register are send/received in positive/direct logic."]
    #[inline]
    pub fn positive(self) -> &'a mut W {
        self.variant(DATAINVW::POSITIVE)
    }
    #[doc = "Logical data from the data register are send/received in negative/inverse logic."]
    #[inline]
    pub fn negative(self) -> &'a mut W {
        self.variant(DATAINVW::NEGATIVE)
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
#[doc = "Values that can be written to the field `TXINV`"]
pub enum TXINVW {
    #[doc = "TX pin signal works using the standard logic levels"]
    NORMAL,
    #[doc = "TX pin signal values are inverted."]
    INVERTED,
}
impl TXINVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXINVW::NORMAL => false,
            TXINVW::INVERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXINVW<'a> {
    w: &'a mut W,
}
impl<'a> _TXINVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXINVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TX pin signal works using the standard logic levels"]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(TXINVW::NORMAL)
    }
    #[doc = "TX pin signal values are inverted."]
    #[inline]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TXINVW::INVERTED)
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
#[doc = "Values that can be written to the field `RXINV`"]
pub enum RXINVW {
    #[doc = "RX pin signal works using the standard logic levels"]
    NORMAL,
    #[doc = "RX pin signal values are inverted."]
    INVERTED,
}
impl RXINVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXINVW::NORMAL => false,
            RXINVW::INVERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXINVW<'a> {
    w: &'a mut W,
}
impl<'a> _RXINVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXINVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RX pin signal works using the standard logic levels"]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(RXINVW::NORMAL)
    }
    #[doc = "RX pin signal values are inverted."]
    #[inline]
    pub fn inverted(self) -> &'a mut W {
        self.variant(RXINVW::INVERTED)
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
#[doc = "Values that can be written to the field `SWAP`"]
pub enum SWAPW {
    #[doc = "TX/RX pins are used as defined in standard pinout."]
    STANDARD,
    #[doc = "The TX and RX pins functions are swapped."]
    SWAPPED,
}
impl SWAPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWAPW::STANDARD => false,
            SWAPW::SWAPPED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWAPW<'a> {
    w: &'a mut W,
}
impl<'a> _SWAPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWAPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TX/RX pins are used as defined in standard pinout."]
    #[inline]
    pub fn standard(self) -> &'a mut W {
        self.variant(SWAPW::STANDARD)
    }
    #[doc = "The TX and RX pins functions are swapped."]
    #[inline]
    pub fn swapped(self) -> &'a mut W {
        self.variant(SWAPW::SWAPPED)
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
#[doc = "Values that can be written to the field `LINEN`"]
pub enum LINENW {
    #[doc = "LIN mode disabled."]
    DISABLED,
    #[doc = "LIN mode enabled."]
    ENABLED,
}
impl LINENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINENW::DISABLED => false,
            LINENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINENW<'a> {
    w: &'a mut W,
}
impl<'a> _LINENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LIN mode disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LINENW::DISABLED)
    }
    #[doc = "LIN mode enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LINENW::ENABLED)
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
#[doc = "Values that can be written to the field `STOP`"]
pub enum STOPW {
    #[doc = "1 stop bit."]
    _1STOP,
    #[doc = "0.5 stop bit."]
    HALFSTOP,
    #[doc = "2 stop bits."]
    _2STOP,
    #[doc = "1.5 stop bits."]
    _1HALFSTOP,
}
impl STOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STOPW::_1STOP => 0,
            STOPW::HALFSTOP => 1,
            STOPW::_2STOP => 2,
            STOPW::_1HALFSTOP => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _STOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STOPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1 stop bit."]
    #[inline]
    pub fn _1stop(self) -> &'a mut W {
        self.variant(STOPW::_1STOP)
    }
    #[doc = "0.5 stop bit."]
    #[inline]
    pub fn half_stop(self) -> &'a mut W {
        self.variant(STOPW::HALFSTOP)
    }
    #[doc = "2 stop bits."]
    #[inline]
    pub fn _2stop(self) -> &'a mut W {
        self.variant(STOPW::_2STOP)
    }
    #[doc = "1.5 stop bits."]
    #[inline]
    pub fn _1half_stop(self) -> &'a mut W {
        self.variant(STOPW::_1HALFSTOP)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLKEN`"]
pub enum CLKENW {
    #[doc = "CK pin disabled."]
    DISABLED,
    #[doc = "CK pin enabled."]
    ENABLED,
}
impl CLKENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLKENW::DISABLED => false,
            CLKENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKENW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CK pin disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CLKENW::DISABLED)
    }
    #[doc = "CK pin enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CLKENW::ENABLED)
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
#[doc = "Values that can be written to the field `CPOL`"]
pub enum CPOLW {
    #[doc = "Steady low value on CK pin outside transmission window."]
    LOW,
    #[doc = "Steady high value on CK pin outside transmission window."]
    HIGH,
}
impl CPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPOLW::LOW => false,
            CPOLW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _CPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Steady low value on CK pin outside transmission window."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(CPOLW::LOW)
    }
    #[doc = "Steady high value on CK pin outside transmission window."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(CPOLW::HIGH)
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
#[doc = "Values that can be written to the field `CPHA`"]
pub enum CPHAW {
    #[doc = "The first clock transition is the first data capture edge."]
    FIRST,
    #[doc = "The second clock transition is the first data capture edge."]
    SECOND,
}
impl CPHAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPHAW::FIRST => false,
            CPHAW::SECOND => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPHAW<'a> {
    w: &'a mut W,
}
impl<'a> _CPHAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPHAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The first clock transition is the first data capture edge."]
    #[inline]
    pub fn first(self) -> &'a mut W {
        self.variant(CPHAW::FIRST)
    }
    #[doc = "The second clock transition is the first data capture edge."]
    #[inline]
    pub fn second(self) -> &'a mut W {
        self.variant(CPHAW::SECOND)
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
#[doc = "Values that can be written to the field `LBCL`"]
pub enum LBCLW {
    #[doc = "The clock pulse of the last data bit is not output to the CK pin."]
    NOOUTPUT,
    #[doc = "The clock pulse of the last data bit is output to the CK pin."]
    OUTPUT,
}
impl LBCLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LBCLW::NOOUTPUT => false,
            LBCLW::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LBCLW<'a> {
    w: &'a mut W,
}
impl<'a> _LBCLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LBCLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The clock pulse of the last data bit is not output to the CK pin."]
    #[inline]
    pub fn nooutput(self) -> &'a mut W {
        self.variant(LBCLW::NOOUTPUT)
    }
    #[doc = "The clock pulse of the last data bit is output to the CK pin."]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(LBCLW::OUTPUT)
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
#[doc = "Values that can be written to the field `LBDIE`"]
pub enum LBDIEW {
    #[doc = "Interrupt is inhibited."]
    DISABLED,
    #[doc = "An interrupt is generated whenever LBDF=1 in the USART_ISR register."]
    ENABLED,
}
impl LBDIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LBDIEW::DISABLED => false,
            LBDIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LBDIEW<'a> {
    w: &'a mut W,
}
impl<'a> _LBDIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LBDIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt is inhibited."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LBDIEW::DISABLED)
    }
    #[doc = "An interrupt is generated whenever LBDF=1 in the USART_ISR register."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LBDIEW::ENABLED)
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
#[doc = "Values that can be written to the field `LBDL`"]
pub enum LBDLW {
    #[doc = "10-bit break detection."]
    _10BITS,
    #[doc = "11-bit break detection."]
    _11BITS,
}
impl LBDLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LBDLW::_10BITS => false,
            LBDLW::_11BITS => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LBDLW<'a> {
    w: &'a mut W,
}
impl<'a> _LBDLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LBDLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "10-bit break detection."]
    #[inline]
    pub fn _10bits(self) -> &'a mut W {
        self.variant(LBDLW::_10BITS)
    }
    #[doc = "11-bit break detection."]
    #[inline]
    pub fn _11bits(self) -> &'a mut W {
        self.variant(LBDLW::_11BITS)
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
#[doc = "Values that can be written to the field `ADDM7`"]
pub enum ADDM7W {
    #[doc = "4-bit address detection."]
    _4BITS,
    #[doc = "7-bit address detection (in 8-bit data mode)."]
    _7BITS,
}
impl ADDM7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADDM7W::_4BITS => false,
            ADDM7W::_7BITS => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADDM7W<'a> {
    w: &'a mut W,
}
impl<'a> _ADDM7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADDM7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "4-bit address detection."]
    #[inline]
    pub fn _4bits(self) -> &'a mut W {
        self.variant(ADDM7W::_4BITS)
    }
    #[doc = "7-bit address detection (in 8-bit data mode)."]
    #[inline]
    pub fn _7bits(self) -> &'a mut W {
        self.variant(ADDM7W::_7BITS)
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
    #[doc = "Bits 28:31 - Address of the USART node"]
    #[inline]
    pub fn add4(&self) -> ADD4R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADD4R { bits }
    }
    #[doc = "Bits 24:27 - Address of the USART node"]
    #[inline]
    pub fn add0(&self) -> ADD0R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADD0R { bits }
    }
    #[doc = "Bit 23 - Receiver timeout enable"]
    #[inline]
    pub fn rtoen(&self) -> RTOENR {
        RTOENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 21:22 - Auto baud rate mode"]
    #[inline]
    pub fn abrmod(&self) -> ABRMODR {
        ABRMODR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - Auto baud rate enable"]
    #[inline]
    pub fn abren(&self) -> ABRENR {
        ABRENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Most significant bit first"]
    #[inline]
    pub fn msbfirst(&self) -> MSBFIRSTR {
        MSBFIRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Binary data inversion"]
    #[inline]
    pub fn datainv(&self) -> DATAINVR {
        DATAINVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - TX pin active level inversion"]
    #[inline]
    pub fn txinv(&self) -> TXINVR {
        TXINVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - RX pin active level inversion"]
    #[inline]
    pub fn rxinv(&self) -> RXINVR {
        RXINVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Swap TX/RX pins"]
    #[inline]
    pub fn swap(&self) -> SWAPR {
        SWAPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline]
    pub fn linen(&self) -> LINENR {
        LINENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 12:13 - STOP bits"]
    #[inline]
    pub fn stop(&self) -> STOPR {
        STOPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - Clock enable"]
    #[inline]
    pub fn clken(&self) -> CLKENR {
        CLKENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline]
    pub fn cpol(&self) -> CPOLR {
        CPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline]
    pub fn cpha(&self) -> CPHAR {
        CPHAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Last bit clock pulse"]
    #[inline]
    pub fn lbcl(&self) -> LBCLR {
        LBCLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline]
    pub fn lbdie(&self) -> LBDIER {
        LBDIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - LIN break detection length"]
    #[inline]
    pub fn lbdl(&self) -> LBDLR {
        LBDLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection"]
    #[inline]
    pub fn addm7(&self) -> ADDM7R {
        ADDM7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
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
    #[doc = "Bits 28:31 - Address of the USART node"]
    #[inline]
    pub fn add4(&mut self) -> _ADD4W {
        _ADD4W { w: self }
    }
    #[doc = "Bits 24:27 - Address of the USART node"]
    #[inline]
    pub fn add0(&mut self) -> _ADD0W {
        _ADD0W { w: self }
    }
    #[doc = "Bit 23 - Receiver timeout enable"]
    #[inline]
    pub fn rtoen(&mut self) -> _RTOENW {
        _RTOENW { w: self }
    }
    #[doc = "Bits 21:22 - Auto baud rate mode"]
    #[inline]
    pub fn abrmod(&mut self) -> _ABRMODW {
        _ABRMODW { w: self }
    }
    #[doc = "Bit 20 - Auto baud rate enable"]
    #[inline]
    pub fn abren(&mut self) -> _ABRENW {
        _ABRENW { w: self }
    }
    #[doc = "Bit 19 - Most significant bit first"]
    #[inline]
    pub fn msbfirst(&mut self) -> _MSBFIRSTW {
        _MSBFIRSTW { w: self }
    }
    #[doc = "Bit 18 - Binary data inversion"]
    #[inline]
    pub fn datainv(&mut self) -> _DATAINVW {
        _DATAINVW { w: self }
    }
    #[doc = "Bit 17 - TX pin active level inversion"]
    #[inline]
    pub fn txinv(&mut self) -> _TXINVW {
        _TXINVW { w: self }
    }
    #[doc = "Bit 16 - RX pin active level inversion"]
    #[inline]
    pub fn rxinv(&mut self) -> _RXINVW {
        _RXINVW { w: self }
    }
    #[doc = "Bit 15 - Swap TX/RX pins"]
    #[inline]
    pub fn swap(&mut self) -> _SWAPW {
        _SWAPW { w: self }
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline]
    pub fn linen(&mut self) -> _LINENW {
        _LINENW { w: self }
    }
    #[doc = "Bits 12:13 - STOP bits"]
    #[inline]
    pub fn stop(&mut self) -> _STOPW {
        _STOPW { w: self }
    }
    #[doc = "Bit 11 - Clock enable"]
    #[inline]
    pub fn clken(&mut self) -> _CLKENW {
        _CLKENW { w: self }
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline]
    pub fn cpol(&mut self) -> _CPOLW {
        _CPOLW { w: self }
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline]
    pub fn cpha(&mut self) -> _CPHAW {
        _CPHAW { w: self }
    }
    #[doc = "Bit 8 - Last bit clock pulse"]
    #[inline]
    pub fn lbcl(&mut self) -> _LBCLW {
        _LBCLW { w: self }
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline]
    pub fn lbdie(&mut self) -> _LBDIEW {
        _LBDIEW { w: self }
    }
    #[doc = "Bit 5 - LIN break detection length"]
    #[inline]
    pub fn lbdl(&mut self) -> _LBDLW {
        _LBDLW { w: self }
    }
    #[doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection"]
    #[inline]
    pub fn addm7(&mut self) -> _ADDM7W {
        _ADDM7W { w: self }
    }
}
