#[doc = "Register `R8_EMMC_TIMEOUT` reader"]
pub struct R(crate::R<R8_EMMC_TIMEOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_EMMC_TIMEOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_EMMC_TIMEOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_EMMC_TIMEOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_EMMC_TIMEOUT` writer"]
pub struct W(crate::W<R8_EMMC_TIMEOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_EMMC_TIMEOUT_SPEC>;
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
impl From<crate::W<R8_EMMC_TIMEOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_EMMC_TIMEOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_EMMC_TOCNT_MASK` reader - response /data timeout configuration"]
pub type RB_EMMC_TOCNT_MASK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RB_EMMC_TOCNT_MASK` writer - response /data timeout configuration"]
pub type RB_EMMC_TOCNT_MASK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, R8_EMMC_TIMEOUT_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - response /data timeout configuration"]
    #[inline(always)]
    pub fn rb_emmc_tocnt_mask(&self) -> RB_EMMC_TOCNT_MASK_R {
        RB_EMMC_TOCNT_MASK_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - response /data timeout configuration"]
    #[inline(always)]
    pub fn rb_emmc_tocnt_mask(&mut self) -> RB_EMMC_TOCNT_MASK_W<0> {
        RB_EMMC_TOCNT_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SD 8bits data timeout value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_emmc_timeout](index.html) module"]
pub struct R8_EMMC_TIMEOUT_SPEC;
impl crate::RegisterSpec for R8_EMMC_TIMEOUT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_emmc_timeout::R](R) reader structure"]
impl crate::Readable for R8_EMMC_TIMEOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_emmc_timeout::W](W) writer structure"]
impl crate::Writable for R8_EMMC_TIMEOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_EMMC_TIMEOUT to value 0x0c"]
impl crate::Resettable for R8_EMMC_TIMEOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0c
    }
}
