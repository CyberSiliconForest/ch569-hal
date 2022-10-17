#[doc = "Register `R32_PFIC_FIBADDRR` reader"]
pub struct R(crate::R<R32_PFIC_FIBADDRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R32_PFIC_FIBADDRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R32_PFIC_FIBADDRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R32_PFIC_FIBADDRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R32_PFIC_FIBADDRR` writer"]
pub struct W(crate::W<R32_PFIC_FIBADDRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R32_PFIC_FIBADDRR_SPEC>;
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
impl From<crate::W<R32_PFIC_FIBADDRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R32_PFIC_FIBADDRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BASEADDR` reader - BASEADDR"]
pub type BASEADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BASEADDR` writer - BASEADDR"]
pub type BASEADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, R32_PFIC_FIBADDRR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 28:31 - BASEADDR"]
    #[inline(always)]
    pub fn baseaddr(&self) -> BASEADDR_R {
        BASEADDR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - BASEADDR"]
    #[inline(always)]
    pub fn baseaddr(&mut self) -> BASEADDR_W<28> {
        BASEADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Fast Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r32_pfic_fibaddrr](index.html) module"]
pub struct R32_PFIC_FIBADDRR_SPEC;
impl crate::RegisterSpec for R32_PFIC_FIBADDRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r32_pfic_fibaddrr::R](R) reader structure"]
impl crate::Readable for R32_PFIC_FIBADDRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r32_pfic_fibaddrr::W](W) writer structure"]
impl crate::Writable for R32_PFIC_FIBADDRR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R32_PFIC_FIBADDRR to value 0"]
impl crate::Resettable for R32_PFIC_FIBADDRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
