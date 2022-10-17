#[doc = "Register `R8_CLK_MOD_AUX` reader"]
pub struct R(crate::R<R8_CLK_MOD_AUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_CLK_MOD_AUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_CLK_MOD_AUX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_CLK_MOD_AUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_CLK_MOD_AUX` writer"]
pub struct W(crate::W<R8_CLK_MOD_AUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_CLK_MOD_AUX_SPEC>;
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
impl From<crate::W<R8_CLK_MOD_AUX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_CLK_MOD_AUX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_INT_125M_EN` reader - clock from USB_PHY PCLK(125MHz)"]
pub type RB_INT_125M_EN_R = crate::BitReader<bool>;
#[doc = "Field `RB_INT_125M_EN` writer - clock from USB_PHY PCLK(125MHz)"]
pub type RB_INT_125M_EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_CLK_MOD_AUX_SPEC, bool, O>;
#[doc = "Field `RB_EXT_125M_EN` reader - clock from pin_PA\\[16\\]"]
pub type RB_EXT_125M_EN_R = crate::BitReader<bool>;
#[doc = "Field `RB_EXT_125M_EN` writer - clock from pin_PA\\[16\\]"]
pub type RB_EXT_125M_EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_CLK_MOD_AUX_SPEC, bool, O>;
#[doc = "Field `RB_MCO_SEL_MSK` reader - MCO output selection"]
pub type RB_MCO_SEL_MSK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RB_MCO_SEL_MSK` writer - MCO output selection"]
pub type RB_MCO_SEL_MSK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, R8_CLK_MOD_AUX_SPEC, u8, u8, 2, O>;
#[doc = "Field `RB_MCO_EN` reader - MCO output enable"]
pub type RB_MCO_EN_R = crate::BitReader<bool>;
#[doc = "Field `RB_MCO_EN` writer - MCO output enable"]
pub type RB_MCO_EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_CLK_MOD_AUX_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - clock from USB_PHY PCLK(125MHz)"]
    #[inline(always)]
    pub fn rb_int_125m_en(&self) -> RB_INT_125M_EN_R {
        RB_INT_125M_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - clock from pin_PA\\[16\\]"]
    #[inline(always)]
    pub fn rb_ext_125m_en(&self) -> RB_EXT_125M_EN_R {
        RB_EXT_125M_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - MCO output selection"]
    #[inline(always)]
    pub fn rb_mco_sel_msk(&self) -> RB_MCO_SEL_MSK_R {
        RB_MCO_SEL_MSK_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - MCO output enable"]
    #[inline(always)]
    pub fn rb_mco_en(&self) -> RB_MCO_EN_R {
        RB_MCO_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - clock from USB_PHY PCLK(125MHz)"]
    #[inline(always)]
    pub fn rb_int_125m_en(&mut self) -> RB_INT_125M_EN_W<0> {
        RB_INT_125M_EN_W::new(self)
    }
    #[doc = "Bit 1 - clock from pin_PA\\[16\\]"]
    #[inline(always)]
    pub fn rb_ext_125m_en(&mut self) -> RB_EXT_125M_EN_W<1> {
        RB_EXT_125M_EN_W::new(self)
    }
    #[doc = "Bits 2:3 - MCO output selection"]
    #[inline(always)]
    pub fn rb_mco_sel_msk(&mut self) -> RB_MCO_SEL_MSK_W<2> {
        RB_MCO_SEL_MSK_W::new(self)
    }
    #[doc = "Bit 4 - MCO output enable"]
    #[inline(always)]
    pub fn rb_mco_en(&mut self) -> RB_MCO_EN_W<4> {
        RB_MCO_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clock mode aux register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_clk_mod_aux](index.html) module"]
pub struct R8_CLK_MOD_AUX_SPEC;
impl crate::RegisterSpec for R8_CLK_MOD_AUX_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_clk_mod_aux::R](R) reader structure"]
impl crate::Readable for R8_CLK_MOD_AUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_clk_mod_aux::W](W) writer structure"]
impl crate::Writable for R8_CLK_MOD_AUX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_CLK_MOD_AUX to value 0"]
impl crate::Resettable for R8_CLK_MOD_AUX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
