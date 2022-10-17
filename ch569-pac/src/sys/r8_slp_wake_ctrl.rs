#[doc = "Register `R8_SLP_WAKE_CTRL` reader"]
pub struct R(crate::R<R8_SLP_WAKE_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_SLP_WAKE_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_SLP_WAKE_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_SLP_WAKE_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_SLP_WAKE_CTRL` writer"]
pub struct W(crate::W<R8_SLP_WAKE_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_SLP_WAKE_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<R8_SLP_WAKE_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_SLP_WAKE_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_SLP_USBHS_WAKE` reader - enable USBHS waking"]
pub type RB_SLP_USBHS_WAKE_R = crate::BitReader<bool>;
#[doc = "Field `RB_SLP_USBHS_WAKE` writer - enable USBHS waking"]
pub type RB_SLP_USBHS_WAKE_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SLP_WAKE_CTRL_SPEC, bool, O>;
#[doc = "Field `RB_SLP_USBSS_WAKE` reader - enable USBSS waking"]
pub type RB_SLP_USBSS_WAKE_R = crate::BitReader<bool>;
#[doc = "Field `RB_SLP_USBSS_WAKE` writer - enable USBSS waking"]
pub type RB_SLP_USBSS_WAKE_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SLP_WAKE_CTRL_SPEC, bool, O>;
#[doc = "Field `RB_SLP_CLK_ETH` reader - sleep ETH clock"]
pub type RB_SLP_CLK_ETH_R = crate::BitReader<bool>;
#[doc = "Field `RB_SLP_CLK_ETH` writer - sleep ETH clock"]
pub type RB_SLP_CLK_ETH_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SLP_WAKE_CTRL_SPEC, bool, O>;
#[doc = "Field `RB_SLP_CLK_ECDC` reader - sleep ECDC clock"]
pub type RB_SLP_CLK_ECDC_R = crate::BitReader<bool>;
#[doc = "Field `RB_SLP_CLK_ECDC` writer - sleep ECDC clock"]
pub type RB_SLP_CLK_ECDC_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SLP_WAKE_CTRL_SPEC, bool, O>;
#[doc = "Field `RB_SLP_GPIO_WAKE` reader - enable GPIO waking"]
pub type RB_SLP_GPIO_WAKE_R = crate::BitReader<bool>;
#[doc = "Field `RB_SLP_GPIO_WAKE` writer - enable GPIO waking"]
pub type RB_SLP_GPIO_WAKE_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SLP_WAKE_CTRL_SPEC, bool, O>;
#[doc = "Field `RB_SLP_ETH_WAKE` reader - enable Eth waking"]
pub type RB_SLP_ETH_WAKE_R = crate::BitReader<bool>;
#[doc = "Field `RB_SLP_ETH_WAKE` writer - enable Eth waking"]
pub type RB_SLP_ETH_WAKE_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SLP_WAKE_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - enable USBHS waking"]
    #[inline(always)]
    pub fn rb_slp_usbhs_wake(&self) -> RB_SLP_USBHS_WAKE_R {
        RB_SLP_USBHS_WAKE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable USBSS waking"]
    #[inline(always)]
    pub fn rb_slp_usbss_wake(&self) -> RB_SLP_USBSS_WAKE_R {
        RB_SLP_USBSS_WAKE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - sleep ETH clock"]
    #[inline(always)]
    pub fn rb_slp_clk_eth(&self) -> RB_SLP_CLK_ETH_R {
        RB_SLP_CLK_ETH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - sleep ECDC clock"]
    #[inline(always)]
    pub fn rb_slp_clk_ecdc(&self) -> RB_SLP_CLK_ECDC_R {
        RB_SLP_CLK_ECDC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - enable GPIO waking"]
    #[inline(always)]
    pub fn rb_slp_gpio_wake(&self) -> RB_SLP_GPIO_WAKE_R {
        RB_SLP_GPIO_WAKE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - enable Eth waking"]
    #[inline(always)]
    pub fn rb_slp_eth_wake(&self) -> RB_SLP_ETH_WAKE_R {
        RB_SLP_ETH_WAKE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable USBHS waking"]
    #[inline(always)]
    pub fn rb_slp_usbhs_wake(&mut self) -> RB_SLP_USBHS_WAKE_W<0> {
        RB_SLP_USBHS_WAKE_W::new(self)
    }
    #[doc = "Bit 1 - enable USBSS waking"]
    #[inline(always)]
    pub fn rb_slp_usbss_wake(&mut self) -> RB_SLP_USBSS_WAKE_W<1> {
        RB_SLP_USBSS_WAKE_W::new(self)
    }
    #[doc = "Bit 2 - sleep ETH clock"]
    #[inline(always)]
    pub fn rb_slp_clk_eth(&mut self) -> RB_SLP_CLK_ETH_W<2> {
        RB_SLP_CLK_ETH_W::new(self)
    }
    #[doc = "Bit 3 - sleep ECDC clock"]
    #[inline(always)]
    pub fn rb_slp_clk_ecdc(&mut self) -> RB_SLP_CLK_ECDC_W<3> {
        RB_SLP_CLK_ECDC_W::new(self)
    }
    #[doc = "Bit 4 - enable GPIO waking"]
    #[inline(always)]
    pub fn rb_slp_gpio_wake(&mut self) -> RB_SLP_GPIO_WAKE_W<4> {
        RB_SLP_GPIO_WAKE_W::new(self)
    }
    #[doc = "Bit 5 - enable Eth waking"]
    #[inline(always)]
    pub fn rb_slp_eth_wake(&mut self) -> RB_SLP_ETH_WAKE_W<5> {
        RB_SLP_ETH_WAKE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "wake control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_slp_wake_ctrl](index.html) module"]
pub struct R8_SLP_WAKE_CTRL_SPEC;
impl crate::RegisterSpec for R8_SLP_WAKE_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_slp_wake_ctrl::R](R) reader structure"]
impl crate::Readable for R8_SLP_WAKE_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_slp_wake_ctrl::W](W) writer structure"]
impl crate::Writable for R8_SLP_WAKE_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_SLP_WAKE_CTRL to value 0"]
impl crate::Resettable for R8_SLP_WAKE_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
