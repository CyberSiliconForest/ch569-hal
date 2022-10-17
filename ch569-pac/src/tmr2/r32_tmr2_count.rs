#[doc = "Register `R32_TMR2_COUNT` reader"]
pub struct R(crate::R<R32_TMR2_COUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R32_TMR2_COUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R32_TMR2_COUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R32_TMR2_COUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `R32_TMR2_COUNT` reader - TMR current count"]
pub type R32_TMR2_COUNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - TMR current count"]
    #[inline(always)]
    pub fn r32_tmr2_count(&self) -> R32_TMR2_COUNT_R {
        R32_TMR2_COUNT_R::new(self.bits)
    }
}
#[doc = "TMR2 current count\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r32_tmr2_count](index.html) module"]
pub struct R32_TMR2_COUNT_SPEC;
impl crate::RegisterSpec for R32_TMR2_COUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r32_tmr2_count::R](R) reader structure"]
impl crate::Readable for R32_TMR2_COUNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets R32_TMR2_COUNT to value 0"]
impl crate::Resettable for R32_TMR2_COUNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
