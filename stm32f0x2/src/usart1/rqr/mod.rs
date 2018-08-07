#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RQR {
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
#[doc = "Possible values of the field `TXFRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFRQR {
    #[doc = "sets the TXE flag."]
    SET,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl TXFRQR {
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
            TXFRQR::SET => true,
            TXFRQR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXFRQR {
        match value {
            true => TXFRQR::SET,
            i => TXFRQR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == TXFRQR::SET
    }
}
#[doc = "Possible values of the field `RXFRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFRQR {
    #[doc = "clears the RXNE flag."]
    CLEAR,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RXFRQR {
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
            RXFRQR::CLEAR => true,
            RXFRQR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXFRQR {
        match value {
            true => RXFRQR::CLEAR,
            i => RXFRQR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == RXFRQR::CLEAR
    }
}
#[doc = "Possible values of the field `MMRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMRQR {
    #[doc = "sets the RWU flag and puts the USART in mute mode."]
    SET,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl MMRQR {
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
            MMRQR::SET => true,
            MMRQR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MMRQR {
        match value {
            true => MMRQR::SET,
            i => MMRQR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == MMRQR::SET
    }
}
#[doc = "Possible values of the field `SBKRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBKRQR {
    #[doc = "sets the SBKF flag and request to send a BREAK on the line."]
    SET,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl SBKRQR {
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
            SBKRQR::SET => true,
            SBKRQR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SBKRQR {
        match value {
            true => SBKRQR::SET,
            i => SBKRQR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == SBKRQR::SET
    }
}
#[doc = "Possible values of the field `ABRRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABRRQR {
    #[doc = "resets the ABRF flag in the USART_ISR and request an automatic baud\nrate measurement on the next received data frame."]
    SET,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ABRRQR {
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
            ABRRQR::SET => true,
            ABRRQR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ABRRQR {
        match value {
            true => ABRRQR::SET,
            i => ABRRQR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == ABRRQR::SET
    }
}
#[doc = "Values that can be written to the field `TXFRQ`"]
pub enum TXFRQW {
    #[doc = "sets the TXE flag."]
    SET,
}
impl TXFRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXFRQW::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXFRQW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXFRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "sets the TXE flag."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(TXFRQW::SET)
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
#[doc = "Values that can be written to the field `RXFRQ`"]
pub enum RXFRQW {
    #[doc = "clears the RXNE flag."]
    CLEAR,
}
impl RXFRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXFRQW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXFRQW<'a> {
    w: &'a mut W,
}
impl<'a> _RXFRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXFRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "clears the RXNE flag."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(RXFRQW::CLEAR)
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
#[doc = "Values that can be written to the field `MMRQ`"]
pub enum MMRQW {
    #[doc = "sets the RWU flag and puts the USART in mute mode."]
    SET,
}
impl MMRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MMRQW::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MMRQW<'a> {
    w: &'a mut W,
}
impl<'a> _MMRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MMRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "sets the RWU flag and puts the USART in mute mode."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(MMRQW::SET)
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
#[doc = "Values that can be written to the field `SBKRQ`"]
pub enum SBKRQW {
    #[doc = "sets the SBKF flag and request to send a BREAK on the line."]
    SET,
}
impl SBKRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SBKRQW::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SBKRQW<'a> {
    w: &'a mut W,
}
impl<'a> _SBKRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SBKRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "sets the SBKF flag and request to send a BREAK on the line."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(SBKRQW::SET)
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
#[doc = "Values that can be written to the field `ABRRQ`"]
pub enum ABRRQW {
    #[doc = "resets the ABRF flag in the USART_ISR and request an automatic baud\nrate measurement on the next received data frame."]
    SET,
}
impl ABRRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ABRRQW::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ABRRQW<'a> {
    w: &'a mut W,
}
impl<'a> _ABRRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ABRRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "resets the ABRF flag in the USART_ISR and request an automatic baud rate measurement on the next received data frame."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(ABRRQW::SET)
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
    #[doc = "Bit 4 - Transmit data flush request"]
    #[inline]
    pub fn txfrq(&self) -> TXFRQR {
        TXFRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Receive data flush request"]
    #[inline]
    pub fn rxfrq(&self) -> RXFRQR {
        RXFRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Mute mode request"]
    #[inline]
    pub fn mmrq(&self) -> MMRQR {
        MMRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Send break request"]
    #[inline]
    pub fn sbkrq(&self) -> SBKRQR {
        SBKRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Auto baud rate request"]
    #[inline]
    pub fn abrrq(&self) -> ABRRQR {
        ABRRQR::_from({
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
    #[doc = "Bit 4 - Transmit data flush request"]
    #[inline]
    pub fn txfrq(&mut self) -> _TXFRQW {
        _TXFRQW { w: self }
    }
    #[doc = "Bit 3 - Receive data flush request"]
    #[inline]
    pub fn rxfrq(&mut self) -> _RXFRQW {
        _RXFRQW { w: self }
    }
    #[doc = "Bit 2 - Mute mode request"]
    #[inline]
    pub fn mmrq(&mut self) -> _MMRQW {
        _MMRQW { w: self }
    }
    #[doc = "Bit 1 - Send break request"]
    #[inline]
    pub fn sbkrq(&mut self) -> _SBKRQW {
        _SBKRQW { w: self }
    }
    #[doc = "Bit 0 - Auto baud rate request"]
    #[inline]
    pub fn abrrq(&mut self) -> _ABRRQW {
        _ABRRQW { w: self }
    }
}
