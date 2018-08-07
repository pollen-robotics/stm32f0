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
#[doc = "Possible values of the field `OIS1N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OIS1NR {
    #[doc = "OC1N=0 after a dead-time when MOE=0"]
    RESET,
    #[doc = "OC1N=1 after a dead-time when MOE=0"]
    SET,
}
impl OIS1NR {
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
            OIS1NR::RESET => false,
            OIS1NR::SET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OIS1NR {
        match value {
            false => OIS1NR::RESET,
            true => OIS1NR::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == OIS1NR::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == OIS1NR::SET
    }
}
#[doc = "Possible values of the field `OIS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OIS1R {
    #[doc = "OC1=0 (after a dead-time if OC1N is implemented) when MOE=0"]
    RESET,
    #[doc = "OC1=1 (after a dead-time if OC1N is implemented) when MOE=0"]
    SET,
}
impl OIS1R {
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
            OIS1R::RESET => false,
            OIS1R::SET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OIS1R {
        match value {
            false => OIS1R::RESET,
            true => OIS1R::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == OIS1R::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == OIS1R::SET
    }
}
#[doc = "Possible values of the field `CCDS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCDSR {
    #[doc = "CCx DMA request sent when CCx event occurs"]
    CCXEVENT,
    #[doc = "CCx DMA requests sent when update event occurs"]
    UPDATE,
}
impl CCDSR {
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
            CCDSR::CCXEVENT => false,
            CCDSR::UPDATE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCDSR {
        match value {
            false => CCDSR::CCXEVENT,
            true => CCDSR::UPDATE,
        }
    }
    #[doc = "Checks if the value of the field is `CCXEVENT`"]
    #[inline]
    pub fn is_ccx_event(&self) -> bool {
        *self == CCDSR::CCXEVENT
    }
    #[doc = "Checks if the value of the field is `UPDATE`"]
    #[inline]
    pub fn is_update(&self) -> bool {
        *self == CCDSR::UPDATE
    }
}
#[doc = "Possible values of the field `CCUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCUSR {
    #[doc = "When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit only"]
    COMGBITONLY,
    #[doc = "When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit or when an rising edge occurs on TRGI"]
    COMGBIT_EDGE,
}
impl CCUSR {
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
            CCUSR::COMGBITONLY => false,
            CCUSR::COMGBIT_EDGE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCUSR {
        match value {
            false => CCUSR::COMGBITONLY,
            true => CCUSR::COMGBIT_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `COMGBITONLY`"]
    #[inline]
    pub fn is_comgbit_only(&self) -> bool {
        *self == CCUSR::COMGBITONLY
    }
    #[doc = "Checks if the value of the field is `COMGBIT_EDGE`"]
    #[inline]
    pub fn is_comgbit_edge(&self) -> bool {
        *self == CCUSR::COMGBIT_EDGE
    }
}
#[doc = "Possible values of the field `CCPC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCPCR {
    #[doc = "CCxE, CCxNE and OCxM bits are not preloaded"]
    NOTPRELOADED,
    #[doc = "CCxE, CCxNE and OCxM bits are preloaded, after having been written, they are updated only when a communication event (COM) occurs (COMG bit set or rising edge detected on TRGI, depending on the CCUS bit)."]
    PRELOADED,
}
impl CCPCR {
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
            CCPCR::NOTPRELOADED => false,
            CCPCR::PRELOADED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCPCR {
        match value {
            false => CCPCR::NOTPRELOADED,
            true => CCPCR::PRELOADED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPRELOADED`"]
    #[inline]
    pub fn is_not_preloaded(&self) -> bool {
        *self == CCPCR::NOTPRELOADED
    }
    #[doc = "Checks if the value of the field is `PRELOADED`"]
    #[inline]
    pub fn is_preloaded(&self) -> bool {
        *self == CCPCR::PRELOADED
    }
}
#[doc = "Values that can be written to the field `OIS1N`"]
pub enum OIS1NW {
    #[doc = "OC1N=0 after a dead-time when MOE=0"]
    RESET,
    #[doc = "OC1N=1 after a dead-time when MOE=0"]
    SET,
}
impl OIS1NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OIS1NW::RESET => false,
            OIS1NW::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OIS1NW<'a> {
    w: &'a mut W,
}
impl<'a> _OIS1NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OIS1NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "OC1N=0 after a dead-time when MOE=0"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(OIS1NW::RESET)
    }
    #[doc = "OC1N=1 after a dead-time when MOE=0"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(OIS1NW::SET)
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
#[doc = "Values that can be written to the field `OIS1`"]
pub enum OIS1W {
    #[doc = "OC1=0 (after a dead-time if OC1N is implemented) when MOE=0"]
    RESET,
    #[doc = "OC1=1 (after a dead-time if OC1N is implemented) when MOE=0"]
    SET,
}
impl OIS1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OIS1W::RESET => false,
            OIS1W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OIS1W<'a> {
    w: &'a mut W,
}
impl<'a> _OIS1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OIS1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "OC1=0 (after a dead-time if OC1N is implemented) when MOE=0"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(OIS1W::RESET)
    }
    #[doc = "OC1=1 (after a dead-time if OC1N is implemented) when MOE=0"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(OIS1W::SET)
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
#[doc = "Values that can be written to the field `CCDS`"]
pub enum CCDSW {
    #[doc = "CCx DMA request sent when CCx event occurs"]
    CCXEVENT,
    #[doc = "CCx DMA requests sent when update event occurs"]
    UPDATE,
}
impl CCDSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCDSW::CCXEVENT => false,
            CCDSW::UPDATE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCDSW<'a> {
    w: &'a mut W,
}
impl<'a> _CCDSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCDSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CCx DMA request sent when CCx event occurs"]
    #[inline]
    pub fn ccx_event(self) -> &'a mut W {
        self.variant(CCDSW::CCXEVENT)
    }
    #[doc = "CCx DMA requests sent when update event occurs"]
    #[inline]
    pub fn update(self) -> &'a mut W {
        self.variant(CCDSW::UPDATE)
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
#[doc = "Values that can be written to the field `CCUS`"]
pub enum CCUSW {
    #[doc = "When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit only"]
    COMGBITONLY,
    #[doc = "When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit or when an rising edge occurs on TRGI"]
    COMGBIT_EDGE,
}
impl CCUSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCUSW::COMGBITONLY => false,
            CCUSW::COMGBIT_EDGE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCUSW<'a> {
    w: &'a mut W,
}
impl<'a> _CCUSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCUSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit only"]
    #[inline]
    pub fn comgbit_only(self) -> &'a mut W {
        self.variant(CCUSW::COMGBITONLY)
    }
    #[doc = "When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit or when an rising edge occurs on TRGI"]
    #[inline]
    pub fn comgbit_edge(self) -> &'a mut W {
        self.variant(CCUSW::COMGBIT_EDGE)
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
#[doc = "Values that can be written to the field `CCPC`"]
pub enum CCPCW {
    #[doc = "CCxE, CCxNE and OCxM bits are not preloaded"]
    NOTPRELOADED,
    #[doc = "CCxE, CCxNE and OCxM bits are preloaded, after having been written, they are updated only when a communication event (COM) occurs (COMG bit set or rising edge detected on TRGI, depending on the CCUS bit)."]
    PRELOADED,
}
impl CCPCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCPCW::NOTPRELOADED => false,
            CCPCW::PRELOADED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCPCW<'a> {
    w: &'a mut W,
}
impl<'a> _CCPCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCPCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CCxE, CCxNE and OCxM bits are not preloaded"]
    #[inline]
    pub fn not_preloaded(self) -> &'a mut W {
        self.variant(CCPCW::NOTPRELOADED)
    }
    #[doc = "CCxE, CCxNE and OCxM bits are preloaded, after having been written, they are updated only when a communication event (COM) occurs (COMG bit set or rising edge detected on TRGI, depending on the CCUS bit)."]
    #[inline]
    pub fn preloaded(self) -> &'a mut W {
        self.variant(CCPCW::PRELOADED)
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
    #[doc = "Bit 9 - Output Idle state 1"]
    #[inline]
    pub fn ois1n(&self) -> OIS1NR {
        OIS1NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Output Idle state 1"]
    #[inline]
    pub fn ois1(&self) -> OIS1R {
        OIS1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline]
    pub fn ccds(&self) -> CCDSR {
        CCDSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Capture/compare control update selection"]
    #[inline]
    pub fn ccus(&self) -> CCUSR {
        CCUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Capture/compare preloaded control"]
    #[inline]
    pub fn ccpc(&self) -> CCPCR {
        CCPCR::_from({
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
    #[doc = "Bit 9 - Output Idle state 1"]
    #[inline]
    pub fn ois1n(&mut self) -> _OIS1NW {
        _OIS1NW { w: self }
    }
    #[doc = "Bit 8 - Output Idle state 1"]
    #[inline]
    pub fn ois1(&mut self) -> _OIS1W {
        _OIS1W { w: self }
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline]
    pub fn ccds(&mut self) -> _CCDSW {
        _CCDSW { w: self }
    }
    #[doc = "Bit 2 - Capture/compare control update selection"]
    #[inline]
    pub fn ccus(&mut self) -> _CCUSW {
        _CCUSW { w: self }
    }
    #[doc = "Bit 0 - Capture/compare preloaded control"]
    #[inline]
    pub fn ccpc(&mut self) -> _CCPCW {
        _CCPCW { w: self }
    }
}
