#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR1 {
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
#[doc = "Possible values of the field `UE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UER {
    #[doc = "USART Disabled."] DISABLED,
    #[doc = "USART Enabled."] ENABLED,
}
impl UER {
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
            UER::DISABLED => false,
            UER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UER {
        match value {
            false => UER::DISABLED,
            true => UER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == UER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == UER::ENABLED
    }
}
#[doc = "Possible values of the field `UESM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UESMR {
    #[doc = "USART not able to wake up the MCU from Stop mode."] NOTABLE,
    #[doc = "USART able to wake up the MCU from Stop mode."] ABLE,
}
impl UESMR {
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
            UESMR::NOTABLE => false,
            UESMR::ABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UESMR {
        match value {
            false => UESMR::NOTABLE,
            true => UESMR::ABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTABLE`"]
    #[inline]
    pub fn is_not_able(&self) -> bool {
        *self == UESMR::NOTABLE
    }
    #[doc = "Checks if the value of the field is `ABLE`"]
    #[inline]
    pub fn is_able(&self) -> bool {
        *self == UESMR::ABLE
    }
}
#[doc = "Possible values of the field `RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RER {
    #[doc = "Receiver is disabled."] DISABLED,
    #[doc = "Receiver is enabled and begins searching for a start bit."] ENABLED,
}
impl RER {
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
            RER::DISABLED => false,
            RER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RER {
        match value {
            false => RER::DISABLED,
            true => RER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RER::ENABLED
    }
}
#[doc = "Possible values of the field `TE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TER {
    #[doc = "Transmitter is disabled."] DISABLED,
    #[doc = "Transmitter is enabled."] ENABLED,
}
impl TER {
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
            TER::DISABLED => false,
            TER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TER {
        match value {
            false => TER::DISABLED,
            true => TER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TER::ENABLED
    }
}
#[doc = "Possible values of the field `IDLEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLEIER {
    #[doc = "Interrupt is inhibited."] DISABLED,
    #[doc = "An USART interrupt is generated whenever IDLE=1 in the USART_ISR register."] ENABLED,
}
impl IDLEIER {
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
            IDLEIER::DISABLED => false,
            IDLEIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IDLEIER {
        match value {
            false => IDLEIER::DISABLED,
            true => IDLEIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == IDLEIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == IDLEIER::ENABLED
    }
}
#[doc = "Possible values of the field `RXNEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXNEIER {
    #[doc = "Interrupt is inhibited."] DISABLED,
    #[doc = "An USART interrupt is generated whenever ORE=1 or RXNE=1 in the USART_ISR register."]
    ENABLED,
}
impl RXNEIER {
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
            RXNEIER::DISABLED => false,
            RXNEIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXNEIER {
        match value {
            false => RXNEIER::DISABLED,
            true => RXNEIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RXNEIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RXNEIER::ENABLED
    }
}
#[doc = "Possible values of the field `TCIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIER {
    #[doc = "Interrupt is inhibited."] DISABLED,
    #[doc = "An USART interrupt is generated whenever TC=1 in the USART_ISR register."] ENABLED,
}
impl TCIER {
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
            TCIER::DISABLED => false,
            TCIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCIER {
        match value {
            false => TCIER::DISABLED,
            true => TCIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TCIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TCIER::ENABLED
    }
}
#[doc = "Possible values of the field `TXEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEIER {
    #[doc = "Interrupt is inhibited."] DISABLED,
    #[doc = "An USART interrupt is generated whenever TXE=1 in the USART_ISR register."] ENABLED,
}
impl TXEIER {
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
            TXEIER::DISABLED => false,
            TXEIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXEIER {
        match value {
            false => TXEIER::DISABLED,
            true => TXEIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TXEIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TXEIER::ENABLED
    }
}
#[doc = "Possible values of the field `PEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEIER {
    #[doc = "Interrupt is inhibited."] DISABLED,
    #[doc = "An USART interrupt is generated whenever PE=1 in the USART_ISR register."] ENABLED,
}
impl PEIER {
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
            PEIER::DISABLED => false,
            PEIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEIER {
        match value {
            false => PEIER::DISABLED,
            true => PEIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PEIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PEIER::ENABLED
    }
}
#[doc = "Possible values of the field `PS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSR {
    #[doc = "Even parity."] EVEN,
    #[doc = "Odd parity."] ODD,
}
impl PSR {
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
            PSR::EVEN => false,
            PSR::ODD => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PSR {
        match value {
            false => PSR::EVEN,
            true => PSR::ODD,
        }
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline]
    pub fn is_even(&self) -> bool {
        *self == PSR::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline]
    pub fn is_odd(&self) -> bool {
        *self == PSR::ODD
    }
}
#[doc = "Possible values of the field `PCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCER {
    #[doc = "Parity control disabled."] DISABLED,
    #[doc = "Parity control enabled."] ENABLED,
}
impl PCER {
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
            PCER::DISABLED => false,
            PCER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PCER {
        match value {
            false => PCER::DISABLED,
            true => PCER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PCER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PCER::ENABLED
    }
}
#[doc = "Possible values of the field `WAKE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKER {
    #[doc = "Idle line."] IDLE,
    #[doc = "Address mark."] ADDRESS,
}
impl WAKER {
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
            WAKER::IDLE => false,
            WAKER::ADDRESS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKER {
        match value {
            false => WAKER::IDLE,
            true => WAKER::ADDRESS,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline]
    pub fn is_idle(&self) -> bool {
        *self == WAKER::IDLE
    }
    #[doc = "Checks if the value of the field is `ADDRESS`"]
    #[inline]
    pub fn is_address(&self) -> bool {
        *self == WAKER::ADDRESS
    }
}
#[doc = "Possible values of the field `M`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR {
    #[doc = "1 Start bit, 8 data bits, n stop bits (Do not miss M1 bit = 0)."] _8BITS,
    #[doc = "1 Start bit, 9 data bits, n stop bits (Do not miss M1 bit = 0)."] _9BITS,
}
impl MR {
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
            MR::_8BITS => false,
            MR::_9BITS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MR {
        match value {
            false => MR::_8BITS,
            true => MR::_9BITS,
        }
    }
    #[doc = "Checks if the value of the field is `_8BITS`"]
    #[inline]
    pub fn is_8bits(&self) -> bool {
        *self == MR::_8BITS
    }
    #[doc = "Checks if the value of the field is `_9BITS`"]
    #[inline]
    pub fn is_9bits(&self) -> bool {
        *self == MR::_9BITS
    }
}
#[doc = "Possible values of the field `MME`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMER {
    #[doc = "Receiver in active mode permanently."] DISABLED,
    #[doc = "Receiver can switch between mute mode and active mode."] ENABLED,
}
impl MMER {
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
            MMER::DISABLED => false,
            MMER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MMER {
        match value {
            false => MMER::DISABLED,
            true => MMER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MMER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == MMER::ENABLED
    }
}
#[doc = "Possible values of the field `CMIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMIER {
    #[doc = "Interrupt is inhibited."] DISABLED,
    #[doc = "An USART interrupt is generated when the CMF bit is set in the USART_ISR register."]
    ENABLED,
}
impl CMIER {
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
            CMIER::DISABLED => false,
            CMIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMIER {
        match value {
            false => CMIER::DISABLED,
            true => CMIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CMIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CMIER::ENABLED
    }
}
#[doc = "Possible values of the field `OVER8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVER8R {
    #[doc = "Oversampling by 16."] OVER16,
    #[doc = "Oversampling by 8."] OVER8,
}
impl OVER8R {
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
            OVER8R::OVER16 => false,
            OVER8R::OVER8 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OVER8R {
        match value {
            false => OVER8R::OVER16,
            true => OVER8R::OVER8,
        }
    }
    #[doc = "Checks if the value of the field is `OVER16`"]
    #[inline]
    pub fn is_over16(&self) -> bool {
        *self == OVER8R::OVER16
    }
    #[doc = "Checks if the value of the field is `OVER8`"]
    #[inline]
    pub fn is_over8(&self) -> bool {
        *self == OVER8R::OVER8
    }
}
#[doc = r" Value of the field"]
pub struct DEDTR {
    bits: u8,
}
impl DEDTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DEATR {
    bits: u8,
}
impl DEATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `RTOIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTOIER {
    #[doc = "Interrupt is inhibited."] DISABLED,
    #[doc = "An USART interrupt is generated when the RTOF bit is set in the USART_ISR register."]
    ENABLED,
}
impl RTOIER {
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
            RTOIER::DISABLED => false,
            RTOIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTOIER {
        match value {
            false => RTOIER::DISABLED,
            true => RTOIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RTOIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RTOIER::ENABLED
    }
}
#[doc = "Possible values of the field `EOBIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOBIER {
    #[doc = "Interrupt is inhibited."] DISABLED,
    #[doc = "An USART interrupt is generated when the EOBF flag is set in the USART_ISR register."]
    ENABLED,
}
impl EOBIER {
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
            EOBIER::DISABLED => false,
            EOBIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EOBIER {
        match value {
            false => EOBIER::DISABLED,
            true => EOBIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == EOBIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == EOBIER::ENABLED
    }
}
#[doc = "Possible values of the field `M1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M1R {
    #[doc = "1 Start bit, 9 data bits, n stop bits (Do not miss M0 bit = 1)."] _9BITS,
    #[doc = "1 Start bit, 7 data bits, n stop bits (Do not miss M0 bit = 0)."] _7BITS,
}
impl M1R {
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
            M1R::_9BITS => false,
            M1R::_7BITS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> M1R {
        match value {
            false => M1R::_9BITS,
            true => M1R::_7BITS,
        }
    }
    #[doc = "Checks if the value of the field is `_9BITS`"]
    #[inline]
    pub fn is_9bits(&self) -> bool {
        *self == M1R::_9BITS
    }
    #[doc = "Checks if the value of the field is `_7BITS`"]
    #[inline]
    pub fn is_7bits(&self) -> bool {
        *self == M1R::_7BITS
    }
}
#[doc = "Values that can be written to the field `UE`"]
pub enum UEW {
    #[doc = "USART Disabled."] DISABLED,
    #[doc = "USART Enabled."] ENABLED,
}
impl UEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UEW::DISABLED => false,
            UEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UEW<'a> {
    w: &'a mut W,
}
impl<'a> _UEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "USART Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UEW::DISABLED)
    }
    #[doc = "USART Enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UEW::ENABLED)
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
#[doc = "Values that can be written to the field `UESM`"]
pub enum UESMW {
    #[doc = "USART not able to wake up the MCU from Stop mode."] NOTABLE,
    #[doc = "USART able to wake up the MCU from Stop mode."] ABLE,
}
impl UESMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UESMW::NOTABLE => false,
            UESMW::ABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UESMW<'a> {
    w: &'a mut W,
}
impl<'a> _UESMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UESMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "USART not able to wake up the MCU from Stop mode."]
    #[inline]
    pub fn not_able(self) -> &'a mut W {
        self.variant(UESMW::NOTABLE)
    }
    #[doc = "USART able to wake up the MCU from Stop mode."]
    #[inline]
    pub fn able(self) -> &'a mut W {
        self.variant(UESMW::ABLE)
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
#[doc = "Values that can be written to the field `RE`"]
pub enum REW {
    #[doc = "Receiver is disabled."] DISABLED,
    #[doc = "Receiver is enabled and begins searching for a start bit."] ENABLED,
}
impl REW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REW::DISABLED => false,
            REW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REW<'a> {
    w: &'a mut W,
}
impl<'a> _REW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receiver is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REW::DISABLED)
    }
    #[doc = "Receiver is enabled and begins searching for a start bit."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REW::ENABLED)
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
#[doc = "Values that can be written to the field `TE`"]
pub enum TEW {
    #[doc = "Transmitter is disabled."] DISABLED,
    #[doc = "Transmitter is enabled."] ENABLED,
}
impl TEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TEW::DISABLED => false,
            TEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TEW<'a> {
    w: &'a mut W,
}
impl<'a> _TEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transmitter is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEW::DISABLED)
    }
    #[doc = "Transmitter is enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEW::ENABLED)
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
#[doc = "Values that can be written to the field `IDLEIE`"]
pub enum IDLEIEW {
    #[doc = "Interrupt is inhibited."] DISABLED,
    #[doc = "An USART interrupt is generated whenever IDLE=1 in the USART_ISR register."] ENABLED,
}
impl IDLEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IDLEIEW::DISABLED => false,
            IDLEIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDLEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _IDLEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDLEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt is inhibited."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IDLEIEW::DISABLED)
    }
    #[doc = "An USART interrupt is generated whenever IDLE=1 in the USART_ISR register."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IDLEIEW::ENABLED)
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
#[doc = "Values that can be written to the field `RXNEIE`"]
pub enum RXNEIEW {
    #[doc = "Interrupt is inhibited."] DISABLED,
    #[doc = "An USART interrupt is generated whenever ORE=1 or RXNE=1 in the USART_ISR register."]
    ENABLED,
}
impl RXNEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXNEIEW::DISABLED => false,
            RXNEIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXNEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXNEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXNEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt is inhibited."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXNEIEW::DISABLED)
    }
    #[doc = "An USART interrupt is generated whenever ORE=1 or RXNE=1 in the USART_ISR register."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXNEIEW::ENABLED)
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
#[doc = "Values that can be written to the field `TCIE`"]
pub enum TCIEW {
    #[doc = "Interrupt is inhibited."] DISABLED,
    #[doc = "An USART interrupt is generated whenever TC=1 in the USART_ISR register."] ENABLED,
}
impl TCIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCIEW::DISABLED => false,
            TCIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TCIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt is inhibited."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TCIEW::DISABLED)
    }
    #[doc = "An USART interrupt is generated whenever TC=1 in the USART_ISR register."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TCIEW::ENABLED)
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
#[doc = "Values that can be written to the field `TXEIE`"]
pub enum TXEIEW {
    #[doc = "Interrupt is inhibited."] DISABLED,
    #[doc = "An USART interrupt is generated whenever TXE=1 in the USART_ISR register."] ENABLED,
}
impl TXEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXEIEW::DISABLED => false,
            TXEIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt is inhibited."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXEIEW::DISABLED)
    }
    #[doc = "An USART interrupt is generated whenever TXE=1 in the USART_ISR register."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXEIEW::ENABLED)
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
#[doc = "Values that can be written to the field `PEIE`"]
pub enum PEIEW {
    #[doc = "Interrupt is inhibited."] DISABLED,
    #[doc = "An USART interrupt is generated whenever PE=1 in the USART_ISR register."] ENABLED,
}
impl PEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEIEW::DISABLED => false,
            PEIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _PEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt is inhibited."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PEIEW::DISABLED)
    }
    #[doc = "An USART interrupt is generated whenever PE=1 in the USART_ISR register."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PEIEW::ENABLED)
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
#[doc = "Values that can be written to the field `PS`"]
pub enum PSW {
    #[doc = "Even parity."] EVEN,
    #[doc = "Odd parity."] ODD,
}
impl PSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PSW::EVEN => false,
            PSW::ODD => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSW<'a> {
    w: &'a mut W,
}
impl<'a> _PSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Even parity."]
    #[inline]
    pub fn even(self) -> &'a mut W {
        self.variant(PSW::EVEN)
    }
    #[doc = "Odd parity."]
    #[inline]
    pub fn odd(self) -> &'a mut W {
        self.variant(PSW::ODD)
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
#[doc = "Values that can be written to the field `PCE`"]
pub enum PCEW {
    #[doc = "Parity control disabled."] DISABLED,
    #[doc = "Parity control enabled."] ENABLED,
}
impl PCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PCEW::DISABLED => false,
            PCEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCEW<'a> {
    w: &'a mut W,
}
impl<'a> _PCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Parity control disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PCEW::DISABLED)
    }
    #[doc = "Parity control enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PCEW::ENABLED)
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
#[doc = "Values that can be written to the field `WAKE`"]
pub enum WAKEW {
    #[doc = "Idle line."] IDLE,
    #[doc = "Address mark."] ADDRESS,
}
impl WAKEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKEW::IDLE => false,
            WAKEW::ADDRESS => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKEW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Idle line."]
    #[inline]
    pub fn idle(self) -> &'a mut W {
        self.variant(WAKEW::IDLE)
    }
    #[doc = "Address mark."]
    #[inline]
    pub fn address(self) -> &'a mut W {
        self.variant(WAKEW::ADDRESS)
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
#[doc = "Values that can be written to the field `M`"]
pub enum MW {
    #[doc = "1 Start bit, 8 data bits, n stop bits (Do not miss M1 bit = 0)."] _8BITS,
    #[doc = "1 Start bit, 9 data bits, n stop bits (Do not miss M1 bit = 0)."] _9BITS,
}
impl MW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MW::_8BITS => false,
            MW::_9BITS => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MW<'a> {
    w: &'a mut W,
}
impl<'a> _MW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "1 Start bit, 8 data bits, n stop bits (Do not miss M1 bit = 0)."]
    #[inline]
    pub fn _8bits(self) -> &'a mut W {
        self.variant(MW::_8BITS)
    }
    #[doc = "1 Start bit, 9 data bits, n stop bits (Do not miss M1 bit = 0)."]
    #[inline]
    pub fn _9bits(self) -> &'a mut W {
        self.variant(MW::_9BITS)
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
#[doc = "Values that can be written to the field `MME`"]
pub enum MMEW {
    #[doc = "Receiver in active mode permanently."] DISABLED,
    #[doc = "Receiver can switch between mute mode and active mode."] ENABLED,
}
impl MMEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MMEW::DISABLED => false,
            MMEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MMEW<'a> {
    w: &'a mut W,
}
impl<'a> _MMEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MMEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receiver in active mode permanently."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MMEW::DISABLED)
    }
    #[doc = "Receiver can switch between mute mode and active mode."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MMEW::ENABLED)
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
#[doc = "Values that can be written to the field `CMIE`"]
pub enum CMIEW {
    #[doc = "Interrupt is inhibited."] DISABLED,
    #[doc = "An USART interrupt is generated when the CMF bit is set in the USART_ISR register."]
    ENABLED,
}
impl CMIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMIEW::DISABLED => false,
            CMIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMIEW<'a> {
    w: &'a mut W,
}
impl<'a> _CMIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt is inhibited."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CMIEW::DISABLED)
    }
    #[doc = "An USART interrupt is generated when the CMF bit is set in the USART_ISR register."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMIEW::ENABLED)
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
#[doc = "Values that can be written to the field `OVER8`"]
pub enum OVER8W {
    #[doc = "Oversampling by 16."] OVER16,
    #[doc = "Oversampling by 8."] OVER8,
}
impl OVER8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OVER8W::OVER16 => false,
            OVER8W::OVER8 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OVER8W<'a> {
    w: &'a mut W,
}
impl<'a> _OVER8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OVER8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Oversampling by 16."]
    #[inline]
    pub fn over16(self) -> &'a mut W {
        self.variant(OVER8W::OVER16)
    }
    #[doc = "Oversampling by 8."]
    #[inline]
    pub fn over8(self) -> &'a mut W {
        self.variant(OVER8W::OVER8)
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
#[doc = r" Proxy"]
pub struct _DEDTW<'a> {
    w: &'a mut W,
}
impl<'a> _DEDTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DEATW<'a> {
    w: &'a mut W,
}
impl<'a> _DEATW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTOIE`"]
pub enum RTOIEW {
    #[doc = "Interrupt is inhibited."] DISABLED,
    #[doc = "An USART interrupt is generated when the RTOF bit is set in the USART_ISR register."]
    ENABLED,
}
impl RTOIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTOIEW::DISABLED => false,
            RTOIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTOIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RTOIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTOIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt is inhibited."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTOIEW::DISABLED)
    }
    #[doc = "An USART interrupt is generated when the RTOF bit is set in the USART_ISR register."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RTOIEW::ENABLED)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EOBIE`"]
