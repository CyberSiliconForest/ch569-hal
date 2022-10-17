#[doc = "Register `R8_SPI0_BUFFER` reader"]
pub struct R(crate::R<R8_SPI0_BUFFER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_SPI0_BUFFER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_SPI0_BUFFER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_SPI0_BUFFER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `R8_SPI0_BUFFER` reader - SPI data buffer"]
pub type R8_SPI0_BUFFER_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - SPI data buffer"]
    #[inline(always)]
    pub fn r8_spi0_buffer(&self) -> R8_SPI0_BUFFER_R {
        R8_SPI0_BUFFER_R::new(self.bits)
    }
}
#[doc = "SPI0 data buffer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_spi0_buffer](index.html) module"]
pub struct R8_SPI0_BUFFER_SPEC;
impl crate::RegisterSpec for R8_SPI0_BUFFER_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_spi0_buffer::R](R) reader structure"]
impl crate::Readable for R8_SPI0_BUFFER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets R8_SPI0_BUFFER to value 0"]
impl crate::Resettable for R8_SPI0_BUFFER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
