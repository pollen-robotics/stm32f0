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
#[doc = "Possible values of the field `OIS4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OIS4R {
    #[doc = "OC4=0 when MOE=0"]
    RESET,
    #[doc = "OC4=1 when MOE=0"]
    SET,
}
impl OIS4R {
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
            OIS4R::RESET => false,
            OIS4R::SET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OIS4R {
        match value {
            false => OIS4R::RESET,
            true => OIS4R::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == OIS4R::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == OIS4R::SET
    }
}
#[doc = "Possible values of the field `OIS3N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OIS3NR {
    #[doc = "OC3N=0 after a dead-time when MOE=0"]
    RESET,
    #[doc = "OC3N=1 after a dead-time when MOE=0"]
    SET,
}
impl OIS3NR {
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
            OIS3NR::RESET => false,
            OIS3NR::SET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OIS3NR {
        match value {
            false => OIS3NR::RESET,
            true => OIS3NR::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == OIS3NR::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == OIS3NR::SET
    }
}
#[doc = "Possible values of the field `OIS3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OIS3R {
    #[doc = "OC3=0 when MOE=0"]
    RESET,
    #[doc = "OC3=1 when MOE=0"]
    SET,
}
impl OIS3R {
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
            OIS3R::RESET => false,
            OIS3R::SET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OIS3R {
        match value {
            false => OIS3R::RESET,
            true => OIS3R::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == OIS3R::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == OIS3R::SET
    }
}
#[doc = "Possible values of the field `OIS2N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OIS2NR {
    #[doc = "OC2N=0 after a dead-time when MOE=0"]
    RESET,
    #[doc = "OC2N=1 after a dead-time when MOE=0"]
    SET,
}
impl OIS2NR {
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
            OIS2NR::RESET => false,
            OIS2NR::SET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OIS2NR {
        match value {
            false => OIS2NR::RESET,
            true => OIS2NR::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == OIS2NR::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == OIS2NR::SET
    }
}
#[doc = "Possible values of the field `OIS2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OIS2R {
    #[doc = "OC2=0 when MOE=0"]
    RESET,
    #[doc = "OC2=1 when MOE=0"]
    SET,
}
impl OIS2R {
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
            OIS2R::RESET => false,
            OIS2R::SET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OIS2R {
        match value {
            false => OIS2R::RESET,
            true => OIS2R::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == OIS2R::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == OIS2R::SET
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
    #[doc = "OC1=0 when MOE=0"]
    RESET,
    #[doc = "OC1=1 when MOE=0"]
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
#[doc = "Possible values of the field `TI1S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TI1SR {
    #[doc = "The TIMx CH1 pin is connected to TI1 input"]
    TI1INPUT,
    #[doc = "The TIMx CH1, CH2 and CH3 pins are connected to the TI1 input (XOR combination)"]
    ALLTI1INPUT,
}
impl TI1SR {
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
            TI1SR::TI1INPUT => false,
            TI1SR::ALLTI1INPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TI1SR {
        match value {
            false => TI1SR::TI1INPUT,
            true => TI1SR::ALLTI1INPUT,
        }
    }
    #[doc = "Checks if the value of the field is `TI1INPUT`"]
    #[inline]
    pub fn is_ti1input(&self) -> bool {
        *self == TI1SR::TI1INPUT
    }
    #[doc = "Checks if the value of the field is `ALLTI1INPUT`"]
    #[inline]
    pub fn is_all_ti1input(&self) -> bool {
        *self == TI1SR::ALLTI1INPUT
    }
}
#[doc = "Possible values of the field `MMS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMSR {
    #[doc = "the UG bit from the TIMx_EGR register is used as trigger output (TRGO)."]
    RESET,
    #[doc = "the Counter Enable signal CNT_EN is used as trigger output (TRGO)."]
    ENABLE,
    #[doc = "The update event is selected as trigger output (TRGO)."]
    UPDATE,
    #[doc = "The trigger output send a positive pulse when the CC1IF flag is to be set."]
    COMPAREPULSE,
    #[doc = "OC1REF signal is used as trigger output (TRGO)"]
    COMPAREOC1REF,
    #[doc = "OC2REF signal is used as trigger output (TRGO)"]
    COMPAREOC2REF,
    #[doc = "OC3REF signal is used as trigger output (TRGO)"]
    COMPAREOC3REF,
    #[doc = "OC4REF signal is used as trigger output (TRGO)"]
    COMPAREOC4REF,
}
impl MMSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MMSR::RESET => 0,
            MMSR::ENABLE => 1,
            MMSR::UPDATE => 2,
            MMSR::COMPAREPULSE => 3,
            MMSR::COMPAREOC1REF => 4,
            MMSR::COMPAREOC2REF => 5,
            MMSR::COMPAREOC3REF => 6,
            MMSR::COMPAREOC4REF => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MMSR {
        match value {
            0 => MMSR::RESET,
            1 => MMSR::ENABLE,
            2 => MMSR::UPDATE,
            3 => MMSR::COMPAREPULSE,
            4 => MMSR::COMPAREOC1REF,
            5 => MMSR::COMPAREOC2REF,
            6 => MMSR::COMPAREOC3REF,
            7 => MMSR::COMPAREOC4REF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == MMSR::RESET
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == MMSR::ENABLE
    }
    #[doc = "Checks if the value of the field is `UPDATE`"]
    #[inline]
    pub fn is_update(&self) -> bool {
        *self == MMSR::UPDATE
    }
    #[doc = "Checks if the value of the field is `COMPAREPULSE`"]
    #[inline]
    pub fn is_compare_pulse(&self) -> bool {
        *self == MMSR::COMPAREPULSE
    }
    #[doc = "Checks if the value of the field is `COMPAREOC1REF`"]
    #[inline]
    pub fn is_compare_oc1ref(&self) -> bool {
        *self == MMSR::COMPAREOC1REF
    }
    #[doc = "Checks if the value of the field is `COMPAREOC2REF`"]
    #[inline]
    pub fn is_compare_oc2ref(&self) -> bool {
        *self == MMSR::COMPAREOC2REF
    }
    #[doc = "Checks if the value of the field is `COMPAREOC3REF`"]
    #[inline]
    pub fn is_compare_oc3ref(&self) -> bool {
        *self == MMSR::COMPAREOC3REF
    }
    #[doc = "Checks if the value of the field is `COMPAREOC4REF`"]
    #[inline]
    pub fn is_compare_oc4ref(&self) -> bool {
        *self == MMSR::COMPAREOC4REF
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
#[doc = "Values that can be written to the field `OIS4`"]
pub enum OIS4W {
    #[doc = "OC4=0 when MOE=0"]
    RESET,
    #[doc = "OC4=1 when MOE=0"]
    SET,
}
impl OIS4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OIS4W::RESET => false,
            OIS4W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OIS4W<'a> {
    w: &'a mut W,
}
impl<'a> _OIS4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OIS4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "OC4=0 when MOE=0"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(OIS4W::RESET)
    }
    #[doc = "OC4=1 when MOE=0"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(OIS4W::SET)
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
#[doc = "Values that can be written to the field `OIS3N`"]
pub enum OIS3NW {
    #[doc = "OC3N=0 after a dead-time when MOE=0"]
    RESET,
    #[doc = "OC3N=1 after a dead-time when MOE=0"]
    SET,
}
impl OIS3NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OIS3NW::RESET => false,
            OIS3NW::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OIS3NW<'a> {
    w: &'a mut W,
}
impl<'a> _OIS3NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OIS3NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "OC3N=0 after a dead-time when MOE=0"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(OIS3NW::RESET)
    }
    #[doc = "OC3N=1 after a dead-time when MOE=0"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(OIS3NW::SET)
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
#[doc = "Values that can be written to the field `OIS3`"]
pub enum OIS3W {
    #[doc = "OC3=0 when MOE=0"]
    RESET,
    #[doc = "OC3=1 when MOE=0"]
    SET,
}
impl OIS3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OIS3W::RESET => false,
            OIS3W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OIS3W<'a> {
    w: &'a mut W,
}
impl<'a> _OIS3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OIS3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "OC3=0 when MOE=0"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(OIS3W::RESET)
    }
    #[doc = "OC3=1 when MOE=0"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(OIS3W::SET)
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
#[doc = "Values that can be written to the field `OIS2N`"]
pub enum OIS2NW {
    #[doc = "OC2N=0 after a dead-time when MOE=0"]
    RESET,
    #[doc = "OC2N=1 after a dead-time when MOE=0"]
    SET,
}
impl OIS2NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OIS2NW::RESET => false,
            OIS2NW::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OIS2NW<'a> {
    w: &'a mut W,
}
impl<'a> _OIS2NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OIS2NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "OC2N=0 after a dead-time when MOE=0"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(OIS2NW::RESET)
    }
    #[doc = "OC2N=1 after a dead-time when MOE=0"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(OIS2NW::SET)
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
#[doc = "Values that can be written to the field `OIS2`"]
pub enum OIS2W {
    #[doc = "OC2=0 when MOE=0"]
    RESET,
    #[doc = "OC2=1 when MOE=0"]
    SET,
}
impl OIS2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OIS2W::RESET => false,
            OIS2W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OIS2W<'a> {
    w: &'a mut W,
}
impl<'a> _OIS2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OIS2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "OC2=0 when MOE=0"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(OIS2W::RESET)
    }
    #[doc = "OC2=1 when MOE=0"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(OIS2W::SET)
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
    #[doc = "OC1=0 when MOE=0"]
    RESET,
    #[doc = "OC1=1 when MOE=0"]
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
    #[doc = "OC1=0 when MOE=0"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(OIS1W::RESET)
    }
    #[doc = "OC1=1 when MOE=0"]
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
#[doc = "Values that can be written to the field `TI1S`"]
pub enum TI1SW {
    #[doc = "The TIMx CH1 pin is connected to TI1 input"]
    TI1INPUT,
    #[doc = "The TIMx CH1, CH2 and CH3 pins are connected to the TI1 input (XOR combination)"]
    ALLTI1INPUT,
}
impl TI1SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TI1SW::TI1INPUT => false,
            TI1SW::ALLTI1INPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TI1SW<'a> {
    w: &'a mut W,
}
impl<'a> _TI1SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TI1SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The TIMx CH1 pin is connected to TI1 input"]
    #[inline]
    pub fn ti1input(self) -> &'a mut W {
        self.variant(TI1SW::TI1INPUT)
    }
    #[doc = "The TIMx CH1, CH2 and CH3 pins are connected to the TI1 input (XOR combination)"]
    #[inline]
    pub fn all_ti1input(self) -> &'a mut W {
        self.variant(TI1SW::ALLTI1INPUT)
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
#[doc = "Values that can be written to the field `MMS`"]
pub enum MMSW {
    #[doc = "the UG bit from the TIMx_EGR register is used as trigger output (TRGO)."]
    RESET,
    #[doc = "the Counter Enable signal CNT_EN is used as trigger output (TRGO)."]
    ENABLE,
    #[doc = "The update event is selected as trigger output (TRGO)."]
    UPDATE,
    #[doc = "The trigger output send a positive pulse when the CC1IF flag is to be set."]
    COMPAREPULSE,
    #[doc = "OC1REF signal is used as trigger output (TRGO)"]
    COMPAREOC1REF,
    #[doc = "OC2REF signal is used as trigger output (TRGO)"]
    COMPAREOC2REF,
    #[doc = "OC3REF signal is used as trigger output (TRGO)"]
    COMPAREOC3REF,
    #[doc = "OC4REF signal is used as trigger output (TRGO)"]
    COMPAREOC4REF,
}
impl MMSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MMSW::RESET => 0,
            MMSW::ENABLE => 1,
            MMSW::UPDATE => 2,
            MMSW::COMPAREPULSE => 3,
            MMSW::COMPAREOC1REF => 4,
            MMSW::COMPAREOC2REF => 5,
            MMSW::COMPAREOC3REF => 6,
            MMSW::COMPAREOC4REF => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MMSW<'a> {
    w: &'a mut W,
}
impl<'a> _MMSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MMSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "the UG bit from the TIMx_EGR register is used as trigger output (TRGO)."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(MMSW::RESET)
    }
    #[doc = "the Counter Enable signal CNT_EN is used as trigger output (TRGO)."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(MMSW::ENABLE)
    }
    #[doc = "The update event is selected as trigger output (TRGO)."]
    #[inline]
    pub fn update(self) -> &'a mut W {
        self.variant(MMSW::UPDATE)
    }
    #[doc = "The trigger output send a positive pulse when the CC1IF flag is to be set."]
    #[inline]
    pub fn compare_pulse(self) -> &'a mut W {
        self.variant(MMSW::COMPAREPULSE)
    }
    #[doc = "OC1REF signal is used as trigger output (TRGO)"]
    #[inline]
    pub fn compare_oc1ref(self) -> &'a mut W {
        self.variant(MMSW::COMPAREOC1REF)
    }
    #[doc = "OC2REF signal is used as trigger output (TRGO)"]
    #[inline]
    pub fn compare_oc2ref(self) -> &'a mut W {
        self.variant(MMSW::COMPAREOC2REF)
    }
    #[doc = "OC3REF signal is used as trigger output (TRGO)"]
    #[inline]
    pub fn compare_oc3ref(self) -> &'a mut W {
        self.variant(MMSW::COMPAREOC3REF)
    }
    #[doc = "OC4REF signal is used as trigger output (TRGO)"]
    #[inline]
    pub fn compare_oc4ref(self) -> &'a mut W {
        self.variant(MMSW::COMPAREOC4REF)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
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
    #[doc = "Bit 14 - Output Idle state 4"]
    #[inline]
    pub fn ois4(&self) -> OIS4R {
        OIS4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Output Idle state 3"]
    #[inline]
    pub fn ois3n(&self) -> OIS3NR {
        OIS3NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Output Idle state 3"]
    #[inline]
    pub fn ois3(&self) -> OIS3R {
        OIS3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Output Idle state 2"]
    #[inline]
    pub fn ois2n(&self) -> OIS2NR {
        OIS2NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Output Idle state 2"]
    #[inline]
    pub fn ois2(&self) -> OIS2R {
        OIS2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
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
    #[doc = "Bit 7 - TI1 selection"]
    #[inline]
    pub fn ti1s(&self) -> TI1SR {
        TI1SR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:6 - Master mode selection"]
    #[inline]
    pub fn mms(&self) -> MMSR {
        MMSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bit 14 - Output Idle state 4"]
    #[inline]
    pub fn ois4(&mut self) -> _OIS4W {
        _OIS4W { w: self }
    }
    #[doc = "Bit 13 - Output Idle state 3"]
    #[inline]
    pub fn ois3n(&mut self) -> _OIS3NW {
        _OIS3NW { w: self }
    }
    #[doc = "Bit 12 - Output Idle state 3"]
    #[inline]
    pub fn ois3(&mut self) -> _OIS3W {
        _OIS3W { w: self }
    }
    #[doc = "Bit 11 - Output Idle state 2"]
    #[inline]
    pub fn ois2n(&mut self) -> _OIS2NW {
        _OIS2NW { w: self }
    }
    #[doc = "Bit 10 - Output Idle state 2"]
    #[inline]
    pub fn ois2(&mut self) -> _OIS2W {
        _OIS2W { w: self }
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
    #[doc = "Bit 7 - TI1 selection"]
    #[inline]
    pub fn ti1s(&mut self) -> _TI1SW {
        _TI1SW { w: self }
    }
    #[doc = "Bits 4:6 - Master mode selection"]
    #[inline]
    pub fn mms(&mut self) -> _MMSW {
        _MMSW { w: self }
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
