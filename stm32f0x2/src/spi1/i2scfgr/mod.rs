#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::I2SCFGR {
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
#[doc = "Possible values of the field `I2SMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2SMODR {
    #[doc = "SPI mode is selected"] SPI,
    #[doc = "I2S mode is selected"] I2S,
}
impl I2SMODR {
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
            I2SMODR::SPI => false,
            I2SMODR::I2S => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> I2SMODR {
        match value {
            false => I2SMODR::SPI,
            true => I2SMODR::I2S,
        }
    }
    #[doc = "Checks if the value of the field is `SPI`"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == I2SMODR::SPI
    }
    #[doc = "Checks if the value of the field is `I2S`"]
    #[inline(always)]
    pub fn is_i2s(&self) -> bool {
        *self == I2SMODR::I2S
    }
}
#[doc = "Possible values of the field `I2SE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2SER {
    #[doc = "I2S peripheral is disabled"] DISABLE,
    #[doc = "I2S peripheral is enabled"] ENABLE,
}
impl I2SER {
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
            I2SER::DISABLE => false,
            I2SER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> I2SER {
        match value {
            false => I2SER::DISABLE,
            true => I2SER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == I2SER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == I2SER::ENABLE
    }
}
#[doc = "Possible values of the field `I2SCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2SCFGR {
    #[doc = "Slave - transmit"] SLAVE_TRANSMIT,
    #[doc = "Slave - receive"] SLAVE_RECEIVE,
    #[doc = "Master - transmit"] MASTER_TRANSMIT,
    #[doc = "Master - receive"] MASTER_RECEIVE,
}
impl I2SCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            I2SCFGR::SLAVE_TRANSMIT => 0,
            I2SCFGR::SLAVE_RECEIVE => 1,
            I2SCFGR::MASTER_TRANSMIT => 2,
            I2SCFGR::MASTER_RECEIVE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> I2SCFGR {
        match value {
            0 => I2SCFGR::SLAVE_TRANSMIT,
            1 => I2SCFGR::SLAVE_RECEIVE,
            2 => I2SCFGR::MASTER_TRANSMIT,
            3 => I2SCFGR::MASTER_RECEIVE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE_TRANSMIT`"]
    #[inline(always)]
    pub fn is_slave_transmit(&self) -> bool {
        *self == I2SCFGR::SLAVE_TRANSMIT
    }
    #[doc = "Checks if the value of the field is `SLAVE_RECEIVE`"]
    #[inline(always)]
    pub fn is_slave_receive(&self) -> bool {
        *self == I2SCFGR::SLAVE_RECEIVE
    }
    #[doc = "Checks if the value of the field is `MASTER_TRANSMIT`"]
    #[inline(always)]
    pub fn is_master_transmit(&self) -> bool {
        *self == I2SCFGR::MASTER_TRANSMIT
    }
    #[doc = "Checks if the value of the field is `MASTER_RECEIVE`"]
    #[inline(always)]
    pub fn is_master_receive(&self) -> bool {
        *self == I2SCFGR::MASTER_RECEIVE
    }
}
#[doc = "Possible values of the field `PCMSYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCMSYNCR {
    #[doc = "Short frame synchronization"] SHORT,
    #[doc = "Long frame synchronization"] LONG,
}
impl PCMSYNCR {
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
            PCMSYNCR::SHORT => false,
            PCMSYNCR::LONG => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> PCMSYNCR {
        match value {
            false => PCMSYNCR::SHORT,
            true => PCMSYNCR::LONG,
        }
    }
    #[doc = "Checks if the value of the field is `SHORT`"]
    #[inline(always)]
    pub fn is_short(&self) -> bool {
        *self == PCMSYNCR::SHORT
    }
    #[doc = "Checks if the value of the field is `LONG`"]
    #[inline(always)]
    pub fn is_long(&self) -> bool {
        *self == PCMSYNCR::LONG
    }
}
#[doc = "Possible values of the field `I2SSTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2SSTDR {
    #[doc = "I2S Philips standard."] I2S,
    #[doc = "MSB justified standard (left justified)"] MSB,
    #[doc = "LSB justified standard (right justified)"] LSB,
    #[doc = "PCM standard"] PCM,
}
impl I2SSTDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            I2SSTDR::I2S => 0,
            I2SSTDR::MSB => 1,
            I2SSTDR::LSB => 2,
            I2SSTDR::PCM => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> I2SSTDR {
        match value {
            0 => I2SSTDR::I2S,
            1 => I2SSTDR::MSB,
            2 => I2SSTDR::LSB,
            3 => I2SSTDR::PCM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `I2S`"]
    #[inline(always)]
    pub fn is_i2s(&self) -> bool {
        *self == I2SSTDR::I2S
    }
    #[doc = "Checks if the value of the field is `MSB`"]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == I2SSTDR::MSB
    }
    #[doc = "Checks if the value of the field is `LSB`"]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == I2SSTDR::LSB
    }
    #[doc = "Checks if the value of the field is `PCM`"]
    #[inline(always)]
    pub fn is_pcm(&self) -> bool {
        *self == I2SSTDR::PCM
    }
}
#[doc = "Possible values of the field `CKPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKPOLR {
    #[doc = "I2S clock inactive state is low level"] IDLELOW,
    #[doc = "I2S clock inactive state is high level"] IDLEHIGH,
}
impl CKPOLR {
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
            CKPOLR::IDLELOW => false,
            CKPOLR::IDLEHIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> CKPOLR {
        match value {
            false => CKPOLR::IDLELOW,
            true => CKPOLR::IDLEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `IDLELOW`"]
    #[inline(always)]
    pub fn is_idle_low(&self) -> bool {
        *self == CKPOLR::IDLELOW
    }
    #[doc = "Checks if the value of the field is `IDLEHIGH`"]
    #[inline(always)]
    pub fn is_idle_high(&self) -> bool {
        *self == CKPOLR::IDLEHIGH
    }
}
#[doc = "Possible values of the field `DATLEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATLENR {
    #[doc = "16-bit data length"] _16BITS,
    #[doc = "24-bit data length"] _24BITS,
    #[doc = "32-bit data length"] _32BITS,
    #[doc = r" Reserved"] _Reserved(u8),
}
impl DATLENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            DATLENR::_16BITS => 0,
            DATLENR::_24BITS => 1,
            DATLENR::_32BITS => 2,
            DATLENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> DATLENR {
        match value {
            0 => DATLENR::_16BITS,
            1 => DATLENR::_24BITS,
            2 => DATLENR::_32BITS,
            i => DATLENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_16BITS`"]
    #[inline(always)]
    pub fn is_16bits(&self) -> bool {
        *self == DATLENR::_16BITS
    }
    #[doc = "Checks if the value of the field is `_24BITS`"]
    #[inline(always)]
    pub fn is_24bits(&self) -> bool {
        *self == DATLENR::_24BITS
    }
    #[doc = "Checks if the value of the field is `_32BITS`"]
    #[inline(always)]
    pub fn is_32bits(&self) -> bool {
        *self == DATLENR::_32BITS
    }
}
#[doc = "Possible values of the field `CHLEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHLENR {
    #[doc = "16-bit wide"] _16BITS,
    #[doc = "32-bit wide"] _32BITS,
}
impl CHLENR {
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
            CHLENR::_16BITS => false,
            CHLENR::_32BITS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> CHLENR {
        match value {
            false => CHLENR::_16BITS,
            true => CHLENR::_32BITS,
        }
    }
    #[doc = "Checks if the value of the field is `_16BITS`"]
    #[inline(always)]
    pub fn is_16bits(&self) -> bool {
        *self == CHLENR::_16BITS
    }
    #[doc = "Checks if the value of the field is `_32BITS`"]
    #[inline(always)]
    pub fn is_32bits(&self) -> bool {
        *self == CHLENR::_32BITS
    }
}
#[doc = "Values that can be written to the field `I2SMOD`"]
pub enum I2SMODW {
    #[doc = "SPI mode is selected"] SPI,
    #[doc = "I2S mode is selected"] I2S,
}
impl I2SMODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            I2SMODW::SPI => false,
            I2SMODW::I2S => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2SMODW<'a> {
    w: &'a mut W,
}
impl<'a> _I2SMODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2SMODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SPI mode is selected"]
    #[inline(always)]
    pub fn spi(self) -> &'a mut W {
        self.variant(I2SMODW::SPI)
    }
    #[doc = "I2S mode is selected"]
    #[inline(always)]
    pub fn i2s(self) -> &'a mut W {
        self.variant(I2SMODW::I2S)
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
#[doc = "Values that can be written to the field `I2SE`"]
pub enum I2SEW {
    #[doc = "I2S peripheral is disabled"] DISABLE,
    #[doc = "I2S peripheral is enabled"] ENABLE,
}
impl I2SEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            I2SEW::DISABLE => false,
            I2SEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2SEW<'a> {
    w: &'a mut W,
}
impl<'a> _I2SEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2SEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "I2S peripheral is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(I2SEW::DISABLE)
    }
    #[doc = "I2S peripheral is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(I2SEW::ENABLE)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `I2SCFG`"]
