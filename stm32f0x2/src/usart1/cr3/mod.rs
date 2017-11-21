#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR3 {
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
#[doc = "Possible values of the field `WUFIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUFIER {
    #[doc = "Interrupt is inhibited."] DISABLED,
    #[doc = "An USART interrupt is generated whenever WUF=1 in the USART_ISR register."] ENABLED,
}
impl WUFIER {
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
            WUFIER::DISABLED => false,
            WUFIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> WUFIER {
        match value {
            false => WUFIER::DISABLED,
            true => WUFIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WUFIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WUFIER::ENABLED
    }
}
#[doc = "Possible values of the field `WUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUSR {
    #[doc = "WUF active on address match."] ADDRESS,
    #[doc = "WuF active on Start bit detection."] STARTBIT,
    #[doc = "WUF active on RXNE."] RXNE,
}
impl WUSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            WUSR::ADDRESS => 0,
            WUSR::STARTBIT => 2,
            WUSR::RXNE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> WUSR {
        match value {
            0 => WUSR::ADDRESS,
            2 => WUSR::STARTBIT,
            3 => WUSR::RXNE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADDRESS`"]
    #[inline(always)]
    pub fn is_address(&self) -> bool {
        *self == WUSR::ADDRESS
    }
    #[doc = "Checks if the value of the field is `STARTBIT`"]
    #[inline(always)]
    pub fn is_startbit(&self) -> bool {
        *self == WUSR::STARTBIT
    }
    #[doc = "Checks if the value of the field is `RXNE`"]
    #[inline(always)]
    pub fn is_rxne(&self) -> bool {
        *self == WUSR::RXNE
    }
}
#[doc = r" Value of the field"]
pub struct SCARCNTR {
    bits: u8,
}
impl SCARCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `DEP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEPR {
    #[doc = "DE signal is active high."] HIGH,
    #[doc = "DE signal is active low."] LOW,
}
impl DEPR {
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
            DEPR::HIGH => false,
            DEPR::LOW => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> DEPR {
        match value {
            false => DEPR::HIGH,
            true => DEPR::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == DEPR::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == DEPR::LOW
    }
}
#[doc = "Possible values of the field `DEM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEMR {
    #[doc = "DE function is disabled."] DISABLED,
    #[doc = "DE function is enabled."] ENABLED,
}
impl DEMR {
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
            DEMR::DISABLED => false,
            DEMR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> DEMR {
        match value {
            false => DEMR::DISABLED,
            true => DEMR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DEMR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DEMR::ENABLED
    }
}
#[doc = "Possible values of the field `DDRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDRER {
    #[doc = "DMA is not disabled in case of reception error."] NOTDISABLED,
    #[doc = "DMA is disabled following a reception error."] DISABLED,
}
impl DDRER {
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
            DDRER::NOTDISABLED => false,
            DDRER::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> DDRER {
        match value {
            false => DDRER::NOTDISABLED,
            true => DDRER::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDISABLED`"]
    #[inline(always)]
    pub fn is_notdisabled(&self) -> bool {
        *self == DDRER::NOTDISABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DDRER::DISABLED
    }
}
#[doc = "Possible values of the field `OVRDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRDISR {
    #[doc = "Overrun Error Flag, ORE, is set when received data is not read before receiving new data."] NOTDISABLED,
    #[doc = "Overrun functionality is disabled."] DISABLED,
}
impl OVRDISR {
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
            OVRDISR::NOTDISABLED => false,
            OVRDISR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> OVRDISR {
        match value {
            false => OVRDISR::NOTDISABLED,
            true => OVRDISR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDISABLED`"]
    #[inline(always)]
    pub fn is_notdisabled(&self) -> bool {
        *self == OVRDISR::NOTDISABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVRDISR::DISABLED
    }
}
#[doc = "Possible values of the field `ONEBIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONEBITR {
    #[doc = "Three sample bit method."] THREESAMPLES,
    #[doc = "One sample bit method."] ONESAMPLE,
}
impl ONEBITR {
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
            ONEBITR::THREESAMPLES => false,
            ONEBITR::ONESAMPLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> ONEBITR {
        match value {
            false => ONEBITR::THREESAMPLES,
            true => ONEBITR::ONESAMPLE,
        }
    }
    #[doc = "Checks if the value of the field is `THREESAMPLES`"]
    #[inline(always)]
    pub fn is_three_samples(&self) -> bool {
        *self == ONEBITR::THREESAMPLES
    }
    #[doc = "Checks if the value of the field is `ONESAMPLE`"]
    #[inline(always)]
    pub fn is_one_sample(&self) -> bool {
        *self == ONEBITR::ONESAMPLE
    }
}
#[doc = "Possible values of the field `CTSIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSIER {
    #[doc = "Interrupt is inhibited."] DISABLED,
    #[doc = "An interrupt is generated whenever CTSIF=1 in the USART_ISR register."] ENABLED,
}
impl CTSIER {
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
            CTSIER::DISABLED => false,
            CTSIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> CTSIER {
        match value {
            false => CTSIER::DISABLED,
            true => CTSIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTSIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTSIER::ENABLED
    }
}
#[doc = "Possible values of the field `CTSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSER {
    #[doc = "CTS hardware flow control disabled."] DISABLED,
    #[doc = "CTS mode enabled, data is only transmitted when the CTS input is asserted (tied to 0)."] ENABLED,
}
impl CTSER {
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
            CTSER::DISABLED => false,
            CTSER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> CTSER {
        match value {
            false => CTSER::DISABLED,
            true => CTSER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTSER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTSER::ENABLED
    }
}
#[doc = "Possible values of the field `RTSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTSER {
    #[doc = "RTS hardware flow control disabled."] DISABLED,
    #[doc = "RTS output enabled, data is only requested when there is space in the receive buffer."] ENABLED,
}
impl RTSER {
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
            RTSER::DISABLED => false,
            RTSER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> RTSER {
        match value {
            false => RTSER::DISABLED,
            true => RTSER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTSER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTSER::ENABLED
    }
}
#[doc = "Possible values of the field `DMAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMATR {
    #[doc = "DMA mode is disabled for transmission."] DISABLED,
    #[doc = "DMA mode is enabled for transmission."] ENABLED,
}
impl DMATR {
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
            DMATR::DISABLED => false,
            DMATR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> DMATR {
        match value {
            false => DMATR::DISABLED,
            true => DMATR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMATR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMATR::ENABLED
    }
}
#[doc = "Possible values of the field `DMAR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMARR {
    #[doc = "DMA mode is disabled for reception."] DISABLED,
    #[doc = "DMA mode is enabled for reception."] ENABLED,
}
impl DMARR {
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
            DMARR::DISABLED => false,
            DMARR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> DMARR {
        match value {
            false => DMARR::DISABLED,
            true => DMARR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMARR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMARR::ENABLED
    }
}
#[doc = "Possible values of the field `SCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCENR {
    #[doc = "Smartcard Mode disabled."] DISABLED,
    #[doc = "Smartcard Mode enabled."] ENABLED,
}
impl SCENR {
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
            SCENR::DISABLED => false,
            SCENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> SCENR {
        match value {
            false => SCENR::DISABLED,
            true => SCENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SCENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SCENR::ENABLED
    }
}
#[doc = "Possible values of the field `NACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACKR {
    #[doc = "NACK transmission in case of parity error is disabled."] DISABLED,
    #[doc = "NACK transmission during parity error is enabled."] ENABLED,
}
impl NACKR {
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
            NACKR::DISABLED => false,
            NACKR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> NACKR {
        match value {
            false => NACKR::DISABLED,
            true => NACKR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NACKR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NACKR::ENABLED
    }
}
#[doc = "Possible values of the field `HDSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDSELR {
    #[doc = "Half duplex mode is not selected."] DISABLED,
    #[doc = "Half duplex mode is selected."] ENABLED,
}
impl HDSELR {
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
            HDSELR::DISABLED => false,
            HDSELR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> HDSELR {
        match value {
            false => HDSELR::DISABLED,
            true => HDSELR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HDSELR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HDSELR::ENABLED
    }
}
#[doc = "Possible values of the field `IRLP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRLPR {
    #[doc = "Normal mode."] NORMAL,
    #[doc = "Low-power mode."] LOWPOWER,
}
impl IRLPR {
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
            IRLPR::NORMAL => false,
            IRLPR::LOWPOWER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> IRLPR {
        match value {
            false => IRLPR::NORMAL,
            true => IRLPR::LOWPOWER,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == IRLPR::NORMAL
    }
    #[doc = "Checks if the value of the field is `LOWPOWER`"]
    #[inline(always)]
    pub fn is_lowpower(&self) -> bool {
        *self == IRLPR::LOWPOWER
    }
}
#[doc = "Possible values of the field `IREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRENR {
    #[doc = "IrDA disabled."] DISABLED,
    #[doc = "IrDA enabled."] ENABLED,
}
impl IRENR {
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
            IRENR::DISABLED => false,
            IRENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> IRENR {
        match value {
            false => IRENR::DISABLED,
            true => IRENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IRENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IRENR::ENABLED
    }
}
#[doc = "Possible values of the field `EIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EIER {
    #[doc = "Interrupt is inhibited."] DISABLED,
    #[doc = "An interrupt is generated when FE=1 or ORE=1 or NF=1 in the USART_ISR register."] ENABLED,
}
impl EIER {
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
            EIER::DISABLED => false,
            EIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: bool) -> EIER {
        match value {
            false => EIER::DISABLED,
            true => EIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EIER::ENABLED
    }
}
#[doc = "Values that can be written to the field `WUFIE`"]
pub enum WUFIEW {
    #[doc = "Interrupt is inhibited."] DISABLED,
    #[doc = "An USART interrupt is generated whenever WUF=1 in the USART_ISR register."] ENABLED,
}
impl WUFIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WUFIEW::DISABLED => false,
            WUFIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUFIEW<'a> {
    w: &'a mut W,
}
impl<'a> _WUFIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUFIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt is inhibited."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WUFIEW::DISABLED)
    }
    #[doc = "An USART interrupt is generated whenever WUF=1 in the USART_ISR register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WUFIEW::ENABLED)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WUS`"]
pub enum WUSW {
    #[doc = "WUF active on address match."] ADDRESS,
    #[doc = "WuF active on Start bit detection."] STARTBIT,
    #[doc = "WUF active on RXNE."] RXNE,
}
impl WUSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            WUSW::ADDRESS => 0,
            WUSW::STARTBIT => 2,
            WUSW::RXNE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUSW<'a> {
    w: &'a mut W,
}
impl<'a> _WUSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "WUF active on address match."]
    #[inline(always)]
    pub fn address(self) -> &'a mut W {
        self.variant(WUSW::ADDRESS)
    }
    #[doc = "WuF active on Start bit detection."]
    #[inline(always)]
    pub fn startbit(self) -> &'a mut W {
        self.variant(WUSW::STARTBIT)
    }
    #[doc = "WUF active on RXNE."]
    #[inline(always)]
    pub fn rxne(self) -> &'a mut W {
        self.variant(WUSW::RXNE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SCARCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _SCARCNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DEP`"]
pub enum DEPW {
    #[doc = "DE signal is active high."] HIGH,
    #[doc = "DE signal is active low."] LOW,
}
impl DEPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            DEPW::HIGH => false,
            DEPW::LOW => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DEPW<'a> {
    w: &'a mut W,
}
impl<'a> _DEPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DE signal is active high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(DEPW::HIGH)
    }
    #[doc = "DE signal is active low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(DEPW::LOW)
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
#[doc = "Values that can be written to the field `DEM`"]
pub enum DEMW {
    #[doc = "DE function is disabled."] DISABLED,
    #[doc = "DE function is enabled."] ENABLED,
}
impl DEMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            DEMW::DISABLED => false,
            DEMW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DEMW<'a> {
    w: &'a mut W,
}
impl<'a> _DEMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DE function is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DEMW::DISABLED)
    }
    #[doc = "DE function is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DEMW::ENABLED)
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
#[doc = "Values that can be written to the field `DDRE`"]
pub enum DDREW {
    #[doc = "DMA is not disabled in case of reception error."] NOTDISABLED,
    #[doc = "DMA is disabled following a reception error."] DISABLED,
}
impl DDREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            DDREW::NOTDISABLED => false,
            DDREW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DDREW<'a> {
    w: &'a mut W,
}
impl<'a> _DDREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DDREW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA is not disabled in case of reception error."]
    #[inline(always)]
    pub fn notdisabled(self) -> &'a mut W {
        self.variant(DDREW::NOTDISABLED)
    }
    #[doc = "DMA is disabled following a reception error."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DDREW::DISABLED)
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
#[doc = "Values that can be written to the field `OVRDIS`"]
pub enum OVRDISW {
    #[doc = "Overrun Error Flag, ORE, is set when received data is not read before receiving new data."] NOTDISABLED,
    #[doc = "Overrun functionality is disabled."] DISABLED,
}
impl OVRDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            OVRDISW::NOTDISABLED => false,
            OVRDISW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OVRDISW<'a> {
    w: &'a mut W,
}
impl<'a> _OVRDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVRDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Overrun Error Flag, ORE, is set when received data is not read before receiving new data."]
    #[inline(always)]
    pub fn notdisabled(self) -> &'a mut W {
        self.variant(OVRDISW::NOTDISABLED)
    }
    #[doc = "Overrun functionality is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OVRDISW::DISABLED)
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
#[doc = "Values that can be written to the field `ONEBIT`"]
pub enum ONEBITW {
    #[doc = "Three sample bit method."] THREESAMPLES,
    #[doc = "One sample bit method."] ONESAMPLE,
}
impl ONEBITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            ONEBITW::THREESAMPLES => false,
            ONEBITW::ONESAMPLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ONEBITW<'a> {
    w: &'a mut W,
}
impl<'a> _ONEBITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ONEBITW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Three sample bit method."]
    #[inline(always)]
    pub fn three_samples(self) -> &'a mut W {
        self.variant(ONEBITW::THREESAMPLES)
    }
    #[doc = "One sample bit method."]
    #[inline(always)]
    pub fn one_sample(self) -> &'a mut W {
        self.variant(ONEBITW::ONESAMPLE)
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
#[doc = "Values that can be written to the field `CTSIE`"]
pub enum CTSIEW {
    #[doc = "Interrupt is inhibited."] DISABLED,
    #[doc = "An interrupt is generated whenever CTSIF=1 in the USART_ISR register."] ENABLED,
}
impl CTSIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CTSIEW::DISABLED => false,
            CTSIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTSIEW<'a> {
    w: &'a mut W,
}
impl<'a> _CTSIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTSIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt is inhibited."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTSIEW::DISABLED)
    }
    #[doc = "An interrupt is generated whenever CTSIF=1 in the USART_ISR register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTSIEW::ENABLED)
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
#[doc = "Values that can be written to the field `CTSE`"]
pub enum CTSEW {
    #[doc = "CTS hardware flow control disabled."] DISABLED,
    #[doc = "CTS mode enabled, data is only transmitted when the CTS input is asserted (tied to 0)."] ENABLED,
}
impl CTSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CTSEW::DISABLED => false,
            CTSEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTSEW<'a> {
    w: &'a mut W,
}
impl<'a> _CTSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTSEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CTS hardware flow control disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTSEW::DISABLED)
    }
    #[doc = "CTS mode enabled, data is only transmitted when the CTS input is asserted (tied to 0)."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTSEW::ENABLED)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTSE`"]
pub enum RTSEW {
    #[doc = "RTS hardware flow control disabled."] DISABLED,
    #[doc = "RTS output enabled, data is only requested when there is space in the receive buffer."] ENABLED,
}
impl RTSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            RTSEW::DISABLED => false,
            RTSEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTSEW<'a> {
    w: &'a mut W,
}
impl<'a> _RTSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTSEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RTS hardware flow control disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTSEW::DISABLED)
    }
    #[doc = "RTS output enabled, data is only requested when there is space in the receive buffer."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RTSEW::ENABLED)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMAT`"]
pub enum DMATW {
    #[doc = "DMA mode is disabled for transmission."] DISABLED,
    #[doc = "DMA mode is enabled for transmission."] ENABLED,
}
impl DMATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            DMATW::DISABLED => false,
            DMATW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMATW<'a> {
    w: &'a mut W,
}
impl<'a> _DMATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMATW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA mode is disabled for transmission."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMATW::DISABLED)
    }
    #[doc = "DMA mode is enabled for transmission."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMATW::ENABLED)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMAR`"]
