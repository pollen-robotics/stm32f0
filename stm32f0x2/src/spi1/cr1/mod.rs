#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR1 {
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
#[doc = "Possible values of the field `BIDIMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIDIMODER {
    #[doc = "2-line unidirectional data mode selected"] _2LINE,
    #[doc = "1-line bidirectional data mode selected"] _1LINE,
}
impl BIDIMODER {
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
            BIDIMODER::_2LINE => false,
            BIDIMODER::_1LINE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BIDIMODER {
        match value {
            false => BIDIMODER::_2LINE,
            true => BIDIMODER::_1LINE,
        }
    }
    #[doc = "Checks if the value of the field is `_2LINE`"]
    #[inline]
    pub fn is_2line(&self) -> bool {
        *self == BIDIMODER::_2LINE
    }
    #[doc = "Checks if the value of the field is `_1LINE`"]
    #[inline]
    pub fn is_1line(&self) -> bool {
        *self == BIDIMODER::_1LINE
    }
}
#[doc = "Possible values of the field `BIDIOE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIDIOER {
    #[doc = "Output disabled (receive-only mode)"] DISABLE,
    #[doc = "Output enabled (transmit-only mode)"] ENABLE,
}
impl BIDIOER {
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
            BIDIOER::DISABLE => false,
            BIDIOER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BIDIOER {
        match value {
            false => BIDIOER::DISABLE,
            true => BIDIOER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == BIDIOER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == BIDIOER::ENABLE
    }
}
#[doc = "Possible values of the field `CRCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCENR {
    #[doc = "CRC calculation disabled"] DISABLE,
    #[doc = "CRC calculation Enabled"] ENABLE,
}
impl CRCENR {
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
            CRCENR::DISABLE => false,
            CRCENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRCENR {
        match value {
            false => CRCENR::DISABLE,
            true => CRCENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CRCENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CRCENR::ENABLE
    }
}
#[doc = "Possible values of the field `CRCNEXT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCNEXTR {
    #[doc = "Next transmit value is from Tx buffer"] TX,
    #[doc = "Next transmit value is from Tx CRC register"] TXCRC,
}
impl CRCNEXTR {
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
            CRCNEXTR::TX => false,
            CRCNEXTR::TXCRC => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRCNEXTR {
        match value {
            false => CRCNEXTR::TX,
            true => CRCNEXTR::TXCRC,
        }
    }
    #[doc = "Checks if the value of the field is `TX`"]
    #[inline]
    pub fn is_tx(&self) -> bool {
        *self == CRCNEXTR::TX
    }
    #[doc = "Checks if the value of the field is `TXCRC`"]
    #[inline]
    pub fn is_tx_crc(&self) -> bool {
        *self == CRCNEXTR::TXCRC
    }
}
#[doc = "Possible values of the field `DFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFFR {
    #[doc = "8-bit CRC length"] _8BITS,
    #[doc = "16-bit CRC length"] _16BITS,
}
impl DFFR {
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
            DFFR::_8BITS => false,
            DFFR::_16BITS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFFR {
        match value {
            false => DFFR::_8BITS,
            true => DFFR::_16BITS,
        }
    }
    #[doc = "Checks if the value of the field is `_8BITS`"]
    #[inline]
    pub fn is_8bits(&self) -> bool {
        *self == DFFR::_8BITS
    }
    #[doc = "Checks if the value of the field is `_16BITS`"]
    #[inline]
    pub fn is_16bits(&self) -> bool {
        *self == DFFR::_16BITS
    }
}
#[doc = "Possible values of the field `RXONLY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXONLYR {
    #[doc = "Full-duplex (Transmit and receive)"] FULLDUPLEX,
    #[doc = "Output disabled (Receive-only mode)"] RECEIVEONLY,
}
impl RXONLYR {
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
            RXONLYR::FULLDUPLEX => false,
            RXONLYR::RECEIVEONLY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXONLYR {
        match value {
            false => RXONLYR::FULLDUPLEX,
            true => RXONLYR::RECEIVEONLY,
        }
    }
    #[doc = "Checks if the value of the field is `FULLDUPLEX`"]
    #[inline]
    pub fn is_full_duplex(&self) -> bool {
        *self == RXONLYR::FULLDUPLEX
    }
    #[doc = "Checks if the value of the field is `RECEIVEONLY`"]
    #[inline]
    pub fn is_receive_only(&self) -> bool {
        *self == RXONLYR::RECEIVEONLY
    }
}
#[doc = "Possible values of the field `SSM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSMR {
    #[doc = "Software slave management disabled"] DISABLED,
    #[doc = "Software slave management enabled"] ENABLED,
}
impl SSMR {
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
            SSMR::DISABLED => false,
            SSMR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SSMR {
        match value {
            false => SSMR::DISABLED,
            true => SSMR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SSMR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SSMR::ENABLED
    }
}
#[doc = "Possible values of the field `SSI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSIR {
    #[doc = "NSS = 1"] NSSDISABLE,
    #[doc = "NSS = 0"] NSSENABLE,
}
impl SSIR {
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
            SSIR::NSSDISABLE => false,
            SSIR::NSSENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SSIR {
        match value {
            false => SSIR::NSSDISABLE,
            true => SSIR::NSSENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NSSDISABLE`"]
    #[inline]
    pub fn is_nssdisable(&self) -> bool {
        *self == SSIR::NSSDISABLE
    }
    #[doc = "Checks if the value of the field is `NSSENABLE`"]
    #[inline]
    pub fn is_nssenable(&self) -> bool {
        *self == SSIR::NSSENABLE
    }
}
#[doc = "Possible values of the field `LSBFIRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSBFIRSTR {
    #[doc = "data is transmitted / received with the MSB first"] MSBFIRST,
    #[doc = "data is transmitted / received with the LSB first"] LSBFIRST,
}
impl LSBFIRSTR {
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
            LSBFIRSTR::MSBFIRST => false,
            LSBFIRSTR::LSBFIRST => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LSBFIRSTR {
        match value {
            false => LSBFIRSTR::MSBFIRST,
            true => LSBFIRSTR::LSBFIRST,
        }
    }
    #[doc = "Checks if the value of the field is `MSBFIRST`"]
    #[inline]
    pub fn is_msbfirst(&self) -> bool {
        *self == LSBFIRSTR::MSBFIRST
    }
    #[doc = "Checks if the value of the field is `LSBFIRST`"]
    #[inline]
    pub fn is_lsbfirst(&self) -> bool {
        *self == LSBFIRSTR::LSBFIRST
    }
}
#[doc = "Possible values of the field `SPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPER {
    #[doc = "Peripheral disabled"] DISABLED,
    #[doc = "Peripheral enabled"] ENABLED,
}
impl SPER {
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
            SPER::DISABLED => false,
            SPER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPER {
        match value {
            false => SPER::DISABLED,
            true => SPER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SPER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SPER::ENABLED
    }
}
#[doc = "Possible values of the field `BR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRR {
    #[doc = "fPCLK/2"] DIV2,
    #[doc = "fPCLK/4"] DIV4,
    #[doc = "fPCLK/8"] DIV8,
    #[doc = "fPCLK/16"] DIV16,
    #[doc = "fPCLK/32"] DIV32,
    #[doc = "fPCLK/64"] DIV64,
    #[doc = "fPCLK/128"] DIV128,
    #[doc = "fPCLK/256"] DIV256,
}
impl BRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BRR::DIV2 => 0,
            BRR::DIV4 => 1,
            BRR::DIV8 => 2,
            BRR::DIV16 => 3,
            BRR::DIV32 => 4,
            BRR::DIV64 => 5,
            BRR::DIV128 => 6,
            BRR::DIV256 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BRR {
        match value {
            0 => BRR::DIV2,
            1 => BRR::DIV4,
            2 => BRR::DIV8,
            3 => BRR::DIV16,
            4 => BRR::DIV32,
            5 => BRR::DIV64,
            6 => BRR::DIV128,
            7 => BRR::DIV256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == BRR::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == BRR::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == BRR::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == BRR::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline]
    pub fn is_div32(&self) -> bool {
        *self == BRR::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == BRR::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline]
    pub fn is_div128(&self) -> bool {
        *self == BRR::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline]
    pub fn is_div256(&self) -> bool {
        *self == BRR::DIV256
    }
}
#[doc = "Possible values of the field `MSTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTRR {
    #[doc = "Slave Configuration"] SLAVE,
    #[doc = "Master Configuration"] MASTER,
}
impl MSTRR {
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
            MSTRR::SLAVE => false,
            MSTRR::MASTER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSTRR {
        match value {
            false => MSTRR::SLAVE,
            true => MSTRR::MASTER,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE`"]
    #[inline]
    pub fn is_slave(&self) -> bool {
        *self == MSTRR::SLAVE
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline]
    pub fn is_master(&self) -> bool {
        *self == MSTRR::MASTER
    }
}
#[doc = "Possible values of the field `CPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOLR {
    #[doc = "CK to 0 when idle"] IDLELOW,
    #[doc = "CK to 1 when idle"] IDLEHIGH,
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
            CPOLR::IDLELOW => false,
            CPOLR::IDLEHIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPOLR {
        match value {
            false => CPOLR::IDLELOW,
            true => CPOLR::IDLEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `IDLELOW`"]
    #[inline]
    pub fn is_idle_low(&self) -> bool {
        *self == CPOLR::IDLELOW
    }
    #[doc = "Checks if the value of the field is `IDLEHIGH`"]
    #[inline]
    pub fn is_idle_high(&self) -> bool {
        *self == CPOLR::IDLEHIGH
    }
}
#[doc = "Possible values of the field `CPHA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHAR {
    #[doc = "The first clock transition is the first data capture edge"] FIRSTEDGE,
    #[doc = "The second clock transition is the first data capture edge"] SECONDEDGE,
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
            CPHAR::FIRSTEDGE => false,
            CPHAR::SECONDEDGE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPHAR {
        match value {
            false => CPHAR::FIRSTEDGE,
            true => CPHAR::SECONDEDGE,
        }
    }
    #[doc = "Checks if the value of the field is `FIRSTEDGE`"]
    #[inline]
    pub fn is_first_edge(&self) -> bool {
        *self == CPHAR::FIRSTEDGE
    }
    #[doc = "Checks if the value of the field is `SECONDEDGE`"]
    #[inline]
    pub fn is_second_edge(&self) -> bool {
        *self == CPHAR::SECONDEDGE
    }
}
#[doc = "Values that can be written to the field `BIDIMODE`"]
pub enum BIDIMODEW {
    #[doc = "2-line unidirectional data mode selected"] _2LINE,
    #[doc = "1-line bidirectional data mode selected"] _1LINE,
}
impl BIDIMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BIDIMODEW::_2LINE => false,
            BIDIMODEW::_1LINE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BIDIMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _BIDIMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BIDIMODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "2-line unidirectional data mode selected"]
    #[inline]
    pub fn _2line(self) -> &'a mut W {
        self.variant(BIDIMODEW::_2LINE)
    }
    #[doc = "1-line bidirectional data mode selected"]
    #[inline]
    pub fn _1line(self) -> &'a mut W {
        self.variant(BIDIMODEW::_1LINE)
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
#[doc = "Values that can be written to the field `BIDIOE`"]
pub enum BIDIOEW {
    #[doc = "Output disabled (receive-only mode)"] DISABLE,
    #[doc = "Output enabled (transmit-only mode)"] ENABLE,
}
impl BIDIOEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BIDIOEW::DISABLE => false,
            BIDIOEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BIDIOEW<'a> {
    w: &'a mut W,
}
impl<'a> _BIDIOEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BIDIOEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Output disabled (receive-only mode)"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(BIDIOEW::DISABLE)
    }
    #[doc = "Output enabled (transmit-only mode)"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(BIDIOEW::ENABLE)
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
#[doc = "Values that can be written to the field `CRCEN`"]
pub enum CRCENW {
    #[doc = "CRC calculation disabled"] DISABLE,
    #[doc = "CRC calculation Enabled"] ENABLE,
}
impl CRCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRCENW::DISABLE => false,
            CRCENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRCENW<'a> {
    w: &'a mut W,
}
impl<'a> _CRCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CRC calculation disabled"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CRCENW::DISABLE)
    }
    #[doc = "CRC calculation Enabled"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CRCENW::ENABLE)
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
#[doc = "Values that can be written to the field `CRCNEXT`"]
pub enum CRCNEXTW {
    #[doc = "Next transmit value is from Tx buffer"] TX,
    #[doc = "Next transmit value is from Tx CRC register"] TXCRC,
}
impl CRCNEXTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRCNEXTW::TX => false,
            CRCNEXTW::TXCRC => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRCNEXTW<'a> {
    w: &'a mut W,
}
impl<'a> _CRCNEXTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRCNEXTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Next transmit value is from Tx buffer"]
    #[inline]
    pub fn tx(self) -> &'a mut W {
        self.variant(CRCNEXTW::TX)
    }
    #[doc = "Next transmit value is from Tx CRC register"]
    #[inline]
    pub fn tx_crc(self) -> &'a mut W {
        self.variant(CRCNEXTW::TXCRC)
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
#[doc = "Values that can be written to the field `DFF`"]
pub enum DFFW {
    #[doc = "8-bit CRC length"] _8BITS,
    #[doc = "16-bit CRC length"] _16BITS,
}
impl DFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFFW::_8BITS => false,
            DFFW::_16BITS => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFFW<'a> {
    w: &'a mut W,
}
impl<'a> _DFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "8-bit CRC length"]
    #[inline]
    pub fn _8bits(self) -> &'a mut W {
        self.variant(DFFW::_8BITS)
    }
    #[doc = "16-bit CRC length"]
    #[inline]
    pub fn _16bits(self) -> &'a mut W {
        self.variant(DFFW::_16BITS)
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
#[doc = "Values that can be written to the field `RXONLY`"]
pub enum RXONLYW {
    #[doc = "Full-duplex (Transmit and receive)"] FULLDUPLEX,
    #[doc = "Output disabled (Receive-only mode)"] RECEIVEONLY,
}
impl RXONLYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXONLYW::FULLDUPLEX => false,
            RXONLYW::RECEIVEONLY => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXONLYW<'a> {
    w: &'a mut W,
}
impl<'a> _RXONLYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXONLYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Full-duplex (Transmit and receive)"]
    #[inline]
    pub fn full_duplex(self) -> &'a mut W {
        self.variant(RXONLYW::FULLDUPLEX)
    }
    #[doc = "Output disabled (Receive-only mode)"]
    #[inline]
    pub fn receive_only(self) -> &'a mut W {
        self.variant(RXONLYW::RECEIVEONLY)
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
#[doc = "Values that can be written to the field `SSM`"]
pub enum SSMW {
    #[doc = "Software slave management disabled"] DISABLED,
    #[doc = "Software slave management enabled"] ENABLED,
}
impl SSMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SSMW::DISABLED => false,
            SSMW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSMW<'a> {
    w: &'a mut W,
}
impl<'a> _SSMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Software slave management disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSMW::DISABLED)
    }
    #[doc = "Software slave management enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSMW::ENABLED)
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
#[doc = "Values that can be written to the field `SSI`"]
pub enum SSIW {
    #[doc = "NSS = 1"] NSSDISABLE,
    #[doc = "NSS = 0"] NSSENABLE,
}
impl SSIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SSIW::NSSDISABLE => false,
            SSIW::NSSENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSIW<'a> {
    w: &'a mut W,
}
impl<'a> _SSIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "NSS = 1"]
    #[inline]
    pub fn nssdisable(self) -> &'a mut W {
        self.variant(SSIW::NSSDISABLE)
    }
    #[doc = "NSS = 0"]
    #[inline]
    pub fn nssenable(self) -> &'a mut W {
        self.variant(SSIW::NSSENABLE)
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
#[doc = "Values that can be written to the field `LSBFIRST`"]
pub enum LSBFIRSTW {
    #[doc = "data is transmitted / received with the MSB first"] MSBFIRST,
    #[doc = "data is transmitted / received with the LSB first"] LSBFIRST,
}
impl LSBFIRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LSBFIRSTW::MSBFIRST => false,
            LSBFIRSTW::LSBFIRST => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LSBFIRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _LSBFIRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LSBFIRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "data is transmitted / received with the MSB first"]
    #[inline]
    pub fn msbfirst(self) -> &'a mut W {
        self.variant(LSBFIRSTW::MSBFIRST)
    }
    #[doc = "data is transmitted / received with the LSB first"]
    #[inline]
    pub fn lsbfirst(self) -> &'a mut W {
        self.variant(LSBFIRSTW::LSBFIRST)
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
#[doc = "Values that can be written to the field `SPE`"]
pub enum SPEW {
    #[doc = "Peripheral disabled"] DISABLED,
    #[doc = "Peripheral enabled"] ENABLED,
}
impl SPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPEW::DISABLED => false,
            SPEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPEW<'a> {
    w: &'a mut W,
}
impl<'a> _SPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Peripheral disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SPEW::DISABLED)
    }
    #[doc = "Peripheral enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SPEW::ENABLED)
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
#[doc = "Values that can be written to the field `BR`"]
pub enum BRW {
    #[doc = "fPCLK/2"] DIV2,
    #[doc = "fPCLK/4"] DIV4,
    #[doc = "fPCLK/8"] DIV8,
    #[doc = "fPCLK/16"] DIV16,
    #[doc = "fPCLK/32"] DIV32,
    #[doc = "fPCLK/64"] DIV64,
    #[doc = "fPCLK/128"] DIV128,
    #[doc = "fPCLK/256"] DIV256,
}
impl BRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BRW::DIV2 => 0,
            BRW::DIV4 => 1,
            BRW::DIV8 => 2,
            BRW::DIV16 => 3,
            BRW::DIV32 => 4,
            BRW::DIV64 => 5,
            BRW::DIV128 => 6,
            BRW::DIV256 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BRW<'a> {
    w: &'a mut W,
}
impl<'a> _BRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BRW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "fPCLK/2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(BRW::DIV2)
    }
    #[doc = "fPCLK/4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(BRW::DIV4)
    }
    #[doc = "fPCLK/8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(BRW::DIV8)
    }
    #[doc = "fPCLK/16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(BRW::DIV16)
    }
    #[doc = "fPCLK/32"]
    #[inline]
    pub fn div32(self) -> &'a mut W {
        self.variant(BRW::DIV32)
    }
    #[doc = "fPCLK/64"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(BRW::DIV64)
    }
    #[doc = "fPCLK/128"]
    #[inline]
    pub fn div128(self) -> &'a mut W {
        self.variant(BRW::DIV128)
    }
    #[doc = "fPCLK/256"]
    #[inline]
    pub fn div256(self) -> &'a mut W {
        self.variant(BRW::DIV256)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MSTR`"]
pub enum MSTRW {
    #[doc = "Slave Configuration"] SLAVE,
    #[doc = "Master Configuration"] MASTER,
}
impl MSTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTRW::SLAVE => false,
            MSTRW::MASTER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSTRW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSTRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Slave Configuration"]
    #[inline]
    pub fn slave(self) -> &'a mut W {
        self.variant(MSTRW::SLAVE)
    }
    #[doc = "Master Configuration"]
    #[inline]
    pub fn master(self) -> &'a mut W {
        self.variant(MSTRW::MASTER)
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
#[doc = "Values that can be written to the field `CPOL`"]
pub enum CPOLW {
    #[doc = "CK to 0 when idle"] IDLELOW,
    #[doc = "CK to 1 when idle"] IDLEHIGH,
}
impl CPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPOLW::IDLELOW => false,
            CPOLW::IDLEHIGH => true,
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
    #[doc = "CK to 0 when idle"]
    #[inline]
    pub fn idle_low(self) -> &'a mut W {
        self.variant(CPOLW::IDLELOW)
    }
    #[doc = "CK to 1 when idle"]
    #[inline]
    pub fn idle_high(self) -> &'a mut W {
        self.variant(CPOLW::IDLEHIGH)
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
#[doc = "Values that can be written to the field `CPHA`"]
pub enum CPHAW {
    #[doc = "The first clock transition is the first data capture edge"] FIRSTEDGE,
    #[doc = "The second clock transition is the first data capture edge"] SECONDEDGE,
}
impl CPHAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPHAW::FIRSTEDGE => false,
            CPHAW::SECONDEDGE => true,
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
    #[doc = "The first clock transition is the first data capture edge"]
    #[inline]
    pub fn first_edge(self) -> &'a mut W {
        self.variant(CPHAW::FIRSTEDGE)
    }
    #[doc = "The second clock transition is the first data capture edge"]
    #[inline]
    pub fn second_edge(self) -> &'a mut W {
        self.variant(CPHAW::SECONDEDGE)
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
    #[doc = "Bit 15 - Bidirectional data mode enable"]
    #[inline]
    pub fn bidimode(&self) -> BIDIMODER {
        BIDIMODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Output enable in bidirectional mode"]
    #[inline]
    pub fn bidioe(&self) -> BIDIOER {
        BIDIOER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Hardware CRC calculation enable"]
    #[inline]
    pub fn crcen(&self) -> CRCENR {
        CRCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - CRC transfer next"]
    #[inline]
    pub fn crcnext(&self) -> CRCNEXTR {
        CRCNEXTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Data frame format"]
    #[inline]
    pub fn dff(&self) -> DFFR {
        DFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Receive only"]
    #[inline]
    pub fn rxonly(&self) -> RXONLYR {
        RXONLYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Software slave management"]
    #[inline]
    pub fn ssm(&self) -> SSMR {
        SSMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Internal slave select"]
    #[inline]
    pub fn ssi(&self) -> SSIR {
        SSIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Frame format"]
    #[inline]
    pub fn lsbfirst(&self) -> LSBFIRSTR {
        LSBFIRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - SPI enable"]
    #[inline]
    pub fn spe(&self) -> SPER {
        SPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 3:5 - Baud rate control"]
    #[inline]
    pub fn br(&self) -> BRR {
        BRR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Master selection"]
    #[inline]
    pub fn mstr(&self) -> MSTRR {
        MSTRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Clock polarity"]
    #[inline]
    pub fn cpol(&self) -> CPOLR {
        CPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Clock phase"]
    #[inline]
    pub fn cpha(&self) -> CPHAR {
        CPHAR::_from({
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
    #[doc = "Bit 15 - Bidirectional data mode enable"]
    #[inline]
    pub fn bidimode(&mut self) -> _BIDIMODEW {
        _BIDIMODEW { w: self }
    }
    #[doc = "Bit 14 - Output enable in bidirectional mode"]
    #[inline]
    pub fn bidioe(&mut self) -> _BIDIOEW {
        _BIDIOEW { w: self }
    }
    #[doc = "Bit 13 - Hardware CRC calculation enable"]
    #[inline]
    pub fn crcen(&mut self) -> _CRCENW {
        _CRCENW { w: self }
    }
    #[doc = "Bit 12 - CRC transfer next"]
    #[inline]
    pub fn crcnext(&mut self) -> _CRCNEXTW {
        _CRCNEXTW { w: self }
    }
    #[doc = "Bit 11 - Data frame format"]
    #[inline]
    pub fn dff(&mut self) -> _DFFW {
        _DFFW { w: self }
    }
    #[doc = "Bit 10 - Receive only"]
    #[inline]
    pub fn rxonly(&mut self) -> _RXONLYW {
        _RXONLYW { w: self }
    }
    #[doc = "Bit 9 - Software slave management"]
    #[inline]
    pub fn ssm(&mut self) -> _SSMW {
        _SSMW { w: self }
    }
    #[doc = "Bit 8 - Internal slave select"]
    #[inline]
    pub fn ssi(&mut self) -> _SSIW {
        _SSIW { w: self }
    }
    #[doc = "Bit 7 - Frame format"]
    #[inline]
    pub fn lsbfirst(&mut self) -> _LSBFIRSTW {
        _LSBFIRSTW { w: self }
    }
    #[doc = "Bit 6 - SPI enable"]
    #[inline]
    pub fn spe(&mut self) -> _SPEW {
        _SPEW { w: self }
    }
    #[doc = "Bits 3:5 - Baud rate control"]
    #[inline]
    pub fn br(&mut self) -> _BRW {
        _BRW { w: self }
    }
    #[doc = "Bit 2 - Master selection"]
    #[inline]
    pub fn mstr(&mut self) -> _MSTRW {
        _MSTRW { w: self }
    }
    #[doc = "Bit 1 - Clock polarity"]
    #[inline]
    pub fn cpol(&mut self) -> _CPOLW {
        _CPOLW { w: self }
    }
    #[doc = "Bit 0 - Clock phase"]
    #[inline]
    pub fn cpha(&mut self) -> _CPHAW {
        _CPHAW { w: self }
    }
}