pub enum I2SCFGW {
    #[doc = "Slave - transmit"] SLAVE_TRANSMIT,
    #[doc = "Slave - receive"] SLAVE_RECEIVE,
    #[doc = "Master - transmit"] MASTER_TRANSMIT,
    #[doc = "Master - receive"] MASTER_RECEIVE,
}
impl I2SCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            I2SCFGW::SLAVE_TRANSMIT => 0,
            I2SCFGW::SLAVE_RECEIVE => 1,
            I2SCFGW::MASTER_TRANSMIT => 2,
            I2SCFGW::MASTER_RECEIVE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2SCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _I2SCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2SCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Slave - transmit"]
    #[inline(always)]
    pub fn slave_transmit(self) -> &'a mut W {
        self.variant(I2SCFGW::SLAVE_TRANSMIT)
    }
    #[doc = "Slave - receive"]
    #[inline(always)]
    pub fn slave_receive(self) -> &'a mut W {
        self.variant(I2SCFGW::SLAVE_RECEIVE)
    }
    #[doc = "Master - transmit"]
    #[inline(always)]
    pub fn master_transmit(self) -> &'a mut W {
        self.variant(I2SCFGW::MASTER_TRANSMIT)
    }
    #[doc = "Master - receive"]
    #[inline(always)]
    pub fn master_receive(self) -> &'a mut W {
        self.variant(I2SCFGW::MASTER_RECEIVE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PCMSYNC`"]
pub enum PCMSYNCW {
    #[doc = "Short frame synchronization"] SHORT,
    #[doc = "Long frame synchronization"] LONG,
}
impl PCMSYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PCMSYNCW::SHORT => false,
            PCMSYNCW::LONG => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCMSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _PCMSYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCMSYNCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Short frame synchronization"]
    #[inline(always)]
    pub fn short(self) -> &'a mut W {
        self.variant(PCMSYNCW::SHORT)
    }
    #[doc = "Long frame synchronization"]
    #[inline(always)]
    pub fn long(self) -> &'a mut W {
        self.variant(PCMSYNCW::LONG)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `I2SSTD`"]
pub enum I2SSTDW {
    #[doc = "I2S Philips standard."] I2S,
    #[doc = "MSB justified standard (left justified)"] MSB,
    #[doc = "LSB justified standard (right justified)"] LSB,
    #[doc = "PCM standard"] PCM,
}
impl I2SSTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            I2SSTDW::I2S => 0,
            I2SSTDW::MSB => 1,
            I2SSTDW::LSB => 2,
            I2SSTDW::PCM => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2SSTDW<'a> {
    w: &'a mut W,
}
impl<'a> _I2SSTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2SSTDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "I2S Philips standard."]
    #[inline(always)]
    pub fn i2s(self) -> &'a mut W {
        self.variant(I2SSTDW::I2S)
    }
    #[doc = "MSB justified standard (left justified)"]
    #[inline(always)]
    pub fn msb(self) -> &'a mut W {
        self.variant(I2SSTDW::MSB)
    }
    #[doc = "LSB justified standard (right justified)"]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut W {
        self.variant(I2SSTDW::LSB)
    }
    #[doc = "PCM standard"]
    #[inline(always)]
    pub fn pcm(self) -> &'a mut W {
        self.variant(I2SSTDW::PCM)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CKPOL`"]
pub enum CKPOLW {
    #[doc = "I2S clock inactive state is low level"] IDLELOW,
    #[doc = "I2S clock inactive state is high level"] IDLEHIGH,
}
impl CKPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CKPOLW::IDLELOW => false,
            CKPOLW::IDLEHIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CKPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _CKPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "I2S clock inactive state is low level"]
    #[inline(always)]
    pub fn idle_low(self) -> &'a mut W {
        self.variant(CKPOLW::IDLELOW)
    }
    #[doc = "I2S clock inactive state is high level"]
    #[inline(always)]
    pub fn idle_high(self) -> &'a mut W {
        self.variant(CKPOLW::IDLEHIGH)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DATLEN`"]
pub enum DATLENW {
    #[doc = "16-bit data length"] _16BITS,
    #[doc = "24-bit data length"] _24BITS,
    #[doc = "32-bit data length"] _32BITS,
}
impl DATLENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            DATLENW::_16BITS => 0,
            DATLENW::_24BITS => 1,
            DATLENW::_32BITS => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATLENW<'a> {
    w: &'a mut W,
}
impl<'a> _DATLENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATLENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "16-bit data length"]
    #[inline(always)]
    pub fn _16bits(self) -> &'a mut W {
        self.variant(DATLENW::_16BITS)
    }
    #[doc = "24-bit data length"]
    #[inline(always)]
    pub fn _24bits(self) -> &'a mut W {
        self.variant(DATLENW::_24BITS)
    }
    #[doc = "32-bit data length"]
    #[inline(always)]
    pub fn _32bits(self) -> &'a mut W {
        self.variant(DATLENW::_32BITS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CHLEN`"]
