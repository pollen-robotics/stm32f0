//! General Purpose Input / Output
//!
//! GPIO Configuration: cf. [Reference manual p.149](http://www.st.com/resource/en/reference_manual/dm00031936.pdf)

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
/// Analog input (type state)
pub struct Analog;

/// Output mode (type state)
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

/// Push pull output (type state)
pub struct PushPull;
/// Open drain output (type state)
pub struct OpenDrain;

/// Alternate function
pub struct Alternate<MODE, AF> {
    _mode: PhantomData<MODE>,
    _af: PhantomData<AF>,
}

pub trait AlternateFunction {
    fn bit_id(&self) -> u8;
}
macro_rules! AF {
    ([$($AFX:ident : $i:expr)+]) => {
        $(
            pub struct $AFX;
            impl AlternateFunction for $AFX {
                fn bit_id(&self) -> u8 {
                    $i
                }
            }
        )+
    }
}
AF!([AF0:0 AF1:1 AF2:2 AF3:3 AF4:4 AF5:5 AF6:6 AF7:7]);

macro_rules! gpio {
    ($GPIOX:ident, $gpiox:ident, $gpioy:ident, $iopxenr:ident, $iopxrst:ident, $PXx:ident,
            [$($PXi:ident: (
                    $pxi:ident,
                    $idr:ident, $odr:ident, $bs:ident, $br:ident,
                    $moderx:ident, $otyperx:ident, $ospeedrx:ident, $pupdrx:ident,
                    $afrx:ident, $afrxx:ident,
                    $MODE:ty,
                    $MODER:ident, $OTYPER:ident, $PUPDR:ident,
                    $AFRX:ident
                ),)+]
    ) => {
        /// GPIO
        pub mod $gpiox {
            use core::marker::PhantomData;

            use hal::digital::{InputPin, OutputPin};
            use stm32f0x2::{$gpioy, $GPIOX};

            use rcc::AHB;
            use super::{
                Alternate,
                AlternateFunction,
                Analog,
                Floating, GpioExt, Input,
                OpenDrain,
                Output,
                PullDown, PullUp,
                PushPull,
            };

            /// GPIO parts
            pub struct Parts {
                pub moder: MODER,
                pub otyper: OTYPER,
                pub pupdr: PUPDR,
                pub afr: AFR,

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
                        afr: AFR { _0: () },
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

          /// Pull-Up Pull-Down register
         pub struct PUPDR {
             _0: (),
         }
         impl PUPDR {
             pub(crate) fn pupdr(&mut self) -> &$gpioy::PUPDR {
                 unsafe { &(*$GPIOX::ptr()).pupdr }
             }
         }

         /// Alternate Function Register
         pub struct AFR {
            _0: (),
         }
         impl AFR {
             pub(crate) fn afrh(&mut self) -> &$gpioy::AFRH {
                 unsafe { &(*$GPIOX::ptr()).afrh }
             }
             pub(crate) fn afrl(&mut self) -> &$gpioy::AFRL {
                 unsafe { &(*$GPIOX::ptr()).afrl }
             }
         }

            $(
                /// Pin
                pub struct $PXi<MODE> {
                    _mode: PhantomData<MODE>,
                }

                impl<MODE> $PXi<MODE> {
                    /// Configures the pin to operate as an alternate function push pull output pin
                    pub fn into_alternate_push_pull<AF>(
                        self,
                        moder: &mut $MODER,
                        afr: &mut AFR,
                        af: AF,
                    ) -> $PXi<Alternate<PushPull, AF>>
                    where AF: AlternateFunction
                    {
                        moder
                            .moder()
                            .modify(|_, w| {
                                w.$moderx().alternate()
                            });

                        afr.$afrx().modify(|_, w| {
                            unsafe {
                                w.$afrxx().bits(af.bit_id())
                            }
                        });

                        $PXi { _mode: PhantomData }
                    }

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

                    pub fn into_analog(
                        self,
                        moder: &mut MODER
                    ) -> $PXi<Input<Analog>> {
                        moder.moder().modify(|_, w| {
                            w.$moderx().analog()
                        });

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as an open drain output pin
                    pub fn into_open_drain_output(
                         self,
                         moder: &mut MODER,
                         otyper: &mut OTYPER,
                     ) -> $PXi<Output<OpenDrain>> {
                         moder
                            .moder()
                            .modify(|_, w| {
                                w.$moderx().output()
                            });

                        otyper
                                .otyper()
                                .modify(|_, w| {
                                w.$otyperx().open_drain()
                        });

                         $PXi { _mode: PhantomData }
                     }

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
        PC3: (pc3, idr3, odr3, bs3, br3, moder3, ot3, ospeedr3, pupdr3, afrl, afrl3, Output<PushPull>, MODER, OTYPER, PUPDR, AFRL),
        PC6: (pc6, idr6, odr6, bs6, br6, moder6, ot6, ospeedr6, pupdr6, afrl, afrl6, Output<PushPull>, MODER, OTYPER, PUPDR, AFRL),
        PC7: (pc7, idr7, odr7, bs7, br7, moder7, ot7, ospeedr7, pupdr7, afrl, afrl7, Output<PushPull>, MODER, OTYPER, PUPDR, AFRL),
        PC8: (pc8, idr8, odr8, bs8, br8, moder8, ot8, ospeedr8, pupdr8, afrh, afrh8, Output<PushPull>, MODER, OTYPER, PUPDR, AFRH),
        PC9: (pc9, idr9, odr9, bs9, br9, moder9, ot9, ospeedr9, pupdr9, afrh, afrh9, Output<PushPull>, MODER, OTYPER, PUPDR, AFRH),
    ]
);

#[rustfmt_skip]
gpio!(
    GPIOA, gpioa, gpioa, iopaen, ioparst, PAx,
    [
        PA0: (pa0, idr0, odr0, bs0, br0, moder0, ot0, ospeedr0, pupdr0, afrl, afrl0, Input<Floating>, MODER, OTYPER, PUPDR, AFRL),
        PA1: (pa1, idr1, odr1, bs1, br1, moder1, ot1, ospeedr1, pupdr1, afrl, afrl1, Input<Floating>, MODER, OTYPER, PUPDR, AFRL),

        PA9: (pa9, idr9, odr9, bs9, br9, moder9, ot9, ospeedr9, pupdr9, afrh, afrh9, Input<Floating>, MODER, OTYPER, PUPDR, AFRH),
        PA10: (pa10, idr10, odr10, bs10, br10, moder10, ot10, ospeedr10, pupdr10, afrh, afrh10, Input<Floating>, MODER, OTYPER, PUPDR, AFRH),
    ]
);

#[rustfmt_skip]
gpio!(
    GPIOB, gpiob, gpiof, iopben, iopbrst, PBx,
    [
        PB0: (pb0, idr0, odr0, bs0, br0, moder0, ot0, ospeedr0, pupdr0, afrl, afrl0, Input<Floating>, MODER, OTYPER, PUPDR, AFRL),
        PB1: (pb1, idr1, odr1, bs1, br1, moder1, ot1, ospeedr1, pupdr1, afrl, afrl1, Input<Floating>, MODER, OTYPER, PUPDR, AFRL),
        PB3: (pb3, idr3, odr3, bs3, br3, moder3, ot3, ospeedr3, pupdr3, afrl, afrl3, Input<Floating>, MODER, OTYPER, PUPDR, AFRL),
        PB4: (pb4, idr4, odr4, bs4, br4, moder4, ot4, ospeedr4, pupdr4, afrl, afrl4, Input<Floating>, MODER, OTYPER, PUPDR, AFRL),
        PB5: (pb5, idr5, odr5, bs5, br5, moder5, ot5, ospeedr5, pupdr5, afrl, afrl5, Input<Floating>, MODER, OTYPER, PUPDR, AFRL),

        PB9: (pb9, idr9, odr9, bs9, br9, moder9, ot9, ospeedr9, pupdr9, afrh, afrh9, Input<Floating>, MODER, OTYPER, PUPDR, AFRH),
        PB10: (pb10, idr10, odr10, bs10, br10, moder10, ot10, ospeedr10, pupdr10, afrh, afrh10, Input<Floating>, MODER, OTYPER, PUPDR, AFRH),
        PB11: (pb11, idr11, odr11, bs11, br11, moder11, ot11, ospeedr11, pupdr11, afrh, afrh11, Input<Floating>, MODER, OTYPER, PUPDR, AFRH),
        PB14: (pb14, idr14, odr14, bs14, br14, moder14, ot14, ospeedr14, pupdr14, afrh, afrh14, Input<Floating>, MODER, OTYPER, PUPDR, AFRH),
        PB15: (pb15, idr15, odr15, bs15, br15, moder15, ot15, ospeedr15, pupdr15, afrh, afrh15, Input<Floating>, MODER, OTYPER, PUPDR, AFRH),
    ]
);
