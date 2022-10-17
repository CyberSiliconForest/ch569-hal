#[doc = "Register `R8_CLK_CFG_CTRL` reader"]
pub struct R(crate::R<R8_CLK_CFG_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_CLK_CFG_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_CLK_CFG_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_CLK_CFG_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_CLK_CFG_CTRL` writer"]
pub struct W(crate::W<R8_CLK_CFG_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_CLK_CFG_CTRL_SPEC>;
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
impl From<crate::W<R8_CLK_CFG_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_CLK_CFG_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_CLK_PLL_SLEEP` reader - PLL sleep control"]
pub type RB_CLK_PLL_SLEEP_R = crate::BitReader<bool>;
#[doc = "Field `RB_CLK_PLL_SLEEP` writer - PLL sleep control"]
pub type RB_CLK_PLL_SLEEP_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_CLK_CFG_CTRL_SPEC, bool, O>;
#[doc = "Field `RB_CLK_SEL_PLL` reader - clock source selection"]
pub type RB_CLK_SEL_PLL_R = crate::BitReader<bool>;
#[doc = "Field `RB_CLK_SEL_PLL` writer - clock source selection"]
pub type RB_CLK_SEL_PLL_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_CLK_CFG_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - PLL sleep control"]
    #[inline(always)]
    pub fn rb_clk_pll_sleep(&self) -> RB_CLK_PLL_SLEEP_R {
        RB_CLK_PLL_SLEEP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - clock source selection"]
    #[inline(always)]
    pub fn rb_clk_sel_pll(&self) -> RB_CLK_SEL_PLL_R {
        RB_CLK_SEL_PLL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLL sleep control"]
    #[inline(always)]
    pub fn rb_clk_pll_sleep(&mut self) -> RB_CLK_PLL_SLEEP_W<0> {
        RB_CLK_PLL_SLEEP_W::new(self)
    }
    #[doc = "Bit 1 - clock source selection"]
    #[inline(always)]
    pub fn rb_clk_sel_pll(&mut self) -> RB_CLK_SEL_PLL_W<1> {
        RB_CLK_SEL_PLL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clock control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_clk_cfg_ctrl](index.html) module"]
pub struct R8_CLK_CFG_CTRL_SPEC;
impl crate::RegisterSpec for R8_CLK_CFG_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_clk_cfg_ctrl::R](R) reader structure"]
impl crate::Readable for R8_CLK_CFG_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_clk_cfg_ctrl::W](W) writer structure"]
impl crate::Writable for R8_CLK_CFG_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_CLK_CFG_CTRL to value 0x80"]
impl crate::Resettable for R8_CLK_CFG_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
