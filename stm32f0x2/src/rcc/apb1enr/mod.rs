#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::APB1ENR {
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
#[doc = "Possible values of the field `TIM2EN`"]
pub type TIM2ENR = super::ahbenr::DMA1ENR;
#[doc = "Possible values of the field `TIM3EN`"]
pub type TIM3ENR = super::ahbenr::DMA1ENR;
#[doc = "Possible values of the field `TIM6EN`"]
pub type TIM6ENR = super::ahbenr::DMA1ENR;
#[doc = "Possible values of the field `TIM7EN`"]
pub type TIM7ENR = super::ahbenr::DMA1ENR;
#[doc = "Possible values of the field `TIM14EN`"]
pub type TIM14ENR = super::ahbenr::DMA1ENR;
#[doc = "Possible values of the field `WWDGEN`"]
pub type WWDGENR = super::ahbenr::DMA1ENR;
#[doc = "Possible values of the field `SPI2EN`"]
pub type SPI2ENR = super::ahbenr::DMA1ENR;
#[doc = "Possible values of the field `USART2EN`"]
pub type USART2ENR = super::ahbenr::DMA1ENR;
#[doc = "Possible values of the field `USART3EN`"]
pub type USART3ENR = super::ahbenr::DMA1ENR;
#[doc = "Possible values of the field `USART4EN`"]
pub type USART4ENR = super::ahbenr::DMA1ENR;
#[doc = "Possible values of the field `I2C1EN`"]
pub type I2C1ENR = super::ahbenr::DMA1ENR;
#[doc = "Possible values of the field `I2C2EN`"]
pub type I2C2ENR = super::ahbenr::DMA1ENR;
#[doc = "Possible values of the field `USBRST`"]
pub type USBRSTR = super::ahbenr::DMA1ENR;
#[doc = "Possible values of the field `CANEN`"]
pub type CANENR = super::ahbenr::DMA1ENR;
#[doc = "Possible values of the field `CRSEN`"]
pub type CRSENR = super::ahbenr::DMA1ENR;
#[doc = "Possible values of the field `PWREN`"]
pub type PWRENR = super::ahbenr::DMA1ENR;
#[doc = "Possible values of the field `DACEN`"]
pub type DACENR = super::ahbenr::DMA1ENR;
#[doc = "Possible values of the field `CECEN`"]
pub type CECENR = super::ahbenr::DMA1ENR;
#[doc = "Values that can be written to the field `TIM2EN`"]
pub type TIM2ENW = super::ahbenr::DMA1ENW;
#[doc = r" Proxy"]
pub struct _TIM2ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM2ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM2ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
#[doc = "Values that can be written to the field `TIM3EN`"]
pub type TIM3ENW = super::ahbenr::DMA1ENW;
#[doc = r" Proxy"]
pub struct _TIM3ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM3ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM3ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
#[doc = "Values that can be written to the field `TIM6EN`"]
pub type TIM6ENW = super::ahbenr::DMA1ENW;
#[doc = r" Proxy"]
pub struct _TIM6ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM6ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM6ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
#[doc = "Values that can be written to the field `TIM7EN`"]
pub type TIM7ENW = super::ahbenr::DMA1ENW;
#[doc = r" Proxy"]
pub struct _TIM7ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM7ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM7ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
#[doc = "Values that can be written to the field `TIM14EN`"]
pub type TIM14ENW = super::ahbenr::DMA1ENW;
#[doc = r" Proxy"]
pub struct _TIM14ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM14ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM14ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
#[doc = "Values that can be written to the field `WWDGEN`"]
pub type WWDGENW = super::ahbenr::DMA1ENW;
#[doc = r" Proxy"]
pub struct _WWDGENW<'a> {
    w: &'a mut W,
}
impl<'a> _WWDGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WWDGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
#[doc = "Values that can be written to the field `SPI2EN`"]
pub type SPI2ENW = super::ahbenr::DMA1ENW;
#[doc = r" Proxy"]
pub struct _SPI2ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI2ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI2ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
#[doc = "Values that can be written to the field `USART2EN`"]
pub type USART2ENW = super::ahbenr::DMA1ENW;
#[doc = r" Proxy"]
pub struct _USART2ENW<'a> {
    w: &'a mut W,
}
impl<'a> _USART2ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USART2ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USART3EN`"]
pub type USART3ENW = super::ahbenr::DMA1ENW;
#[doc = r" Proxy"]
pub struct _USART3ENW<'a> {
    w: &'a mut W,
}
impl<'a> _USART3ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USART3ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USART4EN`"]
pub type USART4ENW = super::ahbenr::DMA1ENW;
#[doc = r" Proxy"]
pub struct _USART4ENW<'a> {
    w: &'a mut W,
}
impl<'a> _USART4ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USART4ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `I2C1EN`"]
pub type I2C1ENW = super::ahbenr::DMA1ENW;
#[doc = r" Proxy"]
pub struct _I2C1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C1ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2C1ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `I2C2EN`"]
pub type I2C2ENW = super::ahbenr::DMA1ENW;
#[doc = r" Proxy"]
pub struct _I2C2ENW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C2ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2C2ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USBRST`"]
pub type USBRSTW = super::ahbenr::DMA1ENW;
#[doc = r" Proxy"]
pub struct _USBRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _USBRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CANEN`"]
pub type CANENW = super::ahbenr::DMA1ENW;
#[doc = r" Proxy"]
pub struct _CANENW<'a> {
    w: &'a mut W,
}
impl<'a> _CANENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CANENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CRSEN`"]
pub type CRSENW = super::ahbenr::DMA1ENW;
#[doc = r" Proxy"]
pub struct _CRSENW<'a> {
    w: &'a mut W,
}
impl<'a> _CRSENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
#[doc = "Values that can be written to the field `PWREN`"]
pub type PWRENW = super::ahbenr::DMA1ENW;
#[doc = r" Proxy"]
pub struct _PWRENW<'a> {
    w: &'a mut W,
}
impl<'a> _PWRENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWRENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
#[doc = "Values that can be written to the field `DACEN`"]
pub type DACENW = super::ahbenr::DMA1ENW;
#[doc = r" Proxy"]
pub struct _DACENW<'a> {
    w: &'a mut W,
}
impl<'a> _DACENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DACENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CECEN`"]
pub type CECENW = super::ahbenr::DMA1ENW;
#[doc = r" Proxy"]
pub struct _CECENW<'a> {
    w: &'a mut W,
}
impl<'a> _CECENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CECENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahbenr::DMA1ENW::ENABLED)
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
        const OFFSET: u8 = 30;
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
    #[doc = "Bit 0 - Timer 2 clock enable"]
    #[inline]
    pub fn tim2en(&self) -> TIM2ENR {
        TIM2ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Timer 3 clock enable"]
    #[inline]
    pub fn tim3en(&self) -> TIM3ENR {
        TIM3ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Timer 6 clock enable"]
    #[inline]
    pub fn tim6en(&self) -> TIM6ENR {
        TIM6ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - TIM7 timer clock enable"]
    #[inline]
    pub fn tim7en(&self) -> TIM7ENR {
        TIM7ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Timer 14 clock enable"]
    #[inline]
    pub fn tim14en(&self) -> TIM14ENR {
        TIM14ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Window watchdog clock enable"]
    #[inline]
    pub fn wwdgen(&self) -> WWDGENR {
        WWDGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - SPI 2 clock enable"]
    #[inline]
    pub fn spi2en(&self) -> SPI2ENR {
        SPI2ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - USART 2 clock enable"]
    #[inline]
    pub fn usart2en(&self) -> USART2ENR {
        USART2ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - USART3 clock enable"]
    #[inline]
    pub fn usart3en(&self) -> USART3ENR {
        USART3ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - USART4 clock enable"]
    #[inline]
    pub fn usart4en(&self) -> USART4ENR {
        USART4ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - I2C 1 clock enable"]
    #[inline]
    pub fn i2c1en(&self) -> I2C1ENR {
        I2C1ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - I2C 2 clock enable"]
    #[inline]
    pub fn i2c2en(&self) -> I2C2ENR {
        I2C2ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - USB interface clock enable"]
    #[inline]
    pub fn usbrst(&self) -> USBRSTR {
        USBRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - CAN interface clock enable"]
    #[inline]
    pub fn canen(&self) -> CANENR {
        CANENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Clock Recovery System interface clock enable"]
    #[inline]
    pub fn crsen(&self) -> CRSENR {
        CRSENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline]
    pub fn pwren(&self) -> PWRENR {
        PWRENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - DAC interface clock enable"]
    #[inline]
    pub fn dacen(&self) -> DACENR {
        DACENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - HDMI CEC interface clock enable"]
    #[inline]
    pub fn cecen(&self) -> CECENR {
        CECENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
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
    #[doc = "Bit 0 - Timer 2 clock enable"]
    #[inline]
    pub fn tim2en(&mut self) -> _TIM2ENW {
        _TIM2ENW { w: self }
    }
    #[doc = "Bit 1 - Timer 3 clock enable"]
    #[inline]
    pub fn tim3en(&mut self) -> _TIM3ENW {
        _TIM3ENW { w: self }
    }
    #[doc = "Bit 4 - Timer 6 clock enable"]
    #[inline]
    pub fn tim6en(&mut self) -> _TIM6ENW {
        _TIM6ENW { w: self }
    }
    #[doc = "Bit 5 - TIM7 timer clock enable"]
    #[inline]
    pub fn tim7en(&mut self) -> _TIM7ENW {
        _TIM7ENW { w: self }
    }
    #[doc = "Bit 8 - Timer 14 clock enable"]
    #[inline]
    pub fn tim14en(&mut self) -> _TIM14ENW {
        _TIM14ENW { w: self }
    }
    #[doc = "Bit 11 - Window watchdog clock enable"]
    #[inline]
    pub fn wwdgen(&mut self) -> _WWDGENW {
        _WWDGENW { w: self }
    }
    #[doc = "Bit 14 - SPI 2 clock enable"]
    #[inline]
    pub fn spi2en(&mut self) -> _SPI2ENW {
        _SPI2ENW { w: self }
    }
    #[doc = "Bit 17 - USART 2 clock enable"]
    #[inline]
    pub fn usart2en(&mut self) -> _USART2ENW {
        _USART2ENW { w: self }
    }
    #[doc = "Bit 18 - USART3 clock enable"]
    #[inline]
    pub fn usart3en(&mut self) -> _USART3ENW {
        _USART3ENW { w: self }
    }
    #[doc = "Bit 19 - USART4 clock enable"]
    #[inline]
    pub fn usart4en(&mut self) -> _USART4ENW {
        _USART4ENW { w: self }
    }
    #[doc = "Bit 21 - I2C 1 clock enable"]
    #[inline]
    pub fn i2c1en(&mut self) -> _I2C1ENW {
        _I2C1ENW { w: self }
    }
    #[doc = "Bit 22 - I2C 2 clock enable"]
    #[inline]
    pub fn i2c2en(&mut self) -> _I2C2ENW {
        _I2C2ENW { w: self }
    }
    #[doc = "Bit 23 - USB interface clock enable"]
    #[inline]
    pub fn usbrst(&mut self) -> _USBRSTW {
        _USBRSTW { w: self }
    }
    #[doc = "Bit 25 - CAN interface clock enable"]
    #[inline]
    pub fn canen(&mut self) -> _CANENW {
        _CANENW { w: self }
    }
    #[doc = "Bit 27 - Clock Recovery System interface clock enable"]
    #[inline]
    pub fn crsen(&mut self) -> _CRSENW {
        _CRSENW { w: self }
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline]
    pub fn pwren(&mut self) -> _PWRENW {
        _PWRENW { w: self }
    }
    #[doc = "Bit 29 - DAC interface clock enable"]
    #[inline]
    pub fn dacen(&mut self) -> _DACENW {
        _DACENW { w: self }
    }
    #[doc = "Bit 30 - HDMI CEC interface clock enable"]
    #[inline]
    pub fn cecen(&mut self) -> _CECENW {
        _CECENW { w: self }
    }
}