pub enum DMARW {
    #[doc = "DMA mode is disabled for reception."] DISABLED,
    #[doc = "DMA mode is enabled for reception."] ENABLED,
}
impl DMARW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            DMARW::DISABLED => false,
            DMARW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMARW<'a> {
    w: &'a mut W,
}
impl<'a> _DMARW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMARW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA mode is disabled for reception."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMARW::DISABLED)
    }
    #[doc = "DMA mode is enabled for reception."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMARW::ENABLED)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SCEN`"]
pub enum SCENW {
    #[doc = "Smartcard Mode disabled."] DISABLED,
    #[doc = "Smartcard Mode enabled."] ENABLED,
}
impl SCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SCENW::DISABLED => false,
            SCENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCENW<'a> {
    w: &'a mut W,
}
impl<'a> _SCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Smartcard Mode disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SCENW::DISABLED)
    }
    #[doc = "Smartcard Mode enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SCENW::ENABLED)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NACK`"]
pub enum NACKW {
    #[doc = "NACK transmission in case of parity error is disabled."] DISABLED,
    #[doc = "NACK transmission during parity error is enabled."] ENABLED,
}
impl NACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            NACKW::DISABLED => false,
            NACKW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NACKW<'a> {
    w: &'a mut W,
}
impl<'a> _NACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "NACK transmission in case of parity error is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NACKW::DISABLED)
    }
    #[doc = "NACK transmission during parity error is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NACKW::ENABLED)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HDSEL`"]
