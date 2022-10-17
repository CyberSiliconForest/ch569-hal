#[doc = "Register `R8_UART2_TFC` reader"]
pub struct R(crate::R<R8_UART2_TFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_UART2_TFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_UART2_TFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_UART2_TFC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `R8_UART2_TFC` reader - UART transmitter FIFO count"]
pub type R8_UART2_TFC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - UART transmitter FIFO count"]
    #[inline(always)]
    pub fn r8_uart2_tfc(&self) -> R8_UART2_TFC_R {
        R8_UART2_TFC_R::new(self.bits)
    }
}
#[doc = "UART2 transmitter FIFO count\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_uart2_tfc](index.html) module"]
pub struct R8_UART2_TFC_SPEC;
impl crate::RegisterSpec for R8_UART2_TFC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_uart2_tfc::R](R) reader structure"]
impl crate::Readable for R8_UART2_TFC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets R8_UART2_TFC to value 0"]
impl crate::Resettable for R8_UART2_TFC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
