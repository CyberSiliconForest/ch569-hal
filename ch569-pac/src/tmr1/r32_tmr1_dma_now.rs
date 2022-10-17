#[doc = "Register `R32_TMR1_DMA_NOW` reader"]
pub struct R(crate::R<R32_TMR1_DMA_NOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R32_TMR1_DMA_NOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R32_TMR1_DMA_NOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R32_TMR1_DMA_NOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R32_TMR1_DMA_NOW` writer"]
pub struct W(crate::W<R32_TMR1_DMA_NOW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R32_TMR1_DMA_NOW_SPEC>;
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
impl From<crate::W<R32_TMR1_DMA_NOW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R32_TMR1_DMA_NOW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `R16_TMR1_DMA_NOW` reader - TMR DMA current address"]
pub type R16_TMR1_DMA_NOW_R = crate::FieldReader<u32, u32>;
#[doc = "Field `R16_TMR1_DMA_NOW` writer - TMR DMA current address"]
pub type R16_TMR1_DMA_NOW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, R32_TMR1_DMA_NOW_SPEC, u32, u32, 18, O>;
impl R {
    #[doc = "Bits 0:17 - TMR DMA current address"]
    #[inline(always)]
    pub fn r16_tmr1_dma_now(&self) -> R16_TMR1_DMA_NOW_R {
        R16_TMR1_DMA_NOW_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - TMR DMA current address"]
    #[inline(always)]
    pub fn r16_tmr1_dma_now(&mut self) -> R16_TMR1_DMA_NOW_W<0> {
        R16_TMR1_DMA_NOW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TMR1 DMA current address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r32_tmr1_dma_now](index.html) module"]
pub struct R32_TMR1_DMA_NOW_SPEC;
impl crate::RegisterSpec for R32_TMR1_DMA_NOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r32_tmr1_dma_now::R](R) reader structure"]
impl crate::Readable for R32_TMR1_DMA_NOW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r32_tmr1_dma_now::W](W) writer structure"]
impl crate::Writable for R32_TMR1_DMA_NOW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R32_TMR1_DMA_NOW to value 0"]
impl crate::Resettable for R32_TMR1_DMA_NOW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