pub enum EOBIEW {
    #[doc = "Interrupt is inhibited."] DISABLED,
    #[doc = "An USART interrupt is generated when the EOBF flag is set in the USART_ISR register."]
    ENABLED,
}
impl EOBIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EOBIEW::DISABLED => false,
            EOBIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EOBIEW<'a> {
    w: &'a mut W,
}
impl<'a> _EOBIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EOBIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt is inhibited."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EOBIEW::DISABLED)
    }
    #[doc = "An USART interrupt is generated when the EOBF flag is set in the USART_ISR register."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EOBIEW::ENABLED)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `M1`"]
pub enum M1W {
    #[doc = "1 Start bit, 9 data bits, n stop bits (Do not miss M0 bit = 1)."] _9BITS,
    #[doc = "1 Start bit, 7 data bits, n stop bits (Do not miss M0 bit = 0)."] _7BITS,
}
impl M1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            M1W::_9BITS => false,
            M1W::_7BITS => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M1W<'a> {
    w: &'a mut W,
}
impl<'a> _M1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "1 Start bit, 9 data bits, n stop bits (Do not miss M0 bit = 1)."]
    #[inline]
    pub fn _9bits(self) -> &'a mut W {
        self.variant(M1W::_9BITS)
    }
    #[doc = "1 Start bit, 7 data bits, n stop bits (Do not miss M0 bit = 0)."]
    #[inline]
    pub fn _7bits(self) -> &'a mut W {
        self.variant(M1W::_7BITS)
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
        const OFFSET: u8 = 28;
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
    #[doc = "Bit 0 - USART enable"]
    #[inline]
    pub fn ue(&self) -> UER {
        UER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - USART enable in Stop mode"]
    #[inline]
    pub fn uesm(&self) -> UESMR {
        UESMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline]
    pub fn re(&self) -> RER {
        RER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline]
    pub fn te(&self) -> TER {
        TER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - IDLE interrupt enable"]
    #[inline]
    pub fn idleie(&self) -> IDLEIER {
        IDLEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - RXNE interrupt enable"]
    #[inline]
    pub fn rxneie(&self) -> RXNEIER {
        RXNEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable"]
    #[inline]
    pub fn tcie(&self) -> TCIER {
        TCIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - interrupt enable"]
    #[inline]
    pub fn txeie(&self) -> TXEIER {
        TXEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - PE interrupt enable"]
    #[inline]
    pub fn peie(&self) -> PEIER {
        PEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Parity selection"]
    #[inline]
    pub fn ps(&self) -> PSR {
        PSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Parity control enable"]
    #[inline]
    pub fn pce(&self) -> PCER {
        PCER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Receiver wakeup method"]
    #[inline]
    pub fn wake(&self) -> WAKER {
        WAKER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Word length"]
    #[inline]
    pub fn m(&self) -> MR {
        MR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Mute mode enable"]
    #[inline]
    pub fn mme(&self) -> MMER {
        MMER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Character match interrupt enable"]
    #[inline]
    pub fn cmie(&self) -> CMIER {
        CMIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Oversampling mode"]
    #[inline]
    pub fn over8(&self) -> OVER8R {
        OVER8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:20 - Driver Enable deassertion time"]
    #[inline]
    pub fn dedt(&self) -> DEDTR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DEDTR { bits }
    }
    #[doc = "Bits 21:25 - Driver Enable assertion time"]
    #[inline]
    pub fn deat(&self) -> DEATR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DEATR { bits }
    }
    #[doc = "Bit 26 - Receiver timeout interrupt enable"]
    #[inline]
    pub fn rtoie(&self) -> RTOIER {
        RTOIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - End of Block interrupt enable"]
    #[inline]
    pub fn eobie(&self) -> EOBIER {
        EOBIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Word length"]
    #[inline]
    pub fn m1(&self) -> M1R {
        M1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
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
    #[doc = "Bit 0 - USART enable"]
    #[inline]
    pub fn ue(&mut self) -> _UEW {
        _UEW { w: self }
    }
    #[doc = "Bit 1 - USART enable in Stop mode"]
    #[inline]
    pub fn uesm(&mut self) -> _UESMW {
        _UESMW { w: self }
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline]
    pub fn re(&mut self) -> _REW {
        _REW { w: self }
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline]
    pub fn te(&mut self) -> _TEW {
        _TEW { w: self }
    }
    #[doc = "Bit 4 - IDLE interrupt enable"]
    #[inline]
    pub fn idleie(&mut self) -> _IDLEIEW {
        _IDLEIEW { w: self }
    }
    #[doc = "Bit 5 - RXNE interrupt enable"]
    #[inline]
    pub fn rxneie(&mut self) -> _RXNEIEW {
        _RXNEIEW { w: self }
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable"]
    #[inline]
    pub fn tcie(&mut self) -> _TCIEW {
        _TCIEW { w: self }
    }
    #[doc = "Bit 7 - interrupt enable"]
    #[inline]
    pub fn txeie(&mut self) -> _TXEIEW {
        _TXEIEW { w: self }
    }
    #[doc = "Bit 8 - PE interrupt enable"]
    #[inline]
    pub fn peie(&mut self) -> _PEIEW {
        _PEIEW { w: self }
    }
    #[doc = "Bit 9 - Parity selection"]
    #[inline]
    pub fn ps(&mut self) -> _PSW {
        _PSW { w: self }
    }
    #[doc = "Bit 10 - Parity control enable"]
    #[inline]
    pub fn pce(&mut self) -> _PCEW {
        _PCEW { w: self }
    }
    #[doc = "Bit 11 - Receiver wakeup method"]
    #[inline]
    pub fn wake(&mut self) -> _WAKEW {
        _WAKEW { w: self }
    }
    #[doc = "Bit 12 - Word length"]
    #[inline]
    pub fn m(&mut self) -> _MW {
        _MW { w: self }
    }
    #[doc = "Bit 13 - Mute mode enable"]
    #[inline]
    pub fn mme(&mut self) -> _MMEW {
        _MMEW { w: self }
    }
    #[doc = "Bit 14 - Character match interrupt enable"]
    #[inline]
    pub fn cmie(&mut self) -> _CMIEW {
        _CMIEW { w: self }
    }
    #[doc = "Bit 15 - Oversampling mode"]
    #[inline]
    pub fn over8(&mut self) -> _OVER8W {
        _OVER8W { w: self }
    }
    #[doc = "Bits 16:20 - Driver Enable deassertion time"]
    #[inline]
    pub fn dedt(&mut self) -> _DEDTW {
        _DEDTW { w: self }
    }
    #[doc = "Bits 21:25 - Driver Enable assertion time"]
    #[inline]
    pub fn deat(&mut self) -> _DEATW {
        _DEATW { w: self }
    }
    #[doc = "Bit 26 - Receiver timeout interrupt enable"]
    #[inline]
    pub fn rtoie(&mut self) -> _RTOIEW {
        _RTOIEW { w: self }
    }
    #[doc = "Bit 27 - End of Block interrupt enable"]
    #[inline]
    pub fn eobie(&mut self) -> _EOBIEW {
        _EOBIEW { w: self }
    }
    #[doc = "Bit 28 - Word length"]
    #[inline]
    pub fn m1(&mut self) -> _M1W {
        _M1W { w: self }
    }
}
