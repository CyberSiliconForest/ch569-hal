#[doc = "Register `R32_PFIC_IPRIOR62` reader"]
pub struct R(crate::R<R32_PFIC_IPRIOR62_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R32_PFIC_IPRIOR62_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R32_PFIC_IPRIOR62_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R32_PFIC_IPRIOR62_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R32_PFIC_IPRIOR62` writer"]
pub struct W(crate::W<R32_PFIC_IPRIOR62_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R32_PFIC_IPRIOR62_SPEC>;
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
impl From<crate::W<R32_PFIC_IPRIOR62_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R32_PFIC_IPRIOR62_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IPRIOR62` reader - IPRIOR62"]
pub type IPRIOR62_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IPRIOR62` writer - IPRIOR62"]
pub type IPRIOR62_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, R32_PFIC_IPRIOR62_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR62"]
    #[inline(always)]
    pub fn iprior62(&self) -> IPRIOR62_R {
        IPRIOR62_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR62"]
    #[inline(always)]
    pub fn iprior62(&mut self) -> IPRIOR62_W<0> {
        IPRIOR62_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r32_pfic_iprior62](index.html) module"]
pub struct R32_PFIC_IPRIOR62_SPEC;
impl crate::RegisterSpec for R32_PFIC_IPRIOR62_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r32_pfic_iprior62::R](R) reader structure"]
impl crate::Readable for R32_PFIC_IPRIOR62_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r32_pfic_iprior62::W](W) writer structure"]
impl crate::Writable for R32_PFIC_IPRIOR62_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R32_PFIC_IPRIOR62 to value 0"]
impl crate::Resettable for R32_PFIC_IPRIOR62_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
