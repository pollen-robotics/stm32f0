#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BDTR {
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
#[doc = "Possible values of the field `MOE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOER {
    #[doc = "OC and OCN outputs are disabled or forced to idle state"] DISABLED,
    #[doc = "OC and OCN outputs are enabled if their respective enable bits are set"] ENABLED,
}
impl MOER {
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
            MOER::DISABLED => false,
            MOER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> MOER {
        match value {
            false => MOER::DISABLED,
            true => MOER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MOER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MOER::ENABLED
    }
}
#[doc = "Possible values of the field `AOE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AOER {
    #[doc = "MOE can be set only by software"] DISABLED,
    #[doc = "MOE can be set by software or automatically at the next update event"] ENABLED,
}
impl AOER {
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
            AOER::DISABLED => false,
            AOER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> AOER {
        match value {
            false => AOER::DISABLED,
            true => AOER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AOER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AOER::ENABLED
    }
}
#[doc = "Possible values of the field `BKP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BKPR {
    #[doc = "Break input BRK is active low"] ACTIVELOW,
    #[doc = "Break input BRK is active high"] ACTIVEHIGH,
}
impl BKPR {
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
            BKPR::ACTIVELOW => false,
            BKPR::ACTIVEHIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> BKPR {
        match value {
            false => BKPR::ACTIVELOW,
            true => BKPR::ACTIVEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVELOW`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == BKPR::ACTIVELOW
    }
    #[doc = "Checks if the value of the field is `ACTIVEHIGH`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == BKPR::ACTIVEHIGH
    }
}
#[doc = "Possible values of the field `BKE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BKER {
    #[doc = "Break inputs (BRK and CCS clock failure event) disabled"] DISABLED,
    #[doc = "Break inputs (BRK and CCS clock failure event) enabled"] ENABLED,
}
impl BKER {
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
            BKER::DISABLED => false,
            BKER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> BKER {
        match value {
            false => BKER::DISABLED,
            true => BKER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BKER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BKER::ENABLED
    }
}
#[doc = "Possible values of the field `OSSR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSSRR {
    #[doc = "When inactive, OC/OCN outputs are disabled"] DISABLED,
    #[doc = "When inactive, OC/OCN outputs are enabled"] ENABLED,
}
impl OSSRR {
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
            OSSRR::DISABLED => false,
            OSSRR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> OSSRR {
        match value {
            false => OSSRR::DISABLED,
            true => OSSRR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSSRR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OSSRR::ENABLED
    }
}
#[doc = "Possible values of the field `OSSI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSSIR {
    #[doc = "When inactive, OC/OCN outputs are disabled"] DISABLED,
    #[doc = "When inactive, OC/OCN outputs are forced first with their idle level as soon as CCxE=1 or CCxNE=1. OC/OCN enable output signal=1"] ENABLED,
}
impl OSSIR {
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
            OSSIR::DISABLED => false,
            OSSIR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> OSSIR {
        match value {
            false => OSSIR::DISABLED,
            true => OSSIR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSSIR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OSSIR::ENABLED
    }
}
#[doc = "Possible values of the field `LOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKR {
    #[doc = "LOCK OFF - No bit is write protected."] LOCKOFF,
    #[doc = "LOCK Level 1 = DTG bits in TIMx_BDTR register, OISx and OISxN bits in TIMx_CR2 register and BKE/BKP/AOE bits in TIMx_BDTR register can no longer be written."] LOCKLVL1,
    #[doc = "LOCK Level 2 = LOCK Level 1 + CC Polarity bits (CCxP/CCxNP bits in TIMx_CCER register, as long as the related channel is configured in output through the CCxS bits) as well as OSSR and OSSI bits can no longer be written."] LOCKLVL2,
    #[doc = "LOCK Level 3 = LOCK Level 2 + CC Control bits (OCxM and OCxPE bits in TIMx_CCMRx registers, as long as the related channel is configured in output through the CCxS bits) can no longer be written."] LOCKLVL3,
}
impl LOCKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            LOCKR::LOCKOFF => 0,
            LOCKR::LOCKLVL1 => 1,
            LOCKR::LOCKLVL2 => 2,
            LOCKR::LOCKLVL3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> LOCKR {
        match value {
            0 => LOCKR::LOCKOFF,
            1 => LOCKR::LOCKLVL1,
            2 => LOCKR::LOCKLVL2,
            3 => LOCKR::LOCKLVL3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOCKOFF`"]
    #[inline(always)]
    pub fn is_lock_off(&self) -> bool {
        *self == LOCKR::LOCKOFF
    }
    #[doc = "Checks if the value of the field is `LOCKLVL1`"]
    #[inline(always)]
    pub fn is_lock_lvl1(&self) -> bool {
        *self == LOCKR::LOCKLVL1
    }
    #[doc = "Checks if the value of the field is `LOCKLVL2`"]
    #[inline(always)]
    pub fn is_lock_lvl2(&self) -> bool {
        *self == LOCKR::LOCKLVL2
    }
    #[doc = "Checks if the value of the field is `LOCKLVL3`"]
    #[inline(always)]
    pub fn is_lock_lvl3(&self) -> bool {
        *self == LOCKR::LOCKLVL3
    }
}
#[doc = r" Value of the field"]
pub struct DTGR {
    bits: u8,
}
impl DTGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `MOE`"]
pub enum MOEW {
    #[doc = "OC and OCN outputs are disabled or forced to idle state"] DISABLED,
    #[doc = "OC and OCN outputs are enabled if their respective enable bits are set"] ENABLED,
}
impl MOEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MOEW::DISABLED => false,
            MOEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MOEW<'a> {
    w: &'a mut W,
}
impl<'a> _MOEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MOEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "OC and OCN outputs are disabled or forced to idle state"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MOEW::DISABLED)
    }
    #[doc = "OC and OCN outputs are enabled if their respective enable bits are set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MOEW::ENABLED)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AOE`"]
pub enum AOEW {
    #[doc = "MOE can be set only by software"] DISABLED,
    #[doc = "MOE can be set by software or automatically at the next update event"] ENABLED,
}
impl AOEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            AOEW::DISABLED => false,
            AOEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AOEW<'a> {
    w: &'a mut W,
}
impl<'a> _AOEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AOEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MOE can be set only by software"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AOEW::DISABLED)
    }
    #[doc = "MOE can be set by software or automatically at the next update event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AOEW::ENABLED)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BKP`"]
pub enum BKPW {
    #[doc = "Break input BRK is active low"] ACTIVELOW,
    #[doc = "Break input BRK is active high"] ACTIVEHIGH,
}
impl BKPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            BKPW::ACTIVELOW => false,
            BKPW::ACTIVEHIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BKPW<'a> {
    w: &'a mut W,
}
impl<'a> _BKPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BKPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Break input BRK is active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(BKPW::ACTIVELOW)
    }
    #[doc = "Break input BRK is active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(BKPW::ACTIVEHIGH)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BKE`"]
pub enum BKEW {
    #[doc = "Break inputs (BRK and CCS clock failure event) disabled"] DISABLED,
    #[doc = "Break inputs (BRK and CCS clock failure event) enabled"] ENABLED,
}
impl BKEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            BKEW::DISABLED => false,
            BKEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BKEW<'a> {
    w: &'a mut W,
}
impl<'a> _BKEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BKEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Break inputs (BRK and CCS clock failure event) disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BKEW::DISABLED)
    }
    #[doc = "Break inputs (BRK and CCS clock failure event) enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BKEW::ENABLED)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSSR`"]
