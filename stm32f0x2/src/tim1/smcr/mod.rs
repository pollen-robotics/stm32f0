#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SMCR {
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
#[doc = "Possible values of the field `ETP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETPR {
    #[doc = "ETR is non-inverted, active at high level or rising edge."]
    NONINVERTED,
    #[doc = "ETR is inverted, active at low level or falling edge."]
    INVERTED,
}
impl ETPR {
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
            ETPR::NONINVERTED => false,
            ETPR::INVERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ETPR {
        match value {
            false => ETPR::NONINVERTED,
            true => ETPR::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NONINVERTED`"]
    #[inline]
    pub fn is_non_inverted(&self) -> bool {
        *self == ETPR::NONINVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline]
    pub fn is_inverted(&self) -> bool {
        *self == ETPR::INVERTED
    }
}
#[doc = "Possible values of the field `ECE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECER {
    #[doc = "External clock mode 2 disabled."]
    DISABLED,
    #[doc = "External clock mode 2 enabled.."]
    ENABLED,
}
impl ECER {
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
            ECER::DISABLED => false,
            ECER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ECER {
        match value {
            false => ECER::DISABLED,
            true => ECER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ECER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ECER::ENABLED
    }
}
#[doc = "Possible values of the field `ETPS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETPSR {
    #[doc = "Prescaler OFF."]
    OFF,
    #[doc = "ETRP frequency divided by 2."]
    ETRPDIV2,
    #[doc = "ETRP frequency divided by 4."]
    ETRPDIV4,
    #[doc = "ETRP frequency divided by 8."]
    ETRPDIV8,
}
impl ETPSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ETPSR::OFF => 0,
            ETPSR::ETRPDIV2 => 1,
            ETPSR::ETRPDIV4 => 2,
            ETPSR::ETRPDIV8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ETPSR {
        match value {
            0 => ETPSR::OFF,
            1 => ETPSR::ETRPDIV2,
            2 => ETPSR::ETRPDIV4,
            3 => ETPSR::ETRPDIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == ETPSR::OFF
    }
    #[doc = "Checks if the value of the field is `ETRPDIV2`"]
    #[inline]
    pub fn is_etrpdiv2(&self) -> bool {
        *self == ETPSR::ETRPDIV2
    }
    #[doc = "Checks if the value of the field is `ETRPDIV4`"]
    #[inline]
    pub fn is_etrpdiv4(&self) -> bool {
        *self == ETPSR::ETRPDIV4
    }
    #[doc = "Checks if the value of the field is `ETRPDIV8`"]
    #[inline]
    pub fn is_etrpdiv8(&self) -> bool {
        *self == ETPSR::ETRPDIV8
    }
}
#[doc = "Possible values of the field `ETF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETFR {
    #[doc = "No filter, sampling is done at fDTS"]
    NOFILTER,
    #[doc = "fSAMPLING = fCK_INT, N = 2"]
    FCK_INT_N2,
    #[doc = "fSAMPLING = fCK_INT, N = 4"]
    FCK_INT_N4,
    #[doc = "fSAMPLING = fCK_INT, N = 8"]
    FCK_INT_N8,
    #[doc = "fSAMPLING = fDTS / 2, N = 6"]
    FDTSDIV2_N6,
    #[doc = "fSAMPLING = fDTS / 2, N = 8"]
    FDTSDIV2_N8,
    #[doc = "fSAMPLING = fDTS / 4, N = 6"]
    FDTSDIV4_N6,
    #[doc = "fSAMPLING = fDTS / 4, N = 8"]
    FDTSDIV4_N8,
    #[doc = "fSAMPLING = fDTS / 8, N = 6"]
    FDTSDIV8_N6,
    #[doc = "fSAMPLING = fDTS / 8, N = 8"]
    FDTSDIV8_N8,
    #[doc = "fSAMPLING = fDTS / 16, N = 5"]
    FDTSDIV16_N5,
    #[doc = "fSAMPLING = fDTS / 16, N = 6"]
    FDTSDIV16_N6,
    #[doc = "fSAMPLING = fDTS / 16, N = 8"]
    FDTSDIV16_N8,
    #[doc = "fSAMPLING = fDTS / 32, N = 5"]
    FDTSDIV32_N5,
    #[doc = "fSAMPLING = fDTS / 32, N = 6"]
    FDTSDIV32_N6,
    #[doc = "fSAMPLING = fDTS / 32, N = 8"]
    FDTSDIV32_N8,
}
impl ETFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ETFR::NOFILTER => 0,
            ETFR::FCK_INT_N2 => 1,
            ETFR::FCK_INT_N4 => 2,
            ETFR::FCK_INT_N8 => 3,
            ETFR::FDTSDIV2_N6 => 4,
            ETFR::FDTSDIV2_N8 => 5,
            ETFR::FDTSDIV4_N6 => 6,
            ETFR::FDTSDIV4_N8 => 7,
            ETFR::FDTSDIV8_N6 => 8,
            ETFR::FDTSDIV8_N8 => 9,
            ETFR::FDTSDIV16_N5 => 10,
            ETFR::FDTSDIV16_N6 => 11,
            ETFR::FDTSDIV16_N8 => 12,
            ETFR::FDTSDIV32_N5 => 13,
            ETFR::FDTSDIV32_N6 => 14,
            ETFR::FDTSDIV32_N8 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ETFR {
        match value {
            0 => ETFR::NOFILTER,
            1 => ETFR::FCK_INT_N2,
            2 => ETFR::FCK_INT_N4,
            3 => ETFR::FCK_INT_N8,
            4 => ETFR::FDTSDIV2_N6,
            5 => ETFR::FDTSDIV2_N8,
            6 => ETFR::FDTSDIV4_N6,
            7 => ETFR::FDTSDIV4_N8,
            8 => ETFR::FDTSDIV8_N6,
            9 => ETFR::FDTSDIV8_N8,
            10 => ETFR::FDTSDIV16_N5,
            11 => ETFR::FDTSDIV16_N6,
            12 => ETFR::FDTSDIV16_N8,
            13 => ETFR::FDTSDIV32_N5,
            14 => ETFR::FDTSDIV32_N6,
            15 => ETFR::FDTSDIV32_N8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOFILTER`"]
    #[inline]
    pub fn is_no_filter(&self) -> bool {
        *self == ETFR::NOFILTER
    }
    #[doc = "Checks if the value of the field is `FCK_INT_N2`"]
    #[inline]
    pub fn is_f_ck_int_n2(&self) -> bool {
        *self == ETFR::FCK_INT_N2
    }
    #[doc = "Checks if the value of the field is `FCK_INT_N4`"]
    #[inline]
    pub fn is_f_ck_int_n4(&self) -> bool {
        *self == ETFR::FCK_INT_N4
    }
    #[doc = "Checks if the value of the field is `FCK_INT_N8`"]
    #[inline]
    pub fn is_f_ck_int_n8(&self) -> bool {
        *self == ETFR::FCK_INT_N8
    }
    #[doc = "Checks if the value of the field is `FDTSDIV2_N6`"]
    #[inline]
    pub fn is_f_dtsdiv2_n6(&self) -> bool {
        *self == ETFR::FDTSDIV2_N6
    }
    #[doc = "Checks if the value of the field is `FDTSDIV2_N8`"]
    #[inline]
    pub fn is_f_dtsdiv2_n8(&self) -> bool {
        *self == ETFR::FDTSDIV2_N8
    }
    #[doc = "Checks if the value of the field is `FDTSDIV4_N6`"]
    #[inline]
    pub fn is_f_dtsdiv4_n6(&self) -> bool {
        *self == ETFR::FDTSDIV4_N6
    }
    #[doc = "Checks if the value of the field is `FDTSDIV4_N8`"]
    #[inline]
    pub fn is_f_dtsdiv4_n8(&self) -> bool {
        *self == ETFR::FDTSDIV4_N8
    }
    #[doc = "Checks if the value of the field is `FDTSDIV8_N6`"]
    #[inline]
    pub fn is_f_dtsdiv8_n6(&self) -> bool {
        *self == ETFR::FDTSDIV8_N6
    }
    #[doc = "Checks if the value of the field is `FDTSDIV8_N8`"]
    #[inline]
    pub fn is_f_dtsdiv8_n8(&self) -> bool {
        *self == ETFR::FDTSDIV8_N8
    }
    #[doc = "Checks if the value of the field is `FDTSDIV16_N5`"]
    #[inline]
    pub fn is_f_dtsdiv16_n5(&self) -> bool {
        *self == ETFR::FDTSDIV16_N5
    }
    #[doc = "Checks if the value of the field is `FDTSDIV16_N6`"]
    #[inline]
    pub fn is_f_dtsdiv16_n6(&self) -> bool {
        *self == ETFR::FDTSDIV16_N6
    }
    #[doc = "Checks if the value of the field is `FDTSDIV16_N8`"]
    #[inline]
    pub fn is_f_dtsdiv16_n8(&self) -> bool {
        *self == ETFR::FDTSDIV16_N8
    }
    #[doc = "Checks if the value of the field is `FDTSDIV32_N5`"]
    #[inline]
    pub fn is_f_dtsdiv32_n5(&self) -> bool {
        *self == ETFR::FDTSDIV32_N5
    }
    #[doc = "Checks if the value of the field is `FDTSDIV32_N6`"]
    #[inline]
    pub fn is_f_dtsdiv32_n6(&self) -> bool {
        *self == ETFR::FDTSDIV32_N6
    }
    #[doc = "Checks if the value of the field is `FDTSDIV32_N8`"]
    #[inline]
    pub fn is_f_dtsdiv32_n8(&self) -> bool {
        *self == ETFR::FDTSDIV32_N8
    }
}
#[doc = "Possible values of the field `MSM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSMR {
    #[doc = "No action"]
    NOACTION,
    #[doc = "The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
    SYNCHRONIZATION,
}
impl MSMR {
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
            MSMR::NOACTION => false,
            MSMR::SYNCHRONIZATION => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSMR {
        match value {
            false => MSMR::NOACTION,
            true => MSMR::SYNCHRONIZATION,
        }
    }
    #[doc = "Checks if the value of the field is `NOACTION`"]
    #[inline]
    pub fn is_no_action(&self) -> bool {
        *self == MSMR::NOACTION
    }
    #[doc = "Checks if the value of the field is `SYNCHRONIZATION`"]
    #[inline]
    pub fn is_synchronization(&self) -> bool {
        *self == MSMR::SYNCHRONIZATION
    }
}
#[doc = "Possible values of the field `TS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSR {
    #[doc = "Internal Trigger 0 (ITR0)"]
    ITR0,
    #[doc = "Internal Trigger 1 (ITR1)"]
    ITR1,
    #[doc = "Internal Trigger 2 (ITR2)"]
    ITR2,
    #[doc = "Internal Trigger 3 (ITR3)"]
    ITR3,
    #[doc = "TI1 Edge Detector"]
    TI1F_ED,
    #[doc = "Filtered Timer Input 1"]
    TI1FP1,
    #[doc = "Filtered Timer Input 2"]
    TI2FP2,
    #[doc = "External Trigger input"]
    ETRF,
}
impl TSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TSR::ITR0 => 0,
            TSR::ITR1 => 1,
            TSR::ITR2 => 2,
            TSR::ITR3 => 3,
            TSR::TI1F_ED => 4,
            TSR::TI1FP1 => 5,
            TSR::TI2FP2 => 6,
            TSR::ETRF => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TSR {
        match value {
            0 => TSR::ITR0,
            1 => TSR::ITR1,
            2 => TSR::ITR2,
            3 => TSR::ITR3,
            4 => TSR::TI1F_ED,
            5 => TSR::TI1FP1,
            6 => TSR::TI2FP2,
            7 => TSR::ETRF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ITR0`"]
    #[inline]
    pub fn is_itr0(&self) -> bool {
        *self == TSR::ITR0
    }
    #[doc = "Checks if the value of the field is `ITR1`"]
    #[inline]
    pub fn is_itr1(&self) -> bool {
        *self == TSR::ITR1
    }
    #[doc = "Checks if the value of the field is `ITR2`"]
    #[inline]
    pub fn is_itr2(&self) -> bool {
        *self == TSR::ITR2
    }
    #[doc = "Checks if the value of the field is `ITR3`"]
    #[inline]
    pub fn is_itr3(&self) -> bool {
        *self == TSR::ITR3
    }
    #[doc = "Checks if the value of the field is `TI1F_ED`"]
    #[inline]
    pub fn is_ti1f_ed(&self) -> bool {
        *self == TSR::TI1F_ED
    }
    #[doc = "Checks if the value of the field is `TI1FP1`"]
    #[inline]
    pub fn is_ti1fp1(&self) -> bool {
        *self == TSR::TI1FP1
    }
    #[doc = "Checks if the value of the field is `TI2FP2`"]
    #[inline]
    pub fn is_ti2fp2(&self) -> bool {
        *self == TSR::TI2FP2
    }
    #[doc = "Checks if the value of the field is `ETRF`"]
    #[inline]
    pub fn is_etrf(&self) -> bool {
        *self == TSR::ETRF
    }
}
#[doc = "Possible values of the field `SMS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMSR {
    #[doc = "Counter disabled"]
    DISABLED,
    #[doc = "Encoder mode, count up/down on TI2FP1"]
    ENCODERTI2,
    #[doc = "Encoder mode, count up/down on TI1FP2"]
    ENCODERTI1,
    #[doc = "Encoder mode, count up/down on both TI1FP1 and TI2FP2"]
    ENCODERTI1TI2,
    #[doc = "Rising edge of the selected trigger input (TRGI) reinitializes the counter"]
    RESET,
    #[doc = " The counter clock is enabled when the trigger input (TRGI) is high"]
    GATED,
    #[doc = "The counter starts at a rising edge of the trigger TRGI "]
    TRIGGER,
    #[doc = " Rising edges of the selected trigger (TRGI) clock the counter"]
    EXTERNAL,
}
impl SMSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SMSR::DISABLED => 0,
            SMSR::ENCODERTI2 => 1,
            SMSR::ENCODERTI1 => 2,
            SMSR::ENCODERTI1TI2 => 3,
            SMSR::RESET => 4,
            SMSR::GATED => 5,
            SMSR::TRIGGER => 6,
            SMSR::EXTERNAL => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SMSR {
        match value {
            0 => SMSR::DISABLED,
            1 => SMSR::ENCODERTI2,
            2 => SMSR::ENCODERTI1,
            3 => SMSR::ENCODERTI1TI2,
            4 => SMSR::RESET,
            5 => SMSR::GATED,
            6 => SMSR::TRIGGER,
            7 => SMSR::EXTERNAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SMSR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENCODERTI2`"]
    #[inline]
    pub fn is_encoder_ti2(&self) -> bool {
        *self == SMSR::ENCODERTI2
    }
    #[doc = "Checks if the value of the field is `ENCODERTI1`"]
    #[inline]
    pub fn is_encoder_ti1(&self) -> bool {
        *self == SMSR::ENCODERTI1
    }
    #[doc = "Checks if the value of the field is `ENCODERTI1TI2`"]
    #[inline]
    pub fn is_encoder_ti1ti2(&self) -> bool {
        *self == SMSR::ENCODERTI1TI2
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == SMSR::RESET
    }
    #[doc = "Checks if the value of the field is `GATED`"]
    #[inline]
    pub fn is_gated(&self) -> bool {
        *self == SMSR::GATED
    }
    #[doc = "Checks if the value of the field is `TRIGGER`"]
    #[inline]
    pub fn is_trigger(&self) -> bool {
        *self == SMSR::TRIGGER
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline]
    pub fn is_external(&self) -> bool {
        *self == SMSR::EXTERNAL
    }
}
#[doc = "Values that can be written to the field `ETP`"]
pub enum ETPW {
    #[doc = "ETR is non-inverted, active at high level or rising edge."]
    NONINVERTED,
    #[doc = "ETR is inverted, active at low level or falling edge."]
    INVERTED,
}
impl ETPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ETPW::NONINVERTED => false,
            ETPW::INVERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ETPW<'a> {
    w: &'a mut W,
}
impl<'a> _ETPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ETPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ETR is non-inverted, active at high level or rising edge."]
    #[inline]
    pub fn non_inverted(self) -> &'a mut W {
        self.variant(ETPW::NONINVERTED)
    }
    #[doc = "ETR is inverted, active at low level or falling edge."]
    #[inline]
    pub fn inverted(self) -> &'a mut W {
        self.variant(ETPW::INVERTED)
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
#[doc = "Values that can be written to the field `ECE`"]
pub enum ECEW {
    #[doc = "External clock mode 2 disabled."]
    DISABLED,
    #[doc = "External clock mode 2 enabled.."]
    ENABLED,
}
impl ECEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ECEW::DISABLED => false,
            ECEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ECEW<'a> {
    w: &'a mut W,
}
impl<'a> _ECEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ECEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "External clock mode 2 disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ECEW::DISABLED)
    }
    #[doc = "External clock mode 2 enabled.."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ECEW::ENABLED)
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
#[doc = "Values that can be written to the field `ETPS`"]
pub enum ETPSW {
    #[doc = "Prescaler OFF."]
    OFF,
    #[doc = "ETRP frequency divided by 2."]
    ETRPDIV2,
    #[doc = "ETRP frequency divided by 4."]
    ETRPDIV4,
    #[doc = "ETRP frequency divided by 8."]
    ETRPDIV8,
}
impl ETPSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ETPSW::OFF => 0,
            ETPSW::ETRPDIV2 => 1,
            ETPSW::ETRPDIV4 => 2,
            ETPSW::ETRPDIV8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ETPSW<'a> {
    w: &'a mut W,
}
impl<'a> _ETPSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ETPSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Prescaler OFF."]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(ETPSW::OFF)
    }
    #[doc = "ETRP frequency divided by 2."]
    #[inline]
    pub fn etrpdiv2(self) -> &'a mut W {
        self.variant(ETPSW::ETRPDIV2)
    }
    #[doc = "ETRP frequency divided by 4."]
    #[inline]
    pub fn etrpdiv4(self) -> &'a mut W {
        self.variant(ETPSW::ETRPDIV4)
    }
    #[doc = "ETRP frequency divided by 8."]
    #[inline]
    pub fn etrpdiv8(self) -> &'a mut W {
        self.variant(ETPSW::ETRPDIV8)
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
#[doc = "Values that can be written to the field `ETF`"]
pub enum ETFW {
    #[doc = "No filter, sampling is done at fDTS"]
    NOFILTER,
    #[doc = "fSAMPLING = fCK_INT, N = 2"]
    FCK_INT_N2,
    #[doc = "fSAMPLING = fCK_INT, N = 4"]
    FCK_INT_N4,
    #[doc = "fSAMPLING = fCK_INT, N = 8"]
    FCK_INT_N8,
    #[doc = "fSAMPLING = fDTS / 2, N = 6"]
    FDTSDIV2_N6,
    #[doc = "fSAMPLING = fDTS / 2, N = 8"]
    FDTSDIV2_N8,
    #[doc = "fSAMPLING = fDTS / 4, N = 6"]
    FDTSDIV4_N6,
    #[doc = "fSAMPLING = fDTS / 4, N = 8"]
    FDTSDIV4_N8,
    #[doc = "fSAMPLING = fDTS / 8, N = 6"]
    FDTSDIV8_N6,
    #[doc = "fSAMPLING = fDTS / 8, N = 8"]
    FDTSDIV8_N8,
    #[doc = "fSAMPLING = fDTS / 16, N = 5"]
    FDTSDIV16_N5,
    #[doc = "fSAMPLING = fDTS / 16, N = 6"]
    FDTSDIV16_N6,
    #[doc = "fSAMPLING = fDTS / 16, N = 8"]
    FDTSDIV16_N8,
    #[doc = "fSAMPLING = fDTS / 32, N = 5"]
    FDTSDIV32_N5,
    #[doc = "fSAMPLING = fDTS / 32, N = 6"]
    FDTSDIV32_N6,
    #[doc = "fSAMPLING = fDTS / 32, N = 8"]
    FDTSDIV32_N8,
}
impl ETFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ETFW::NOFILTER => 0,
            ETFW::FCK_INT_N2 => 1,
            ETFW::FCK_INT_N4 => 2,
            ETFW::FCK_INT_N8 => 3,
            ETFW::FDTSDIV2_N6 => 4,
            ETFW::FDTSDIV2_N8 => 5,
            ETFW::FDTSDIV4_N6 => 6,
            ETFW::FDTSDIV4_N8 => 7,
            ETFW::FDTSDIV8_N6 => 8,
            ETFW::FDTSDIV8_N8 => 9,
            ETFW::FDTSDIV16_N5 => 10,
            ETFW::FDTSDIV16_N6 => 11,
            ETFW::FDTSDIV16_N8 => 12,
            ETFW::FDTSDIV32_N5 => 13,
            ETFW::FDTSDIV32_N6 => 14,
            ETFW::FDTSDIV32_N8 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ETFW<'a> {
    w: &'a mut W,
}
impl<'a> _ETFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ETFW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No filter, sampling is done at fDTS"]
    #[inline]
    pub fn no_filter(self) -> &'a mut W {
        self.variant(ETFW::NOFILTER)
    }
    #[doc = "fSAMPLING = fCK_INT, N = 2"]
    #[inline]
    pub fn f_ck_int_n2(self) -> &'a mut W {
        self.variant(ETFW::FCK_INT_N2)
    }
    #[doc = "fSAMPLING = fCK_INT, N = 4"]
    #[inline]
    pub fn f_ck_int_n4(self) -> &'a mut W {
        self.variant(ETFW::FCK_INT_N4)
    }
    #[doc = "fSAMPLING = fCK_INT, N = 8"]
    #[inline]
    pub fn f_ck_int_n8(self) -> &'a mut W {
        self.variant(ETFW::FCK_INT_N8)
    }
    #[doc = "fSAMPLING = fDTS / 2, N = 6"]
    #[inline]
    pub fn f_dtsdiv2_n6(self) -> &'a mut W {
        self.variant(ETFW::FDTSDIV2_N6)
    }
    #[doc = "fSAMPLING = fDTS / 2, N = 8"]
    #[inline]
    pub fn f_dtsdiv2_n8(self) -> &'a mut W {
        self.variant(ETFW::FDTSDIV2_N8)
    }
    #[doc = "fSAMPLING = fDTS / 4, N = 6"]
    #[inline]
    pub fn f_dtsdiv4_n6(self) -> &'a mut W {
        self.variant(ETFW::FDTSDIV4_N6)
    }
    #[doc = "fSAMPLING = fDTS / 4, N = 8"]
    #[inline]
    pub fn f_dtsdiv4_n8(self) -> &'a mut W {
        self.variant(ETFW::FDTSDIV4_N8)
    }
    #[doc = "fSAMPLING = fDTS / 8, N = 6"]
    #[inline]
    pub fn f_dtsdiv8_n6(self) -> &'a mut W {
        self.variant(ETFW::FDTSDIV8_N6)
    }
    #[doc = "fSAMPLING = fDTS / 8, N = 8"]
    #[inline]
    pub fn f_dtsdiv8_n8(self) -> &'a mut W {
        self.variant(ETFW::FDTSDIV8_N8)
    }
    #[doc = "fSAMPLING = fDTS / 16, N = 5"]
    #[inline]
    pub fn f_dtsdiv16_n5(self) -> &'a mut W {
        self.variant(ETFW::FDTSDIV16_N5)
    }
    #[doc = "fSAMPLING = fDTS / 16, N = 6"]
    #[inline]
    pub fn f_dtsdiv16_n6(self) -> &'a mut W {
        self.variant(ETFW::FDTSDIV16_N6)
    }
    #[doc = "fSAMPLING = fDTS / 16, N = 8"]
    #[inline]
    pub fn f_dtsdiv16_n8(self) -> &'a mut W {
        self.variant(ETFW::FDTSDIV16_N8)
    }
    #[doc = "fSAMPLING = fDTS / 32, N = 5"]
    #[inline]
    pub fn f_dtsdiv32_n5(self) -> &'a mut W {
        self.variant(ETFW::FDTSDIV32_N5)
    }
    #[doc = "fSAMPLING = fDTS / 32, N = 6"]
    #[inline]
    pub fn f_dtsdiv32_n6(self) -> &'a mut W {
        self.variant(ETFW::FDTSDIV32_N6)
    }
    #[doc = "fSAMPLING = fDTS / 32, N = 8"]
    #[inline]
    pub fn f_dtsdiv32_n8(self) -> &'a mut W {
        self.variant(ETFW::FDTSDIV32_N8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MSM`"]
pub enum MSMW {
    #[doc = "No action"]
    NOACTION,
    #[doc = "The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
    SYNCHRONIZATION,
}
impl MSMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSMW::NOACTION => false,
            MSMW::SYNCHRONIZATION => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSMW<'a> {
    w: &'a mut W,
}
impl<'a> _MSMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn no_action(self) -> &'a mut W {
        self.variant(MSMW::NOACTION)
    }
    #[doc = "The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
    #[inline]
    pub fn synchronization(self) -> &'a mut W {
        self.variant(MSMW::SYNCHRONIZATION)
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
#[doc = "Values that can be written to the field `TS`"]
pub enum TSW {
    #[doc = "Internal Trigger 0 (ITR0)"]
    ITR0,
    #[doc = "Internal Trigger 1 (ITR1)"]
    ITR1,
    #[doc = "Internal Trigger 2 (ITR2)"]
    ITR2,
    #[doc = "Internal Trigger 3 (ITR3)"]
    ITR3,
    #[doc = "TI1 Edge Detector"]
    TI1F_ED,
    #[doc = "Filtered Timer Input 1"]
    TI1FP1,
    #[doc = "Filtered Timer Input 2"]
    TI2FP2,
    #[doc = "External Trigger input"]
    ETRF,
}
impl TSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TSW::ITR0 => 0,
            TSW::ITR1 => 1,
            TSW::ITR2 => 2,
            TSW::ITR3 => 3,
            TSW::TI1F_ED => 4,
            TSW::TI1FP1 => 5,
            TSW::TI2FP2 => 6,
            TSW::ETRF => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSW<'a> {
    w: &'a mut W,
}
impl<'a> _TSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Internal Trigger 0 (ITR0)"]
    #[inline]
    pub fn itr0(self) -> &'a mut W {
        self.variant(TSW::ITR0)
    }
    #[doc = "Internal Trigger 1 (ITR1)"]
    #[inline]
    pub fn itr1(self) -> &'a mut W {
        self.variant(TSW::ITR1)
    }
    #[doc = "Internal Trigger 2 (ITR2)"]
    #[inline]
    pub fn itr2(self) -> &'a mut W {
        self.variant(TSW::ITR2)
    }
    #[doc = "Internal Trigger 3 (ITR3)"]
    #[inline]
    pub fn itr3(self) -> &'a mut W {
        self.variant(TSW::ITR3)
    }
    #[doc = "TI1 Edge Detector"]
    #[inline]
    pub fn ti1f_ed(self) -> &'a mut W {
        self.variant(TSW::TI1F_ED)
    }
    #[doc = "Filtered Timer Input 1"]
    #[inline]
    pub fn ti1fp1(self) -> &'a mut W {
        self.variant(TSW::TI1FP1)
    }
    #[doc = "Filtered Timer Input 2"]
    #[inline]
    pub fn ti2fp2(self) -> &'a mut W {
        self.variant(TSW::TI2FP2)
    }
    #[doc = "External Trigger input"]
    #[inline]
    pub fn etrf(self) -> &'a mut W {
        self.variant(TSW::ETRF)
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
#[doc = "Values that can be written to the field `SMS`"]
pub enum SMSW {
    #[doc = "Counter disabled"]
    DISABLED,
    #[doc = "Encoder mode, count up/down on TI2FP1"]
    ENCODERTI2,
    #[doc = "Encoder mode, count up/down on TI1FP2"]
    ENCODERTI1,
    #[doc = "Encoder mode, count up/down on both TI1FP1 and TI2FP2"]
    ENCODERTI1TI2,
    #[doc = "Rising edge of the selected trigger input (TRGI) reinitializes the counter"]
    RESET,
    #[doc = " The counter clock is enabled when the trigger input (TRGI) is high"]
    GATED,
    #[doc = "The counter starts at a rising edge of the trigger TRGI "]
    TRIGGER,
    #[doc = " Rising edges of the selected trigger (TRGI) clock the counter"]
    EXTERNAL,
}
impl SMSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SMSW::DISABLED => 0,
            SMSW::ENCODERTI2 => 1,
            SMSW::ENCODERTI1 => 2,
            SMSW::ENCODERTI1TI2 => 3,
            SMSW::RESET => 4,
            SMSW::GATED => 5,
            SMSW::TRIGGER => 6,
            SMSW::EXTERNAL => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMSW<'a> {
    w: &'a mut W,
}
impl<'a> _SMSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Counter disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SMSW::DISABLED)
    }
    #[doc = "Encoder mode, count up/down on TI2FP1"]
    #[inline]
    pub fn encoder_ti2(self) -> &'a mut W {
        self.variant(SMSW::ENCODERTI2)
    }
    #[doc = "Encoder mode, count up/down on TI1FP2"]
    #[inline]
    pub fn encoder_ti1(self) -> &'a mut W {
        self.variant(SMSW::ENCODERTI1)
    }
    #[doc = "Encoder mode, count up/down on both TI1FP1 and TI2FP2"]
    #[inline]
    pub fn encoder_ti1ti2(self) -> &'a mut W {
        self.variant(SMSW::ENCODERTI1TI2)
    }
    #[doc = "Rising edge of the selected trigger input (TRGI) reinitializes the counter"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(SMSW::RESET)
    }
    #[doc = "The counter clock is enabled when the trigger input (TRGI) is high"]
    #[inline]
    pub fn gated(self) -> &'a mut W {
        self.variant(SMSW::GATED)
    }
    #[doc = "The counter starts at a rising edge of the trigger TRGI"]
    #[inline]
    pub fn trigger(self) -> &'a mut W {
        self.variant(SMSW::TRIGGER)
    }
    #[doc = "Rising edges of the selected trigger (TRGI) clock the counter"]
    #[inline]
    pub fn external(self) -> &'a mut W {
        self.variant(SMSW::EXTERNAL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bit 15 - External trigger polarity"]
    #[inline]
    pub fn etp(&self) -> ETPR {
        ETPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - External clock enable"]
    #[inline]
    pub fn ece(&self) -> ECER {
        ECER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 12:13 - External trigger prescaler"]
    #[inline]
    pub fn etps(&self) -> ETPSR {
        ETPSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - External trigger filter"]
    #[inline]
    pub fn etf(&self) -> ETFR {
        ETFR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline]
    pub fn msm(&self) -> MSMR {
        MSMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline]
    pub fn ts(&self) -> TSR {
        TSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline]
    pub fn sms(&self) -> SMSR {
        SMSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bit 15 - External trigger polarity"]
    #[inline]
    pub fn etp(&mut self) -> _ETPW {
        _ETPW { w: self }
    }
    #[doc = "Bit 14 - External clock enable"]
    #[inline]
    pub fn ece(&mut self) -> _ECEW {
        _ECEW { w: self }
    }
    #[doc = "Bits 12:13 - External trigger prescaler"]
    #[inline]
    pub fn etps(&mut self) -> _ETPSW {
        _ETPSW { w: self }
    }
    #[doc = "Bits 8:11 - External trigger filter"]
    #[inline]
    pub fn etf(&mut self) -> _ETFW {
        _ETFW { w: self }
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline]
    pub fn msm(&mut self) -> _MSMW {
        _MSMW { w: self }
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline]
    pub fn ts(&mut self) -> _TSW {
        _TSW { w: self }
    }
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline]
    pub fn sms(&mut self) -> _SMSW {
        _SMSW { w: self }
    }
}
