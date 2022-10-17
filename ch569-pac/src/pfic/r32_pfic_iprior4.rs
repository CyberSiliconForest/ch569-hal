#[doc = "Register `R32_PFIC_IPRIOR4` reader"]
pub struct R(crate::R<R32_PFIC_IPRIOR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R32_PFIC_IPRIOR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R32_PFIC_IPRIOR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R32_PFIC_IPRIOR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R32_PFIC_IPRIOR4` writer"]
pub struct W(crate::W<R32_PFIC_IPRIOR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R32_PFIC_IPRIOR4_SPEC>;
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
impl From<crate::W<R32_PFIC_IPRIOR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R32_PFIC_IPRIOR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IPRIOR4` reader - IPRIOR4"]
pub type IPRIOR4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IPRIOR4` writer - IPRIOR4"]
pub type IPRIOR4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, R32_PFIC_IPRIOR4_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR4"]
    #[inline(always)]
    pub fn iprior4(&self) -> IPRIOR4_R {
        IPRIOR4_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR4"]
    #[inline(always)]
    pub fn iprior4(&mut self) -> IPRIOR4_W<0> {
        IPRIOR4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r32_pfic_iprior4](index.html) module"]
pub struct R32_PFIC_IPRIOR4_SPEC;
impl crate::RegisterSpec for R32_PFIC_IPRIOR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r32_pfic_iprior4::R](R) reader structure"]
impl crate::Readable for R32_PFIC_IPRIOR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r32_pfic_iprior4::W](W) writer structure"]
impl crate::Writable for R32_PFIC_IPRIOR4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R32_PFIC_IPRIOR4 to value 0"]
impl crate::Resettable for R32_PFIC_IPRIOR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
