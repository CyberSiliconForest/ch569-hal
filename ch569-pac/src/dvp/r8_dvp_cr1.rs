#[doc = "Register `R8_DVP_CR1` reader"]
pub struct R(crate::R<R8_DVP_CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_DVP_CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_DVP_CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_DVP_CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_DVP_CR1` writer"]
pub struct W(crate::W<R8_DVP_CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_DVP_CR1_SPEC>;
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
impl From<crate::W<R8_DVP_CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_DVP_CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_DVP_DMA_ENABLE` reader - DVP dma enable"]
pub type RB_DVP_DMA_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `RB_DVP_DMA_ENABLE` writer - DVP dma enable"]
pub type RB_DVP_DMA_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_DVP_CR1_SPEC, bool, O>;
#[doc = "Field `RB_DVP_ALL_CLR` reader - DVP all clear, high action"]
pub type RB_DVP_ALL_CLR_R = crate::BitReader<bool>;
#[doc = "Field `RB_DVP_ALL_CLR` writer - DVP all clear, high action"]
pub type RB_DVP_ALL_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_DVP_CR1_SPEC, bool, O>;
#[doc = "Field `RB_DVP_RCV_CLR` reader - DVP receive logic clear, high action"]
pub type RB_DVP_RCV_CLR_R = crate::BitReader<bool>;
#[doc = "Field `RB_DVP_RCV_CLR` writer - DVP receive logic clear, high action"]
pub type RB_DVP_RCV_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_DVP_CR1_SPEC, bool, O>;
#[doc = "Field `RB_DVP_BUF_TOG` reader - DVP bug toggle by software"]
pub type RB_DVP_BUF_TOG_R = crate::BitReader<bool>;
#[doc = "Field `RB_DVP_BUF_TOG` writer - DVP bug toggle by software"]
pub type RB_DVP_BUF_TOG_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_DVP_CR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DVP dma enable"]
    #[inline(always)]
    pub fn rb_dvp_dma_enable(&self) -> RB_DVP_DMA_ENABLE_R {
        RB_DVP_DMA_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DVP all clear, high action"]
    #[inline(always)]
    pub fn rb_dvp_all_clr(&self) -> RB_DVP_ALL_CLR_R {
        RB_DVP_ALL_CLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DVP receive logic clear, high action"]
    #[inline(always)]
    pub fn rb_dvp_rcv_clr(&self) -> RB_DVP_RCV_CLR_R {
        RB_DVP_RCV_CLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DVP bug toggle by software"]
    #[inline(always)]
    pub fn rb_dvp_buf_tog(&self) -> RB_DVP_BUF_TOG_R {
        RB_DVP_BUF_TOG_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DVP dma enable"]
    #[inline(always)]
    pub fn rb_dvp_dma_enable(&mut self) -> RB_DVP_DMA_ENABLE_W<0> {
        RB_DVP_DMA_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - DVP all clear, high action"]
    #[inline(always)]
    pub fn rb_dvp_all_clr(&mut self) -> RB_DVP_ALL_CLR_W<1> {
        RB_DVP_ALL_CLR_W::new(self)
    }
    #[doc = "Bit 2 - DVP receive logic clear, high action"]
    #[inline(always)]
    pub fn rb_dvp_rcv_clr(&mut self) -> RB_DVP_RCV_CLR_W<2> {
        RB_DVP_RCV_CLR_W::new(self)
    }
    #[doc = "Bit 3 - DVP bug toggle by software"]
    #[inline(always)]
    pub fn rb_dvp_buf_tog(&mut self) -> RB_DVP_BUF_TOG_W<3> {
        RB_DVP_BUF_TOG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DVP control register1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_dvp_cr1](index.html) module"]
pub struct R8_DVP_CR1_SPEC;
impl crate::RegisterSpec for R8_DVP_CR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_dvp_cr1::R](R) reader structure"]
impl crate::Readable for R8_DVP_CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_dvp_cr1::W](W) writer structure"]
impl crate::Writable for R8_DVP_CR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_DVP_CR1 to value 0x06"]
impl crate::Resettable for R8_DVP_CR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x06
    }
}
