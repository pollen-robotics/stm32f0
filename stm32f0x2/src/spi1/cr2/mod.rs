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
#[doc = "Possible values of the field `RXDMAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDMAENR {
    #[doc = "Rx buffer DMA disabled"]
    DISABLED,
    #[doc = "Rx buffer DMA enabled"]
    ENABLED,
}
impl RXDMAENR {
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
            RXDMAENR::DISABLED => false,
            RXDMAENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXDMAENR {
        match value {
            false => RXDMAENR::DISABLED,
            true => RXDMAENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RXDMAENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RXDMAENR::ENABLED
    }
}
#[doc = "Possible values of the field `TXDMAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDMAENR {
    #[doc = "Tx buffer DMA disabled"]
    DISABLED,
    #[doc = "Rx buffer DMA enabled"]
    ENABLED,
}
impl TXDMAENR {
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
            TXDMAENR::DISABLED => false,
            TXDMAENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXDMAENR {
        match value {
            false => TXDMAENR::DISABLED,
            true => TXDMAENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TXDMAENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TXDMAENR::ENABLED
    }
}
#[doc = "Possible values of the field `SSOE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSOER {
    #[doc = "SS output is disabled in master mode and the SPI interface can work in multimaster\nconfiguration"]
    DISABLED,
    #[doc = "SS output is enabled in master mode and when the SPI interface is enabled. The SPI\ninterface cannot work in a multimaster environment"]
    ENABLED,
}
impl SSOER {
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
            SSOER::DISABLED => false,
            SSOER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SSOER {
        match value {
            false => SSOER::DISABLED,
            true => SSOER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SSOER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SSOER::ENABLED
    }
}
#[doc = "Possible values of the field `NSSP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSSPR {
    #[doc = "No NSS pulse"]
    NOPULSE,
    #[doc = "NSS pulse generated"]
    PULSE,
}
impl NSSPR {
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
            NSSPR::NOPULSE => false,
            NSSPR::PULSE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NSSPR {
        match value {
            false => NSSPR::NOPULSE,
            true => NSSPR::PULSE,
        }
    }
    #[doc = "Checks if the value of the field is `NOPULSE`"]
    #[inline]
    pub fn is_no_pulse(&self) -> bool {
        *self == NSSPR::NOPULSE
    }
    #[doc = "Checks if the value of the field is `PULSE`"]
    #[inline]
    pub fn is_pulse(&self) -> bool {
        *self == NSSPR::PULSE
    }
}
#[doc = "Possible values of the field `FRF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRFR {
    #[doc = "SPI Motorola mode"]
    SPI_MOTO,
    #[doc = "SPI TI mode"]
    SPI_TI,
}
impl FRFR {
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
            FRFR::SPI_MOTO => false,
            FRFR::SPI_TI => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRFR {
        match value {
            false => FRFR::SPI_MOTO,
            true => FRFR::SPI_TI,
        }
    }
    #[doc = "Checks if the value of the field is `SPI_MOTO`"]
    #[inline]
    pub fn is_spi_moto(&self) -> bool {
        *self == FRFR::SPI_MOTO
    }
    #[doc = "Checks if the value of the field is `SPI_TI`"]
    #[inline]
    pub fn is_spi_ti(&self) -> bool {
        *self == FRFR::SPI_TI
    }
}
#[doc = "Possible values of the field `ERRIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRIER {
    #[doc = "Error interrupt is masked"]
    DISABLED,
    #[doc = "Error interrupt is enabled"]
    ENABLED,
}
impl ERRIER {
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
            ERRIER::DISABLED => false,
            ERRIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERRIER {
        match value {
            false => ERRIER::DISABLED,
            true => ERRIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ERRIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ERRIER::ENABLED
    }
}
#[doc = "Possible values of the field `RXNEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXNEIER {
    #[doc = "RXNE interrupt masked"]
    DISABLED,
    #[doc = "RXNE interrupt not masked. Used to generate an interrupt request when the RXNE flag is set."]
    ENABLED,
}
impl RXNEIER {
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
            RXNEIER::DISABLED => false,
            RXNEIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXNEIER {
        match value {
            false => RXNEIER::DISABLED,
            true => RXNEIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RXNEIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RXNEIER::ENABLED
    }
}
#[doc = "Possible values of the field `TXEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEIER {
    #[doc = "TXE interrupt masked"]
    DISABLED,
    #[doc = "TXE interrupt not masked. Used to generate an interrupt request when the TXE flag is set."]
    ENABLED,
}
impl TXEIER {
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
            TXEIER::DISABLED => false,
            TXEIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXEIER {
        match value {
            false => TXEIER::DISABLED,
            true => TXEIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TXEIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TXEIER::ENABLED
    }
}
#[doc = "Possible values of the field `DS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSR {
    #[doc = "4-bits"]
    _4BITS,
    #[doc = "5-bits"]
    _5BITS,
    #[doc = "6-bits"]
    _6BITS,
    #[doc = "7-bits"]
    _7BITS,
    #[doc = "8-bits"]
    _8BITS,
    #[doc = "9-bits"]
    _9BITS,
    #[doc = "10-bits"]
    _10BITS,
    #[doc = "11-bits"]
    _11BITS,
    #[doc = "12-bits"]
    _12BITS,
    #[doc = "13-bits"]
    _13BITS,
    #[doc = "14-bits"]
    _14BITS,
    #[doc = "15-bits"]
    _15BITS,
    #[doc = "16-bits"]
    _16BITS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DSR::_4BITS => 3,
            DSR::_5BITS => 4,
            DSR::_6BITS => 5,
            DSR::_7BITS => 6,
            DSR::_8BITS => 7,
            DSR::_9BITS => 8,
            DSR::_10BITS => 9,
            DSR::_11BITS => 10,
            DSR::_12BITS => 11,
            DSR::_13BITS => 12,
            DSR::_14BITS => 13,
            DSR::_15BITS => 14,
            DSR::_16BITS => 15,
            DSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DSR {
        match value {
            3 => DSR::_4BITS,
            4 => DSR::_5BITS,
            5 => DSR::_6BITS,
            6 => DSR::_7BITS,
            7 => DSR::_8BITS,
            8 => DSR::_9BITS,
            9 => DSR::_10BITS,
            10 => DSR::_11BITS,
            11 => DSR::_12BITS,
            12 => DSR::_13BITS,
            13 => DSR::_14BITS,
            14 => DSR::_15BITS,
            15 => DSR::_16BITS,
            i => DSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_4BITS`"]
    #[inline]
    pub fn is_4bits(&self) -> bool {
        *self == DSR::_4BITS
    }
    #[doc = "Checks if the value of the field is `_5BITS`"]
    #[inline]
    pub fn is_5bits(&self) -> bool {
        *self == DSR::_5BITS
    }
    #[doc = "Checks if the value of the field is `_6BITS`"]
    #[inline]
    pub fn is_6bits(&self) -> bool {
        *self == DSR::_6BITS
    }
    #[doc = "Checks if the value of the field is `_7BITS`"]
    #[inline]
    pub fn is_7bits(&self) -> bool {
        *self == DSR::_7BITS
    }
    #[doc = "Checks if the value of the field is `_8BITS`"]
    #[inline]
    pub fn is_8bits(&self) -> bool {
        *self == DSR::_8BITS
    }
    #[doc = "Checks if the value of the field is `_9BITS`"]
    #[inline]
    pub fn is_9bits(&self) -> bool {
        *self == DSR::_9BITS
    }
    #[doc = "Checks if the value of the field is `_10BITS`"]
    #[inline]
    pub fn is_10bits(&self) -> bool {
        *self == DSR::_10BITS
    }
    #[doc = "Checks if the value of the field is `_11BITS`"]
    #[inline]
    pub fn is_11bits(&self) -> bool {
        *self == DSR::_11BITS
    }
    #[doc = "Checks if the value of the field is `_12BITS`"]
    #[inline]
    pub fn is_12bits(&self) -> bool {
        *self == DSR::_12BITS
    }
    #[doc = "Checks if the value of the field is `_13BITS`"]
    #[inline]
    pub fn is_13bits(&self) -> bool {
        *self == DSR::_13BITS
    }
    #[doc = "Checks if the value of the field is `_14BITS`"]
    #[inline]
    pub fn is_14bits(&self) -> bool {
        *self == DSR::_14BITS
    }
    #[doc = "Checks if the value of the field is `_15BITS`"]
    #[inline]
    pub fn is_15bits(&self) -> bool {
        *self == DSR::_15BITS
    }
    #[doc = "Checks if the value of the field is `_16BITS`"]
    #[inline]
    pub fn is_16bits(&self) -> bool {
        *self == DSR::_16BITS
    }
}
#[doc = "Possible values of the field `FRXTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRXTHR {
    #[doc = "RXNE event is generated if the FIFO level is greater than or equal to 1/2 (16-bit)"]
    _16BITS,
    #[doc = "RXNE event is generated if the FIFO level is greater than or equal to 1/4 (8-bit)"]
    _8BITS,
}
impl FRXTHR {
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
            FRXTHR::_16BITS => false,
            FRXTHR::_8BITS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRXTHR {
        match value {
            false => FRXTHR::_16BITS,
            true => FRXTHR::_8BITS,
        }
    }
    #[doc = "Checks if the value of the field is `_16BITS`"]
    #[inline]
    pub fn is_16bits(&self) -> bool {
        *self == FRXTHR::_16BITS
    }
    #[doc = "Checks if the value of the field is `_8BITS`"]
    #[inline]
    pub fn is_8bits(&self) -> bool {
        *self == FRXTHR::_8BITS
    }
}
#[doc = "Possible values of the field `LDMA_RX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDMA_RXR {
    #[doc = "Number of data to receive is even"]
    EVEN,
    #[doc = "Number of data to receive is odd"]
    ODD,
}
impl LDMA_RXR {
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
            LDMA_RXR::EVEN => false,
            LDMA_RXR::ODD => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LDMA_RXR {
        match value {
            false => LDMA_RXR::EVEN,
            true => LDMA_RXR::ODD,
        }
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline]
    pub fn is_even(&self) -> bool {
        *self == LDMA_RXR::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline]
    pub fn is_odd(&self) -> bool {
        *self == LDMA_RXR::ODD
    }
}
#[doc = "Possible values of the field `LDMA_TX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDMA_TXR {
    #[doc = "Number of data to transfer is even"]
    EVEN,
    #[doc = "Number of data to transfer is odd"]
    ODD,
}
impl LDMA_TXR {
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
            LDMA_TXR::EVEN => false,
            LDMA_TXR::ODD => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LDMA_TXR {
        match value {
            false => LDMA_TXR::EVEN,
            true => LDMA_TXR::ODD,
        }
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline]
    pub fn is_even(&self) -> bool {
        *self == LDMA_TXR::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline]
    pub fn is_odd(&self) -> bool {
        *self == LDMA_TXR::ODD
    }
}
#[doc = "Values that can be written to the field `RXDMAEN`"]
pub enum RXDMAENW {
    #[doc = "Rx buffer DMA disabled"]
    DISABLED,
    #[doc = "Rx buffer DMA enabled"]
    ENABLED,
}
impl RXDMAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXDMAENW::DISABLED => false,
            RXDMAENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXDMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXDMAENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXDMAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Rx buffer DMA disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXDMAENW::DISABLED)
    }
    #[doc = "Rx buffer DMA enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXDMAENW::ENABLED)
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
#[doc = "Values that can be written to the field `TXDMAEN`"]
pub enum TXDMAENW {
    #[doc = "Tx buffer DMA disabled"]
    DISABLED,
    #[doc = "Rx buffer DMA enabled"]
    ENABLED,
}
impl TXDMAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXDMAENW::DISABLED => false,
            TXDMAENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXDMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDMAENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXDMAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Tx buffer DMA disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXDMAENW::DISABLED)
    }
    #[doc = "Rx buffer DMA enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXDMAENW::ENABLED)
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
#[doc = "Values that can be written to the field `SSOE`"]
pub enum SSOEW {
    #[doc = "SS output is disabled in master mode and the SPI interface can work in multimaster\nconfiguration"]
    DISABLED,
    #[doc = "SS output is enabled in master mode and when the SPI interface is enabled. The SPI\ninterface cannot work in a multimaster environment"]
    ENABLED,
}
impl SSOEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SSOEW::DISABLED => false,
            SSOEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSOEW<'a> {
    w: &'a mut W,
}
impl<'a> _SSOEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSOEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SS output is disabled in master mode and the SPI interface can work in multimaster configuration"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSOEW::DISABLED)
    }
    #[doc = "SS output is enabled in master mode and when the SPI interface is enabled. The SPI interface cannot work in a multimaster environment"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSOEW::ENABLED)
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
#[doc = "Values that can be written to the field `NSSP`"]
pub enum NSSPW {
    #[doc = "No NSS pulse"]
    NOPULSE,
    #[doc = "NSS pulse generated"]
    PULSE,
}
impl NSSPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NSSPW::NOPULSE => false,
            NSSPW::PULSE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NSSPW<'a> {
    w: &'a mut W,
}
impl<'a> _NSSPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NSSPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No NSS pulse"]
    #[inline]
    pub fn no_pulse(self) -> &'a mut W {
        self.variant(NSSPW::NOPULSE)
    }
    #[doc = "NSS pulse generated"]
    #[inline]
    pub fn pulse(self) -> &'a mut W {
        self.variant(NSSPW::PULSE)
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
#[doc = "Values that can be written to the field `FRF`"]
pub enum FRFW {
    #[doc = "SPI Motorola mode"]
    SPI_MOTO,
    #[doc = "SPI TI mode"]
    SPI_TI,
}
impl FRFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRFW::SPI_MOTO => false,
            FRFW::SPI_TI => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRFW<'a> {
    w: &'a mut W,
}
impl<'a> _FRFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SPI Motorola mode"]
    #[inline]
    pub fn spi_moto(self) -> &'a mut W {
        self.variant(FRFW::SPI_MOTO)
    }
    #[doc = "SPI TI mode"]
    #[inline]
    pub fn spi_ti(self) -> &'a mut W {
        self.variant(FRFW::SPI_TI)
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
#[doc = "Values that can be written to the field `ERRIE`"]
pub enum ERRIEW {
    #[doc = "Error interrupt is masked"]
    DISABLED,
    #[doc = "Error interrupt is enabled"]
    ENABLED,
}
impl ERRIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERRIEW::DISABLED => false,
            ERRIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERRIEW<'a> {
    w: &'a mut W,
}
impl<'a> _ERRIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERRIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Error interrupt is masked"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ERRIEW::DISABLED)
    }
    #[doc = "Error interrupt is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ERRIEW::ENABLED)
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
#[doc = "Values that can be written to the field `RXNEIE`"]
pub enum RXNEIEW {
    #[doc = "RXNE interrupt masked"]
    DISABLED,
    #[doc = "RXNE interrupt not masked. Used to generate an interrupt request when the RXNE flag is set."]
    ENABLED,
}
impl RXNEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXNEIEW::DISABLED => false,
            RXNEIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXNEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXNEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXNEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RXNE interrupt masked"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXNEIEW::DISABLED)
    }
    #[doc = "RXNE interrupt not masked. Used to generate an interrupt request when the RXNE flag is set."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXNEIEW::ENABLED)
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
#[doc = "Values that can be written to the field `TXEIE`"]
pub enum TXEIEW {
    #[doc = "TXE interrupt masked"]
    DISABLED,
    #[doc = "TXE interrupt not masked. Used to generate an interrupt request when the TXE flag is set."]
    ENABLED,
}
impl TXEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXEIEW::DISABLED => false,
            TXEIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TXE interrupt masked"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXEIEW::DISABLED)
    }
    #[doc = "TXE interrupt not masked. Used to generate an interrupt request when the TXE flag is set."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXEIEW::ENABLED)
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
#[doc = "Values that can be written to the field `DS`"]
pub enum DSW {
    #[doc = "4-bits"]
    _4BITS,
    #[doc = "5-bits"]
    _5BITS,
    #[doc = "6-bits"]
    _6BITS,
    #[doc = "7-bits"]
    _7BITS,
    #[doc = "8-bits"]
    _8BITS,
    #[doc = "9-bits"]
    _9BITS,
    #[doc = "10-bits"]
    _10BITS,
    #[doc = "11-bits"]
    _11BITS,
    #[doc = "12-bits"]
    _12BITS,
    #[doc = "13-bits"]
    _13BITS,
    #[doc = "14-bits"]
    _14BITS,
    #[doc = "15-bits"]
    _15BITS,
    #[doc = "16-bits"]
    _16BITS,
}
impl DSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DSW::_4BITS => 3,
            DSW::_5BITS => 4,
            DSW::_6BITS => 5,
            DSW::_7BITS => 6,
            DSW::_8BITS => 7,
            DSW::_9BITS => 8,
            DSW::_10BITS => 9,
            DSW::_11BITS => 10,
            DSW::_12BITS => 11,
            DSW::_13BITS => 12,
            DSW::_14BITS => 13,
            DSW::_15BITS => 14,
            DSW::_16BITS => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DSW<'a> {
    w: &'a mut W,
}
impl<'a> _DSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "4-bits"]
    #[inline]
    pub fn _4bits(self) -> &'a mut W {
        self.variant(DSW::_4BITS)
    }
    #[doc = "5-bits"]
    #[inline]
    pub fn _5bits(self) -> &'a mut W {
        self.variant(DSW::_5BITS)
    }
    #[doc = "6-bits"]
    #[inline]
    pub fn _6bits(self) -> &'a mut W {
        self.variant(DSW::_6BITS)
    }
    #[doc = "7-bits"]
    #[inline]
    pub fn _7bits(self) -> &'a mut W {
        self.variant(DSW::_7BITS)
    }
    #[doc = "8-bits"]
    #[inline]
    pub fn _8bits(self) -> &'a mut W {
        self.variant(DSW::_8BITS)
    }
    #[doc = "9-bits"]
    #[inline]
    pub fn _9bits(self) -> &'a mut W {
        self.variant(DSW::_9BITS)
    }
    #[doc = "10-bits"]
    #[inline]
    pub fn _10bits(self) -> &'a mut W {
        self.variant(DSW::_10BITS)
    }
    #[doc = "11-bits"]
    #[inline]
    pub fn _11bits(self) -> &'a mut W {
        self.variant(DSW::_11BITS)
    }
    #[doc = "12-bits"]
    #[inline]
    pub fn _12bits(self) -> &'a mut W {
        self.variant(DSW::_12BITS)
    }
    #[doc = "13-bits"]
    #[inline]
    pub fn _13bits(self) -> &'a mut W {
        self.variant(DSW::_13BITS)
    }
    #[doc = "14-bits"]
    #[inline]
    pub fn _14bits(self) -> &'a mut W {
        self.variant(DSW::_14BITS)
    }
    #[doc = "15-bits"]
    #[inline]
    pub fn _15bits(self) -> &'a mut W {
        self.variant(DSW::_15BITS)
    }
    #[doc = "16-bits"]
    #[inline]
    pub fn _16bits(self) -> &'a mut W {
        self.variant(DSW::_16BITS)
    }
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
#[doc = "Values that can be written to the field `FRXTH`"]
pub enum FRXTHW {
    #[doc = "RXNE event is generated if the FIFO level is greater than or equal to 1/2 (16-bit)"]
    _16BITS,
    #[doc = "RXNE event is generated if the FIFO level is greater than or equal to 1/4 (8-bit)"]
    _8BITS,
}
impl FRXTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRXTHW::_16BITS => false,
            FRXTHW::_8BITS => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRXTHW<'a> {
    w: &'a mut W,
}
impl<'a> _FRXTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRXTHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RXNE event is generated if the FIFO level is greater than or equal to 1/2 (16-bit)"]
    #[inline]
    pub fn _16bits(self) -> &'a mut W {
        self.variant(FRXTHW::_16BITS)
    }
    #[doc = "RXNE event is generated if the FIFO level is greater than or equal to 1/4 (8-bit)"]
    #[inline]
    pub fn _8bits(self) -> &'a mut W {
        self.variant(FRXTHW::_8BITS)
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
#[doc = "Values that can be written to the field `LDMA_RX`"]
pub enum LDMA_RXW {
    #[doc = "Number of data to receive is even"]
    EVEN,
    #[doc = "Number of data to receive is odd"]
    ODD,
}
impl LDMA_RXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LDMA_RXW::EVEN => false,
            LDMA_RXW::ODD => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LDMA_RXW<'a> {
    w: &'a mut W,
}
impl<'a> _LDMA_RXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LDMA_RXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Number of data to receive is even"]
    #[inline]
    pub fn even(self) -> &'a mut W {
        self.variant(LDMA_RXW::EVEN)
    }
    #[doc = "Number of data to receive is odd"]
    #[inline]
    pub fn odd(self) -> &'a mut W {
        self.variant(LDMA_RXW::ODD)
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
#[doc = "Values that can be written to the field `LDMA_TX`"]
pub enum LDMA_TXW {
    #[doc = "Number of data to transfer is even"]
    EVEN,
    #[doc = "Number of data to transfer is odd"]
    ODD,
}
impl LDMA_TXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LDMA_TXW::EVEN => false,
            LDMA_TXW::ODD => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LDMA_TXW<'a> {
    w: &'a mut W,
}
impl<'a> _LDMA_TXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LDMA_TXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Number of data to transfer is even"]
    #[inline]
    pub fn even(self) -> &'a mut W {
        self.variant(LDMA_TXW::EVEN)
    }
    #[doc = "Number of data to transfer is odd"]
    #[inline]
    pub fn odd(self) -> &'a mut W {
        self.variant(LDMA_TXW::ODD)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline]
    pub fn rxdmaen(&self) -> RXDMAENR {
        RXDMAENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Tx buffer DMA enable"]
    #[inline]
    pub fn txdmaen(&self) -> TXDMAENR {
        TXDMAENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - SS output enable"]
    #[inline]
    pub fn ssoe(&self) -> SSOER {
        SSOER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - NSS pulse management"]
    #[inline]
    pub fn nssp(&self) -> NSSPR {
        NSSPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Frame format"]
    #[inline]
    pub fn frf(&self) -> FRFR {
        FRFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline]
    pub fn errie(&self) -> ERRIER {
        ERRIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - RX buffer not empty interrupt enable"]
    #[inline]
    pub fn rxneie(&self) -> RXNEIER {
        RXNEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Tx buffer empty interrupt enable"]
    #[inline]
    pub fn txeie(&self) -> TXEIER {
        TXEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:11 - Data size"]
    #[inline]
    pub fn ds(&self) -> DSR {
        DSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - FIFO reception threshold"]
    #[inline]
    pub fn frxth(&self) -> FRXTHR {
        FRXTHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Last DMA transfer for reception"]
    #[inline]
    pub fn ldma_rx(&self) -> LDMA_RXR {
        LDMA_RXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Last DMA transfer for transmission"]
    #[inline]
    pub fn ldma_tx(&self) -> LDMA_TXR {
        LDMA_TXR::_from({
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
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline]
    pub fn rxdmaen(&mut self) -> _RXDMAENW {
        _RXDMAENW { w: self }
    }
    #[doc = "Bit 1 - Tx buffer DMA enable"]
    #[inline]
    pub fn txdmaen(&mut self) -> _TXDMAENW {
        _TXDMAENW { w: self }
    }
    #[doc = "Bit 2 - SS output enable"]
    #[inline]
    pub fn ssoe(&mut self) -> _SSOEW {
        _SSOEW { w: self }
    }
    #[doc = "Bit 3 - NSS pulse management"]
    #[inline]
    pub fn nssp(&mut self) -> _NSSPW {
        _NSSPW { w: self }
    }
    #[doc = "Bit 4 - Frame format"]
    #[inline]
    pub fn frf(&mut self) -> _FRFW {
        _FRFW { w: self }
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline]
    pub fn errie(&mut self) -> _ERRIEW {
        _ERRIEW { w: self }
    }
    #[doc = "Bit 6 - RX buffer not empty interrupt enable"]
    #[inline]
    pub fn rxneie(&mut self) -> _RXNEIEW {
        _RXNEIEW { w: self }
    }
    #[doc = "Bit 7 - Tx buffer empty interrupt enable"]
    #[inline]
    pub fn txeie(&mut self) -> _TXEIEW {
        _TXEIEW { w: self }
    }
    #[doc = "Bits 8:11 - Data size"]
    #[inline]
    pub fn ds(&mut self) -> _DSW {
        _DSW { w: self }
    }
    #[doc = "Bit 12 - FIFO reception threshold"]
    #[inline]
    pub fn frxth(&mut self) -> _FRXTHW {
        _FRXTHW { w: self }
    }
    #[doc = "Bit 13 - Last DMA transfer for reception"]
    #[inline]
    pub fn ldma_rx(&mut self) -> _LDMA_RXW {
        _LDMA_RXW { w: self }
    }
    #[doc = "Bit 14 - Last DMA transfer for transmission"]
    #[inline]
    pub fn ldma_tx(&mut self) -> _LDMA_TXW {
        _LDMA_TXW { w: self }
    }
}
