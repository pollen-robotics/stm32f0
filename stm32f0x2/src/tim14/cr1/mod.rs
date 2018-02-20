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
#[doc = "Possible values of the field `CKD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKDR {
    #[doc = "Clock is not divided"]
    DIV1,
    #[doc = "Clock is divided by 2"]
    DIV2,
    #[doc = "Clock is divided by 4"]
    DIV4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CKDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CKDR::DIV1 => 0,
            CKDR::DIV2 => 1,
            CKDR::DIV4 => 2,
            CKDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CKDR {
        match value {
            0 => CKDR::DIV1,
            1 => CKDR::DIV2,
            2 => CKDR::DIV4,
            i => CKDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == CKDR::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == CKDR::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == CKDR::DIV4
    }
}
#[doc = "Possible values of the field `ARPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARPER {
    #[doc = "TIMx_ARR register is not buffered"]
    NOTBUFFERED,
    #[doc = "TIMx_ARR register is buffered"]
    BUFFERED,
}
impl ARPER {
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
            ARPER::NOTBUFFERED => false,
            ARPER::BUFFERED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ARPER {
        match value {
            false => ARPER::NOTBUFFERED,
            true => ARPER::BUFFERED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTBUFFERED`"]
    #[inline]
    pub fn is_not_buffered(&self) -> bool {
        *self == ARPER::NOTBUFFERED
    }
    #[doc = "Checks if the value of the field is `BUFFERED`"]
    #[inline]
    pub fn is_buffered(&self) -> bool {
        *self == ARPER::BUFFERED
    }
}
#[doc = "Possible values of the field `URS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum URSR {# [ doc = "Any of the events generate an update interrupt or DMA request if enabled" ] ANYEVENT , # [ doc = "Only counter overflow/underflow generates an update interrupt or DMA request if enabled" ] ONLYOVERUNDER ,}
impl URSR {
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
            URSR::ANYEVENT => false,
            URSR::ONLYOVERUNDER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> URSR {
        match value {
            false => URSR::ANYEVENT,
            true => URSR::ONLYOVERUNDER,
        }
    }
    #[doc = "Checks if the value of the field is `ANYEVENT`"]
    #[inline]
    pub fn is_any_event(&self) -> bool {
        *self == URSR::ANYEVENT
    }
    #[doc = "Checks if the value of the field is `ONLYOVERUNDER`"]
    #[inline]
    pub fn is_only_over_under(&self) -> bool {
        *self == URSR::ONLYOVERUNDER
    }
}
#[doc = "Possible values of the field `UDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UDISR {
    #[doc = "UEV enabled."]
    ENABLED,
    #[doc = "UEV disabled."]
    DISABLED,
}
impl UDISR {
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
            UDISR::ENABLED => false,
            UDISR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UDISR {
        match value {
            false => UDISR::ENABLED,
            true => UDISR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == UDISR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == UDISR::DISABLED
    }
}
#[doc = "Possible values of the field `CEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CENR {
    #[doc = "Counter disabled"]
    DISABLED,
    #[doc = "Counter enabled"]
    ENABLED,
}
impl CENR {
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
            CENR::DISABLED => false,
            CENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CENR {
        match value {
            false => CENR::DISABLED,
            true => CENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `CKD`"]
pub enum CKDW {
    #[doc = "Clock is not divided"]
    DIV1,
    #[doc = "Clock is divided by 2"]
    DIV2,
    #[doc = "Clock is divided by 4"]
    DIV4,
}
impl CKDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CKDW::DIV1 => 0,
            CKDW::DIV2 => 1,
            CKDW::DIV4 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CKDW<'a> {
    w: &'a mut W,
}
impl<'a> _CKDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CKDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Clock is not divided"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(CKDW::DIV1)
    }
    #[doc = "Clock is divided by 2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(CKDW::DIV2)
    }
    #[doc = "Clock is divided by 4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(CKDW::DIV4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ARPE`"]
pub enum ARPEW {
    #[doc = "TIMx_ARR register is not buffered"]
    NOTBUFFERED,
    #[doc = "TIMx_ARR register is buffered"]
    BUFFERED,
}
impl ARPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ARPEW::NOTBUFFERED => false,
            ARPEW::BUFFERED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ARPEW<'a> {
    w: &'a mut W,
}
impl<'a> _ARPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ARPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TIMx_ARR register is not buffered"]
    #[inline]
    pub fn not_buffered(self) -> &'a mut W {
        self.variant(ARPEW::NOTBUFFERED)
    }
    #[doc = "TIMx_ARR register is buffered"]
    #[inline]
    pub fn buffered(self) -> &'a mut W {
        self.variant(ARPEW::BUFFERED)
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
#[doc = "Values that can be written to the field `URS`"]
pub enum URSW {# [ doc = "Any of the events generate an update interrupt or DMA request if enabled" ] ANYEVENT , # [ doc = "Only counter overflow/underflow generates an update interrupt or DMA request if enabled" ] ONLYOVERUNDER ,}
impl URSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            URSW::ANYEVENT => false,
            URSW::ONLYOVERUNDER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _URSW<'a> {
    w: &'a mut W,
}
impl<'a> _URSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: URSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Any of the events generate an update interrupt or DMA request if enabled"]
    #[inline]
    pub fn any_event(self) -> &'a mut W {
        self.variant(URSW::ANYEVENT)
    }
    # [ doc = "Only counter overflow/underflow generates an update interrupt or DMA request if enabled" ] # [ inline ]
    pub fn only_over_under(self) -> &'a mut W {
        self.variant(URSW::ONLYOVERUNDER)
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
#[doc = "Values that can be written to the field `UDIS`"]
pub enum UDISW {
    #[doc = "UEV enabled."]
    ENABLED,
    #[doc = "UEV disabled."]
    DISABLED,
}
impl UDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UDISW::ENABLED => false,
            UDISW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UDISW<'a> {
    w: &'a mut W,
}
impl<'a> _UDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "UEV enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UDISW::ENABLED)
    }
    #[doc = "UEV disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UDISW::DISABLED)
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
#[doc = "Values that can be written to the field `CEN`"]
pub enum CENW {
    #[doc = "Counter disabled"]
    DISABLED,
    #[doc = "Counter enabled"]
    ENABLED,
}
impl CENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CENW::DISABLED => false,
            CENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CENW<'a> {
    w: &'a mut W,
}
impl<'a> _CENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Counter disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CENW::DISABLED)
    }
    #[doc = "Counter enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CENW::ENABLED)
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
    # [ doc = "Bits 8:9 - Division ratio between the timer clock (CK_INT) frequency and sampling clock" ] # [ inline ]
    pub fn ckd(&self) -> CKDR {
        CKDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Auto-reload preload enable"]
    #[inline]
    pub fn arpe(&self) -> ARPER {
        ARPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Update request source"]
    #[inline]
    pub fn urs(&self) -> URSR {
        URSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline]
    pub fn udis(&self) -> UDISR {
        UDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Counter enable"]
    #[inline]
    pub fn cen(&self) -> CENR {
        CENR::_from({
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
    # [ doc = "Bits 8:9 - Division ratio between the timer clock (CK_INT) frequency and sampling clock" ] # [ inline ]
    pub fn ckd(&mut self) -> _CKDW {
        _CKDW { w: self }
    }
    #[doc = "Bit 7 - Auto-reload preload enable"]
    #[inline]
    pub fn arpe(&mut self) -> _ARPEW {
        _ARPEW { w: self }
    }
    #[doc = "Bit 2 - Update request source"]
    #[inline]
    pub fn urs(&mut self) -> _URSW {
        _URSW { w: self }
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline]
    pub fn udis(&mut self) -> _UDISW {
        _UDISW { w: self }
    }
    #[doc = "Bit 0 - Counter enable"]
    #[inline]
    pub fn cen(&mut self) -> _CENW {
        _CENW { w: self }
    }
}
