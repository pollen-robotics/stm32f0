//! General Purpose Input / Output

use core::marker::PhantomData;

use rcc::AHB;

/// Extension trait to split a GPIO peripheral in independent pins and registers
pub trait GpioExt {
    /// The to split the GPIO into
    type Parts;

    /// Splits the GPIO block into independent pins and registers
    fn split(self, ahb: &mut AHB) -> Self::Parts;
}

/// Input mode (type state)
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Floating input (type state)
pub struct Floating;
/// Pulled down input (type state)
pub struct PullDown;
/// Pulled up input (type state)
pub struct PullUp;

/// Output mode (type state)
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

/// Push pull output (type state)
pub struct PushPull;
/// Open drain output (type state)
pub struct OpenDrain;

/// Alternate function
pub struct Alternate<MODE> {
    _mode: PhantomData<MODE>,
}

pub trait InputPin {
    fn is_high(&self) -> bool;
    fn is_low(&self) -> bool;
}

macro_rules! gpio {
    ($GPIOX:ident, $gpiox:ident, $gpioy:ident, $iopxenr:ident, $iopxrst:ident, $PXx:ident, [
        $($PXi:ident: (
            $pxi:ident,
            $idr:ident, $odr:ident, $bs:ident, $br:ident,
            $moderx:ident, $otyperx:ident, $ospeedrx:ident, $pupdrx:ident,
            $MODE:ty,
            $MODER:ident, $OTYPER:ident, $PUPDR:ident),)+
    ]) => {
        /// GPIO
        pub mod $gpiox {
            use core::marker::PhantomData;

            use hal::digital::OutputPin;
            use stm32f0x2::{$gpioy, $GPIOX};

            use rcc::AHB;
            use super::{
                // Alternate,
                Floating, GpioExt, Input,
                // OpenDrain,
                Output,
                PullDown, PullUp,
                PushPull,
                InputPin,
            };

            /// GPIO parts
            pub struct Parts {
                pub moder: MODER,
                pub otyper: OTYPER,
                pub pupdr: PUPDR,

                $(
                    /// Pin
                    pub $pxi: $PXi<$MODE>,
                )+
            }

            impl GpioExt for $GPIOX {
                type Parts = Parts;

                fn split(self, ahb: &mut AHB) -> Parts {
                    ahb.enr().modify(|_, w| w.$iopxenr().enabled());
                    ahb.rstr().modify(|_, w| w.$iopxrst().set_bit());
                    ahb.rstr().modify(|_, w| w.$iopxrst().clear_bit());

                    Parts {
                        moder: MODER { _0: () },
                        otyper: OTYPER { _0: () },
                        pupdr: PUPDR { _0: () },
                        $(
                            $pxi: $PXi { _mode: PhantomData },
                        )+
                    }
                }
            }

            /// Partially erased pin
            pub struct $PXx<MODE> {
                _mode: PhantomData<MODE>,
            }

            /// Output mode register
           pub struct MODER {
               _0: (),
           }
           impl MODER {
               pub(crate) fn moder(&mut self) -> &$gpioy::MODER {
                   unsafe { &(*$GPIOX::ptr()).moder }
               }
           }

           /// Output type register
          pub struct OTYPER {
              _0: (),
          }
          impl OTYPER {
              pub(crate) fn otyper(&mut self) -> &$gpioy::OTYPER {
                  unsafe { &(*$GPIOX::ptr()).otyper }
              }
          }

          /// Output type register
         pub struct PUPDR {
             _0: (),
         }
         impl PUPDR {
             pub(crate) fn pupdr(&mut self) -> &$gpioy::PUPDR {
                 unsafe { &(*$GPIOX::ptr()).pupdr }
             }
         }

            $(
                /// Pin
                pub struct $PXi<MODE> {
                    _mode: PhantomData<MODE>,
                }

                impl<MODE> $PXi<MODE> {
                    // /// Configures the pin to operate as an alternate function push pull output pin
                    // pub fn into_alternate_push_pull(
                    //     self,
                    //     moder: &mut $MODER,
                    //     pupdr: &mut $PUPDR,
                    // ) -> $PXi<Alternate<PushPull>> {
                    //
                    //     moder
                    //         .moder()
                    //         .modify(|_, w| {
                    //             w.$moderx().alternate()
                    //         });
                    //
                    //     pupdr.pupdr().modify(|_, w| {
                    //         w.$pupdrx().$afr().
                    //     });
                    //
                    //     $PXi { _mode: PhantomData }
                    // }

                    /// Configures the pin to operate as a floating input pin
                    pub fn into_floating_input(
                        self,
                        moder: &mut $MODER,
                        pupdr: &mut $PUPDR,
                    ) -> $PXi<Input<Floating>> {
                        moder
                            .moder()
                            .modify(|_, w| {
                                w.$moderx().input()
                            });

                        pupdr.pupdr().modify(|_, w| { w.$pupdrx().no_pull() });

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as a pulled down input pin
                    pub fn into_pull_down_input(
                        self,
                        moder: &mut MODER,
                        pupdr: &mut PUPDR,
                    ) -> $PXi<Input<PullDown>> {
                        moder
                            .moder()
                            .modify(|_, w| {
                                w.$moderx().input()
                            });

                        pupdr.pupdr().modify(|_, w| { w.$pupdrx().pull_down() });

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as a pulled up input pin
                    pub fn into_pull_up_input(
                        self,
                        moder: &mut MODER,
                        pupdr: &mut PUPDR,
                    ) -> $PXi<Input<PullUp>> {
                        moder
                            .moder()
                            .modify(|_, w| {
                                w.$moderx().input()
                            });

                        pupdr.pupdr().modify(|_, w| { w.$pupdrx().pull_up() });

                        $PXi { _mode: PhantomData }
                    }

                    // /// Configures the pin to operate as an open drain output pin
                    // pub fn into_open_drain_output(
                    //     self,
                    //     moder: &mut MODER,
                    //     otyper: &mut OTYPER,
                    // ) -> $PXi<Output<OpenDrain>> {
                    //     let offset = 2 * $i;

                    //     // general purpose output mode
                    //     let mode = 0b01;
                    //     moder.moder().modify(|r, w| unsafe {
                    //         w.bits((r.bits() & !(0b11 << offset)) | (mode << offset))
                    //     });

                    //     // open drain output
                    //     otyper
                    //         .otyper()
                    //         .modify(|r, w| unsafe { w.bits(r.bits() | (0b1 << $i)) });

                    //     $PXi { _mode: PhantomData }
                    // }

                    /// Configures the pin to operate as an push pull output pin
                    pub fn into_push_pull_output(
                        self,
                        moder: &mut $MODER,
                        otyper: &mut $OTYPER,
                    ) -> $PXi<Output<PushPull>> {

                        moder
                            .moder()
                            .modify(|_, w| {
                                w.$moderx().output()
                            });

                        otyper
                            .otyper()
                            .modify(|_, w| {
                                w.$otyperx().push_pull()
                            });

                        $PXi { _mode: PhantomData }
                    }
                }

                impl<MODE> InputPin for $PXi<Input<MODE>> {
                    fn is_low(&self) -> bool {
                        !self.is_high()
                    }

                    fn is_high(&self) -> bool {
                        // NOTE(unsafe) atomic read with no side effects
                        unsafe { (*$GPIOX::ptr()).idr.read().$idr().bit_is_set() }
                    }
                }

                impl<MODE> OutputPin for $PXi<Output<MODE>> {
                    fn is_high(&self) -> bool {
                        !self.is_low()
                    }

                    fn is_low(&self) -> bool {
                        // NOTE(unsafe) atomic read with no side effects
                        unsafe { (*$GPIOX::ptr()).odr.read().$odr().bit_is_clear() }
                    }

                    fn set_high(&mut self) {
                        // NOTE(unsafe) atomic write to a stateless register
                        unsafe { (*$GPIOX::ptr()).bsrr.write(|w| w.$bs().set_bit()) }
                    }

                    fn set_low(&mut self) {
                        // NOTE(unsafe) atomic write to a stateless register
                        unsafe { (*$GPIOX::ptr()).bsrr.write(|w| w.$br().set_bit()) }
                    }
                }
            )+
        }
    }
}

#[rustfmt_skip]
gpio!(
    GPIOC, gpioc, gpiof, iopcen, iopcrst, PCx,
    [
        PC7: (pc7, idr7, odr7, bs7, br7, moder7, ot7, ospeedr7, pupdr7, Output<PushPull>, MODER, OTYPER, PUPDR),
        PC8: (pc8, idr8, odr8, bs8, br8, moder8, ot8, ospeedr8, pupdr8, Output<PushPull>, MODER, OTYPER, PUPDR),
    ]
);

#[rustfmt_skip]
gpio!(
    GPIOA, gpioa, gpioa, iopaen, ioparst, PAx,
    [
        PA0: (pa0, idr0, odr0, bs0, br0, moder0, ot0, ospeedr0, pupdr0, Input<Floating>, MODER, OTYPER, PUPDR),
        PA1: (pa1, idr1, odr1, bs1, br1, moder1, ot1, ospeedr1, pupdr1, Input<Floating>, MODER, OTYPER, PUPDR),
    ]
);
