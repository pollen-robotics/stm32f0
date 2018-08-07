#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EGR {
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
}
#[doc = "Values that can be written to the field `BG`"]
pub enum BGW {
    #[doc = "No action."]
    NOACTION,
    #[doc = "A break event is generated. MOE bit is cleared and BIF flag is set. Related interrupt or DMA transfer can occur if enabled."]
    BREAKEVENT,
}
impl BGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BGW::NOACTION => false,
            BGW::BREAKEVENT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BGW<'a> {
    w: &'a mut W,
}
impl<'a> _BGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action."]
    #[inline]
    pub fn no_action(self) -> &'a mut W {
        self.variant(BGW::NOACTION)
    }
    #[doc = "A break event is generated. MOE bit is cleared and BIF flag is set. Related interrupt or DMA transfer can occur if enabled."]
    #[inline]
    pub fn break_event(self) -> &'a mut W {
        self.variant(BGW::BREAKEVENT)
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
#[doc = "Values that can be written to the field `TG`"]
pub enum TGW {
    #[doc = "The TIF flag is set in TIMx_SR register. Related interrupt or DMA transfer can occur if enabled."]
    TIFSET,
}
impl TGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TGW::TIFSET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TGW<'a> {
    w: &'a mut W,
}
impl<'a> _TGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The TIF flag is set in TIMx_SR register. Related interrupt or DMA transfer can occur if enabled."]
    #[inline]
    pub fn tifset(self) -> &'a mut W {
        self.variant(TGW::TIFSET)
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
#[doc = "Values that can be written to the field `COMG`"]
pub enum COMGW {
    #[doc = "When CCPC bit is set, it allows to update CCxE, CCxNE and OCxM bits."]
    CCUPDATE,
}
impl COMGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMGW::CCUPDATE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMGW<'a> {
    w: &'a mut W,
}
impl<'a> _COMGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "When CCPC bit is set, it allows to update CCxE, CCxNE and OCxM bits."]
    #[inline]
    pub fn ccupdate(self) -> &'a mut W {
        self.variant(COMGW::CCUPDATE)
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
#[doc = "Values that can be written to the field `CC4G`"]
pub enum CC4GW {
    #[doc = "generate an event."]
    GENERATED,
}
impl CC4GW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CC4GW::GENERATED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC4GW<'a> {
    w: &'a mut W,
}
impl<'a> _CC4GW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC4GW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "generate an event."]
    #[inline]
    pub fn generated(self) -> &'a mut W {
        self.variant(CC4GW::GENERATED)
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
#[doc = "Values that can be written to the field `CC3G`"]
pub enum CC3GW {
    #[doc = "generate an event."]
    GENERATED,
}
impl CC3GW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CC3GW::GENERATED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC3GW<'a> {
    w: &'a mut W,
}
impl<'a> _CC3GW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC3GW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "generate an event."]
    #[inline]
    pub fn generated(self) -> &'a mut W {
        self.variant(CC3GW::GENERATED)
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
#[doc = "Values that can be written to the field `CC2G`"]
pub enum CC2GW {
    #[doc = "generate an event."]
    GENERATED,
}
impl CC2GW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CC2GW::GENERATED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC2GW<'a> {
    w: &'a mut W,
}
impl<'a> _CC2GW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC2GW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "generate an event."]
    #[inline]
    pub fn generated(self) -> &'a mut W {
        self.variant(CC2GW::GENERATED)
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
#[doc = "Values that can be written to the field `CC1G`"]
pub enum CC1GW {
    #[doc = "generate an event."]
    GENERATED,
}
impl CC1GW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CC1GW::GENERATED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC1GW<'a> {
    w: &'a mut W,
}
impl<'a> _CC1GW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC1GW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "generate an event."]
    #[inline]
    pub fn generated(self) -> &'a mut W {
        self.variant(CC1GW::GENERATED)
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
#[doc = "Values that can be written to the field `UG`"]
pub enum UGW {
    #[doc = "Reinitialize the counter and generates an update of the registers."]
    RST_UPDATE,
}
impl UGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UGW::RST_UPDATE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UGW<'a> {
    w: &'a mut W,
}
impl<'a> _UGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reinitialize the counter and generates an update of the registers."]
    #[inline]
    pub fn rst_update(self) -> &'a mut W {
        self.variant(UGW::RST_UPDATE)
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
    #[doc = "Bit 7 - Break generation"]
    #[inline]
    pub fn bg(&mut self) -> _BGW {
        _BGW { w: self }
    }
    #[doc = "Bit 6 - Trigger generation"]
    #[inline]
    pub fn tg(&mut self) -> _TGW {
        _TGW { w: self }
    }
    #[doc = "Bit 5 - Capture/Compare control update generation"]
    #[inline]
    pub fn comg(&mut self) -> _COMGW {
        _COMGW { w: self }
    }
    #[doc = "Bit 4 - Capture/compare 4 generation"]
    #[inline]
    pub fn cc4g(&mut self) -> _CC4GW {
        _CC4GW { w: self }
    }
    #[doc = "Bit 3 - Capture/compare 3 generation"]
    #[inline]
    pub fn cc3g(&mut self) -> _CC3GW {
        _CC3GW { w: self }
    }
    #[doc = "Bit 2 - Capture/compare 2 generation"]
    #[inline]
    pub fn cc2g(&mut self) -> _CC2GW {
        _CC2GW { w: self }
    }
    #[doc = "Bit 1 - Capture/compare 1 generation"]
    #[inline]
    pub fn cc1g(&mut self) -> _CC1GW {
        _CC1GW { w: self }
    }
    #[doc = "Bit 0 - Update generation"]
    #[inline]
    pub fn ug(&mut self) -> _UGW {
        _UGW { w: self }
    }
}
