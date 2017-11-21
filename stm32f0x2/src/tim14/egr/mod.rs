#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EGR {
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
}
#[doc = "Values that can be written to the field `CC1G`"]
pub enum CC1GW {
    #[doc = "generate an event."] GENERATED,
}
impl CC1GW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
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
    #[inline(always)]
    pub fn variant(self, variant: CC1GW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "generate an event."]
    #[inline(always)]
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
    #[inline(always)]
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
    #[doc = "Reinitialize the counter and generates an update of the registers."] RST_UPDATE,
}
impl UGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
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
    #[inline(always)]
    pub fn variant(self, variant: UGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reinitialize the counter and generates an update of the registers."]
    #[inline(always)]
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
    #[inline(always)]
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
    #[doc = "Bit 1 - Capture/compare 1 generation"]
    #[inline(always)]
    pub fn cc1g(&mut self) -> _CC1GW {
        _CC1GW { w: self }
    }
    #[doc = "Bit 0 - Update generation"]
    #[inline(always)]
    pub fn ug(&mut self) -> _UGW {
        _UGW { w: self }
    }
}