pub enum HDSELW {
    #[doc = "Half duplex mode is not selected."] DISABLED,
    #[doc = "Half duplex mode is selected."] ENABLED,
}
impl HDSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            HDSELW::DISABLED => false,
            HDSELW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HDSELW<'a> {
    w: &'a mut W,
}
impl<'a> _HDSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HDSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Half duplex mode is not selected."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HDSELW::DISABLED)
    }
    #[doc = "Half duplex mode is selected."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HDSELW::ENABLED)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IRLP`"]
pub enum IRLPW {
    #[doc = "Normal mode."] NORMAL,
    #[doc = "Low-power mode."] LOWPOWER,
}
impl IRLPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            IRLPW::NORMAL => false,
            IRLPW::LOWPOWER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IRLPW<'a> {
    w: &'a mut W,
}
impl<'a> _IRLPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRLPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(IRLPW::NORMAL)
    }
    #[doc = "Low-power mode."]
    #[inline(always)]
    pub fn lowpower(self) -> &'a mut W {
        self.variant(IRLPW::LOWPOWER)
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
#[doc = "Values that can be written to the field `IREN`"]
pub enum IRENW {
    #[doc = "IrDA disabled."] DISABLED,
    #[doc = "IrDA enabled."] ENABLED,
}
impl IRENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            IRENW::DISABLED => false,
            IRENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IRENW<'a> {
    w: &'a mut W,
}
impl<'a> _IRENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "IrDA disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IRENW::DISABLED)
    }
    #[doc = "IrDA enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IRENW::ENABLED)
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
#[doc = "Values that can be written to the field `EIE`"]
pub enum EIEW {
    #[doc = "Interrupt is inhibited."] DISABLED,
    #[doc = "An interrupt is generated when FE=1 or ORE=1 or NF=1 in the USART_ISR register."] ENABLED,
}
impl EIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            EIEW::DISABLED => false,
            EIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EIEW<'a> {
    w: &'a mut W,
}
impl<'a> _EIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt is inhibited."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EIEW::DISABLED)
    }
    #[doc = "An interrupt is generated when FE=1 or ORE=1 or NF=1 in the USART_ISR register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EIEW::ENABLED)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 22 - Wakeup from Stop mode interrupt enable"]
    #[inline(always)]
    pub fn wufie(&self) -> WUFIER {
        WUFIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 20:21 - Wakeup from Stop mode interrupt flag selection"]
    #[inline(always)]
    pub fn wus(&self) -> WUSR {
        WUSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 17:19 - Smartcard auto-retry count"]
    #[inline(always)]
    pub fn scarcnt(&self) -> SCARCNTR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SCARCNTR { bits }
    }
    #[doc = "Bit 15 - Driver enable polarity selection"]
    #[inline(always)]
    pub fn dep(&self) -> DEPR {
        DEPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Driver enable mode"]
    #[inline(always)]
    pub fn dem(&self) -> DEMR {
        DEMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - DMA Disable on Reception Error"]
    #[inline(always)]
    pub fn ddre(&self) -> DDRER {
        DDRER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Overrun Disable"]
    #[inline(always)]
    pub fn ovrdis(&self) -> OVRDISR {
        OVRDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - One sample bit method enable"]
    #[inline(always)]
    pub fn onebit(&self) -> ONEBITR {
        ONEBITR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - CTS interrupt enable"]
    #[inline(always)]
    pub fn ctsie(&self) -> CTSIER {
        CTSIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline(always)]
    pub fn ctse(&self) -> CTSER {
        CTSER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline(always)]
    pub fn rtse(&self) -> RTSER {
        RTSER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - DMA enable transmitter"]
    #[inline(always)]
    pub fn dmat(&self) -> DMATR {
        DMATR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - DMA enable receiver"]
    #[inline(always)]
    pub fn dmar(&self) -> DMARR {
        DMARR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Smartcard mode enable"]
    #[inline(always)]
    pub fn scen(&self) -> SCENR {
        SCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Smartcard NACK enable"]
    #[inline(always)]
    pub fn nack(&self) -> NACKR {
        NACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Half-duplex selection"]
    #[inline(always)]
    pub fn hdsel(&self) -> HDSELR {
        HDSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - IrDA low-power"]
    #[inline(always)]
    pub fn irlp(&self) -> IRLPR {
        IRLPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - IrDA mode enable"]
    #[inline(always)]
    pub fn iren(&self) -> IRENR {
        IRENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    pub fn eie(&self) -> EIER {
        EIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
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
    #[doc = "Bit 22 - Wakeup from Stop mode interrupt enable"]
    #[inline(always)]
    pub fn wufie(&mut self) -> _WUFIEW {
        _WUFIEW { w: self }
    }
    #[doc = "Bits 20:21 - Wakeup from Stop mode interrupt flag selection"]
    #[inline(always)]
    pub fn wus(&mut self) -> _WUSW {
        _WUSW { w: self }
    }
    #[doc = "Bits 17:19 - Smartcard auto-retry count"]
    #[inline(always)]
    pub fn scarcnt(&mut self) -> _SCARCNTW {
        _SCARCNTW { w: self }
    }
    #[doc = "Bit 15 - Driver enable polarity selection"]
    #[inline(always)]
    pub fn dep(&mut self) -> _DEPW {
        _DEPW { w: self }
    }
    #[doc = "Bit 14 - Driver enable mode"]
    #[inline(always)]
    pub fn dem(&mut self) -> _DEMW {
        _DEMW { w: self }
    }
    #[doc = "Bit 13 - DMA Disable on Reception Error"]
    #[inline(always)]
    pub fn ddre(&mut self) -> _DDREW {
        _DDREW { w: self }
    }
    #[doc = "Bit 12 - Overrun Disable"]
    #[inline(always)]
    pub fn ovrdis(&mut self) -> _OVRDISW {
        _OVRDISW { w: self }
    }
    #[doc = "Bit 11 - One sample bit method enable"]
    #[inline(always)]
    pub fn onebit(&mut self) -> _ONEBITW {
        _ONEBITW { w: self }
    }
    #[doc = "Bit 10 - CTS interrupt enable"]
    #[inline(always)]
    pub fn ctsie(&mut self) -> _CTSIEW {
        _CTSIEW { w: self }
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline(always)]
    pub fn ctse(&mut self) -> _CTSEW {
        _CTSEW { w: self }
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline(always)]
    pub fn rtse(&mut self) -> _RTSEW {
        _RTSEW { w: self }
    }
    #[doc = "Bit 7 - DMA enable transmitter"]
    #[inline(always)]
    pub fn dmat(&mut self) -> _DMATW {
        _DMATW { w: self }
    }
    #[doc = "Bit 6 - DMA enable receiver"]
    #[inline(always)]
    pub fn dmar(&mut self) -> _DMARW {
        _DMARW { w: self }
    }
    #[doc = "Bit 5 - Smartcard mode enable"]
    #[inline(always)]
    pub fn scen(&mut self) -> _SCENW {
        _SCENW { w: self }
    }
    #[doc = "Bit 4 - Smartcard NACK enable"]
    #[inline(always)]
    pub fn nack(&mut self) -> _NACKW {
        _NACKW { w: self }
    }
    #[doc = "Bit 3 - Half-duplex selection"]
    #[inline(always)]
    pub fn hdsel(&mut self) -> _HDSELW {
        _HDSELW { w: self }
    }
    #[doc = "Bit 2 - IrDA low-power"]
    #[inline(always)]
    pub fn irlp(&mut self) -> _IRLPW {
        _IRLPW { w: self }
    }
    #[doc = "Bit 1 - IrDA mode enable"]
    #[inline(always)]
    pub fn iren(&mut self) -> _IRENW {
        _IRENW { w: self }
    }
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    pub fn eie(&mut self) -> _EIEW {
        _EIEW { w: self }
    }
}
