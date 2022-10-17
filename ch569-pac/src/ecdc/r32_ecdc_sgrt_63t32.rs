#[doc = "Register `R32_ECDC_SGRT_63T32` reader"]
pub struct R(crate::R<R32_ECDC_SGRT_63T32_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R32_ECDC_SGRT_63T32_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R32_ECDC_SGRT_63T32_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R32_ECDC_SGRT_63T32_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R32_ECDC_SGRT_63T32` writer"]
pub struct W(crate::W<R32_ECDC_SGRT_63T32_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R32_ECDC_SGRT_63T32_SPEC>;
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
impl From<crate::W<R32_ECDC_SGRT_63T32_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R32_ECDC_SGRT_63T32_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_ECDC_SGRT_63T32` reader - Single encryption and decryption result 0-31 register"]
pub type RB_ECDC_SGRT_63T32_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RB_ECDC_SGRT_63T32` writer - Single encryption and decryption result 0-31 register"]
pub type RB_ECDC_SGRT_63T32_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, R32_ECDC_SGRT_63T32_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Single encryption and decryption result 0-31 register"]
    #[inline(always)]
    pub fn rb_ecdc_sgrt_63t32(&self) -> RB_ECDC_SGRT_63T32_R {
        RB_ECDC_SGRT_63T32_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Single encryption and decryption result 0-31 register"]
    #[inline(always)]
    pub fn rb_ecdc_sgrt_63t32(&mut self) -> RB_ECDC_SGRT_63T32_W<0> {
        RB_ECDC_SGRT_63T32_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Single encryption and decryption result 0-31 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r32_ecdc_sgrt_63t32](index.html) module"]
pub struct R32_ECDC_SGRT_63T32_SPEC;
impl crate::RegisterSpec for R32_ECDC_SGRT_63T32_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r32_ecdc_sgrt_63t32::R](R) reader structure"]
impl crate::Readable for R32_ECDC_SGRT_63T32_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r32_ecdc_sgrt_63t32::W](W) writer structure"]
impl crate::Writable for R32_ECDC_SGRT_63T32_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R32_ECDC_SGRT_63T32 to value 0"]
impl crate::Resettable for R32_ECDC_SGRT_63T32_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