pub enum CHLENW {
    #[doc = "16-bit wide"] _16BITS,
    #[doc = "32-bit wide"] _32BITS,
}
impl CHLENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CHLENW::_16BITS => false,
            CHLENW::_32BITS => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHLENW<'a> {
    w: &'a mut W,
}
impl<'a> _CHLENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHLENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "16-bit wide"]
    #[inline(always)]
    pub fn _16bits(self) -> &'a mut W {
        self.variant(CHLENW::_16BITS)
    }
    #[doc = "32-bit wide"]
    #[inline(always)]
    pub fn _32bits(self) -> &'a mut W {
        self.variant(CHLENW::_32BITS)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 11 - I2S mode selection"]
    #[inline(always)]
    pub fn i2smod(&self) -> I2SMODR {
        I2SMODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - I2S Enable"]
    #[inline(always)]
    pub fn i2se(&self) -> I2SER {
        I2SER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:9 - I2S configuration mode"]
    #[inline(always)]
    pub fn i2scfg(&self) -> I2SCFGR {
        I2SCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - PCM frame synchronization"]
    #[inline(always)]
    pub fn pcmsync(&self) -> PCMSYNCR {
        PCMSYNCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:5 - I2S standard selection"]
    #[inline(always)]
    pub fn i2sstd(&self) -> I2SSTDR {
        I2SSTDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Steady state clock polarity"]
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOLR {
        CKPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:2 - Data length to be transferred"]
    #[inline(always)]
    pub fn datlen(&self) -> DATLENR {
        DATLENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 0 - Channel length (number of bits per audio channel)"]
    #[inline(always)]
    pub fn chlen(&self) -> CHLENR {
        CHLENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
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
    #[doc = "Bit 11 - I2S mode selection"]
    #[inline(always)]
    pub fn i2smod(&mut self) -> _I2SMODW {
        _I2SMODW { w: self }
    }
    #[doc = "Bit 10 - I2S Enable"]
    #[inline(always)]
    pub fn i2se(&mut self) -> _I2SEW {
        _I2SEW { w: self }
    }
    #[doc = "Bits 8:9 - I2S configuration mode"]
    #[inline(always)]
    pub fn i2scfg(&mut self) -> _I2SCFGW {
        _I2SCFGW { w: self }
    }
    #[doc = "Bit 7 - PCM frame synchronization"]
    #[inline(always)]
    pub fn pcmsync(&mut self) -> _PCMSYNCW {
        _PCMSYNCW { w: self }
    }
    #[doc = "Bits 4:5 - I2S standard selection"]
    #[inline(always)]
    pub fn i2sstd(&mut self) -> _I2SSTDW {
        _I2SSTDW { w: self }
    }
    #[doc = "Bit 3 - Steady state clock polarity"]
    #[inline(always)]
    pub fn ckpol(&mut self) -> _CKPOLW {
        _CKPOLW { w: self }
    }
    #[doc = "Bits 1:2 - Data length to be transferred"]
    #[inline(always)]
    pub fn datlen(&mut self) -> _DATLENW {
        _DATLENW { w: self }
    }
    #[doc = "Bit 0 - Channel length (number of bits per audio channel)"]
    #[inline(always)]
    pub fn chlen(&mut self) -> _CHLENW {
        _CHLENW { w: self }
    }
}
