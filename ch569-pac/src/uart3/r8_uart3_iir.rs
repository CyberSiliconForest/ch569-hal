#[doc = "Register `R8_UART3_IIR` reader"]
pub struct R(crate::R<R8_UART3_IIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_UART3_IIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_UART3_IIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_UART3_IIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RB_IIR_NO_INT` reader - UART no interrupt flag"]
pub type RB_IIR_NO_INT_R = crate::BitReader<bool>;
#[doc = "Field `RB_IIR_INT_MASK` reader - UART interrupt flag bit mask"]
pub type RB_IIR_INT_MASK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RB_IIR_FIFO_ID` reader - UART FIFO enabled flag"]
pub type RB_IIR_FIFO_ID_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - UART no interrupt flag"]
    #[inline(always)]
    pub fn rb_iir_no_int(&self) -> RB_IIR_NO_INT_R {
        RB_IIR_NO_INT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - UART interrupt flag bit mask"]
    #[inline(always)]
    pub fn rb_iir_int_mask(&self) -> RB_IIR_INT_MASK_R {
        RB_IIR_INT_MASK_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 6:7 - UART FIFO enabled flag"]
    #[inline(always)]
    pub fn rb_iir_fifo_id(&self) -> RB_IIR_FIFO_ID_R {
        RB_IIR_FIFO_ID_R::new(((self.bits >> 6) & 3) as u8)
    }
}
#[doc = "UART3 interrupt identification\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_uart3_iir](index.html) module"]
pub struct R8_UART3_IIR_SPEC;
impl crate::RegisterSpec for R8_UART3_IIR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_uart3_iir::R](R) reader structure"]
impl crate::Readable for R8_UART3_IIR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets R8_UART3_IIR to value 0x01"]
impl crate::Resettable for R8_UART3_IIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
