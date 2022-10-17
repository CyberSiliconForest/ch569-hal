#[doc = "Register `R16_HSPI_DMA_LEN1` reader"]
pub struct R(crate::R<R16_HSPI_DMA_LEN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R16_HSPI_DMA_LEN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R16_HSPI_DMA_LEN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R16_HSPI_DMA_LEN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R16_HSPI_DMA_LEN1` writer"]
pub struct W(crate::W<R16_HSPI_DMA_LEN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R16_HSPI_DMA_LEN1_SPEC>;
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
impl From<crate::W<R16_HSPI_DMA_LEN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R16_HSPI_DMA_LEN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_HSPI_DMA_LEN1` reader - parallel if dma length1"]
pub type RB_HSPI_DMA_LEN1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RB_HSPI_DMA_LEN1` writer - parallel if dma length1"]
pub type RB_HSPI_DMA_LEN1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, R16_HSPI_DMA_LEN1_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - parallel if dma length1"]
    #[inline(always)]
    pub fn rb_hspi_dma_len1(&self) -> RB_HSPI_DMA_LEN1_R {
        RB_HSPI_DMA_LEN1_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - parallel if dma length1"]
    #[inline(always)]
    pub fn rb_hspi_dma_len1(&mut self) -> RB_HSPI_DMA_LEN1_W<0> {
        RB_HSPI_DMA_LEN1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "parallel if dma length1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r16_hspi_dma_len1](index.html) module"]
pub struct R16_HSPI_DMA_LEN1_SPEC;
impl crate::RegisterSpec for R16_HSPI_DMA_LEN1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [r16_hspi_dma_len1::R](R) reader structure"]
impl crate::Readable for R16_HSPI_DMA_LEN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r16_hspi_dma_len1::W](W) writer structure"]
impl crate::Writable for R16_HSPI_DMA_LEN1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R16_HSPI_DMA_LEN1 to value 0"]
impl crate::Resettable for R16_HSPI_DMA_LEN1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
