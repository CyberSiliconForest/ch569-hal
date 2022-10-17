#[doc = "Register `R8_SPI1_FIFO_COUNT` reader"]
pub struct R(crate::R<R8_SPI1_FIFO_COUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_SPI1_FIFO_COUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_SPI1_FIFO_COUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_SPI1_FIFO_COUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_SPI1_FIFO_COUNT` writer"]
pub struct W(crate::W<R8_SPI1_FIFO_COUNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_SPI1_FIFO_COUNT_SPEC>;
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
impl From<crate::W<R8_SPI1_FIFO_COUNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_SPI1_FIFO_COUNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `R8_SPI1_FIFO_COUNT` reader - SPI FIFO count status"]
pub type R8_SPI1_FIFO_COUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `R8_SPI1_FIFO_COUNT` writer - SPI FIFO count status"]
pub type R8_SPI1_FIFO_COUNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, R8_SPI1_FIFO_COUNT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - SPI FIFO count status"]
    #[inline(always)]
    pub fn r8_spi1_fifo_count(&self) -> R8_SPI1_FIFO_COUNT_R {
        R8_SPI1_FIFO_COUNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - SPI FIFO count status"]
    #[inline(always)]
    pub fn r8_spi1_fifo_count(&mut self) -> R8_SPI1_FIFO_COUNT_W<0> {
        R8_SPI1_FIFO_COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 FIFO count status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_spi1_fifo_count](index.html) module"]
pub struct R8_SPI1_FIFO_COUNT_SPEC;
impl crate::RegisterSpec for R8_SPI1_FIFO_COUNT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_spi1_fifo_count::R](R) reader structure"]
impl crate::Readable for R8_SPI1_FIFO_COUNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_spi1_fifo_count::W](W) writer structure"]
impl crate::Writable for R8_SPI1_FIFO_COUNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_SPI1_FIFO_COUNT to value 0"]
impl crate::Resettable for R8_SPI1_FIFO_COUNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