pub enum OSSRW {
    #[doc = "When inactive, OC/OCN outputs are disabled"] DISABLED,
    #[doc = "When inactive, OC/OCN outputs are enabled"] ENABLED,
}
impl OSSRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            OSSRW::DISABLED => false,
            OSSRW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSSRW<'a> {
    w: &'a mut W,
}
impl<'a> _OSSRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSSRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "When inactive, OC/OCN outputs are disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OSSRW::DISABLED)
    }
    #[doc = "When inactive, OC/OCN outputs are enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OSSRW::ENABLED)
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
#[doc = "Values that can be written to the field `OSSI`"]
pub enum OSSIW {
    #[doc = "When inactive, OC/OCN outputs are disabled"] DISABLED,
    #[doc = "When inactive, OC/OCN outputs are forced first with their idle level as soon as CCxE=1 or CCxNE=1. OC/OCN enable output signal=1"] ENABLED,
}
impl OSSIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            OSSIW::DISABLED => false,
            OSSIW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSSIW<'a> {
    w: &'a mut W,
}
impl<'a> _OSSIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSSIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "When inactive, OC/OCN outputs are disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OSSIW::DISABLED)
    }
    #[doc = "When inactive, OC/OCN outputs are forced first with their idle level as soon as CCxE=1 or CCxNE=1. OC/OCN enable output signal=1"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OSSIW::ENABLED)
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
#[doc = "Values that can be written to the field `LOCK`"]
pub enum LOCKW {
    #[doc = "LOCK OFF - No bit is write protected."] LOCKOFF,
    #[doc = "LOCK Level 1 = DTG bits in TIMx_BDTR register, OISx and OISxN bits in TIMx_CR2 register and BKE/BKP/AOE bits in TIMx_BDTR register can no longer be written."] LOCKLVL1,
    #[doc = "LOCK Level 2 = LOCK Level 1 + CC Polarity bits (CCxP/CCxNP bits in TIMx_CCER register, as long as the related channel is configured in output through the CCxS bits) as well as OSSR and OSSI bits can no longer be written."] LOCKLVL2,
    #[doc = "LOCK Level 3 = LOCK Level 2 + CC Control bits (OCxM and OCxPE bits in TIMx_CCMRx registers, as long as the related channel is configured in output through the CCxS bits) can no longer be written."] LOCKLVL3,
}
impl LOCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            LOCKW::LOCKOFF => 0,
            LOCKW::LOCKLVL1 => 1,
            LOCKW::LOCKLVL2 => 2,
            LOCKW::LOCKLVL3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCKW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "LOCK OFF - No bit is write protected."]
    #[inline(always)]
    pub fn lock_off(self) -> &'a mut W {
        self.variant(LOCKW::LOCKOFF)
    }
    #[doc = "LOCK Level 1 = DTG bits in TIMx_BDTR register, OISx and OISxN bits in TIMx_CR2 register and BKE/BKP/AOE bits in TIMx_BDTR register can no longer be written."]
    #[inline(always)]
    pub fn lock_lvl1(self) -> &'a mut W {
        self.variant(LOCKW::LOCKLVL1)
    }
    #[doc = "LOCK Level 2 = LOCK Level 1 + CC Polarity bits (CCxP/CCxNP bits in TIMx_CCER register, as long as the related channel is configured in output through the CCxS bits) as well as OSSR and OSSI bits can no longer be written."]
    #[inline(always)]
    pub fn lock_lvl2(self) -> &'a mut W {
        self.variant(LOCKW::LOCKLVL2)
    }
    #[doc = "LOCK Level 3 = LOCK Level 2 + CC Control bits (OCxM and OCxPE bits in TIMx_CCMRx registers, as long as the related channel is configured in output through the CCxS bits) can no longer be written."]
    #[inline(always)]
    pub fn lock_lvl3(self) -> &'a mut W {
        self.variant(LOCKW::LOCKLVL3)
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
#[doc = r" Proxy"]
pub struct _DTGW<'a> {
    w: &'a mut W,
}
impl<'a> _DTGW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bit 15 - Main output enable"]
    #[inline(always)]
    pub fn moe(&self) -> MOER {
        MOER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Automatic output enable"]
    #[inline(always)]
    pub fn aoe(&self) -> AOER {
        AOER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Break polarity"]
    #[inline(always)]
    pub fn bkp(&self) -> BKPR {
        BKPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Break enable"]
    #[inline(always)]
    pub fn bke(&self) -> BKER {
        BKER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Off-state selection for Run mode"]
    #[inline(always)]
    pub fn ossr(&self) -> OSSRR {
        OSSRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Off-state selection for Idle mode"]
    #[inline(always)]
    pub fn ossi(&self) -> OSSIR {
        OSSIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:9 - Lock configuration"]
    #[inline(always)]
    pub fn lock(&self) -> LOCKR {
        LOCKR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:7 - Dead-time generator setup"]
    #[inline(always)]
    pub fn dtg(&self) -> DTGR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DTGR { bits }
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
    #[doc = "Bit 15 - Main output enable"]
    #[inline(always)]
    pub fn moe(&mut self) -> _MOEW {
        _MOEW { w: self }
    }
    #[doc = "Bit 14 - Automatic output enable"]
    #[inline(always)]
    pub fn aoe(&mut self) -> _AOEW {
        _AOEW { w: self }
    }
    #[doc = "Bit 13 - Break polarity"]
    #[inline(always)]
    pub fn bkp(&mut self) -> _BKPW {
        _BKPW { w: self }
    }
    #[doc = "Bit 12 - Break enable"]
    #[inline(always)]
    pub fn bke(&mut self) -> _BKEW {
        _BKEW { w: self }
    }
    #[doc = "Bit 11 - Off-state selection for Run mode"]
    #[inline(always)]
    pub fn ossr(&mut self) -> _OSSRW {
        _OSSRW { w: self }
    }
    #[doc = "Bit 10 - Off-state selection for Idle mode"]
    #[inline(always)]
    pub fn ossi(&mut self) -> _OSSIW {
        _OSSIW { w: self }
    }
    #[doc = "Bits 8:9 - Lock configuration"]
    #[inline(always)]
    pub fn lock(&mut self) -> _LOCKW {
        _LOCKW { w: self }
    }
    #[doc = "Bits 0:7 - Dead-time generator setup"]
    #[inline(always)]
    pub fn dtg(&mut self) -> _DTGW {
        _DTGW { w: self }
    }
}
