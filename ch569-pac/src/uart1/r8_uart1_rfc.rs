#[doc = "Register `R8_UART1_RFC` reader"]
pub struct R(crate::R<R8_UART1_RFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_UART1_RFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_UART1_RFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_UART1_RFC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `R8_UART1_RFC` reader - UART receiver FIFO count"]
pub type R8_UART1_RFC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - UART receiver FIFO count"]
    #[inline(always)]
    pub fn r8_uart1_rfc(&self) -> R8_UART1_RFC_R {
        R8_UART1_RFC_R::new(self.bits)
    }
}
#[doc = "UART1 receiver FIFO count\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_uart1_rfc](index.html) module"]
pub struct R8_UART1_RFC_SPEC;
impl crate::RegisterSpec for R8_UART1_RFC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_uart1_rfc::R](R) reader structure"]
impl crate::Readable for R8_UART1_RFC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets R8_UART1_RFC to value 0"]
impl crate::Resettable for R8_UART1_RFC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
