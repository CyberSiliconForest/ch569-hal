#[doc = "Register `R8_TMR1_FIFO_COUNT` reader"]
pub struct R(crate::R<R8_TMR1_FIFO_COUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_TMR1_FIFO_COUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_TMR1_FIFO_COUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_TMR1_FIFO_COUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `R8_TMR1_FIFO_COUNT` reader - TMR FIFO count status"]
pub type R8_TMR1_FIFO_COUNT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - TMR FIFO count status"]
    #[inline(always)]
    pub fn r8_tmr1_fifo_count(&self) -> R8_TMR1_FIFO_COUNT_R {
        R8_TMR1_FIFO_COUNT_R::new(self.bits)
    }
}
#[doc = "TMR1 FIFO count status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_tmr1_fifo_count](index.html) module"]
pub struct R8_TMR1_FIFO_COUNT_SPEC;
impl crate::RegisterSpec for R8_TMR1_FIFO_COUNT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_tmr1_fifo_count::R](R) reader structure"]
impl crate::Readable for R8_TMR1_FIFO_COUNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets R8_TMR1_FIFO_COUNT to value 0"]
impl crate::Resettable for R8_TMR1_FIFO_COUNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
