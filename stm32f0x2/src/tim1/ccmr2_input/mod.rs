#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCMR2_INPUT {
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
#[doc = "Possible values of the field `IC4F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IC4FR {
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
impl IC4FR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IC4FR::NOFILTER => 0,
            IC4FR::FCK_INT_N2 => 1,
            IC4FR::FCK_INT_N4 => 2,
            IC4FR::FCK_INT_N8 => 3,
            IC4FR::FDTSDIV2_N6 => 4,
            IC4FR::FDTSDIV2_N8 => 5,
            IC4FR::FDTSDIV4_N6 => 6,
            IC4FR::FDTSDIV4_N8 => 7,
            IC4FR::FDTSDIV8_N6 => 8,
            IC4FR::FDTSDIV8_N8 => 9,
            IC4FR::FDTSDIV16_N5 => 10,
            IC4FR::FDTSDIV16_N6 => 11,
            IC4FR::FDTSDIV16_N8 => 12,
            IC4FR::FDTSDIV32_N5 => 13,
            IC4FR::FDTSDIV32_N6 => 14,
            IC4FR::FDTSDIV32_N8 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IC4FR {
        match value {
            0 => IC4FR::NOFILTER,
            1 => IC4FR::FCK_INT_N2,
            2 => IC4FR::FCK_INT_N4,
            3 => IC4FR::FCK_INT_N8,
            4 => IC4FR::FDTSDIV2_N6,
            5 => IC4FR::FDTSDIV2_N8,
            6 => IC4FR::FDTSDIV4_N6,
            7 => IC4FR::FDTSDIV4_N8,
            8 => IC4FR::FDTSDIV8_N6,
            9 => IC4FR::FDTSDIV8_N8,
            10 => IC4FR::FDTSDIV16_N5,
            11 => IC4FR::FDTSDIV16_N6,
            12 => IC4FR::FDTSDIV16_N8,
            13 => IC4FR::FDTSDIV32_N5,
            14 => IC4FR::FDTSDIV32_N6,
            15 => IC4FR::FDTSDIV32_N8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOFILTER`"]
    #[inline]
    pub fn is_no_filter(&self) -> bool {
        *self == IC4FR::NOFILTER
    }
    #[doc = "Checks if the value of the field is `FCK_INT_N2`"]
    #[inline]
    pub fn is_f_ck_int_n2(&self) -> bool {
        *self == IC4FR::FCK_INT_N2
    }
    #[doc = "Checks if the value of the field is `FCK_INT_N4`"]
    #[inline]
    pub fn is_f_ck_int_n4(&self) -> bool {
        *self == IC4FR::FCK_INT_N4
    }
    #[doc = "Checks if the value of the field is `FCK_INT_N8`"]
    #[inline]
    pub fn is_f_ck_int_n8(&self) -> bool {
        *self == IC4FR::FCK_INT_N8
    }
    #[doc = "Checks if the value of the field is `FDTSDIV2_N6`"]
    #[inline]
    pub fn is_f_dtsdiv2_n6(&self) -> bool {
        *self == IC4FR::FDTSDIV2_N6
    }
    #[doc = "Checks if the value of the field is `FDTSDIV2_N8`"]
    #[inline]
    pub fn is_f_dtsdiv2_n8(&self) -> bool {
        *self == IC4FR::FDTSDIV2_N8
    }
    #[doc = "Checks if the value of the field is `FDTSDIV4_N6`"]
    #[inline]
    pub fn is_f_dtsdiv4_n6(&self) -> bool {
        *self == IC4FR::FDTSDIV4_N6
    }
    #[doc = "Checks if the value of the field is `FDTSDIV4_N8`"]
    #[inline]
    pub fn is_f_dtsdiv4_n8(&self) -> bool {
        *self == IC4FR::FDTSDIV4_N8
    }
    #[doc = "Checks if the value of the field is `FDTSDIV8_N6`"]
    #[inline]
    pub fn is_f_dtsdiv8_n6(&self) -> bool {
        *self == IC4FR::FDTSDIV8_N6
    }
    #[doc = "Checks if the value of the field is `FDTSDIV8_N8`"]
    #[inline]
    pub fn is_f_dtsdiv8_n8(&self) -> bool {
        *self == IC4FR::FDTSDIV8_N8
    }
    #[doc = "Checks if the value of the field is `FDTSDIV16_N5`"]
    #[inline]
    pub fn is_f_dtsdiv16_n5(&self) -> bool {
        *self == IC4FR::FDTSDIV16_N5
    }
    #[doc = "Checks if the value of the field is `FDTSDIV16_N6`"]
    #[inline]
    pub fn is_f_dtsdiv16_n6(&self) -> bool {
        *self == IC4FR::FDTSDIV16_N6
    }
    #[doc = "Checks if the value of the field is `FDTSDIV16_N8`"]
    #[inline]
    pub fn is_f_dtsdiv16_n8(&self) -> bool {
        *self == IC4FR::FDTSDIV16_N8
    }
    #[doc = "Checks if the value of the field is `FDTSDIV32_N5`"]
    #[inline]
    pub fn is_f_dtsdiv32_n5(&self) -> bool {
        *self == IC4FR::FDTSDIV32_N5
    }
    #[doc = "Checks if the value of the field is `FDTSDIV32_N6`"]
    #[inline]
    pub fn is_f_dtsdiv32_n6(&self) -> bool {
        *self == IC4FR::FDTSDIV32_N6
    }
    #[doc = "Checks if the value of the field is `FDTSDIV32_N8`"]
    #[inline]
    pub fn is_f_dtsdiv32_n8(&self) -> bool {
        *self == IC4FR::FDTSDIV32_N8
    }
}
#[doc = "Possible values of the field `IC4PSC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IC4PSCR {
    #[doc = "no prescaler, capture is done each time an edge is detected on the capture input"]
    NO,
    #[doc = "capture is done once every 2 events"]
    _2EVENTS,
    #[doc = "capture is done once every 4 events"]
    _4EVENTS,
    #[doc = "capture is done once every 8 events"]
    _8EVENTS,
}
impl IC4PSCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IC4PSCR::NO => 0,
            IC4PSCR::_2EVENTS => 1,
            IC4PSCR::_4EVENTS => 2,
            IC4PSCR::_8EVENTS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IC4PSCR {
        match value {
            0 => IC4PSCR::NO,
            1 => IC4PSCR::_2EVENTS,
            2 => IC4PSCR::_4EVENTS,
            3 => IC4PSCR::_8EVENTS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == IC4PSCR::NO
    }
    #[doc = "Checks if the value of the field is `_2EVENTS`"]
    #[inline]
    pub fn is_2events(&self) -> bool {
        *self == IC4PSCR::_2EVENTS
    }
    #[doc = "Checks if the value of the field is `_4EVENTS`"]
    #[inline]
    pub fn is_4events(&self) -> bool {
        *self == IC4PSCR::_4EVENTS
    }
    #[doc = "Checks if the value of the field is `_8EVENTS`"]
    #[inline]
    pub fn is_8events(&self) -> bool {
        *self == IC4PSCR::_8EVENTS
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
#[doc = "Possible values of the field `IC3F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IC3FR {
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
impl IC3FR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IC3FR::NOFILTER => 0,
            IC3FR::FCK_INT_N2 => 1,
            IC3FR::FCK_INT_N4 => 2,
            IC3FR::FCK_INT_N8 => 3,
            IC3FR::FDTSDIV2_N6 => 4,
            IC3FR::FDTSDIV2_N8 => 5,
            IC3FR::FDTSDIV4_N6 => 6,
            IC3FR::FDTSDIV4_N8 => 7,
            IC3FR::FDTSDIV8_N6 => 8,
            IC3FR::FDTSDIV8_N8 => 9,
            IC3FR::FDTSDIV16_N5 => 10,
            IC3FR::FDTSDIV16_N6 => 11,
            IC3FR::FDTSDIV16_N8 => 12,
            IC3FR::FDTSDIV32_N5 => 13,
            IC3FR::FDTSDIV32_N6 => 14,
            IC3FR::FDTSDIV32_N8 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IC3FR {
        match value {
            0 => IC3FR::NOFILTER,
            1 => IC3FR::FCK_INT_N2,
            2 => IC3FR::FCK_INT_N4,
            3 => IC3FR::FCK_INT_N8,
            4 => IC3FR::FDTSDIV2_N6,
            5 => IC3FR::FDTSDIV2_N8,
            6 => IC3FR::FDTSDIV4_N6,
            7 => IC3FR::FDTSDIV4_N8,
            8 => IC3FR::FDTSDIV8_N6,
            9 => IC3FR::FDTSDIV8_N8,
            10 => IC3FR::FDTSDIV16_N5,
            11 => IC3FR::FDTSDIV16_N6,
            12 => IC3FR::FDTSDIV16_N8,
            13 => IC3FR::FDTSDIV32_N5,
            14 => IC3FR::FDTSDIV32_N6,
            15 => IC3FR::FDTSDIV32_N8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOFILTER`"]
    #[inline]
    pub fn is_no_filter(&self) -> bool {
        *self == IC3FR::NOFILTER
    }
    #[doc = "Checks if the value of the field is `FCK_INT_N2`"]
    #[inline]
    pub fn is_f_ck_int_n2(&self) -> bool {
        *self == IC3FR::FCK_INT_N2
    }
    #[doc = "Checks if the value of the field is `FCK_INT_N4`"]
    #[inline]
    pub fn is_f_ck_int_n4(&self) -> bool {
        *self == IC3FR::FCK_INT_N4
    }
    #[doc = "Checks if the value of the field is `FCK_INT_N8`"]
    #[inline]
    pub fn is_f_ck_int_n8(&self) -> bool {
        *self == IC3FR::FCK_INT_N8
    }
    #[doc = "Checks if the value of the field is `FDTSDIV2_N6`"]
    #[inline]
    pub fn is_f_dtsdiv2_n6(&self) -> bool {
        *self == IC3FR::FDTSDIV2_N6
    }
    #[doc = "Checks if the value of the field is `FDTSDIV2_N8`"]
    #[inline]
    pub fn is_f_dtsdiv2_n8(&self) -> bool {
        *self == IC3FR::FDTSDIV2_N8
    }
    #[doc = "Checks if the value of the field is `FDTSDIV4_N6`"]
    #[inline]
    pub fn is_f_dtsdiv4_n6(&self) -> bool {
        *self == IC3FR::FDTSDIV4_N6
    }
    #[doc = "Checks if the value of the field is `FDTSDIV4_N8`"]
    #[inline]
    pub fn is_f_dtsdiv4_n8(&self) -> bool {
        *self == IC3FR::FDTSDIV4_N8
    }
    #[doc = "Checks if the value of the field is `FDTSDIV8_N6`"]
    #[inline]
    pub fn is_f_dtsdiv8_n6(&self) -> bool {
        *self == IC3FR::FDTSDIV8_N6
    }
    #[doc = "Checks if the value of the field is `FDTSDIV8_N8`"]
    #[inline]
    pub fn is_f_dtsdiv8_n8(&self) -> bool {
        *self == IC3FR::FDTSDIV8_N8
    }
    #[doc = "Checks if the value of the field is `FDTSDIV16_N5`"]
    #[inline]
    pub fn is_f_dtsdiv16_n5(&self) -> bool {
        *self == IC3FR::FDTSDIV16_N5
    }
    #[doc = "Checks if the value of the field is `FDTSDIV16_N6`"]
    #[inline]
    pub fn is_f_dtsdiv16_n6(&self) -> bool {
        *self == IC3FR::FDTSDIV16_N6
    }
    #[doc = "Checks if the value of the field is `FDTSDIV16_N8`"]
    #[inline]
    pub fn is_f_dtsdiv16_n8(&self) -> bool {
        *self == IC3FR::FDTSDIV16_N8
    }
    #[doc = "Checks if the value of the field is `FDTSDIV32_N5`"]
    #[inline]
    pub fn is_f_dtsdiv32_n5(&self) -> bool {
        *self == IC3FR::FDTSDIV32_N5
    }
    #[doc = "Checks if the value of the field is `FDTSDIV32_N6`"]
    #[inline]
    pub fn is_f_dtsdiv32_n6(&self) -> bool {
        *self == IC3FR::FDTSDIV32_N6
    }
    #[doc = "Checks if the value of the field is `FDTSDIV32_N8`"]
    #[inline]
    pub fn is_f_dtsdiv32_n8(&self) -> bool {
        *self == IC3FR::FDTSDIV32_N8
    }
}
#[doc = "Possible values of the field `IC3PSC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IC3PSCR {
    #[doc = "no prescaler, capture is done each time an edge is detected on the capture input"]
    NO,
    #[doc = "capture is done once every 2 events"]
    _2EVENTS,
    #[doc = "capture is done once every 4 events"]
    _4EVENTS,
    #[doc = "capture is done once every 8 events"]
    _8EVENTS,
}
impl IC3PSCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IC3PSCR::NO => 0,
            IC3PSCR::_2EVENTS => 1,
            IC3PSCR::_4EVENTS => 2,
            IC3PSCR::_8EVENTS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IC3PSCR {
        match value {
            0 => IC3PSCR::NO,
            1 => IC3PSCR::_2EVENTS,
            2 => IC3PSCR::_4EVENTS,
            3 => IC3PSCR::_8EVENTS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == IC3PSCR::NO
    }
    #[doc = "Checks if the value of the field is `_2EVENTS`"]
    #[inline]
    pub fn is_2events(&self) -> bool {
        *self == IC3PSCR::_2EVENTS
    }
    #[doc = "Checks if the value of the field is `_4EVENTS`"]
    #[inline]
    pub fn is_4events(&self) -> bool {
        *self == IC3PSCR::_4EVENTS
    }
    #[doc = "Checks if the value of the field is `_8EVENTS`"]
    #[inline]
    pub fn is_8events(&self) -> bool {
        *self == IC3PSCR::_8EVENTS
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
#[doc = "Values that can be written to the field `IC4F`"]
pub enum IC4FW {
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
impl IC4FW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IC4FW::NOFILTER => 0,
            IC4FW::FCK_INT_N2 => 1,
            IC4FW::FCK_INT_N4 => 2,
            IC4FW::FCK_INT_N8 => 3,
            IC4FW::FDTSDIV2_N6 => 4,
            IC4FW::FDTSDIV2_N8 => 5,
            IC4FW::FDTSDIV4_N6 => 6,
            IC4FW::FDTSDIV4_N8 => 7,
            IC4FW::FDTSDIV8_N6 => 8,
            IC4FW::FDTSDIV8_N8 => 9,
            IC4FW::FDTSDIV16_N5 => 10,
            IC4FW::FDTSDIV16_N6 => 11,
            IC4FW::FDTSDIV16_N8 => 12,
            IC4FW::FDTSDIV32_N5 => 13,
            IC4FW::FDTSDIV32_N6 => 14,
            IC4FW::FDTSDIV32_N8 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IC4FW<'a> {
    w: &'a mut W,
}
impl<'a> _IC4FW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IC4FW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No filter, sampling is done at fDTS"]
    #[inline]
    pub fn no_filter(self) -> &'a mut W {
        self.variant(IC4FW::NOFILTER)
    }
    #[doc = "fSAMPLING = fCK_INT, N = 2"]
    #[inline]
    pub fn f_ck_int_n2(self) -> &'a mut W {
        self.variant(IC4FW::FCK_INT_N2)
    }
    #[doc = "fSAMPLING = fCK_INT, N = 4"]
    #[inline]
    pub fn f_ck_int_n4(self) -> &'a mut W {
        self.variant(IC4FW::FCK_INT_N4)
    }
    #[doc = "fSAMPLING = fCK_INT, N = 8"]
    #[inline]
    pub fn f_ck_int_n8(self) -> &'a mut W {
        self.variant(IC4FW::FCK_INT_N8)
    }
    #[doc = "fSAMPLING = fDTS / 2, N = 6"]
    #[inline]
    pub fn f_dtsdiv2_n6(self) -> &'a mut W {
        self.variant(IC4FW::FDTSDIV2_N6)
    }
    #[doc = "fSAMPLING = fDTS / 2, N = 8"]
    #[inline]
    pub fn f_dtsdiv2_n8(self) -> &'a mut W {
        self.variant(IC4FW::FDTSDIV2_N8)
    }
    #[doc = "fSAMPLING = fDTS / 4, N = 6"]
    #[inline]
    pub fn f_dtsdiv4_n6(self) -> &'a mut W {
        self.variant(IC4FW::FDTSDIV4_N6)
    }
    #[doc = "fSAMPLING = fDTS / 4, N = 8"]
    #[inline]
    pub fn f_dtsdiv4_n8(self) -> &'a mut W {
        self.variant(IC4FW::FDTSDIV4_N8)
    }
    #[doc = "fSAMPLING = fDTS / 8, N = 6"]
    #[inline]
    pub fn f_dtsdiv8_n6(self) -> &'a mut W {
        self.variant(IC4FW::FDTSDIV8_N6)
    }
    #[doc = "fSAMPLING = fDTS / 8, N = 8"]
    #[inline]
    pub fn f_dtsdiv8_n8(self) -> &'a mut W {
        self.variant(IC4FW::FDTSDIV8_N8)
    }
    #[doc = "fSAMPLING = fDTS / 16, N = 5"]
    #[inline]
    pub fn f_dtsdiv16_n5(self) -> &'a mut W {
        self.variant(IC4FW::FDTSDIV16_N5)
    }
    #[doc = "fSAMPLING = fDTS / 16, N = 6"]
    #[inline]
    pub fn f_dtsdiv16_n6(self) -> &'a mut W {
        self.variant(IC4FW::FDTSDIV16_N6)
    }
    #[doc = "fSAMPLING = fDTS / 16, N = 8"]
    #[inline]
    pub fn f_dtsdiv16_n8(self) -> &'a mut W {
        self.variant(IC4FW::FDTSDIV16_N8)
    }
    #[doc = "fSAMPLING = fDTS / 32, N = 5"]
    #[inline]
    pub fn f_dtsdiv32_n5(self) -> &'a mut W {
        self.variant(IC4FW::FDTSDIV32_N5)
    }
    #[doc = "fSAMPLING = fDTS / 32, N = 6"]
    #[inline]
    pub fn f_dtsdiv32_n6(self) -> &'a mut W {
        self.variant(IC4FW::FDTSDIV32_N6)
    }
    #[doc = "fSAMPLING = fDTS / 32, N = 8"]
    #[inline]
    pub fn f_dtsdiv32_n8(self) -> &'a mut W {
        self.variant(IC4FW::FDTSDIV32_N8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IC4PSC`"]
pub enum IC4PSCW {
    #[doc = "no prescaler, capture is done each time an edge is detected on the capture input"]
    NO,
    #[doc = "capture is done once every 2 events"]
    _2EVENTS,
    #[doc = "capture is done once every 4 events"]
    _4EVENTS,
    #[doc = "capture is done once every 8 events"]
    _8EVENTS,
}
impl IC4PSCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IC4PSCW::NO => 0,
            IC4PSCW::_2EVENTS => 1,
            IC4PSCW::_4EVENTS => 2,
            IC4PSCW::_8EVENTS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IC4PSCW<'a> {
    w: &'a mut W,
}
impl<'a> _IC4PSCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IC4PSCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "no prescaler, capture is done each time an edge is detected on the capture input"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(IC4PSCW::NO)
    }
    #[doc = "capture is done once every 2 events"]
    #[inline]
    pub fn _2events(self) -> &'a mut W {
        self.variant(IC4PSCW::_2EVENTS)
    }
    #[doc = "capture is done once every 4 events"]
    #[inline]
    pub fn _4events(self) -> &'a mut W {
        self.variant(IC4PSCW::_4EVENTS)
    }
    #[doc = "capture is done once every 8 events"]
    #[inline]
    pub fn _8events(self) -> &'a mut W {
        self.variant(IC4PSCW::_8EVENTS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
#[doc = "Values that can be written to the field `IC3F`"]
pub enum IC3FW {
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
impl IC3FW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IC3FW::NOFILTER => 0,
            IC3FW::FCK_INT_N2 => 1,
            IC3FW::FCK_INT_N4 => 2,
            IC3FW::FCK_INT_N8 => 3,
            IC3FW::FDTSDIV2_N6 => 4,
            IC3FW::FDTSDIV2_N8 => 5,
            IC3FW::FDTSDIV4_N6 => 6,
            IC3FW::FDTSDIV4_N8 => 7,
            IC3FW::FDTSDIV8_N6 => 8,
            IC3FW::FDTSDIV8_N8 => 9,
            IC3FW::FDTSDIV16_N5 => 10,
            IC3FW::FDTSDIV16_N6 => 11,
            IC3FW::FDTSDIV16_N8 => 12,
            IC3FW::FDTSDIV32_N5 => 13,
            IC3FW::FDTSDIV32_N6 => 14,
            IC3FW::FDTSDIV32_N8 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IC3FW<'a> {
    w: &'a mut W,
}
impl<'a> _IC3FW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IC3FW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No filter, sampling is done at fDTS"]
    #[inline]
    pub fn no_filter(self) -> &'a mut W {
        self.variant(IC3FW::NOFILTER)
    }
    #[doc = "fSAMPLING = fCK_INT, N = 2"]
    #[inline]
    pub fn f_ck_int_n2(self) -> &'a mut W {
        self.variant(IC3FW::FCK_INT_N2)
    }
    #[doc = "fSAMPLING = fCK_INT, N = 4"]
    #[inline]
    pub fn f_ck_int_n4(self) -> &'a mut W {
        self.variant(IC3FW::FCK_INT_N4)
    }
    #[doc = "fSAMPLING = fCK_INT, N = 8"]
    #[inline]
    pub fn f_ck_int_n8(self) -> &'a mut W {
        self.variant(IC3FW::FCK_INT_N8)
    }
    #[doc = "fSAMPLING = fDTS / 2, N = 6"]
    #[inline]
    pub fn f_dtsdiv2_n6(self) -> &'a mut W {
        self.variant(IC3FW::FDTSDIV2_N6)
    }
    #[doc = "fSAMPLING = fDTS / 2, N = 8"]
    #[inline]
    pub fn f_dtsdiv2_n8(self) -> &'a mut W {
        self.variant(IC3FW::FDTSDIV2_N8)
    }
    #[doc = "fSAMPLING = fDTS / 4, N = 6"]
    #[inline]
    pub fn f_dtsdiv4_n6(self) -> &'a mut W {
        self.variant(IC3FW::FDTSDIV4_N6)
    }
    #[doc = "fSAMPLING = fDTS / 4, N = 8"]
    #[inline]
    pub fn f_dtsdiv4_n8(self) -> &'a mut W {
        self.variant(IC3FW::FDTSDIV4_N8)
    }
    #[doc = "fSAMPLING = fDTS / 8, N = 6"]
    #[inline]
    pub fn f_dtsdiv8_n6(self) -> &'a mut W {
        self.variant(IC3FW::FDTSDIV8_N6)
    }
    #[doc = "fSAMPLING = fDTS / 8, N = 8"]
    #[inline]
    pub fn f_dtsdiv8_n8(self) -> &'a mut W {
        self.variant(IC3FW::FDTSDIV8_N8)
    }
    #[doc = "fSAMPLING = fDTS / 16, N = 5"]
    #[inline]
    pub fn f_dtsdiv16_n5(self) -> &'a mut W {
        self.variant(IC3FW::FDTSDIV16_N5)
    }
    #[doc = "fSAMPLING = fDTS / 16, N = 6"]
    #[inline]
    pub fn f_dtsdiv16_n6(self) -> &'a mut W {
        self.variant(IC3FW::FDTSDIV16_N6)
    }
    #[doc = "fSAMPLING = fDTS / 16, N = 8"]
    #[inline]
    pub fn f_dtsdiv16_n8(self) -> &'a mut W {
        self.variant(IC3FW::FDTSDIV16_N8)
    }
    #[doc = "fSAMPLING = fDTS / 32, N = 5"]
    #[inline]
    pub fn f_dtsdiv32_n5(self) -> &'a mut W {
        self.variant(IC3FW::FDTSDIV32_N5)
    }
    #[doc = "fSAMPLING = fDTS / 32, N = 6"]
    #[inline]
    pub fn f_dtsdiv32_n6(self) -> &'a mut W {
        self.variant(IC3FW::FDTSDIV32_N6)
    }
    #[doc = "fSAMPLING = fDTS / 32, N = 8"]
    #[inline]
    pub fn f_dtsdiv32_n8(self) -> &'a mut W {
        self.variant(IC3FW::FDTSDIV32_N8)
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
#[doc = "Values that can be written to the field `IC3PSC`"]
pub enum IC3PSCW {
    #[doc = "no prescaler, capture is done each time an edge is detected on the capture input"]
    NO,
    #[doc = "capture is done once every 2 events"]
    _2EVENTS,
    #[doc = "capture is done once every 4 events"]
    _4EVENTS,
    #[doc = "capture is done once every 8 events"]
    _8EVENTS,
}
impl IC3PSCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IC3PSCW::NO => 0,
            IC3PSCW::_2EVENTS => 1,
            IC3PSCW::_4EVENTS => 2,
            IC3PSCW::_8EVENTS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IC3PSCW<'a> {
    w: &'a mut W,
}
impl<'a> _IC3PSCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IC3PSCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "no prescaler, capture is done each time an edge is detected on the capture input"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(IC3PSCW::NO)
    }
    #[doc = "capture is done once every 2 events"]
    #[inline]
    pub fn _2events(self) -> &'a mut W {
        self.variant(IC3PSCW::_2EVENTS)
    }
    #[doc = "capture is done once every 4 events"]
    #[inline]
    pub fn _4events(self) -> &'a mut W {
        self.variant(IC3PSCW::_4EVENTS)
    }
    #[doc = "capture is done once every 8 events"]
    #[inline]
    pub fn _8events(self) -> &'a mut W {
        self.variant(IC3PSCW::_8EVENTS)
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
    #[doc = "Bits 12:15 - Input capture 4 filter"]
    #[inline]
    pub fn ic4f(&self) -> IC4FR {
        IC4FR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Input capture 4 prescaler"]
    #[inline]
    pub fn ic4psc(&self) -> IC4PSCR {
        IC4PSCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 4:7 - Input capture 3 filter"]
    #[inline]
    pub fn ic3f(&self) -> IC3FR {
        IC3FR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Input capture 3 prescaler"]
    #[inline]
    pub fn ic3psc(&self) -> IC3PSCR {
        IC3PSCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:1 - Capture/compare 3 selection"]
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
    #[doc = "Bits 12:15 - Input capture 4 filter"]
    #[inline]
    pub fn ic4f(&mut self) -> _IC4FW {
        _IC4FW { w: self }
    }
    #[doc = "Bits 10:11 - Input capture 4 prescaler"]
    #[inline]
    pub fn ic4psc(&mut self) -> _IC4PSCW {
        _IC4PSCW { w: self }
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection"]
    #[inline]
    pub fn cc4s(&mut self) -> _CC4SW {
        _CC4SW { w: self }
    }
    #[doc = "Bits 4:7 - Input capture 3 filter"]
    #[inline]
    pub fn ic3f(&mut self) -> _IC3FW {
        _IC3FW { w: self }
    }
    #[doc = "Bits 2:3 - Input capture 3 prescaler"]
    #[inline]
    pub fn ic3psc(&mut self) -> _IC3PSCW {
        _IC3PSCW { w: self }
    }
    #[doc = "Bits 0:1 - Capture/compare 3 selection"]
    #[inline]
    pub fn cc3s(&mut self) -> _CC3SW {
        _CC3SW { w: self }
    }
}
