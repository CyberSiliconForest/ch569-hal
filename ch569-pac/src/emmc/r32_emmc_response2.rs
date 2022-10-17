#[doc = "Register `R32_EMMC_RESPONSE2` reader"]
pub struct R(crate::R<R32_EMMC_RESPONSE2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R32_EMMC_RESPONSE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R32_EMMC_RESPONSE2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R32_EMMC_RESPONSE2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `R32_EMMC_RESPONSE2` reader - response parameter register"]
pub type R32_EMMC_RESPONSE2_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - response parameter register"]
    #[inline(always)]
    pub fn r32_emmc_response2(&self) -> R32_EMMC_RESPONSE2_R {
        R32_EMMC_RESPONSE2_R::new(self.bits)
    }
}
#[doc = "SD 128bits response register, \\[95:64\\]
32bits\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r32_emmc_response2](index.html) module"]
pub struct R32_EMMC_RESPONSE2_SPEC;
impl crate::RegisterSpec for R32_EMMC_RESPONSE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r32_emmc_response2::R](R) reader structure"]
impl crate::Readable for R32_EMMC_RESPONSE2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets R32_EMMC_RESPONSE2 to value 0"]
impl crate::Resettable for R32_EMMC_RESPONSE2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
