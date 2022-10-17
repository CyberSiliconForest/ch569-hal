#[doc = "Register `R16_SPI0_TOTAL_CNT` reader"]
pub struct R(crate::R<R16_SPI0_TOTAL_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R16_SPI0_TOTAL_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R16_SPI0_TOTAL_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R16_SPI0_TOTAL_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R16_SPI0_TOTAL_CNT` writer"]
pub struct W(crate::W<R16_SPI0_TOTAL_CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R16_SPI0_TOTAL_CNT_SPEC>;
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
impl From<crate::W<R16_SPI0_TOTAL_CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R16_SPI0_TOTAL_CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `R16_SPI0_TOTAL_CNT` reader - SPI total byte count, only low 12 bit"]
pub type R16_SPI0_TOTAL_CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `R16_SPI0_TOTAL_CNT` writer - SPI total byte count, only low 12 bit"]
pub type R16_SPI0_TOTAL_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, R16_SPI0_TOTAL_CNT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - SPI total byte count, only low 12 bit"]
    #[inline(always)]
    pub fn r16_spi0_total_cnt(&self) -> R16_SPI0_TOTAL_CNT_R {
        R16_SPI0_TOTAL_CNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - SPI total byte count, only low 12 bit"]
    #[inline(always)]
    pub fn r16_spi0_total_cnt(&mut self) -> R16_SPI0_TOTAL_CNT_W<0> {
        R16_SPI0_TOTAL_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 total byte count, only low 12 bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r16_spi0_total_cnt](index.html) module"]
pub struct R16_SPI0_TOTAL_CNT_SPEC;
impl crate::RegisterSpec for R16_SPI0_TOTAL_CNT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [r16_spi0_total_cnt::R](R) reader structure"]
impl crate::Readable for R16_SPI0_TOTAL_CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r16_spi0_total_cnt::W](W) writer structure"]
impl crate::Writable for R16_SPI0_TOTAL_CNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R16_SPI0_TOTAL_CNT to value 0"]
impl crate::Resettable for R16_SPI0_TOTAL_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
