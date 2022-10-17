#[doc = "Register `R32_PA_PIN` reader"]
pub struct R(crate::R<R32_PA_PIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R32_PA_PIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R32_PA_PIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R32_PA_PIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `R32_PA_PIN` reader - GPIO PA input"]
pub type R32_PA_PIN_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - GPIO PA input"]
    #[inline(always)]
    pub fn r32_pa_pin(&self) -> R32_PA_PIN_R {
        R32_PA_PIN_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
#[doc = "GPIO PA input\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r32_pa_pin](index.html) module"]
pub struct R32_PA_PIN_SPEC;
impl crate::RegisterSpec for R32_PA_PIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r32_pa_pin::R](R) reader structure"]
impl crate::Readable for R32_PA_PIN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets R32_PA_PIN to value 0"]
impl crate::Resettable for R32_PA_PIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
