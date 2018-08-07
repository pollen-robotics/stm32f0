#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCMR1_INPUT {
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
#[doc = "Possible values of the field `IC1F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IC1FR {
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
impl IC1FR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IC1FR::NOFILTER => 0,
            IC1FR::FCK_INT_N2 => 1,
            IC1FR::FCK_INT_N4 => 2,
            IC1FR::FCK_INT_N8 => 3,
            IC1FR::FDTSDIV2_N6 => 4,
            IC1FR::FDTSDIV2_N8 => 5,
            IC1FR::FDTSDIV4_N6 => 6,
            IC1FR::FDTSDIV4_N8 => 7,
            IC1FR::FDTSDIV8_N6 => 8,
            IC1FR::FDTSDIV8_N8 => 9,
            IC1FR::FDTSDIV16_N5 => 10,
            IC1FR::FDTSDIV16_N6 => 11,
            IC1FR::FDTSDIV16_N8 => 12,
            IC1FR::FDTSDIV32_N5 => 13,
            IC1FR::FDTSDIV32_N6 => 14,
            IC1FR::FDTSDIV32_N8 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IC1FR {
        match value {
            0 => IC1FR::NOFILTER,
            1 => IC1FR::FCK_INT_N2,
            2 => IC1FR::FCK_INT_N4,
            3 => IC1FR::FCK_INT_N8,
            4 => IC1FR::FDTSDIV2_N6,
            5 => IC1FR::FDTSDIV2_N8,
            6 => IC1FR::FDTSDIV4_N6,
            7 => IC1FR::FDTSDIV4_N8,
            8 => IC1FR::FDTSDIV8_N6,
            9 => IC1FR::FDTSDIV8_N8,
            10 => IC1FR::FDTSDIV16_N5,
            11 => IC1FR::FDTSDIV16_N6,
            12 => IC1FR::FDTSDIV16_N8,
            13 => IC1FR::FDTSDIV32_N5,
            14 => IC1FR::FDTSDIV32_N6,
            15 => IC1FR::FDTSDIV32_N8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOFILTER`"]
    #[inline]
    pub fn is_no_filter(&self) -> bool {
        *self == IC1FR::NOFILTER
    }
    #[doc = "Checks if the value of the field is `FCK_INT_N2`"]
    #[inline]
    pub fn is_f_ck_int_n2(&self) -> bool {
        *self == IC1FR::FCK_INT_N2
    }
    #[doc = "Checks if the value of the field is `FCK_INT_N4`"]
    #[inline]
    pub fn is_f_ck_int_n4(&self) -> bool {
        *self == IC1FR::FCK_INT_N4
    }
    #[doc = "Checks if the value of the field is `FCK_INT_N8`"]
    #[inline]
    pub fn is_f_ck_int_n8(&self) -> bool {
        *self == IC1FR::FCK_INT_N8
    }
    #[doc = "Checks if the value of the field is `FDTSDIV2_N6`"]
    #[inline]
    pub fn is_f_dtsdiv2_n6(&self) -> bool {
        *self == IC1FR::FDTSDIV2_N6
    }
    #[doc = "Checks if the value of the field is `FDTSDIV2_N8`"]
    #[inline]
    pub fn is_f_dtsdiv2_n8(&self) -> bool {
        *self == IC1FR::FDTSDIV2_N8
    }
    #[doc = "Checks if the value of the field is `FDTSDIV4_N6`"]
    #[inline]
    pub fn is_f_dtsdiv4_n6(&self) -> bool {
        *self == IC1FR::FDTSDIV4_N6
    }
    #[doc = "Checks if the value of the field is `FDTSDIV4_N8`"]
    #[inline]
    pub fn is_f_dtsdiv4_n8(&self) -> bool {
        *self == IC1FR::FDTSDIV4_N8
    }
    #[doc = "Checks if the value of the field is `FDTSDIV8_N6`"]
    #[inline]
    pub fn is_f_dtsdiv8_n6(&self) -> bool {
        *self == IC1FR::FDTSDIV8_N6
    }
    #[doc = "Checks if the value of the field is `FDTSDIV8_N8`"]
    #[inline]
    pub fn is_f_dtsdiv8_n8(&self) -> bool {
        *self == IC1FR::FDTSDIV8_N8
    }
    #[doc = "Checks if the value of the field is `FDTSDIV16_N5`"]
    #[inline]
    pub fn is_f_dtsdiv16_n5(&self) -> bool {
        *self == IC1FR::FDTSDIV16_N5
    }
    #[doc = "Checks if the value of the field is `FDTSDIV16_N6`"]
    #[inline]
    pub fn is_f_dtsdiv16_n6(&self) -> bool {
        *self == IC1FR::FDTSDIV16_N6
    }
    #[doc = "Checks if the value of the field is `FDTSDIV16_N8`"]
    #[inline]
    pub fn is_f_dtsdiv16_n8(&self) -> bool {
        *self == IC1FR::FDTSDIV16_N8
    }
    #[doc = "Checks if the value of the field is `FDTSDIV32_N5`"]
    #[inline]
    pub fn is_f_dtsdiv32_n5(&self) -> bool {
        *self == IC1FR::FDTSDIV32_N5
    }
    #[doc = "Checks if the value of the field is `FDTSDIV32_N6`"]
    #[inline]
    pub fn is_f_dtsdiv32_n6(&self) -> bool {
        *self == IC1FR::FDTSDIV32_N6
    }
    #[doc = "Checks if the value of the field is `FDTSDIV32_N8`"]
    #[inline]
    pub fn is_f_dtsdiv32_n8(&self) -> bool {
        *self == IC1FR::FDTSDIV32_N8
    }
}
#[doc = "Possible values of the field `IC1PSC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IC1PSCR {
    #[doc = "no prescaler, capture is done each time an edge is detected on the capture input"]
    NO,
    #[doc = "capture is done once every 2 events"]
    _2EVENTS,
    #[doc = "capture is done once every 4 events"]
    _4EVENTS,
    #[doc = "capture is done once every 8 events"]
    _8EVENTS,
}
impl IC1PSCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IC1PSCR::NO => 0,
            IC1PSCR::_2EVENTS => 1,
            IC1PSCR::_4EVENTS => 2,
            IC1PSCR::_8EVENTS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IC1PSCR {
        match value {
            0 => IC1PSCR::NO,
            1 => IC1PSCR::_2EVENTS,
            2 => IC1PSCR::_4EVENTS,
            3 => IC1PSCR::_8EVENTS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == IC1PSCR::NO
    }
    #[doc = "Checks if the value of the field is `_2EVENTS`"]
    #[inline]
    pub fn is_2events(&self) -> bool {
        *self == IC1PSCR::_2EVENTS
    }
    #[doc = "Checks if the value of the field is `_4EVENTS`"]
    #[inline]
    pub fn is_4events(&self) -> bool {
        *self == IC1PSCR::_4EVENTS
    }
    #[doc = "Checks if the value of the field is `_8EVENTS`"]
    #[inline]
    pub fn is_8events(&self) -> bool {
        *self == IC1PSCR::_8EVENTS
    }
}
#[doc = "Possible values of the field `CC1S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1SR {
    #[doc = "CC1 channel is configured as output"]
    CC1OUTPUT,
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI1"]
    IC1MAPPEDTI1,
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI2"]
    IC1MAPPEDTI2,
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TRC"]
    IC1MAPPEDTRC,
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
#[doc = "Values that can be written to the field `IC1F`"]
pub enum IC1FW {
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
impl IC1FW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IC1FW::NOFILTER => 0,
            IC1FW::FCK_INT_N2 => 1,
            IC1FW::FCK_INT_N4 => 2,
            IC1FW::FCK_INT_N8 => 3,
            IC1FW::FDTSDIV2_N6 => 4,
            IC1FW::FDTSDIV2_N8 => 5,
            IC1FW::FDTSDIV4_N6 => 6,
            IC1FW::FDTSDIV4_N8 => 7,
            IC1FW::FDTSDIV8_N6 => 8,
            IC1FW::FDTSDIV8_N8 => 9,
            IC1FW::FDTSDIV16_N5 => 10,
            IC1FW::FDTSDIV16_N6 => 11,
            IC1FW::FDTSDIV16_N8 => 12,
            IC1FW::FDTSDIV32_N5 => 13,
            IC1FW::FDTSDIV32_N6 => 14,
            IC1FW::FDTSDIV32_N8 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IC1FW<'a> {
    w: &'a mut W,
}
impl<'a> _IC1FW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IC1FW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No filter, sampling is done at fDTS"]
    #[inline]
    pub fn no_filter(self) -> &'a mut W {
        self.variant(IC1FW::NOFILTER)
    }
    #[doc = "fSAMPLING = fCK_INT, N = 2"]
    #[inline]
    pub fn f_ck_int_n2(self) -> &'a mut W {
        self.variant(IC1FW::FCK_INT_N2)
    }
    #[doc = "fSAMPLING = fCK_INT, N = 4"]
    #[inline]
    pub fn f_ck_int_n4(self) -> &'a mut W {
        self.variant(IC1FW::FCK_INT_N4)
    }
    #[doc = "fSAMPLING = fCK_INT, N = 8"]
    #[inline]
    pub fn f_ck_int_n8(self) -> &'a mut W {
        self.variant(IC1FW::FCK_INT_N8)
    }
    #[doc = "fSAMPLING = fDTS / 2, N = 6"]
    #[inline]
    pub fn f_dtsdiv2_n6(self) -> &'a mut W {
        self.variant(IC1FW::FDTSDIV2_N6)
    }
    #[doc = "fSAMPLING = fDTS / 2, N = 8"]
    #[inline]
    pub fn f_dtsdiv2_n8(self) -> &'a mut W {
        self.variant(IC1FW::FDTSDIV2_N8)
    }
    #[doc = "fSAMPLING = fDTS / 4, N = 6"]
    #[inline]
    pub fn f_dtsdiv4_n6(self) -> &'a mut W {
        self.variant(IC1FW::FDTSDIV4_N6)
    }
    #[doc = "fSAMPLING = fDTS / 4, N = 8"]
    #[inline]
    pub fn f_dtsdiv4_n8(self) -> &'a mut W {
        self.variant(IC1FW::FDTSDIV4_N8)
    }
    #[doc = "fSAMPLING = fDTS / 8, N = 6"]
    #[inline]
    pub fn f_dtsdiv8_n6(self) -> &'a mut W {
        self.variant(IC1FW::FDTSDIV8_N6)
    }
    #[doc = "fSAMPLING = fDTS / 8, N = 8"]
    #[inline]
    pub fn f_dtsdiv8_n8(self) -> &'a mut W {
        self.variant(IC1FW::FDTSDIV8_N8)
    }
    #[doc = "fSAMPLING = fDTS / 16, N = 5"]
    #[inline]
    pub fn f_dtsdiv16_n5(self) -> &'a mut W {
        self.variant(IC1FW::FDTSDIV16_N5)
    }
    #[doc = "fSAMPLING = fDTS / 16, N = 6"]
    #[inline]
    pub fn f_dtsdiv16_n6(self) -> &'a mut W {
        self.variant(IC1FW::FDTSDIV16_N6)
    }
    #[doc = "fSAMPLING = fDTS / 16, N = 8"]
    #[inline]
    pub fn f_dtsdiv16_n8(self) -> &'a mut W {
        self.variant(IC1FW::FDTSDIV16_N8)
    }
    #[doc = "fSAMPLING = fDTS / 32, N = 5"]
    #[inline]
    pub fn f_dtsdiv32_n5(self) -> &'a mut W {
        self.variant(IC1FW::FDTSDIV32_N5)
    }
    #[doc = "fSAMPLING = fDTS / 32, N = 6"]
    #[inline]
    pub fn f_dtsdiv32_n6(self) -> &'a mut W {
        self.variant(IC1FW::FDTSDIV32_N6)
    }
    #[doc = "fSAMPLING = fDTS / 32, N = 8"]
    #[inline]
    pub fn f_dtsdiv32_n8(self) -> &'a mut W {
        self.variant(IC1FW::FDTSDIV32_N8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IC1PSC`"]
pub enum IC1PSCW {
    #[doc = "no prescaler, capture is done each time an edge is detected on the capture input"]
    NO,
    #[doc = "capture is done once every 2 events"]
    _2EVENTS,
    #[doc = "capture is done once every 4 events"]
    _4EVENTS,
    #[doc = "capture is done once every 8 events"]
    _8EVENTS,
}
impl IC1PSCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IC1PSCW::NO => 0,
            IC1PSCW::_2EVENTS => 1,
            IC1PSCW::_4EVENTS => 2,
            IC1PSCW::_8EVENTS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IC1PSCW<'a> {
    w: &'a mut W,
}
impl<'a> _IC1PSCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IC1PSCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "no prescaler, capture is done each time an edge is detected on the capture input"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(IC1PSCW::NO)
    }
    #[doc = "capture is done once every 2 events"]
    #[inline]
    pub fn _2events(self) -> &'a mut W {
        self.variant(IC1PSCW::_2EVENTS)
    }
    #[doc = "capture is done once every 4 events"]
    #[inline]
    pub fn _4events(self) -> &'a mut W {
        self.variant(IC1PSCW::_4EVENTS)
    }
    #[doc = "capture is done once every 8 events"]
    #[inline]
    pub fn _8events(self) -> &'a mut W {
        self.variant(IC1PSCW::_8EVENTS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CC1S`"]
pub enum CC1SW {
    #[doc = "CC1 channel is configured as output"]
    CC1OUTPUT,
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI1"]
    IC1MAPPEDTI1,
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI2"]
    IC1MAPPEDTI2,
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TRC"]
    IC1MAPPEDTRC,
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
    #[doc = "Bits 4:7 - Input capture 1 filter"]
    #[inline]
    pub fn ic1f(&self) -> IC1FR {
        IC1FR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Input capture 1 prescaler"]
    #[inline]
    pub fn ic1psc(&self) -> IC1PSCR {
        IC1PSCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 4:7 - Input capture 1 filter"]
    #[inline]
    pub fn ic1f(&mut self) -> _IC1FW {
        _IC1FW { w: self }
    }
    #[doc = "Bits 2:3 - Input capture 1 prescaler"]
    #[inline]
    pub fn ic1psc(&mut self) -> _IC1PSCW {
        _IC1PSCW { w: self }
    }
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline]
    pub fn cc1s(&mut self) -> _CC1SW {
        _CC1SW { w: self }
    }
}
