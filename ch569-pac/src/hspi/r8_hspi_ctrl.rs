#[doc = "Register `R8_HSPI_CTRL` reader"]
pub struct R(crate::R<R8_HSPI_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_HSPI_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_HSPI_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_HSPI_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_HSPI_CTRL` writer"]
pub struct W(crate::W<R8_HSPI_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_HSPI_CTRL_SPEC>;
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
impl From<crate::W<R8_HSPI_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_HSPI_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_HSPI_ENABLE` reader - parallel if enable"]
pub type RB_HSPI_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `RB_HSPI_ENABLE` writer - parallel if enable"]
pub type RB_HSPI_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_HSPI_CTRL_SPEC, bool, O>;
#[doc = "Field `RB_HSPI_DMA_EN` reader - parallel if dma enable"]
pub type RB_HSPI_DMA_EN_R = crate::BitReader<bool>;
#[doc = "Field `RB_HSPI_DMA_EN` writer - parallel if dma enable"]
pub type RB_HSPI_DMA_EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_HSPI_CTRL_SPEC, bool, O>;
#[doc = "Field `RB_HSPI_SW_ACT` reader - parallel if transmit software trigger"]
pub type RB_HSPI_SW_ACT_R = crate::BitReader<bool>;
#[doc = "Field `RB_HSPI_SW_ACT` writer - parallel if transmit software trigger"]
pub type RB_HSPI_SW_ACT_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_HSPI_CTRL_SPEC, bool, O>;
#[doc = "Field `RB_HSPI_ALL_CLR` reader - parallel if all clear"]
pub type RB_HSPI_ALL_CLR_R = crate::BitReader<bool>;
#[doc = "Field `RB_HSPI_ALL_CLR` writer - parallel if all clear"]
pub type RB_HSPI_ALL_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_HSPI_CTRL_SPEC, bool, O>;
#[doc = "Field `RB_HSPI_TRX_RST` reader - parallel if tx and rx logic clear, high action"]
pub type RB_HSPI_TRX_RST_R = crate::BitReader<bool>;
#[doc = "Field `RB_HSPI_TRX_RST` writer - parallel if tx and rx logic clear, high action"]
pub type RB_HSPI_TRX_RST_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_HSPI_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - parallel if enable"]
    #[inline(always)]
    pub fn rb_hspi_enable(&self) -> RB_HSPI_ENABLE_R {
        RB_HSPI_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - parallel if dma enable"]
    #[inline(always)]
    pub fn rb_hspi_dma_en(&self) -> RB_HSPI_DMA_EN_R {
        RB_HSPI_DMA_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - parallel if transmit software trigger"]
    #[inline(always)]
    pub fn rb_hspi_sw_act(&self) -> RB_HSPI_SW_ACT_R {
        RB_HSPI_SW_ACT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - parallel if all clear"]
    #[inline(always)]
    pub fn rb_hspi_all_clr(&self) -> RB_HSPI_ALL_CLR_R {
        RB_HSPI_ALL_CLR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - parallel if tx and rx logic clear, high action"]
    #[inline(always)]
    pub fn rb_hspi_trx_rst(&self) -> RB_HSPI_TRX_RST_R {
        RB_HSPI_TRX_RST_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - parallel if enable"]
    #[inline(always)]
    pub fn rb_hspi_enable(&mut self) -> RB_HSPI_ENABLE_W<0> {
        RB_HSPI_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - parallel if dma enable"]
    #[inline(always)]
    pub fn rb_hspi_dma_en(&mut self) -> RB_HSPI_DMA_EN_W<1> {
        RB_HSPI_DMA_EN_W::new(self)
    }
    #[doc = "Bit 2 - parallel if transmit software trigger"]
    #[inline(always)]
    pub fn rb_hspi_sw_act(&mut self) -> RB_HSPI_SW_ACT_W<2> {
        RB_HSPI_SW_ACT_W::new(self)
    }
    #[doc = "Bit 3 - parallel if all clear"]
    #[inline(always)]
    pub fn rb_hspi_all_clr(&mut self) -> RB_HSPI_ALL_CLR_W<3> {
        RB_HSPI_ALL_CLR_W::new(self)
    }
    #[doc = "Bit 4 - parallel if tx and rx logic clear, high action"]
    #[inline(always)]
    pub fn rb_hspi_trx_rst(&mut self) -> RB_HSPI_TRX_RST_W<4> {
        RB_HSPI_TRX_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "parallel if tx or rx control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_hspi_ctrl](index.html) module"]
pub struct R8_HSPI_CTRL_SPEC;
impl crate::RegisterSpec for R8_HSPI_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_hspi_ctrl::R](R) reader structure"]
impl crate::Readable for R8_HSPI_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_hspi_ctrl::W](W) writer structure"]
impl crate::Writable for R8_HSPI_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_HSPI_CTRL to value 0x18"]
impl crate::Resettable for R8_HSPI_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x18
    }
}
