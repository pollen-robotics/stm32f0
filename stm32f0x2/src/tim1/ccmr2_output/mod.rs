#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCMR2_OUTPUT {
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
#[doc = "Possible values of the field `OC4CE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OC4CER {
    #[doc = "OC4Ref is not affected by the ETRF Input"]
    DISABLED,
    #[doc = "OC4Ref is cleared as soon as a High level is detected on ETRF input"]
    ENABLED,
}
impl OC4CER {
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
            OC4CER::DISABLED => false,
            OC4CER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OC4CER {
        match value {
            false => OC4CER::DISABLED,
            true => OC4CER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == OC4CER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == OC4CER::ENABLED
    }
}
#[doc = "Possible values of the field `OC4M`"]
pub type OC4MR = super::ccmr1_output::OC1MR;
#[doc = "Possible values of the field `OC4PE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OC4PER {
    #[doc = "Preload register on TIMx_CCR4 disabled. TIMx_CCR4 can be written at anytime, the new value is taken in account immediately"]
    DISABLED,
    #[doc = "Preload register on TIMx_CCR4 enabled. Read/Write operations access the preload register. TIMx_CCR4 preload value is loaded in the active register at each update event."]
    ENABLED,
}
impl OC4PER {
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
            OC4PER::DISABLED => false,
            OC4PER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OC4PER {
        match value {
            false => OC4PER::DISABLED,
            true => OC4PER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == OC4PER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == OC4PER::ENABLED
    }
}
#[doc = "Possible values of the field `OC4FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OC4FER {
    #[doc = "CC4 behaves normally depending on counter and CCR4 values even when the trigger is ON. The minimum delay to activate CC4 output when an edge occurs on the trigger input is 5 clock cycles."]
    DISABLED,
    #[doc = "An active edge on the trigger input acts like a compare match on CC4 output. Then, OC is set to the compare level independently from the result of the comparison. Delay to sample the trigger input and to activate CC3 output is reduced to 3 clock cycles. OCFE acts only if the channel is configured in PWM1 or PWM2 mode."]
    ENABLED,
}
impl OC4FER {
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
            OC4FER::DISABLED => false,
            OC4FER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OC4FER {
        match value {
            false => OC4FER::DISABLED,
            true => OC4FER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == OC4FER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == OC4FER::ENABLED
    }
}
#[doc = "Possible values of the field `CC4S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC4SR {
    #[doc = "CC4 channel is configured as output"]
    CC4OUTPUT,
    #[doc = "CC4 channel is configured as input, IC4 is mapped on TI4"]
    IC4MAPPEDTI4,
    #[doc = "CC4 channel is configured as input, IC4 is mapped on TI3"]
    IC4MAPPEDTI3,
    #[doc = "CC4 channel is configured as input, IC4 is mapped on TRC"]
    IC4MAPPEDTRC,
}
impl CC4SR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CC4SR::CC4OUTPUT => 0,
            CC4SR::IC4MAPPEDTI4 => 1,
            CC4SR::IC4MAPPEDTI3 => 2,
            CC4SR::IC4MAPPEDTRC => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CC4SR {
        match value {
            0 => CC4SR::CC4OUTPUT,
            1 => CC4SR::IC4MAPPEDTI4,
            2 => CC4SR::IC4MAPPEDTI3,
            3 => CC4SR::IC4MAPPEDTRC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CC4OUTPUT`"]
    #[inline]
    pub fn is_cc4output(&self) -> bool {
        *self == CC4SR::CC4OUTPUT
    }
    #[doc = "Checks if the value of the field is `IC4MAPPEDTI4`"]
    #[inline]
    pub fn is_ic4mapped_ti4(&self) -> bool {
        *self == CC4SR::IC4MAPPEDTI4
    }
    #[doc = "Checks if the value of the field is `IC4MAPPEDTI3`"]
    #[inline]
    pub fn is_ic4mapped_ti3(&self) -> bool {
        *self == CC4SR::IC4MAPPEDTI3
    }
    #[doc = "Checks if the value of the field is `IC4MAPPEDTRC`"]
    #[inline]
    pub fn is_ic4mapped_trc(&self) -> bool {
        *self == CC4SR::IC4MAPPEDTRC
    }
}
#[doc = "Possible values of the field `OC3CE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OC3CER {
    #[doc = "OC3Ref is not affected by the ETRF Input"]
    DISABLED,
    #[doc = "OC3Ref is cleared as soon as a High level is detected on ETRF input"]
    ENABLED,
}
impl OC3CER {
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
            OC3CER::DISABLED => false,
            OC3CER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OC3CER {
        match value {
            false => OC3CER::DISABLED,
            true => OC3CER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == OC3CER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == OC3CER::ENABLED
    }
}
#[doc = "Possible values of the field `OC3M`"]
pub type OC3MR = super::ccmr1_output::OC1MR;
#[doc = "Possible values of the field `OC3PE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OC3PER {
    #[doc = "Preload register on TIMx_CCR3 disabled. TIMx_CCR3 can be written at anytime, the new value is taken in account immediately"]
    DISABLED,
    #[doc = "Preload register on TIMx_CCR3 enabled. Read/Write operations access the preload register. TIMx_CCR3 preload value is loaded in the active register at each update event."]
    ENABLED,
}
impl OC3PER {
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
            OC3PER::DISABLED => false,
            OC3PER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OC3PER {
        match value {
            false => OC3PER::DISABLED,
            true => OC3PER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == OC3PER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == OC3PER::ENABLED
    }
}
#[doc = "Possible values of the field `OC3FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OC3FER {
    #[doc = "CC3 behaves normally depending on counter and CCR3 values even when the trigger is ON. The minimum delay to activate CC3 output when an edge occurs on the trigger input is 5 clock cycles."]
    DISABLED,
    #[doc = "An active edge on the trigger input acts like a compare match on CC3 output. Then, OC is set to the compare level independently from the result of the comparison. Delay to sample the trigger input and to activate CC3 output is reduced to 3 clock cycles. OCFE acts only if the channel is configured in PWM1 or PWM2 mode."]
    ENABLED,
}
impl OC3FER {
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
            OC3FER::DISABLED => false,
            OC3FER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OC3FER {
        match value {
            false => OC3FER::DISABLED,
            true => OC3FER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == OC3FER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == OC3FER::ENABLED
    }
}
#[doc = "Possible values of the field `CC3S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC3SR {
    #[doc = "CC3 channel is configured as output"]
    CC3OUTPUT,
    #[doc = "CC3 channel is configured as input, IC3 is mapped on TI3"]
    IC3MAPPEDTI3,
    #[doc = "CC3 channel is configured as input, IC3 is mapped on TI4"]
    IC3MAPPEDTI4,
    #[doc = "CC3 channel is configured as input, IC3 is mapped on TRC"]
    IC3MAPPEDTRC,
}
impl CC3SR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CC3SR::CC3OUTPUT => 0,
            CC3SR::IC3MAPPEDTI3 => 1,
            CC3SR::IC3MAPPEDTI4 => 2,
            CC3SR::IC3MAPPEDTRC => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CC3SR {
        match value {
            0 => CC3SR::CC3OUTPUT,
            1 => CC3SR::IC3MAPPEDTI3,
            2 => CC3SR::IC3MAPPEDTI4,
            3 => CC3SR::IC3MAPPEDTRC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CC3OUTPUT`"]
    #[inline]
    pub fn is_cc3output(&self) -> bool {
        *self == CC3SR::CC3OUTPUT
    }
    #[doc = "Checks if the value of the field is `IC3MAPPEDTI3`"]
    #[inline]
    pub fn is_ic3mapped_ti3(&self) -> bool {
        *self == CC3SR::IC3MAPPEDTI3
    }
    #[doc = "Checks if the value of the field is `IC3MAPPEDTI4`"]
    #[inline]
    pub fn is_ic3mapped_ti4(&self) -> bool {
        *self == CC3SR::IC3MAPPEDTI4
    }
    #[doc = "Checks if the value of the field is `IC3MAPPEDTRC`"]
    #[inline]
    pub fn is_ic3mapped_trc(&self) -> bool {
        *self == CC3SR::IC3MAPPEDTRC
    }
}
#[doc = "Values that can be written to the field `OC4CE`"]
pub enum OC4CEW {
    #[doc = "OC4Ref is not affected by the ETRF Input"]
    DISABLED,
    #[doc = "OC4Ref is cleared as soon as a High level is detected on ETRF input"]
    ENABLED,
}
impl OC4CEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OC4CEW::DISABLED => false,
            OC4CEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OC4CEW<'a> {
    w: &'a mut W,
}
impl<'a> _OC4CEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OC4CEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "OC4Ref is not affected by the ETRF Input"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OC4CEW::DISABLED)
    }
    #[doc = "OC4Ref is cleared as soon as a High level is detected on ETRF input"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OC4CEW::ENABLED)
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
#[doc = "Values that can be written to the field `OC4M`"]
pub type OC4MW = super::ccmr1_output::OC1MW;
#[doc = r" Proxy"]
pub struct _OC4MW<'a> {
    w: &'a mut W,
}
impl<'a> _OC4MW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OC4MW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs("]
    #[inline]
    pub fn frozen(self) -> &'a mut W {
        self.variant(super::ccmr1_output::OC1MW::FROZEN)
    }
    #[doc = "Set channel y to active level on match. OCyREF signal is forced high when the counter TIMx_CNT matches the capture/compare register y (TIMx_CCRy)."]
    #[inline]
    pub fn set_active(self) -> &'a mut W {
        self.variant(super::ccmr1_output::OC1MW::SETACTIVE)
    }
    #[doc = "Set channel y to inactive level on match. OCyREF signal is forced low when the counter TIMx_CNT matches the capture/compare register y (TIMx_CCRy)."]
    #[inline]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(super::ccmr1_output::OC1MW::SETINACTIVE)
    }
    #[doc = "OCyREF toggles when TIMx_CNT=TIMx_CCRy."]
    #[inline]
    pub fn toggle(self) -> &'a mut W {
        self.variant(super::ccmr1_output::OC1MW::TOGGLE)
    }
    #[doc = "OCyREF is forced low."]
    #[inline]
    pub fn force_inactive(self) -> &'a mut W {
        self.variant(super::ccmr1_output::OC1MW::FORCEINACTIVE)
    }
    #[doc = "OCyREF is forced high."]
    #[inline]
    pub fn force_active(self) -> &'a mut W {
        self.variant(super::ccmr1_output::OC1MW::FORCEACTIVE)
    }
    #[doc = "In upcounting, channel 1 is active."]
    #[inline]
    pub fn pwmmode1(self) -> &'a mut W {
        self.variant(super::ccmr1_output::OC1MW::PWMMODE1)
    }
    #[doc = "In upcounting, channel y is inactive."]
    #[inline]
    pub fn pwmmode2(self) -> &'a mut W {
        self.variant(super::ccmr1_output::OC1MW::PWMMODE2)
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
#[doc = "Values that can be written to the field `OC4PE`"]
pub enum OC4PEW {
    #[doc = "Preload register on TIMx_CCR4 disabled. TIMx_CCR4 can be written at anytime, the new value is taken in account immediately"]
    DISABLED,
    #[doc = "Preload register on TIMx_CCR4 enabled. Read/Write operations access the preload register. TIMx_CCR4 preload value is loaded in the active register at each update event."]
    ENABLED,
}
impl OC4PEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OC4PEW::DISABLED => false,
            OC4PEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OC4PEW<'a> {
    w: &'a mut W,
}
impl<'a> _OC4PEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OC4PEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Preload register on TIMx_CCR4 disabled. TIMx_CCR4 can be written at anytime, the new value is taken in account immediately"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OC4PEW::DISABLED)
    }
    #[doc = "Preload register on TIMx_CCR4 enabled. Read/Write operations access the preload register. TIMx_CCR4 preload value is loaded in the active register at each update event."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OC4PEW::ENABLED)
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
#[doc = "Values that can be written to the field `OC4FE`"]
pub enum OC4FEW {
    #[doc = "CC4 behaves normally depending on counter and CCR4 values even when the trigger is ON. The minimum delay to activate CC4 output when an edge occurs on the trigger input is 5 clock cycles."]
    DISABLED,
    #[doc = "An active edge on the trigger input acts like a compare match on CC4 output. Then, OC is set to the compare level independently from the result of the comparison. Delay to sample the trigger input and to activate CC3 output is reduced to 3 clock cycles. OCFE acts only if the channel is configured in PWM1 or PWM2 mode."]
    ENABLED,
}
impl OC4FEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OC4FEW::DISABLED => false,
            OC4FEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OC4FEW<'a> {
    w: &'a mut W,
}
impl<'a> _OC4FEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OC4FEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CC4 behaves normally depending on counter and CCR4 values even when the trigger is ON. The minimum delay to activate CC4 output when an edge occurs on the trigger input is 5 clock cycles."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OC4FEW::DISABLED)
    }
    #[doc = "An active edge on the trigger input acts like a compare match on CC4 output. Then, OC is set to the compare level independently from the result of the comparison. Delay to sample the trigger input and to activate CC3 output is reduced to 3 clock cycles. OCFE acts only if the channel is configured in PWM1 or PWM2 mode."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OC4FEW::ENABLED)
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
#[doc = "Values that can be written to the field `CC4S`"]
pub enum CC4SW {
    #[doc = "CC4 channel is configured as output"]
    CC4OUTPUT,
    #[doc = "CC4 channel is configured as input, IC4 is mapped on TI4"]
    IC4MAPPEDTI4,
    #[doc = "CC4 channel is configured as input, IC4 is mapped on TI3"]
    IC4MAPPEDTI3,
    #[doc = "CC4 channel is configured as input, IC4 is mapped on TRC"]
    IC4MAPPEDTRC,
}
impl CC4SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CC4SW::CC4OUTPUT => 0,
            CC4SW::IC4MAPPEDTI4 => 1,
            CC4SW::IC4MAPPEDTI3 => 2,
            CC4SW::IC4MAPPEDTRC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC4SW<'a> {
    w: &'a mut W,
}
impl<'a> _CC4SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC4SW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CC4 channel is configured as output"]
    #[inline]
    pub fn cc4output(self) -> &'a mut W {
        self.variant(CC4SW::CC4OUTPUT)
    }
    #[doc = "CC4 channel is configured as input, IC4 is mapped on TI4"]
    #[inline]
    pub fn ic4mapped_ti4(self) -> &'a mut W {
        self.variant(CC4SW::IC4MAPPEDTI4)
    }
    #[doc = "CC4 channel is configured as input, IC4 is mapped on TI3"]
    #[inline]
    pub fn ic4mapped_ti3(self) -> &'a mut W {
        self.variant(CC4SW::IC4MAPPEDTI3)
    }
    #[doc = "CC4 channel is configured as input, IC4 is mapped on TRC"]
    #[inline]
    pub fn ic4mapped_trc(self) -> &'a mut W {
        self.variant(CC4SW::IC4MAPPEDTRC)
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
#[doc = "Values that can be written to the field `OC3CE`"]
pub enum OC3CEW {
    #[doc = "OC3Ref is not affected by the ETRF Input"]
    DISABLED,
    #[doc = "OC3Ref is cleared as soon as a High level is detected on ETRF input"]
    ENABLED,
}
impl OC3CEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OC3CEW::DISABLED => false,
            OC3CEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OC3CEW<'a> {
    w: &'a mut W,
}
impl<'a> _OC3CEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OC3CEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "OC3Ref is not affected by the ETRF Input"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OC3CEW::DISABLED)
    }
    #[doc = "OC3Ref is cleared as soon as a High level is detected on ETRF input"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OC3CEW::ENABLED)
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
#[doc = "Values that can be written to the field `OC3M`"]
pub type OC3MW = super::ccmr1_output::OC1MW;
#[doc = r" Proxy"]
pub struct _OC3MW<'a> {
    w: &'a mut W,
}
impl<'a> _OC3MW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OC3MW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs("]
    #[inline]
    pub fn frozen(self) -> &'a mut W {
        self.variant(super::ccmr1_output::OC1MW::FROZEN)
    }
    #[doc = "Set channel y to active level on match. OCyREF signal is forced high when the counter TIMx_CNT matches the capture/compare register y (TIMx_CCRy)."]
    #[inline]
    pub fn set_active(self) -> &'a mut W {
        self.variant(super::ccmr1_output::OC1MW::SETACTIVE)
    }
    #[doc = "Set channel y to inactive level on match. OCyREF signal is forced low when the counter TIMx_CNT matches the capture/compare register y (TIMx_CCRy)."]
    #[inline]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(super::ccmr1_output::OC1MW::SETINACTIVE)
    }
    #[doc = "OCyREF toggles when TIMx_CNT=TIMx_CCRy."]
    #[inline]
    pub fn toggle(self) -> &'a mut W {
        self.variant(super::ccmr1_output::OC1MW::TOGGLE)
    }
    #[doc = "OCyREF is forced low."]
    #[inline]
    pub fn force_inactive(self) -> &'a mut W {
        self.variant(super::ccmr1_output::OC1MW::FORCEINACTIVE)
    }
    #[doc = "OCyREF is forced high."]
    #[inline]
    pub fn force_active(self) -> &'a mut W {
        self.variant(super::ccmr1_output::OC1MW::FORCEACTIVE)
    }
    #[doc = "In upcounting, channel 1 is active."]
    #[inline]
    pub fn pwmmode1(self) -> &'a mut W {
        self.variant(super::ccmr1_output::OC1MW::PWMMODE1)
    }
    #[doc = "In upcounting, channel y is inactive."]
    #[inline]
    pub fn pwmmode2(self) -> &'a mut W {
        self.variant(super::ccmr1_output::OC1MW::PWMMODE2)
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
#[doc = "Values that can be written to the field `OC3PE`"]
pub enum OC3PEW {
    #[doc = "Preload register on TIMx_CCR3 disabled. TIMx_CCR3 can be written at anytime, the new value is taken in account immediately"]
    DISABLED,
    #[doc = "Preload register on TIMx_CCR3 enabled. Read/Write operations access the preload register. TIMx_CCR3 preload value is loaded in the active register at each update event."]
    ENABLED,
}
impl OC3PEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OC3PEW::DISABLED => false,
            OC3PEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OC3PEW<'a> {
    w: &'a mut W,
}
impl<'a> _OC3PEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OC3PEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Preload register on TIMx_CCR3 disabled. TIMx_CCR3 can be written at anytime, the new value is taken in account immediately"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OC3PEW::DISABLED)
    }
    #[doc = "Preload register on TIMx_CCR3 enabled. Read/Write operations access the preload register. TIMx_CCR3 preload value is loaded in the active register at each update event."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OC3PEW::ENABLED)
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
#[doc = "Values that can be written to the field `OC3FE`"]
pub enum OC3FEW {
    #[doc = "CC3 behaves normally depending on counter and CCR3 values even when the trigger is ON. The minimum delay to activate CC3 output when an edge occurs on the trigger input is 5 clock cycles."]
    DISABLED,
    #[doc = "An active edge on the trigger input acts like a compare match on CC3 output. Then, OC is set to the compare level independently from the result of the comparison. Delay to sample the trigger input and to activate CC3 output is reduced to 3 clock cycles. OCFE acts only if the channel is configured in PWM1 or PWM2 mode."]
    ENABLED,
}
impl OC3FEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OC3FEW::DISABLED => false,
            OC3FEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OC3FEW<'a> {
    w: &'a mut W,
}
impl<'a> _OC3FEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OC3FEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CC3 behaves normally depending on counter and CCR3 values even when the trigger is ON. The minimum delay to activate CC3 output when an edge occurs on the trigger input is 5 clock cycles."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OC3FEW::DISABLED)
    }
    #[doc = "An active edge on the trigger input acts like a compare match on CC3 output. Then, OC is set to the compare level independently from the result of the comparison. Delay to sample the trigger input and to activate CC3 output is reduced to 3 clock cycles. OCFE acts only if the channel is configured in PWM1 or PWM2 mode."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OC3FEW::ENABLED)
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
#[doc = "Values that can be written to the field `CC3S`"]
pub enum CC3SW {
    #[doc = "CC3 channel is configured as output"]
    CC3OUTPUT,
    #[doc = "CC3 channel is configured as input, IC3 is mapped on TI3"]
    IC3MAPPEDTI3,
    #[doc = "CC3 channel is configured as input, IC3 is mapped on TI4"]
    IC3MAPPEDTI4,
    #[doc = "CC3 channel is configured as input, IC3 is mapped on TRC"]
    IC3MAPPEDTRC,
}
impl CC3SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CC3SW::CC3OUTPUT => 0,
            CC3SW::IC3MAPPEDTI3 => 1,
            CC3SW::IC3MAPPEDTI4 => 2,
            CC3SW::IC3MAPPEDTRC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC3SW<'a> {
    w: &'a mut W,
}
impl<'a> _CC3SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC3SW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CC3 channel is configured as output"]
    #[inline]
    pub fn cc3output(self) -> &'a mut W {
        self.variant(CC3SW::CC3OUTPUT)
    }
    #[doc = "CC3 channel is configured as input, IC3 is mapped on TI3"]
    #[inline]
    pub fn ic3mapped_ti3(self) -> &'a mut W {
        self.variant(CC3SW::IC3MAPPEDTI3)
    }
    #[doc = "CC3 channel is configured as input, IC3 is mapped on TI4"]
    #[inline]
    pub fn ic3mapped_ti4(self) -> &'a mut W {
        self.variant(CC3SW::IC3MAPPEDTI4)
    }
    #[doc = "CC3 channel is configured as input, IC3 is mapped on TRC"]
    #[inline]
    pub fn ic3mapped_trc(self) -> &'a mut W {
        self.variant(CC3SW::IC3MAPPEDTRC)
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
    #[doc = "Bit 15 - Output compare 4 clear enable"]
    #[inline]
    pub fn oc4ce(&self) -> OC4CER {
        OC4CER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 12:14 - Output compare 4 mode"]
    #[inline]
    pub fn oc4m(&self) -> OC4MR {
        OC4MR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - Output compare 4 preload enable"]
    #[inline]
    pub fn oc4pe(&self) -> OC4PER {
        OC4PER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Output compare 4 fast enable"]
    #[inline]
    pub fn oc4fe(&self) -> OC4FER {
        OC4FER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection"]
    #[inline]
    pub fn cc4s(&self) -> CC4SR {
        CC4SR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Output compare 3 clear enable"]
    #[inline]
    pub fn oc3ce(&self) -> OC3CER {
        OC3CER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:6 - Output compare 3 mode"]
    #[inline]
    pub fn oc3m(&self) -> OC3MR {
        OC3MR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Output compare 3 preload enable"]
    #[inline]
    pub fn oc3pe(&self) -> OC3PER {
        OC3PER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Output compare 3 fast enable"]
    #[inline]
    pub fn oc3fe(&self) -> OC3FER {
        OC3FER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 0:1 - Capture/Compare 3 selection"]
    #[inline]
    pub fn cc3s(&self) -> CC3SR {
        CC3SR::_from({
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
    #[doc = "Bit 15 - Output compare 4 clear enable"]
    #[inline]
    pub fn oc4ce(&mut self) -> _OC4CEW {
        _OC4CEW { w: self }
    }
    #[doc = "Bits 12:14 - Output compare 4 mode"]
    #[inline]
    pub fn oc4m(&mut self) -> _OC4MW {
        _OC4MW { w: self }
    }
    #[doc = "Bit 11 - Output compare 4 preload enable"]
    #[inline]
    pub fn oc4pe(&mut self) -> _OC4PEW {
        _OC4PEW { w: self }
    }
    #[doc = "Bit 10 - Output compare 4 fast enable"]
    #[inline]
    pub fn oc4fe(&mut self) -> _OC4FEW {
        _OC4FEW { w: self }
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection"]
    #[inline]
    pub fn cc4s(&mut self) -> _CC4SW {
        _CC4SW { w: self }
    }
    #[doc = "Bit 7 - Output compare 3 clear enable"]
    #[inline]
    pub fn oc3ce(&mut self) -> _OC3CEW {
        _OC3CEW { w: self }
    }
    #[doc = "Bits 4:6 - Output compare 3 mode"]
    #[inline]
    pub fn oc3m(&mut self) -> _OC3MW {
        _OC3MW { w: self }
    }
    #[doc = "Bit 3 - Output compare 3 preload enable"]
    #[inline]
    pub fn oc3pe(&mut self) -> _OC3PEW {
        _OC3PEW { w: self }
    }
    #[doc = "Bit 2 - Output compare 3 fast enable"]
    #[inline]
    pub fn oc3fe(&mut self) -> _OC3FEW {
        _OC3FEW { w: self }
    }
    #[doc = "Bits 0:1 - Capture/Compare 3 selection"]
    #[inline]
    pub fn cc3s(&mut self) -> _CC3SW {
        _CC3SW { w: self }
    }
}
