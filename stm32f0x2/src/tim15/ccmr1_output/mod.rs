#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCMR1_OUTPUT {
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
#[doc = "Possible values of the field `OC2M`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OC2MR {# [ doc = "Frozen" ] FROZEN , # [ doc = "Set channel 2 to active level on match. OC2REF signal is forced high when the counter TIMx_CNT matches the capture/compare register 2 (TIMx_CCR2)." ] CH2ACTIVEONMATCH , # [ doc = "Set channel 2 to inactive level on match." ] CH2INACTIVEONMATCH , # [ doc = "Toggle - OC2REF toggles when TIMx_CNT=TIMx_CCR2." ] TOGGLE , # [ doc = "Force inactive level - OC2REF is forced low." ] FORCEDLOW , # [ doc = "Force active level - OC2REF is forced high." ] FORCEDHIGH , # [ doc = "PWM Mode 1" ] PWMMODE1 , # [ doc = "PWM Mode 2" ] PWMMODE2 ,}
impl OC2MR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OC2MR::FROZEN => 0,
            OC2MR::CH2ACTIVEONMATCH => 1,
            OC2MR::CH2INACTIVEONMATCH => 2,
            OC2MR::TOGGLE => 3,
            OC2MR::FORCEDLOW => 4,
            OC2MR::FORCEDHIGH => 5,
            OC2MR::PWMMODE1 => 6,
            OC2MR::PWMMODE2 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OC2MR {
        match value {
            0 => OC2MR::FROZEN,
            1 => OC2MR::CH2ACTIVEONMATCH,
            2 => OC2MR::CH2INACTIVEONMATCH,
            3 => OC2MR::TOGGLE,
            4 => OC2MR::FORCEDLOW,
            5 => OC2MR::FORCEDHIGH,
            6 => OC2MR::PWMMODE1,
            7 => OC2MR::PWMMODE2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FROZEN`"]
    #[inline]
    pub fn is_frozen(&self) -> bool {
        *self == OC2MR::FROZEN
    }
    #[doc = "Checks if the value of the field is `CH2ACTIVEONMATCH`"]
    #[inline]
    pub fn is_ch2active_on_match(&self) -> bool {
        *self == OC2MR::CH2ACTIVEONMATCH
    }
    #[doc = "Checks if the value of the field is `CH2INACTIVEONMATCH`"]
    #[inline]
    pub fn is_ch2inactive_on_match(&self) -> bool {
        *self == OC2MR::CH2INACTIVEONMATCH
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline]
    pub fn is_toggle(&self) -> bool {
        *self == OC2MR::TOGGLE
    }
    #[doc = "Checks if the value of the field is `FORCEDLOW`"]
    #[inline]
    pub fn is_forced_low(&self) -> bool {
        *self == OC2MR::FORCEDLOW
    }
    #[doc = "Checks if the value of the field is `FORCEDHIGH`"]
    #[inline]
    pub fn is_forced_high(&self) -> bool {
        *self == OC2MR::FORCEDHIGH
    }
    #[doc = "Checks if the value of the field is `PWMMODE1`"]
    #[inline]
    pub fn is_pwmmode1(&self) -> bool {
        *self == OC2MR::PWMMODE1
    }
    #[doc = "Checks if the value of the field is `PWMMODE2`"]
    #[inline]
    pub fn is_pwmmode2(&self) -> bool {
        *self == OC2MR::PWMMODE2
    }
}
#[doc = "Possible values of the field `OC2PE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OC2PER {# [ doc = "Preload register on TIMx_CCR2 disabled. TIMx_CCR2 can be written at anytime, the new value is taken in account immediately" ] DISABLED , # [ doc = "Preload register on TIMx_CCR2 enabled. Read/Write operations access the preload register. TIMx_CCR2 preload value is loaded in the active register at each update event." ] ENABLED ,}
impl OC2PER {
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
            OC2PER::DISABLED => false,
            OC2PER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OC2PER {
        match value {
            false => OC2PER::DISABLED,
            true => OC2PER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == OC2PER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == OC2PER::ENABLED
    }
}
#[doc = "Possible values of the field `OC2FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OC2FER {# [ doc = "CC2 behaves normally depending on counter and CCR1 values even when the trigger is ON. The minimum delay to activate CC2 output when an edge occurs on the trigger input is 5 clock cycles" ] DISABLED , # [ doc = "An active edge on the trigger input acts like a compare match on CC2 output. Then, OC is set to the compare level independently from the result of the comparison. Delay to sample the trigger input and to activate CC2 output is reduced to 3 clock cycles. OCFE acts only if the channel is configured in PWM1 or PWM2 mode." ] ENABLED ,}
impl OC2FER {
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
            OC2FER::DISABLED => false,
            OC2FER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OC2FER {
        match value {
            false => OC2FER::DISABLED,
            true => OC2FER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == OC2FER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == OC2FER::ENABLED
    }
}
#[doc = "Possible values of the field `CC2S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC2SR {
    #[doc = "CC2 channel is configured as output"] CC2OUTPUT,
    #[doc = "CC2 channel is configured as input, IC2 is mapped on TI2"] IC2MAPPEDTI2,
    #[doc = "CC2 channel is configured as input, IC2 is mapped on TI1"] IC2MAPPEDTI1,
    #[doc = "CC2 channel is configured as input, IC2 is mapped on TRC"] IC2MAPPEDTRC,
}
impl CC2SR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CC2SR::CC2OUTPUT => 0,
            CC2SR::IC2MAPPEDTI2 => 1,
            CC2SR::IC2MAPPEDTI1 => 2,
            CC2SR::IC2MAPPEDTRC => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CC2SR {
        match value {
            0 => CC2SR::CC2OUTPUT,
            1 => CC2SR::IC2MAPPEDTI2,
            2 => CC2SR::IC2MAPPEDTI1,
            3 => CC2SR::IC2MAPPEDTRC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CC2OUTPUT`"]
    #[inline]
    pub fn is_cc2output(&self) -> bool {
        *self == CC2SR::CC2OUTPUT
    }
    #[doc = "Checks if the value of the field is `IC2MAPPEDTI2`"]
    #[inline]
    pub fn is_ic2mapped_ti2(&self) -> bool {
        *self == CC2SR::IC2MAPPEDTI2
    }
    #[doc = "Checks if the value of the field is `IC2MAPPEDTI1`"]
    #[inline]
    pub fn is_ic2mapped_ti1(&self) -> bool {
        *self == CC2SR::IC2MAPPEDTI1
    }
    #[doc = "Checks if the value of the field is `IC2MAPPEDTRC`"]
    #[inline]
    pub fn is_ic2mapped_trc(&self) -> bool {
        *self == CC2SR::IC2MAPPEDTRC
    }
}
#[doc = "Possible values of the field `OC1M`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OC1MR {# [ doc = "The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs(" ] FROZEN , # [ doc = "Set channel y to active level on match. OCyREF signal is forced high when the counter TIMx_CNT matches the capture/compare register y (TIMx_CCRy)." ] SETACTIVE , # [ doc = "Set channel y to inactive level on match. OCyREF signal is forced low when the counter TIMx_CNT matches the capture/compare register y (TIMx_CCRy)." ] SETINACTIVE , # [ doc = "OCyREF toggles when TIMx_CNT=TIMx_CCRy." ] TOGGLE , # [ doc = "OCyREF is forced low." ] FORCEINACTIVE , # [ doc = "OCyREF is forced high." ] FORCEACTIVE , # [ doc = "In upcounting, channel 1 is active." ] PWMMODE1 , # [ doc = "In upcounting, channel y is inactive." ] PWMMODE2 ,}
impl OC1MR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OC1MR::FROZEN => 0,
            OC1MR::SETACTIVE => 1,
            OC1MR::SETINACTIVE => 2,
            OC1MR::TOGGLE => 3,
            OC1MR::FORCEINACTIVE => 4,
            OC1MR::FORCEACTIVE => 5,
            OC1MR::PWMMODE1 => 6,
            OC1MR::PWMMODE2 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OC1MR {
        match value {
            0 => OC1MR::FROZEN,
            1 => OC1MR::SETACTIVE,
            2 => OC1MR::SETINACTIVE,
            3 => OC1MR::TOGGLE,
            4 => OC1MR::FORCEINACTIVE,
            5 => OC1MR::FORCEACTIVE,
            6 => OC1MR::PWMMODE1,
            7 => OC1MR::PWMMODE2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FROZEN`"]
    #[inline]
    pub fn is_frozen(&self) -> bool {
        *self == OC1MR::FROZEN
    }
    #[doc = "Checks if the value of the field is `SETACTIVE`"]
    #[inline]
    pub fn is_set_active(&self) -> bool {
        *self == OC1MR::SETACTIVE
    }
    #[doc = "Checks if the value of the field is `SETINACTIVE`"]
    #[inline]
    pub fn is_set_inactive(&self) -> bool {
        *self == OC1MR::SETINACTIVE
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline]
    pub fn is_toggle(&self) -> bool {
        *self == OC1MR::TOGGLE
    }
    #[doc = "Checks if the value of the field is `FORCEINACTIVE`"]
    #[inline]
    pub fn is_force_inactive(&self) -> bool {
        *self == OC1MR::FORCEINACTIVE
    }
    #[doc = "Checks if the value of the field is `FORCEACTIVE`"]
    #[inline]
    pub fn is_force_active(&self) -> bool {
        *self == OC1MR::FORCEACTIVE
    }
    #[doc = "Checks if the value of the field is `PWMMODE1`"]
    #[inline]
    pub fn is_pwmmode1(&self) -> bool {
        *self == OC1MR::PWMMODE1
    }
    #[doc = "Checks if the value of the field is `PWMMODE2`"]
    #[inline]
    pub fn is_pwmmode2(&self) -> bool {
        *self == OC1MR::PWMMODE2
    }
}
#[doc = "Possible values of the field `OC1PE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OC1PER {# [ doc = "Preload register on TIMx_CCR1 disabled. TIMx_CCR1 can be written at anytime, the new value is taken in account immediately" ] DISABLED , # [ doc = "Preload register on TIMx_CCR1 enabled. Read/Write operations access the preload register. TIMx_CCR1 preload value is loaded in the active register at each update event." ] ENABLED ,}
impl OC1PER {
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
            OC1PER::DISABLED => false,
            OC1PER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OC1PER {
        match value {
            false => OC1PER::DISABLED,
            true => OC1PER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == OC1PER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == OC1PER::ENABLED
    }
}
#[doc = "Possible values of the field `OC1FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OC1FER {# [ doc = "CC1 behaves normally depending on counter and CCR1 values even when the trigger is ON. The minimum delay to activate CC1 output when an edge occurs on the trigger input is 5 clock cycles." ] DISABLED , # [ doc = "An active edge on the trigger input acts like a compare match on CC1 output. Then, OC is set to the compare level independently from the result of the comparison. Delay to sample the trigger input and to activate CC1 output is reduced to 3 clock cycles. OCFE acts only if the channel is configured in PWM1 or PWM2 mode." ] ENABLED ,}
impl OC1FER {
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
            OC1FER::DISABLED => false,
            OC1FER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OC1FER {
        match value {
            false => OC1FER::DISABLED,
            true => OC1FER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == OC1FER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == OC1FER::ENABLED
    }
}
#[doc = "Possible values of the field `CC1S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1SR {
    #[doc = "CC1 channel is configured as output"] CC1OUTPUT,
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI1"] IC1MAPPEDTI1,
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI2"] IC1MAPPEDTI2,
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TRC"] IC1MAPPEDTRC,
}
impl CC1SR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CC1SR::CC1OUTPUT => 0,
            CC1SR::IC1MAPPEDTI1 => 1,
            CC1SR::IC1MAPPEDTI2 => 2,
            CC1SR::IC1MAPPEDTRC => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CC1SR {
        match value {
            0 => CC1SR::CC1OUTPUT,
            1 => CC1SR::IC1MAPPEDTI1,
            2 => CC1SR::IC1MAPPEDTI2,
            3 => CC1SR::IC1MAPPEDTRC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CC1OUTPUT`"]
    #[inline]
    pub fn is_cc1output(&self) -> bool {
        *self == CC1SR::CC1OUTPUT
    }
    #[doc = "Checks if the value of the field is `IC1MAPPEDTI1`"]
    #[inline]
    pub fn is_ic1mapped_ti1(&self) -> bool {
        *self == CC1SR::IC1MAPPEDTI1
    }
    #[doc = "Checks if the value of the field is `IC1MAPPEDTI2`"]
    #[inline]
    pub fn is_ic1mapped_ti2(&self) -> bool {
        *self == CC1SR::IC1MAPPEDTI2
    }
    #[doc = "Checks if the value of the field is `IC1MAPPEDTRC`"]
    #[inline]
    pub fn is_ic1mapped_trc(&self) -> bool {
        *self == CC1SR::IC1MAPPEDTRC
    }
}
#[doc = "Values that can be written to the field `OC2M`"]
pub enum OC2MW {# [ doc = "Frozen" ] FROZEN , # [ doc = "Set channel 2 to active level on match. OC2REF signal is forced high when the counter TIMx_CNT matches the capture/compare register 2 (TIMx_CCR2)." ] CH2ACTIVEONMATCH , # [ doc = "Set channel 2 to inactive level on match." ] CH2INACTIVEONMATCH , # [ doc = "Toggle - OC2REF toggles when TIMx_CNT=TIMx_CCR2." ] TOGGLE , # [ doc = "Force inactive level - OC2REF is forced low." ] FORCEDLOW , # [ doc = "Force active level - OC2REF is forced high." ] FORCEDHIGH , # [ doc = "PWM Mode 1" ] PWMMODE1 , # [ doc = "PWM Mode 2" ] PWMMODE2 ,}
impl OC2MW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OC2MW::FROZEN => 0,
            OC2MW::CH2ACTIVEONMATCH => 1,
            OC2MW::CH2INACTIVEONMATCH => 2,
            OC2MW::TOGGLE => 3,
            OC2MW::FORCEDLOW => 4,
            OC2MW::FORCEDHIGH => 5,
            OC2MW::PWMMODE1 => 6,
            OC2MW::PWMMODE2 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OC2MW<'a> {
    w: &'a mut W,
}
impl<'a> _OC2MW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OC2MW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Frozen"]
    #[inline]
    pub fn frozen(self) -> &'a mut W {
        self.variant(OC2MW::FROZEN)
    }
    # [ doc = "Set channel 2 to active level on match. OC2REF signal is forced high when the counter TIMx_CNT matches the capture/compare register 2 (TIMx_CCR2)." ] # [ inline ]
    pub fn ch2active_on_match(self) -> &'a mut W {
        self.variant(OC2MW::CH2ACTIVEONMATCH)
    }
    #[doc = "Set channel 2 to inactive level on match."]
    #[inline]
    pub fn ch2inactive_on_match(self) -> &'a mut W {
        self.variant(OC2MW::CH2INACTIVEONMATCH)
    }
    #[doc = "Toggle - OC2REF toggles when TIMx_CNT=TIMx_CCR2."]
    #[inline]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OC2MW::TOGGLE)
    }
    #[doc = "Force inactive level - OC2REF is forced low."]
    #[inline]
    pub fn forced_low(self) -> &'a mut W {
        self.variant(OC2MW::FORCEDLOW)
    }
    #[doc = "Force active level - OC2REF is forced high."]
    #[inline]
    pub fn forced_high(self) -> &'a mut W {
        self.variant(OC2MW::FORCEDHIGH)
    }
    #[doc = "PWM Mode 1"]
    #[inline]
    pub fn pwmmode1(self) -> &'a mut W {
        self.variant(OC2MW::PWMMODE1)
    }
    #[doc = "PWM Mode 2"]
    #[inline]
    pub fn pwmmode2(self) -> &'a mut W {
        self.variant(OC2MW::PWMMODE2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OC2PE`"]
pub enum OC2PEW {# [ doc = "Preload register on TIMx_CCR2 disabled. TIMx_CCR2 can be written at anytime, the new value is taken in account immediately" ] DISABLED , # [ doc = "Preload register on TIMx_CCR2 enabled. Read/Write operations access the preload register. TIMx_CCR2 preload value is loaded in the active register at each update event." ] ENABLED ,}
impl OC2PEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OC2PEW::DISABLED => false,
            OC2PEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OC2PEW<'a> {
    w: &'a mut W,
}
impl<'a> _OC2PEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OC2PEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    # [ doc = "Preload register on TIMx_CCR2 disabled. TIMx_CCR2 can be written at anytime, the new value is taken in account immediately" ] # [ inline ]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OC2PEW::DISABLED)
    }
    # [ doc = "Preload register on TIMx_CCR2 enabled. Read/Write operations access the preload register. TIMx_CCR2 preload value is loaded in the active register at each update event." ] # [ inline ]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OC2PEW::ENABLED)
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
#[doc = "Values that can be written to the field `OC2FE`"]
pub enum OC2FEW {# [ doc = "CC2 behaves normally depending on counter and CCR1 values even when the trigger is ON. The minimum delay to activate CC2 output when an edge occurs on the trigger input is 5 clock cycles" ] DISABLED , # [ doc = "An active edge on the trigger input acts like a compare match on CC2 output. Then, OC is set to the compare level independently from the result of the comparison. Delay to sample the trigger input and to activate CC2 output is reduced to 3 clock cycles. OCFE acts only if the channel is configured in PWM1 or PWM2 mode." ] ENABLED ,}
impl OC2FEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OC2FEW::DISABLED => false,
            OC2FEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OC2FEW<'a> {
    w: &'a mut W,
}
impl<'a> _OC2FEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OC2FEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    # [ doc = "CC2 behaves normally depending on counter and CCR1 values even when the trigger is ON. The minimum delay to activate CC2 output when an edge occurs on the trigger input is 5 clock cycles" ] # [ inline ]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OC2FEW::DISABLED)
    }
    # [ doc = "An active edge on the trigger input acts like a compare match on CC2 output. Then, OC is set to the compare level independently from the result of the comparison. Delay to sample the trigger input and to activate CC2 output is reduced to 3 clock cycles. OCFE acts only if the channel is configured in PWM1 or PWM2 mode." ] # [ inline ]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OC2FEW::ENABLED)
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
#[doc = "Values that can be written to the field `CC2S`"]
pub enum CC2SW {
    #[doc = "CC2 channel is configured as output"] CC2OUTPUT,
    #[doc = "CC2 channel is configured as input, IC2 is mapped on TI2"] IC2MAPPEDTI2,
    #[doc = "CC2 channel is configured as input, IC2 is mapped on TI1"] IC2MAPPEDTI1,
    #[doc = "CC2 channel is configured as input, IC2 is mapped on TRC"] IC2MAPPEDTRC,
}
impl CC2SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CC2SW::CC2OUTPUT => 0,
            CC2SW::IC2MAPPEDTI2 => 1,
            CC2SW::IC2MAPPEDTI1 => 2,
            CC2SW::IC2MAPPEDTRC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC2SW<'a> {
    w: &'a mut W,
}
impl<'a> _CC2SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC2SW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CC2 channel is configured as output"]
    #[inline]
    pub fn cc2output(self) -> &'a mut W {
        self.variant(CC2SW::CC2OUTPUT)
    }
    #[doc = "CC2 channel is configured as input, IC2 is mapped on TI2"]
    #[inline]
    pub fn ic2mapped_ti2(self) -> &'a mut W {
        self.variant(CC2SW::IC2MAPPEDTI2)
    }
    #[doc = "CC2 channel is configured as input, IC2 is mapped on TI1"]
    #[inline]
    pub fn ic2mapped_ti1(self) -> &'a mut W {
        self.variant(CC2SW::IC2MAPPEDTI1)
    }
    #[doc = "CC2 channel is configured as input, IC2 is mapped on TRC"]
    #[inline]
    pub fn ic2mapped_trc(self) -> &'a mut W {
        self.variant(CC2SW::IC2MAPPEDTRC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OC1M`"]
pub enum OC1MW {# [ doc = "The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs(" ] FROZEN , # [ doc = "Set channel y to active level on match. OCyREF signal is forced high when the counter TIMx_CNT matches the capture/compare register y (TIMx_CCRy)." ] SETACTIVE , # [ doc = "Set channel y to inactive level on match. OCyREF signal is forced low when the counter TIMx_CNT matches the capture/compare register y (TIMx_CCRy)." ] SETINACTIVE , # [ doc = "OCyREF toggles when TIMx_CNT=TIMx_CCRy." ] TOGGLE , # [ doc = "OCyREF is forced low." ] FORCEINACTIVE , # [ doc = "OCyREF is forced high." ] FORCEACTIVE , # [ doc = "In upcounting, channel 1 is active." ] PWMMODE1 , # [ doc = "In upcounting, channel y is inactive." ] PWMMODE2 ,}
impl OC1MW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OC1MW::FROZEN => 0,
            OC1MW::SETACTIVE => 1,
            OC1MW::SETINACTIVE => 2,
            OC1MW::TOGGLE => 3,
            OC1MW::FORCEINACTIVE => 4,
            OC1MW::FORCEACTIVE => 5,
            OC1MW::PWMMODE1 => 6,
            OC1MW::PWMMODE2 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OC1MW<'a> {
    w: &'a mut W,
}
impl<'a> _OC1MW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OC1MW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    # [ doc = "The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs(" ] # [ inline ]
    pub fn frozen(self) -> &'a mut W {
        self.variant(OC1MW::FROZEN)
    }
    # [ doc = "Set channel y to active level on match. OCyREF signal is forced high when the counter TIMx_CNT matches the capture/compare register y (TIMx_CCRy)." ] # [ inline ]
    pub fn set_active(self) -> &'a mut W {
        self.variant(OC1MW::SETACTIVE)
    }
    # [ doc = "Set channel y to inactive level on match. OCyREF signal is forced low when the counter TIMx_CNT matches the capture/compare register y (TIMx_CCRy)." ] # [ inline ]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(OC1MW::SETINACTIVE)
    }
    #[doc = "OCyREF toggles when TIMx_CNT=TIMx_CCRy."]
    #[inline]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OC1MW::TOGGLE)
    }
    #[doc = "OCyREF is forced low."]
    #[inline]
    pub fn force_inactive(self) -> &'a mut W {
        self.variant(OC1MW::FORCEINACTIVE)
    }
    #[doc = "OCyREF is forced high."]
    #[inline]
    pub fn force_active(self) -> &'a mut W {
        self.variant(OC1MW::FORCEACTIVE)
    }
    #[doc = "In upcounting, channel 1 is active."]
    #[inline]
    pub fn pwmmode1(self) -> &'a mut W {
        self.variant(OC1MW::PWMMODE1)
    }
    #[doc = "In upcounting, channel y is inactive."]
    #[inline]
    pub fn pwmmode2(self) -> &'a mut W {
        self.variant(OC1MW::PWMMODE2)
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
#[doc = "Values that can be written to the field `OC1PE`"]
pub enum OC1PEW {# [ doc = "Preload register on TIMx_CCR1 disabled. TIMx_CCR1 can be written at anytime, the new value is taken in account immediately" ] DISABLED , # [ doc = "Preload register on TIMx_CCR1 enabled. Read/Write operations access the preload register. TIMx_CCR1 preload value is loaded in the active register at each update event." ] ENABLED ,}
impl OC1PEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OC1PEW::DISABLED => false,
            OC1PEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OC1PEW<'a> {
    w: &'a mut W,
}
impl<'a> _OC1PEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OC1PEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    # [ doc = "Preload register on TIMx_CCR1 disabled. TIMx_CCR1 can be written at anytime, the new value is taken in account immediately" ] # [ inline ]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OC1PEW::DISABLED)
    }
    # [ doc = "Preload register on TIMx_CCR1 enabled. Read/Write operations access the preload register. TIMx_CCR1 preload value is loaded in the active register at each update event." ] # [ inline ]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OC1PEW::ENABLED)
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
#[doc = "Values that can be written to the field `OC1FE`"]
pub enum OC1FEW {# [ doc = "CC1 behaves normally depending on counter and CCR1 values even when the trigger is ON. The minimum delay to activate CC1 output when an edge occurs on the trigger input is 5 clock cycles." ] DISABLED , # [ doc = "An active edge on the trigger input acts like a compare match on CC1 output. Then, OC is set to the compare level independently from the result of the comparison. Delay to sample the trigger input and to activate CC1 output is reduced to 3 clock cycles. OCFE acts only if the channel is configured in PWM1 or PWM2 mode." ] ENABLED ,}
impl OC1FEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OC1FEW::DISABLED => false,
            OC1FEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OC1FEW<'a> {
    w: &'a mut W,
}
impl<'a> _OC1FEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OC1FEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    # [ doc = "CC1 behaves normally depending on counter and CCR1 values even when the trigger is ON. The minimum delay to activate CC1 output when an edge occurs on the trigger input is 5 clock cycles." ] # [ inline ]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OC1FEW::DISABLED)
    }
    # [ doc = "An active edge on the trigger input acts like a compare match on CC1 output. Then, OC is set to the compare level independently from the result of the comparison. Delay to sample the trigger input and to activate CC1 output is reduced to 3 clock cycles. OCFE acts only if the channel is configured in PWM1 or PWM2 mode." ] # [ inline ]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OC1FEW::ENABLED)
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
#[doc = "Values that can be written to the field `CC1S`"]
pub enum CC1SW {
    #[doc = "CC1 channel is configured as output"] CC1OUTPUT,
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI1"] IC1MAPPEDTI1,
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI2"] IC1MAPPEDTI2,
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TRC"] IC1MAPPEDTRC,
}
impl CC1SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CC1SW::CC1OUTPUT => 0,
            CC1SW::IC1MAPPEDTI1 => 1,
            CC1SW::IC1MAPPEDTI2 => 2,
            CC1SW::IC1MAPPEDTRC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC1SW<'a> {
    w: &'a mut W,
}
impl<'a> _CC1SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC1SW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CC1 channel is configured as output"]
    #[inline]
    pub fn cc1output(self) -> &'a mut W {
        self.variant(CC1SW::CC1OUTPUT)
    }
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI1"]
    #[inline]
    pub fn ic1mapped_ti1(self) -> &'a mut W {
        self.variant(CC1SW::IC1MAPPEDTI1)
    }
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI2"]
    #[inline]
    pub fn ic1mapped_ti2(self) -> &'a mut W {
        self.variant(CC1SW::IC1MAPPEDTI2)
    }
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TRC"]
    #[inline]
    pub fn ic1mapped_trc(self) -> &'a mut W {
        self.variant(CC1SW::IC1MAPPEDTRC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 12:14 - Output Compare 2 mode"]
    #[inline]
    pub fn oc2m(&self) -> OC2MR {
        OC2MR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - Output Compare 2 preload enable"]
    #[inline]
    pub fn oc2pe(&self) -> OC2PER {
        OC2PER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Output Compare 2 fast enable"]
    #[inline]
    pub fn oc2fe(&self) -> OC2FER {
        OC2FER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:9 - Capture/Compare 2 selection"]
    #[inline]
    pub fn cc2s(&self) -> CC2SR {
        CC2SR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:6 - Output Compare 1 mode"]
    #[inline]
    pub fn oc1m(&self) -> OC1MR {
        OC1MR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Output Compare 1 preload enable"]
    #[inline]
    pub fn oc1pe(&self) -> OC1PER {
        OC1PER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Output Compare 1 fast enable"]
    #[inline]
    pub fn oc1fe(&self) -> OC1FER {
        OC1FER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline]
    pub fn cc1s(&self) -> CC1SR {
        CC1SR::_from({
            const MASK: u8 = 3;
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
    #[doc = "Bits 12:14 - Output Compare 2 mode"]
    #[inline]
    pub fn oc2m(&mut self) -> _OC2MW {
        _OC2MW { w: self }
    }
    #[doc = "Bit 11 - Output Compare 2 preload enable"]
    #[inline]
    pub fn oc2pe(&mut self) -> _OC2PEW {
        _OC2PEW { w: self }
    }
    #[doc = "Bit 10 - Output Compare 2 fast enable"]
    #[inline]
    pub fn oc2fe(&mut self) -> _OC2FEW {
        _OC2FEW { w: self }
    }
    #[doc = "Bits 8:9 - Capture/Compare 2 selection"]
    #[inline]
    pub fn cc2s(&mut self) -> _CC2SW {
        _CC2SW { w: self }
    }
    #[doc = "Bits 4:6 - Output Compare 1 mode"]
    #[inline]
    pub fn oc1m(&mut self) -> _OC1MW {
        _OC1MW { w: self }
    }
    #[doc = "Bit 3 - Output Compare 1 preload enable"]
    #[inline]
    pub fn oc1pe(&mut self) -> _OC1PEW {
        _OC1PEW { w: self }
    }
    #[doc = "Bit 2 - Output Compare 1 fast enable"]
    #[inline]
    pub fn oc1fe(&mut self) -> _OC1FEW {
        _OC1FEW { w: self }
    }
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline]
    pub fn cc1s(&mut self) -> _CC1SW {
        _CC1SW { w: self }
    }
}
