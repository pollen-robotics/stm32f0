#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BDCR {
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
#[doc = "Possible values of the field `LSEON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSEONR {
    #[doc = "Disabled."] DISABLED,
    #[doc = "Enabled."] ENABLED,
}
impl LSEONR {
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
            LSEONR::DISABLED => false,
            LSEONR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> LSEONR {
        match value {
            false => LSEONR::DISABLED,
            true => LSEONR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSEONR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSEONR::ENABLED
    }
}
#[doc = r" Value of the field"]
pub struct LSERDYR {
    bits: bool,
}
impl LSERDYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `LSEBYP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSEBYPR {
    #[doc = "Disabled."] DISABLED,
    #[doc = "Enabled."] ENABLED,
}
impl LSEBYPR {
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
            LSEBYPR::DISABLED => false,
            LSEBYPR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> LSEBYPR {
        match value {
            false => LSEBYPR::DISABLED,
            true => LSEBYPR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSEBYPR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSEBYPR::ENABLED
    }
}
#[doc = "Possible values of the field `LSEDRV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSEDRVR {
    #[doc = "Xtal mode low drive capability."] LOW,
    #[doc = "Xtal mode medium high drive capability."] MEDIUMHIGH,
    #[doc = "Xtal mode medium low drive capability."] MEDIUMLOW,
    #[doc = "Xtal mode high drive capability."] HIGH,
}
impl LSEDRVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            LSEDRVR::LOW => 0,
            LSEDRVR::MEDIUMHIGH => 1,
            LSEDRVR::MEDIUMLOW => 2,
            LSEDRVR::HIGH => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> LSEDRVR {
        match value {
            0 => LSEDRVR::LOW,
            1 => LSEDRVR::MEDIUMHIGH,
            2 => LSEDRVR::MEDIUMLOW,
            3 => LSEDRVR::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == LSEDRVR::LOW
    }
    #[doc = "Checks if the value of the field is `MEDIUMHIGH`"]
    #[inline(always)]
    pub fn is_medium_high(&self) -> bool {
        *self == LSEDRVR::MEDIUMHIGH
    }
    #[doc = "Checks if the value of the field is `MEDIUMLOW`"]
    #[inline(always)]
    pub fn is_medium_low(&self) -> bool {
        *self == LSEDRVR::MEDIUMLOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == LSEDRVR::HIGH
    }
}
#[doc = "Possible values of the field `RTCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCSELR {
    #[doc = "No Clock."] NOCLOCK,
    #[doc = "LSE oscillator clock used as RTC clock."] LSE,
    #[doc = "LSI oscillator clock used as RTC clock."] LSI,
    #[doc = "HSE oscillator clock divided by 32 used as RTC clock."] HSE,
}
impl RTCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            RTCSELR::NOCLOCK => 0,
            RTCSELR::LSE => 1,
            RTCSELR::LSI => 2,
            RTCSELR::HSE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> RTCSELR {
        match value {
            0 => RTCSELR::NOCLOCK,
            1 => RTCSELR::LSE,
            2 => RTCSELR::LSI,
            3 => RTCSELR::HSE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOCLOCK`"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == RTCSELR::NOCLOCK
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == RTCSELR::LSE
    }
    #[doc = "Checks if the value of the field is `LSI`"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == RTCSELR::LSI
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == RTCSELR::HSE
    }
}
#[doc = "Possible values of the field `RTCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCENR {
    #[doc = "Disabled."] DISABLED,
    #[doc = "Enabled."] ENABLED,
}
impl RTCENR {
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
            RTCENR::DISABLED => false,
            RTCENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> RTCENR {
        match value {
            false => RTCENR::DISABLED,
            true => RTCENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTCENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTCENR::ENABLED
    }
}
#[doc = "Possible values of the field `BDRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BDRSTR {
    #[doc = "Reset not activated."] NORESET,
    #[doc = "Resets the entire RTC domain."] RESET,
}
impl BDRSTR {
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
            BDRSTR::NORESET => false,
            BDRSTR::RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> BDRSTR {
        match value {
            false => BDRSTR::NORESET,
            true => BDRSTR::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NORESET`"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == BDRSTR::NORESET
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == BDRSTR::RESET
    }
}
#[doc = "Values that can be written to the field `LSEON`"]
pub enum LSEONW {
    #[doc = "Disabled."] DISABLED,
    #[doc = "Enabled."] ENABLED,
}
impl LSEONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LSEONW::DISABLED => false,
            LSEONW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LSEONW<'a> {
    w: &'a mut W,
}
impl<'a> _LSEONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSEONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSEONW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LSEONW::ENABLED)
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
#[doc = "Values that can be written to the field `LSEBYP`"]
pub enum LSEBYPW {
    #[doc = "Disabled."] DISABLED,
    #[doc = "Enabled."] ENABLED,
}
impl LSEBYPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LSEBYPW::DISABLED => false,
            LSEBYPW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LSEBYPW<'a> {
    w: &'a mut W,
}
impl<'a> _LSEBYPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSEBYPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSEBYPW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LSEBYPW::ENABLED)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LSEDRV`"]
pub enum LSEDRVW {
    #[doc = "Xtal mode low drive capability."] LOW,
    #[doc = "Xtal mode medium high drive capability."] MEDIUMHIGH,
    #[doc = "Xtal mode medium low drive capability."] MEDIUMLOW,
    #[doc = "Xtal mode high drive capability."] HIGH,
}
impl LSEDRVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            LSEDRVW::LOW => 0,
            LSEDRVW::MEDIUMHIGH => 1,
            LSEDRVW::MEDIUMLOW => 2,
            LSEDRVW::HIGH => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LSEDRVW<'a> {
    w: &'a mut W,
}
impl<'a> _LSEDRVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSEDRVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Xtal mode low drive capability."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(LSEDRVW::LOW)
    }
    #[doc = "Xtal mode medium high drive capability."]
    #[inline(always)]
    pub fn medium_high(self) -> &'a mut W {
        self.variant(LSEDRVW::MEDIUMHIGH)
    }
    #[doc = "Xtal mode medium low drive capability."]
    #[inline(always)]
    pub fn medium_low(self) -> &'a mut W {
        self.variant(LSEDRVW::MEDIUMLOW)
    }
    #[doc = "Xtal mode high drive capability."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(LSEDRVW::HIGH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTCSEL`"]
pub enum RTCSELW {
    #[doc = "No Clock."] NOCLOCK,
    #[doc = "LSE oscillator clock used as RTC clock."] LSE,
    #[doc = "LSI oscillator clock used as RTC clock."] LSI,
    #[doc = "HSE oscillator clock divided by 32 used as RTC clock."] HSE,
}
impl RTCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            RTCSELW::NOCLOCK => 0,
            RTCSELW::LSE => 1,
            RTCSELW::LSI => 2,
            RTCSELW::HSE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No Clock."]
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut W {
        self.variant(RTCSELW::NOCLOCK)
    }
    #[doc = "LSE oscillator clock used as RTC clock."]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(RTCSELW::LSE)
    }
    #[doc = "LSI oscillator clock used as RTC clock."]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(RTCSELW::LSI)
    }
    #[doc = "HSE oscillator clock divided by 32 used as RTC clock."]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(RTCSELW::HSE)
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
#[doc = "Values that can be written to the field `RTCEN`"]
pub enum RTCENW {
    #[doc = "Disabled."] DISABLED,
    #[doc = "Enabled."] ENABLED,
}
impl RTCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            RTCENW::DISABLED => false,
            RTCENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTCENW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTCENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RTCENW::ENABLED)
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
#[doc = "Values that can be written to the field `BDRST`"]
pub enum BDRSTW {
    #[doc = "Reset not activated."] NORESET,
    #[doc = "Resets the entire RTC domain."] RESET,
}
impl BDRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            BDRSTW::NORESET => false,
            BDRSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BDRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _BDRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BDRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset not activated."]
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(BDRSTW::NORESET)
    }
    #[doc = "Resets the entire RTC domain."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(BDRSTW::RESET)
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
        const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - External Low Speed oscillator enable"]
    #[inline(always)]
    pub fn lseon(&self) -> LSEONR {
        LSEONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - External Low Speed oscillator ready"]
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LSERDYR { bits }
    }
    #[doc = "Bit 2 - External Low Speed oscillator bypass"]
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYPR {
        LSEBYPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 3:4 - LSE oscillator drive capability"]
    #[inline(always)]
    pub fn lsedrv(&self) -> LSEDRVR {
        LSEDRVR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - RTC clock source selection"]
    #[inline(always)]
    pub fn rtcsel(&self) -> RTCSELR {
        RTCSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline(always)]
    pub fn rtcen(&self) -> RTCENR {
        RTCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Backup domain software reset"]
    #[inline(always)]
    pub fn bdrst(&self) -> BDRSTR {
        BDRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - External Low Speed oscillator enable"]
    #[inline(always)]
    pub fn lseon(&mut self) -> _LSEONW {
        _LSEONW { w: self }
    }
    #[doc = "Bit 2 - External Low Speed oscillator bypass"]
    #[inline(always)]
    pub fn lsebyp(&mut self) -> _LSEBYPW {
        _LSEBYPW { w: self }
    }
    #[doc = "Bits 3:4 - LSE oscillator drive capability"]
    #[inline(always)]
    pub fn lsedrv(&mut self) -> _LSEDRVW {
        _LSEDRVW { w: self }
    }
    #[doc = "Bits 8:9 - RTC clock source selection"]
    #[inline(always)]
    pub fn rtcsel(&mut self) -> _RTCSELW {
        _RTCSELW { w: self }
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline(always)]
    pub fn rtcen(&mut self) -> _RTCENW {
        _RTCENW { w: self }
    }
    #[doc = "Bit 16 - Backup domain software reset"]
    #[inline(always)]
    pub fn bdrst(&mut self) -> _BDRSTW {
        _BDRSTW { w: self }
    }
}
