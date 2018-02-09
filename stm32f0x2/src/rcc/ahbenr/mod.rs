#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AHBENR {
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
#[doc = "Possible values of the field `DMA1EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA1ENR {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Enabled."]
    ENABLED,
}
impl DMA1ENR {
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
            DMA1ENR::DISABLED => false,
            DMA1ENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMA1ENR {
        match value {
            false => DMA1ENR::DISABLED,
            true => DMA1ENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DMA1ENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DMA1ENR::ENABLED
    }
}
#[doc = "Possible values of the field `SRAMEN`"]
pub type SRAMENR = DMA1ENR;
#[doc = "Possible values of the field `FLITFEN`"]
pub type FLITFENR = DMA1ENR;
#[doc = "Possible values of the field `CRCEN`"]
pub type CRCENR = DMA1ENR;
#[doc = "Possible values of the field `IOPAEN`"]
pub type IOPAENR = DMA1ENR;
#[doc = "Possible values of the field `IOPBEN`"]
pub type IOPBENR = DMA1ENR;
#[doc = "Possible values of the field `IOPCEN`"]
pub type IOPCENR = DMA1ENR;
#[doc = "Possible values of the field `IOPDEN`"]
pub type IOPDENR = DMA1ENR;
#[doc = "Possible values of the field `IOPFEN`"]
pub type IOPFENR = DMA1ENR;
#[doc = "Possible values of the field `TSCEN`"]
pub type TSCENR = DMA1ENR;
#[doc = "Values that can be written to the field `DMA1EN`"]
pub enum DMA1ENW {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Enabled."]
    ENABLED,
}
impl DMA1ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMA1ENW::DISABLED => false,
            DMA1ENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMA1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA1ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA1ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1ENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1ENW::ENABLED)
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
#[doc = "Values that can be written to the field `SRAMEN`"]
pub type SRAMENW = DMA1ENW;
#[doc = r" Proxy"]
pub struct _SRAMENW<'a> {
    w: &'a mut W,
}
impl<'a> _SRAMENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRAMENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1ENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1ENW::ENABLED)
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
#[doc = "Values that can be written to the field `FLITFEN`"]
pub type FLITFENW = DMA1ENW;
#[doc = r" Proxy"]
pub struct _FLITFENW<'a> {
    w: &'a mut W,
}
impl<'a> _FLITFENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLITFENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1ENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1ENW::ENABLED)
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
#[doc = "Values that can be written to the field `CRCEN`"]
pub type CRCENW = DMA1ENW;
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
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1ENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1ENW::ENABLED)
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
#[doc = "Values that can be written to the field `IOPAEN`"]
pub type IOPAENW = DMA1ENW;
#[doc = r" Proxy"]
pub struct _IOPAENW<'a> {
    w: &'a mut W,
}
impl<'a> _IOPAENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOPAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1ENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1ENW::ENABLED)
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
#[doc = "Values that can be written to the field `IOPBEN`"]
pub type IOPBENW = DMA1ENW;
#[doc = r" Proxy"]
pub struct _IOPBENW<'a> {
    w: &'a mut W,
}
impl<'a> _IOPBENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOPBENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1ENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1ENW::ENABLED)
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
#[doc = "Values that can be written to the field `IOPCEN`"]
pub type IOPCENW = DMA1ENW;
#[doc = r" Proxy"]
pub struct _IOPCENW<'a> {
    w: &'a mut W,
}
impl<'a> _IOPCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOPCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1ENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1ENW::ENABLED)
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
#[doc = "Values that can be written to the field `IOPDEN`"]
pub type IOPDENW = DMA1ENW;
#[doc = r" Proxy"]
pub struct _IOPDENW<'a> {
    w: &'a mut W,
}
impl<'a> _IOPDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOPDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1ENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1ENW::ENABLED)
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
#[doc = "Values that can be written to the field `IOPFEN`"]
pub type IOPFENW = DMA1ENW;
#[doc = r" Proxy"]
pub struct _IOPFENW<'a> {
    w: &'a mut W,
}
impl<'a> _IOPFENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOPFENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1ENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1ENW::ENABLED)
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
#[doc = "Values that can be written to the field `TSCEN`"]
pub type TSCENW = DMA1ENW;
#[doc = r" Proxy"]
pub struct _TSCENW<'a> {
    w: &'a mut W,
}
impl<'a> _TSCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1ENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1ENW::ENABLED)
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
        const OFFSET: u8 = 24;
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
    #[doc = "Bit 0 - DMA1 clock enable"]
    #[inline]
    pub fn dma1en(&self) -> DMA1ENR {
        DMA1ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - SRAM interface clock enable"]
    #[inline]
    pub fn sramen(&self) -> SRAMENR {
        SRAMENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - FLITF clock enable"]
    #[inline]
    pub fn flitfen(&self) -> FLITFENR {
        FLITFENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline]
    pub fn crcen(&self) -> CRCENR {
        CRCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - I/O port A clock enable"]
    #[inline]
    pub fn iopaen(&self) -> IOPAENR {
        IOPAENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - I/O port B clock enable"]
    #[inline]
    pub fn iopben(&self) -> IOPBENR {
        IOPBENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - I/O port C clock enable"]
    #[inline]
    pub fn iopcen(&self) -> IOPCENR {
        IOPCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - I/O port D clock enable"]
    #[inline]
    pub fn iopden(&self) -> IOPDENR {
        IOPDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - I/O port F clock enable"]
    #[inline]
    pub fn iopfen(&self) -> IOPFENR {
        IOPFENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Touch sensing controller clock enable"]
    #[inline]
    pub fn tscen(&self) -> TSCENR {
        TSCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 20 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - DMA1 clock enable"]
    #[inline]
    pub fn dma1en(&mut self) -> _DMA1ENW {
        _DMA1ENW { w: self }
    }
    #[doc = "Bit 2 - SRAM interface clock enable"]
    #[inline]
    pub fn sramen(&mut self) -> _SRAMENW {
        _SRAMENW { w: self }
    }
    #[doc = "Bit 4 - FLITF clock enable"]
    #[inline]
    pub fn flitfen(&mut self) -> _FLITFENW {
        _FLITFENW { w: self }
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline]
    pub fn crcen(&mut self) -> _CRCENW {
        _CRCENW { w: self }
    }
    #[doc = "Bit 17 - I/O port A clock enable"]
    #[inline]
    pub fn iopaen(&mut self) -> _IOPAENW {
        _IOPAENW { w: self }
    }
    #[doc = "Bit 18 - I/O port B clock enable"]
    #[inline]
    pub fn iopben(&mut self) -> _IOPBENW {
        _IOPBENW { w: self }
    }
    #[doc = "Bit 19 - I/O port C clock enable"]
    #[inline]
    pub fn iopcen(&mut self) -> _IOPCENW {
        _IOPCENW { w: self }
    }
    #[doc = "Bit 20 - I/O port D clock enable"]
    #[inline]
    pub fn iopden(&mut self) -> _IOPDENW {
        _IOPDENW { w: self }
    }
    #[doc = "Bit 22 - I/O port F clock enable"]
    #[inline]
    pub fn iopfen(&mut self) -> _IOPFENW {
        _IOPFENW { w: self }
    }
    #[doc = "Bit 24 - Touch sensing controller clock enable"]
    #[inline]
    pub fn tscen(&mut self) -> _TSCENW {
        _TSCENW { w: self }
    }
}
