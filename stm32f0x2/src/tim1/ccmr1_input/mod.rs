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
#[doc = "Possible values of the field `IC2F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IC2FR {
    #[doc = "No filter, sampling is done at fDTS"] NOFILTER,
    #[doc = "fSAMPLING = fCK_INT, N = 2"] FCK_INT_N2,
    #[doc = "fSAMPLING = fCK_INT, N = 4"] FCK_INT_N4,
    #[doc = "fSAMPLING = fCK_INT, N = 8"] FCK_INT_N8,
    #[doc = "fSAMPLING = fDTS / 2, N = 6"] FDTSDIV2_N6,
    #[doc = "fSAMPLING = fDTS / 2, N = 8"] FDTSDIV2_N8,
    #[doc = "fSAMPLING = fDTS / 4, N = 6"] FDTSDIV4_N6,
    #[doc = "fSAMPLING = fDTS / 4, N = 8"] FDTSDIV4_N8,
    #[doc = "fSAMPLING = fDTS / 8, N = 6"] FDTSDIV8_N6,
    #[doc = "fSAMPLING = fDTS / 8, N = 8"] FDTSDIV8_N8,
    #[doc = "fSAMPLING = fDTS / 16, N = 5"] FDTSDIV16_N5,
    #[doc = "fSAMPLING = fDTS / 16, N = 6"] FDTSDIV16_N6,
    #[doc = "fSAMPLING = fDTS / 16, N = 8"] FDTSDIV16_N8,
    #[doc = "fSAMPLING = fDTS / 32, N = 5"] FDTSDIV32_N5,
    #[doc = "fSAMPLING = fDTS / 32, N = 6"] FDTSDIV32_N6,
    #[doc = "fSAMPLING = fDTS / 32, N = 8"] FDTSDIV32_N8,
}
impl IC2FR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            IC2FR::NOFILTER => 0,
            IC2FR::FCK_INT_N2 => 1,
            IC2FR::FCK_INT_N4 => 2,
            IC2FR::FCK_INT_N8 => 3,
            IC2FR::FDTSDIV2_N6 => 4,
            IC2FR::FDTSDIV2_N8 => 5,
            IC2FR::FDTSDIV4_N6 => 6,
            IC2FR::FDTSDIV4_N8 => 7,
            IC2FR::FDTSDIV8_N6 => 8,
            IC2FR::FDTSDIV8_N8 => 9,
            IC2FR::FDTSDIV16_N5 => 10,
            IC2FR::FDTSDIV16_N6 => 11,
            IC2FR::FDTSDIV16_N8 => 12,
            IC2FR::FDTSDIV32_N5 => 13,
            IC2FR::FDTSDIV32_N6 => 14,
            IC2FR::FDTSDIV32_N8 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> IC2FR {
        match value {
            0 => IC2FR::NOFILTER,
            1 => IC2FR::FCK_INT_N2,
            2 => IC2FR::FCK_INT_N4,
            3 => IC2FR::FCK_INT_N8,
            4 => IC2FR::FDTSDIV2_N6,
            5 => IC2FR::FDTSDIV2_N8,
            6 => IC2FR::FDTSDIV4_N6,
            7 => IC2FR::FDTSDIV4_N8,
            8 => IC2FR::FDTSDIV8_N6,
            9 => IC2FR::FDTSDIV8_N8,
            10 => IC2FR::FDTSDIV16_N5,
            11 => IC2FR::FDTSDIV16_N6,
            12 => IC2FR::FDTSDIV16_N8,
            13 => IC2FR::FDTSDIV32_N5,
            14 => IC2FR::FDTSDIV32_N6,
            15 => IC2FR::FDTSDIV32_N8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOFILTER`"]
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        *self == IC2FR::NOFILTER
    }
    #[doc = "Checks if the value of the field is `FCK_INT_N2`"]
    #[inline(always)]
    pub fn is_f_ck_int_n2(&self) -> bool {
        *self == IC2FR::FCK_INT_N2
    }
    #[doc = "Checks if the value of the field is `FCK_INT_N4`"]
    #[inline(always)]
    pub fn is_f_ck_int_n4(&self) -> bool {
        *self == IC2FR::FCK_INT_N4
    }
    #[doc = "Checks if the value of the field is `FCK_INT_N8`"]
    #[inline(always)]
    pub fn is_f_ck_int_n8(&self) -> bool {
        *self == IC2FR::FCK_INT_N8
    }
    #[doc = "Checks if the value of the field is `FDTSDIV2_N6`"]
    #[inline(always)]
    pub fn is_f_dtsdiv2_n6(&self) -> bool {
        *self == IC2FR::FDTSDIV2_N6
    }
    #[doc = "Checks if the value of the field is `FDTSDIV2_N8`"]
    #[inline(always)]
    pub fn is_f_dtsdiv2_n8(&self) -> bool {
        *self == IC2FR::FDTSDIV2_N8
    }
    #[doc = "Checks if the value of the field is `FDTSDIV4_N6`"]
    #[inline(always)]
    pub fn is_f_dtsdiv4_n6(&self) -> bool {
        *self == IC2FR::FDTSDIV4_N6
    }
    #[doc = "Checks if the value of the field is `FDTSDIV4_N8`"]
    #[inline(always)]
    pub fn is_f_dtsdiv4_n8(&self) -> bool {
        *self == IC2FR::FDTSDIV4_N8
    }
    #[doc = "Checks if the value of the field is `FDTSDIV8_N6`"]
    #[inline(always)]
    pub fn is_f_dtsdiv8_n6(&self) -> bool {
        *self == IC2FR::FDTSDIV8_N6
    }
    #[doc = "Checks if the value of the field is `FDTSDIV8_N8`"]
    #[inline(always)]
    pub fn is_f_dtsdiv8_n8(&self) -> bool {
        *self == IC2FR::FDTSDIV8_N8
    }
    #[doc = "Checks if the value of the field is `FDTSDIV16_N5`"]
    #[inline(always)]
    pub fn is_f_dtsdiv16_n5(&self) -> bool {
        *self == IC2FR::FDTSDIV16_N5
    }
    #[doc = "Checks if the value of the field is `FDTSDIV16_N6`"]
    #[inline(always)]
    pub fn is_f_dtsdiv16_n6(&self) -> bool {
        *self == IC2FR::FDTSDIV16_N6
    }
    #[doc = "Checks if the value of the field is `FDTSDIV16_N8`"]
    #[inline(always)]
    pub fn is_f_dtsdiv16_n8(&self) -> bool {
        *self == IC2FR::FDTSDIV16_N8
    }
    #[doc = "Checks if the value of the field is `FDTSDIV32_N5`"]
    #[inline(always)]
    pub fn is_f_dtsdiv32_n5(&self) -> bool {
        *self == IC2FR::FDTSDIV32_N5
    }
    #[doc = "Checks if the value of the field is `FDTSDIV32_N6`"]
    #[inline(always)]
    pub fn is_f_dtsdiv32_n6(&self) -> bool {
        *self == IC2FR::FDTSDIV32_N6
    }
    #[doc = "Checks if the value of the field is `FDTSDIV32_N8`"]
    #[inline(always)]
    pub fn is_f_dtsdiv32_n8(&self) -> bool {
        *self == IC2FR::FDTSDIV32_N8
    }
}
#[doc = "Possible values of the field `IC2PCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IC2PCSR {
    #[doc = "no prescaler, capture is done each time an edge is detected on the capture input"] _1EVENT,
    #[doc = "capture is done once every 2 events"] _2EVENTS,
    #[doc = "capture is done once every 4 events"] _4EVENTS,
    #[doc = "capture is done once every 8 events"] _8EVENTS,
}
impl IC2PCSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            IC2PCSR::_1EVENT => 0,
            IC2PCSR::_2EVENTS => 1,
            IC2PCSR::_4EVENTS => 2,
            IC2PCSR::_8EVENTS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> IC2PCSR {
        match value {
            0 => IC2PCSR::_1EVENT,
            1 => IC2PCSR::_2EVENTS,
            2 => IC2PCSR::_4EVENTS,
            3 => IC2PCSR::_8EVENTS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1EVENT`"]
    #[inline(always)]
    pub fn is_1event(&self) -> bool {
        *self == IC2PCSR::_1EVENT
    }
    #[doc = "Checks if the value of the field is `_2EVENTS`"]
    #[inline(always)]
    pub fn is_2events(&self) -> bool {
        *self == IC2PCSR::_2EVENTS
    }
    #[doc = "Checks if the value of the field is `_4EVENTS`"]
    #[inline(always)]
    pub fn is_4events(&self) -> bool {
        *self == IC2PCSR::_4EVENTS
    }
    #[doc = "Checks if the value of the field is `_8EVENTS`"]
    #[inline(always)]
    pub fn is_8events(&self) -> bool {
        *self == IC2PCSR::_8EVENTS
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
    pub fn is_cc2output(&self) -> bool {
        *self == CC2SR::CC2OUTPUT
    }
    #[doc = "Checks if the value of the field is `IC2MAPPEDTI2`"]
    #[inline(always)]
    pub fn is_ic2mapped_ti2(&self) -> bool {
        *self == CC2SR::IC2MAPPEDTI2
    }
    #[doc = "Checks if the value of the field is `IC2MAPPEDTI1`"]
    #[inline(always)]
    pub fn is_ic2mapped_ti1(&self) -> bool {
        *self == CC2SR::IC2MAPPEDTI1
    }
    #[doc = "Checks if the value of the field is `IC2MAPPEDTRC`"]
    #[inline(always)]
    pub fn is_ic2mapped_trc(&self) -> bool {
        *self == CC2SR::IC2MAPPEDTRC
    }
}
#[doc = "Possible values of the field `IC1F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IC1FR {
    #[doc = "No filter, sampling is done at fDTS"] NOFILTER,
    #[doc = "fSAMPLING = fCK_INT, N = 2"] FCK_INT_N2,
    #[doc = "fSAMPLING = fCK_INT, N = 4"] FCK_INT_N4,
    #[doc = "fSAMPLING = fCK_INT, N = 8"] FCK_INT_N8,
    #[doc = "fSAMPLING = fDTS / 2, N = 6"] FDTSDIV2_N6,
    #[doc = "fSAMPLING = fDTS / 2, N = 8"] FDTSDIV2_N8,
    #[doc = "fSAMPLING = fDTS / 4, N = 6"] FDTSDIV4_N6,
    #[doc = "fSAMPLING = fDTS / 4, N = 8"] FDTSDIV4_N8,
    #[doc = "fSAMPLING = fDTS / 8, N = 6"] FDTSDIV8_N6,
    #[doc = "fSAMPLING = fDTS / 8, N = 8"] FDTSDIV8_N8,
    #[doc = "fSAMPLING = fDTS / 16, N = 5"] FDTSDIV16_N5,
    #[doc = "fSAMPLING = fDTS / 16, N = 6"] FDTSDIV16_N6,
    #[doc = "fSAMPLING = fDTS / 16, N = 8"] FDTSDIV16_N8,
    #[doc = "fSAMPLING = fDTS / 32, N = 5"] FDTSDIV32_N5,
    #[doc = "fSAMPLING = fDTS / 32, N = 6"] FDTSDIV32_N6,
    #[doc = "fSAMPLING = fDTS / 32, N = 8"] FDTSDIV32_N8,
}
impl IC1FR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        *self == IC1FR::NOFILTER
    }
    #[doc = "Checks if the value of the field is `FCK_INT_N2`"]
    #[inline(always)]
    pub fn is_f_ck_int_n2(&self) -> bool {
        *self == IC1FR::FCK_INT_N2
    }
    #[doc = "Checks if the value of the field is `FCK_INT_N4`"]
    #[inline(always)]
    pub fn is_f_ck_int_n4(&self) -> bool {
        *self == IC1FR::FCK_INT_N4
    }
    #[doc = "Checks if the value of the field is `FCK_INT_N8`"]
    #[inline(always)]
    pub fn is_f_ck_int_n8(&self) -> bool {
        *self == IC1FR::FCK_INT_N8
    }
    #[doc = "Checks if the value of the field is `FDTSDIV2_N6`"]
    #[inline(always)]
    pub fn is_f_dtsdiv2_n6(&self) -> bool {
        *self == IC1FR::FDTSDIV2_N6
    }
    #[doc = "Checks if the value of the field is `FDTSDIV2_N8`"]
    #[inline(always)]
    pub fn is_f_dtsdiv2_n8(&self) -> bool {
        *self == IC1FR::FDTSDIV2_N8
    }
    #[doc = "Checks if the value of the field is `FDTSDIV4_N6`"]
    #[inline(always)]
    pub fn is_f_dtsdiv4_n6(&self) -> bool {
        *self == IC1FR::FDTSDIV4_N6
    }
    #[doc = "Checks if the value of the field is `FDTSDIV4_N8`"]
    #[inline(always)]
    pub fn is_f_dtsdiv4_n8(&self) -> bool {
        *self == IC1FR::FDTSDIV4_N8
    }
    #[doc = "Checks if the value of the field is `FDTSDIV8_N6`"]
    #[inline(always)]
    pub fn is_f_dtsdiv8_n6(&self) -> bool {
        *self == IC1FR::FDTSDIV8_N6
    }
    #[doc = "Checks if the value of the field is `FDTSDIV8_N8`"]
    #[inline(always)]
    pub fn is_f_dtsdiv8_n8(&self) -> bool {
        *self == IC1FR::FDTSDIV8_N8
    }
    #[doc = "Checks if the value of the field is `FDTSDIV16_N5`"]
    #[inline(always)]
    pub fn is_f_dtsdiv16_n5(&self) -> bool {
        *self == IC1FR::FDTSDIV16_N5
    }
    #[doc = "Checks if the value of the field is `FDTSDIV16_N6`"]
    #[inline(always)]
    pub fn is_f_dtsdiv16_n6(&self) -> bool {
        *self == IC1FR::FDTSDIV16_N6
    }
    #[doc = "Checks if the value of the field is `FDTSDIV16_N8`"]
    #[inline(always)]
    pub fn is_f_dtsdiv16_n8(&self) -> bool {
        *self == IC1FR::FDTSDIV16_N8
    }
    #[doc = "Checks if the value of the field is `FDTSDIV32_N5`"]
    #[inline(always)]
    pub fn is_f_dtsdiv32_n5(&self) -> bool {
        *self == IC1FR::FDTSDIV32_N5
    }
    #[doc = "Checks if the value of the field is `FDTSDIV32_N6`"]
    #[inline(always)]
    pub fn is_f_dtsdiv32_n6(&self) -> bool {
        *self == IC1FR::FDTSDIV32_N6
    }
    #[doc = "Checks if the value of the field is `FDTSDIV32_N8`"]
    #[inline(always)]
    pub fn is_f_dtsdiv32_n8(&self) -> bool {
        *self == IC1FR::FDTSDIV32_N8
    }
}
#[doc = "Possible values of the field `IC1PCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IC1PCSR {
    #[doc = "no prescaler, capture is done each time an edge is detected on the capture input"] _1EVENT,
    #[doc = "capture is done once every 2 events"] _2EVENTS,
    #[doc = "capture is done once every 4 events"] _4EVENTS,
    #[doc = "capture is done once every 8 events"] _8EVENTS,
}
impl IC1PCSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            IC1PCSR::_1EVENT => 0,
            IC1PCSR::_2EVENTS => 1,
            IC1PCSR::_4EVENTS => 2,
            IC1PCSR::_8EVENTS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> IC1PCSR {
        match value {
            0 => IC1PCSR::_1EVENT,
            1 => IC1PCSR::_2EVENTS,
            2 => IC1PCSR::_4EVENTS,
            3 => IC1PCSR::_8EVENTS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1EVENT`"]
    #[inline(always)]
    pub fn is_1event(&self) -> bool {
        *self == IC1PCSR::_1EVENT
    }
    #[doc = "Checks if the value of the field is `_2EVENTS`"]
    #[inline(always)]
    pub fn is_2events(&self) -> bool {
        *self == IC1PCSR::_2EVENTS
    }
    #[doc = "Checks if the value of the field is `_4EVENTS`"]
    #[inline(always)]
    pub fn is_4events(&self) -> bool {
        *self == IC1PCSR::_4EVENTS
    }
    #[doc = "Checks if the value of the field is `_8EVENTS`"]
    #[inline(always)]
    pub fn is_8events(&self) -> bool {
        *self == IC1PCSR::_8EVENTS
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
    pub fn is_cc1output(&self) -> bool {
        *self == CC1SR::CC1OUTPUT
    }
    #[doc = "Checks if the value of the field is `IC1MAPPEDTI1`"]
    #[inline(always)]
    pub fn is_ic1mapped_ti1(&self) -> bool {
        *self == CC1SR::IC1MAPPEDTI1
    }
    #[doc = "Checks if the value of the field is `IC1MAPPEDTI2`"]
    #[inline(always)]
    pub fn is_ic1mapped_ti2(&self) -> bool {
        *self == CC1SR::IC1MAPPEDTI2
    }
    #[doc = "Checks if the value of the field is `IC1MAPPEDTRC`"]
    #[inline(always)]
    pub fn is_ic1mapped_trc(&self) -> bool {
        *self == CC1SR::IC1MAPPEDTRC
    }
}
#[doc = "Values that can be written to the field `IC2F`"]
pub enum IC2FW {
    #[doc = "No filter, sampling is done at fDTS"] NOFILTER,
    #[doc = "fSAMPLING = fCK_INT, N = 2"] FCK_INT_N2,
    #[doc = "fSAMPLING = fCK_INT, N = 4"] FCK_INT_N4,
    #[doc = "fSAMPLING = fCK_INT, N = 8"] FCK_INT_N8,
    #[doc = "fSAMPLING = fDTS / 2, N = 6"] FDTSDIV2_N6,
    #[doc = "fSAMPLING = fDTS / 2, N = 8"] FDTSDIV2_N8,
    #[doc = "fSAMPLING = fDTS / 4, N = 6"] FDTSDIV4_N6,
    #[doc = "fSAMPLING = fDTS / 4, N = 8"] FDTSDIV4_N8,
    #[doc = "fSAMPLING = fDTS / 8, N = 6"] FDTSDIV8_N6,
    #[doc = "fSAMPLING = fDTS / 8, N = 8"] FDTSDIV8_N8,
    #[doc = "fSAMPLING = fDTS / 16, N = 5"] FDTSDIV16_N5,
    #[doc = "fSAMPLING = fDTS / 16, N = 6"] FDTSDIV16_N6,
    #[doc = "fSAMPLING = fDTS / 16, N = 8"] FDTSDIV16_N8,
    #[doc = "fSAMPLING = fDTS / 32, N = 5"] FDTSDIV32_N5,
    #[doc = "fSAMPLING = fDTS / 32, N = 6"] FDTSDIV32_N6,
    #[doc = "fSAMPLING = fDTS / 32, N = 8"] FDTSDIV32_N8,
}
impl IC2FW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            IC2FW::NOFILTER => 0,
            IC2FW::FCK_INT_N2 => 1,
            IC2FW::FCK_INT_N4 => 2,
            IC2FW::FCK_INT_N8 => 3,
            IC2FW::FDTSDIV2_N6 => 4,
            IC2FW::FDTSDIV2_N8 => 5,
            IC2FW::FDTSDIV4_N6 => 6,
            IC2FW::FDTSDIV4_N8 => 7,
            IC2FW::FDTSDIV8_N6 => 8,
            IC2FW::FDTSDIV8_N8 => 9,
            IC2FW::FDTSDIV16_N5 => 10,
            IC2FW::FDTSDIV16_N6 => 11,
            IC2FW::FDTSDIV16_N8 => 12,
            IC2FW::FDTSDIV32_N5 => 13,
            IC2FW::FDTSDIV32_N6 => 14,
            IC2FW::FDTSDIV32_N8 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IC2FW<'a> {
    w: &'a mut W,
}
impl<'a> _IC2FW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IC2FW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No filter, sampling is done at fDTS"]
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut W {
        self.variant(IC2FW::NOFILTER)
    }
    #[doc = "fSAMPLING = fCK_INT, N = 2"]
    #[inline(always)]
    pub fn f_ck_int_n2(self) -> &'a mut W {
        self.variant(IC2FW::FCK_INT_N2)
    }
    #[doc = "fSAMPLING = fCK_INT, N = 4"]
    #[inline(always)]
    pub fn f_ck_int_n4(self) -> &'a mut W {
        self.variant(IC2FW::FCK_INT_N4)
    }
    #[doc = "fSAMPLING = fCK_INT, N = 8"]
    #[inline(always)]
    pub fn f_ck_int_n8(self) -> &'a mut W {
        self.variant(IC2FW::FCK_INT_N8)
    }
    #[doc = "fSAMPLING = fDTS / 2, N = 6"]
    #[inline(always)]
    pub fn f_dtsdiv2_n6(self) -> &'a mut W {
        self.variant(IC2FW::FDTSDIV2_N6)
    }
    #[doc = "fSAMPLING = fDTS / 2, N = 8"]
    #[inline(always)]
    pub fn f_dtsdiv2_n8(self) -> &'a mut W {
        self.variant(IC2FW::FDTSDIV2_N8)
    }
    #[doc = "fSAMPLING = fDTS / 4, N = 6"]
    #[inline(always)]
    pub fn f_dtsdiv4_n6(self) -> &'a mut W {
        self.variant(IC2FW::FDTSDIV4_N6)
    }
    #[doc = "fSAMPLING = fDTS / 4, N = 8"]
    #[inline(always)]
    pub fn f_dtsdiv4_n8(self) -> &'a mut W {
        self.variant(IC2FW::FDTSDIV4_N8)
    }
    #[doc = "fSAMPLING = fDTS / 8, N = 6"]
    #[inline(always)]
    pub fn f_dtsdiv8_n6(self) -> &'a mut W {
        self.variant(IC2FW::FDTSDIV8_N6)
    }
    #[doc = "fSAMPLING = fDTS / 8, N = 8"]
    #[inline(always)]
    pub fn f_dtsdiv8_n8(self) -> &'a mut W {
        self.variant(IC2FW::FDTSDIV8_N8)
    }
    #[doc = "fSAMPLING = fDTS / 16, N = 5"]
    #[inline(always)]
    pub fn f_dtsdiv16_n5(self) -> &'a mut W {
        self.variant(IC2FW::FDTSDIV16_N5)
    }
    #[doc = "fSAMPLING = fDTS / 16, N = 6"]
    #[inline(always)]
    pub fn f_dtsdiv16_n6(self) -> &'a mut W {
        self.variant(IC2FW::FDTSDIV16_N6)
    }
    #[doc = "fSAMPLING = fDTS / 16, N = 8"]
    #[inline(always)]
    pub fn f_dtsdiv16_n8(self) -> &'a mut W {
        self.variant(IC2FW::FDTSDIV16_N8)
    }
    #[doc = "fSAMPLING = fDTS / 32, N = 5"]
    #[inline(always)]
    pub fn f_dtsdiv32_n5(self) -> &'a mut W {
        self.variant(IC2FW::FDTSDIV32_N5)
    }
    #[doc = "fSAMPLING = fDTS / 32, N = 6"]
    #[inline(always)]
    pub fn f_dtsdiv32_n6(self) -> &'a mut W {
        self.variant(IC2FW::FDTSDIV32_N6)
    }
    #[doc = "fSAMPLING = fDTS / 32, N = 8"]
    #[inline(always)]
    pub fn f_dtsdiv32_n8(self) -> &'a mut W {
        self.variant(IC2FW::FDTSDIV32_N8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IC2PCS`"]
pub enum IC2PCSW {
    #[doc = "no prescaler, capture is done each time an edge is detected on the capture input"] _1EVENT,
    #[doc = "capture is done once every 2 events"] _2EVENTS,
    #[doc = "capture is done once every 4 events"] _4EVENTS,
    #[doc = "capture is done once every 8 events"] _8EVENTS,
}
impl IC2PCSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            IC2PCSW::_1EVENT => 0,
            IC2PCSW::_2EVENTS => 1,
            IC2PCSW::_4EVENTS => 2,
            IC2PCSW::_8EVENTS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IC2PCSW<'a> {
    w: &'a mut W,
}
impl<'a> _IC2PCSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IC2PCSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "no prescaler, capture is done each time an edge is detected on the capture input"]
    #[inline(always)]
    pub fn _1event(self) -> &'a mut W {
        self.variant(IC2PCSW::_1EVENT)
    }
    #[doc = "capture is done once every 2 events"]
    #[inline(always)]
    pub fn _2events(self) -> &'a mut W {
        self.variant(IC2PCSW::_2EVENTS)
    }
    #[doc = "capture is done once every 4 events"]
    #[inline(always)]
    pub fn _4events(self) -> &'a mut W {
        self.variant(IC2PCSW::_4EVENTS)
    }
    #[doc = "capture is done once every 8 events"]
    #[inline(always)]
    pub fn _8events(self) -> &'a mut W {
        self.variant(IC2PCSW::_8EVENTS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[inline(always)]
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
    #[inline(always)]
    pub fn variant(self, variant: CC2SW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CC2 channel is configured as output"]
    #[inline(always)]
    pub fn cc2output(self) -> &'a mut W {
        self.variant(CC2SW::CC2OUTPUT)
    }
    #[doc = "CC2 channel is configured as input, IC2 is mapped on TI2"]
    #[inline(always)]
    pub fn ic2mapped_ti2(self) -> &'a mut W {
        self.variant(CC2SW::IC2MAPPEDTI2)
    }
    #[doc = "CC2 channel is configured as input, IC2 is mapped on TI1"]
    #[inline(always)]
    pub fn ic2mapped_ti1(self) -> &'a mut W {
        self.variant(CC2SW::IC2MAPPEDTI1)
    }
    #[doc = "CC2 channel is configured as input, IC2 is mapped on TRC"]
    #[inline(always)]
    pub fn ic2mapped_trc(self) -> &'a mut W {
        self.variant(CC2SW::IC2MAPPEDTRC)
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
#[doc = "Values that can be written to the field `IC1F`"]
pub enum IC1FW {
    #[doc = "No filter, sampling is done at fDTS"] NOFILTER,
    #[doc = "fSAMPLING = fCK_INT, N = 2"] FCK_INT_N2,
    #[doc = "fSAMPLING = fCK_INT, N = 4"] FCK_INT_N4,
    #[doc = "fSAMPLING = fCK_INT, N = 8"] FCK_INT_N8,
    #[doc = "fSAMPLING = fDTS / 2, N = 6"] FDTSDIV2_N6,
    #[doc = "fSAMPLING = fDTS / 2, N = 8"] FDTSDIV2_N8,
    #[doc = "fSAMPLING = fDTS / 4, N = 6"] FDTSDIV4_N6,
    #[doc = "fSAMPLING = fDTS / 4, N = 8"] FDTSDIV4_N8,
    #[doc = "fSAMPLING = fDTS / 8, N = 6"] FDTSDIV8_N6,
    #[doc = "fSAMPLING = fDTS / 8, N = 8"] FDTSDIV8_N8,
    #[doc = "fSAMPLING = fDTS / 16, N = 5"] FDTSDIV16_N5,
    #[doc = "fSAMPLING = fDTS / 16, N = 6"] FDTSDIV16_N6,
    #[doc = "fSAMPLING = fDTS / 16, N = 8"] FDTSDIV16_N8,
    #[doc = "fSAMPLING = fDTS / 32, N = 5"] FDTSDIV32_N5,
    #[doc = "fSAMPLING = fDTS / 32, N = 6"] FDTSDIV32_N6,
    #[doc = "fSAMPLING = fDTS / 32, N = 8"] FDTSDIV32_N8,
}
impl IC1FW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
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
    #[inline(always)]
    pub fn variant(self, variant: IC1FW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No filter, sampling is done at fDTS"]
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut W {
        self.variant(IC1FW::NOFILTER)
    }
    #[doc = "fSAMPLING = fCK_INT, N = 2"]
    #[inline(always)]
    pub fn f_ck_int_n2(self) -> &'a mut W {
        self.variant(IC1FW::FCK_INT_N2)
    }
    #[doc = "fSAMPLING = fCK_INT, N = 4"]
    #[inline(always)]
    pub fn f_ck_int_n4(self) -> &'a mut W {
        self.variant(IC1FW::FCK_INT_N4)
    }
    #[doc = "fSAMPLING = fCK_INT, N = 8"]
    #[inline(always)]
    pub fn f_ck_int_n8(self) -> &'a mut W {
        self.variant(IC1FW::FCK_INT_N8)
    }
    #[doc = "fSAMPLING = fDTS / 2, N = 6"]
    #[inline(always)]
    pub fn f_dtsdiv2_n6(self) -> &'a mut W {
        self.variant(IC1FW::FDTSDIV2_N6)
    }
    #[doc = "fSAMPLING = fDTS / 2, N = 8"]
    #[inline(always)]
    pub fn f_dtsdiv2_n8(self) -> &'a mut W {
        self.variant(IC1FW::FDTSDIV2_N8)
    }
    #[doc = "fSAMPLING = fDTS / 4, N = 6"]
    #[inline(always)]
    pub fn f_dtsdiv4_n6(self) -> &'a mut W {
        self.variant(IC1FW::FDTSDIV4_N6)
    }
    #[doc = "fSAMPLING = fDTS / 4, N = 8"]
    #[inline(always)]
    pub fn f_dtsdiv4_n8(self) -> &'a mut W {
        self.variant(IC1FW::FDTSDIV4_N8)
    }
    #[doc = "fSAMPLING = fDTS / 8, N = 6"]
    #[inline(always)]
    pub fn f_dtsdiv8_n6(self) -> &'a mut W {
        self.variant(IC1FW::FDTSDIV8_N6)
    }
    #[doc = "fSAMPLING = fDTS / 8, N = 8"]
    #[inline(always)]
    pub fn f_dtsdiv8_n8(self) -> &'a mut W {
        self.variant(IC1FW::FDTSDIV8_N8)
    }
    #[doc = "fSAMPLING = fDTS / 16, N = 5"]
    #[inline(always)]
    pub fn f_dtsdiv16_n5(self) -> &'a mut W {
        self.variant(IC1FW::FDTSDIV16_N5)
    }
    #[doc = "fSAMPLING = fDTS / 16, N = 6"]
    #[inline(always)]
    pub fn f_dtsdiv16_n6(self) -> &'a mut W {
        self.variant(IC1FW::FDTSDIV16_N6)
    }
    #[doc = "fSAMPLING = fDTS / 16, N = 8"]
    #[inline(always)]
    pub fn f_dtsdiv16_n8(self) -> &'a mut W {
        self.variant(IC1FW::FDTSDIV16_N8)
    }
    #[doc = "fSAMPLING = fDTS / 32, N = 5"]
    #[inline(always)]
    pub fn f_dtsdiv32_n5(self) -> &'a mut W {
        self.variant(IC1FW::FDTSDIV32_N5)
    }
    #[doc = "fSAMPLING = fDTS / 32, N = 6"]
    #[inline(always)]
    pub fn f_dtsdiv32_n6(self) -> &'a mut W {
        self.variant(IC1FW::FDTSDIV32_N6)
    }
    #[doc = "fSAMPLING = fDTS / 32, N = 8"]
    #[inline(always)]
    pub fn f_dtsdiv32_n8(self) -> &'a mut W {
        self.variant(IC1FW::FDTSDIV32_N8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IC1PCS`"]
pub enum IC1PCSW {
    #[doc = "no prescaler, capture is done each time an edge is detected on the capture input"] _1EVENT,
    #[doc = "capture is done once every 2 events"] _2EVENTS,
    #[doc = "capture is done once every 4 events"] _4EVENTS,
    #[doc = "capture is done once every 8 events"] _8EVENTS,
}
impl IC1PCSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            IC1PCSW::_1EVENT => 0,
            IC1PCSW::_2EVENTS => 1,
            IC1PCSW::_4EVENTS => 2,
            IC1PCSW::_8EVENTS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IC1PCSW<'a> {
    w: &'a mut W,
}
impl<'a> _IC1PCSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IC1PCSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "no prescaler, capture is done each time an edge is detected on the capture input"]
    #[inline(always)]
    pub fn _1event(self) -> &'a mut W {
        self.variant(IC1PCSW::_1EVENT)
    }
    #[doc = "capture is done once every 2 events"]
    #[inline(always)]
    pub fn _2events(self) -> &'a mut W {
        self.variant(IC1PCSW::_2EVENTS)
    }
    #[doc = "capture is done once every 4 events"]
    #[inline(always)]
    pub fn _4events(self) -> &'a mut W {
        self.variant(IC1PCSW::_4EVENTS)
    }
    #[doc = "capture is done once every 8 events"]
    #[inline(always)]
    pub fn _8events(self) -> &'a mut W {
        self.variant(IC1PCSW::_8EVENTS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
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
    #[doc = "CC1 channel is configured as output"] CC1OUTPUT,
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI1"] IC1MAPPEDTI1,
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI2"] IC1MAPPEDTI2,
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TRC"] IC1MAPPEDTRC,
}
impl CC1SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
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
    #[inline(always)]
    pub fn variant(self, variant: CC1SW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CC1 channel is configured as output"]
    #[inline(always)]
    pub fn cc1output(self) -> &'a mut W {
        self.variant(CC1SW::CC1OUTPUT)
    }
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI1"]
    #[inline(always)]
    pub fn ic1mapped_ti1(self) -> &'a mut W {
        self.variant(CC1SW::IC1MAPPEDTI1)
    }
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI2"]
    #[inline(always)]
    pub fn ic1mapped_ti2(self) -> &'a mut W {
        self.variant(CC1SW::IC1MAPPEDTI2)
    }
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TRC"]
    #[inline(always)]
    pub fn ic1mapped_trc(self) -> &'a mut W {
        self.variant(CC1SW::IC1MAPPEDTRC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
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
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 12:15 - Input capture 2 filter"]
    #[inline(always)]
    pub fn ic2f(&self) -> IC2FR {
        IC2FR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Input capture 2 prescaler"]
    #[inline(always)]
    pub fn ic2pcs(&self) -> IC2PCSR {
        IC2PCSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Capture/Compare 2 selection"]
    #[inline(always)]
    pub fn cc2s(&self) -> CC2SR {
        CC2SR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Input capture 1 filter"]
    #[inline(always)]
    pub fn ic1f(&self) -> IC1FR {
        IC1FR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Input capture 1 prescaler"]
    #[inline(always)]
    pub fn ic1pcs(&self) -> IC1PCSR {
        IC1PCSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline(always)]
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
    #[doc = "Bits 12:15 - Input capture 2 filter"]
    #[inline(always)]
    pub fn ic2f(&mut self) -> _IC2FW {
        _IC2FW { w: self }
    }
    #[doc = "Bits 10:11 - Input capture 2 prescaler"]
    #[inline(always)]
    pub fn ic2pcs(&mut self) -> _IC2PCSW {
        _IC2PCSW { w: self }
    }
    #[doc = "Bits 8:9 - Capture/Compare 2 selection"]
    #[inline(always)]
    pub fn cc2s(&mut self) -> _CC2SW {
        _CC2SW { w: self }
    }
    #[doc = "Bits 4:7 - Input capture 1 filter"]
    #[inline(always)]
    pub fn ic1f(&mut self) -> _IC1FW {
        _IC1FW { w: self }
    }
    #[doc = "Bits 2:3 - Input capture 1 prescaler"]
    #[inline(always)]
    pub fn ic1pcs(&mut self) -> _IC1PCSW {
        _IC1PCSW { w: self }
    }
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline(always)]
    pub fn cc1s(&mut self) -> _CC1SW {
        _CC1SW { w: self }
    }
}
