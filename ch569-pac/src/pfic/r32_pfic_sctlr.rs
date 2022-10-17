#[doc = "Register `R32_PFIC_SCTLR` reader"]
pub struct R(crate::R<R32_PFIC_SCTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R32_PFIC_SCTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R32_PFIC_SCTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R32_PFIC_SCTLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R32_PFIC_SCTLR` writer"]
pub struct W(crate::W<R32_PFIC_SCTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R32_PFIC_SCTLR_SPEC>;
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
impl From<crate::W<R32_PFIC_SCTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R32_PFIC_SCTLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLEEPONEXIT` reader - SLEEPONEXIT"]
pub type SLEEPONEXIT_R = crate::BitReader<bool>;
#[doc = "Field `SLEEPONEXIT` writer - SLEEPONEXIT"]
pub type SLEEPONEXIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, R32_PFIC_SCTLR_SPEC, bool, O>;
#[doc = "Field `SLEEPDEEP` reader - SLEEPDEEP"]
pub type SLEEPDEEP_R = crate::BitReader<bool>;
#[doc = "Field `SLEEPDEEP` writer - SLEEPDEEP"]
pub type SLEEPDEEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, R32_PFIC_SCTLR_SPEC, bool, O>;
#[doc = "Field `WFITOWFE` reader - WFITOWFE"]
pub type WFITOWFE_R = crate::BitReader<bool>;
#[doc = "Field `WFITOWFE` writer - WFITOWFE"]
pub type WFITOWFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, R32_PFIC_SCTLR_SPEC, bool, O>;
#[doc = "Field `SEVONPEND` reader - SEVONPEND"]
pub type SEVONPEND_R = crate::BitReader<bool>;
#[doc = "Field `SEVONPEND` writer - SEVONPEND"]
pub type SEVONPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, R32_PFIC_SCTLR_SPEC, bool, O>;
#[doc = "Field `SETEVENT` reader - SETEVENT"]
pub type SETEVENT_R = crate::BitReader<bool>;
#[doc = "Field `SETEVENT` writer - SETEVENT"]
pub type SETEVENT_W<'a, const O: u8> = crate::BitWriter<'a, u32, R32_PFIC_SCTLR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - SLEEPONEXIT"]
    #[inline(always)]
    pub fn sleeponexit(&self) -> SLEEPONEXIT_R {
        SLEEPONEXIT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SLEEPDEEP"]
    #[inline(always)]
    pub fn sleepdeep(&self) -> SLEEPDEEP_R {
        SLEEPDEEP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WFITOWFE"]
    #[inline(always)]
    pub fn wfitowfe(&self) -> WFITOWFE_R {
        WFITOWFE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SEVONPEND"]
    #[inline(always)]
    pub fn sevonpend(&self) -> SEVONPEND_R {
        SEVONPEND_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SETEVENT"]
    #[inline(always)]
    pub fn setevent(&self) -> SETEVENT_R {
        SETEVENT_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SLEEPONEXIT"]
    #[inline(always)]
    pub fn sleeponexit(&mut self) -> SLEEPONEXIT_W<1> {
        SLEEPONEXIT_W::new(self)
    }
    #[doc = "Bit 2 - SLEEPDEEP"]
    #[inline(always)]
    pub fn sleepdeep(&mut self) -> SLEEPDEEP_W<2> {
        SLEEPDEEP_W::new(self)
    }
    #[doc = "Bit 3 - WFITOWFE"]
    #[inline(always)]
    pub fn wfitowfe(&mut self) -> WFITOWFE_W<3> {
        WFITOWFE_W::new(self)
    }
    #[doc = "Bit 4 - SEVONPEND"]
    #[inline(always)]
    pub fn sevonpend(&mut self) -> SEVONPEND_W<4> {
        SEVONPEND_W::new(self)
    }
    #[doc = "Bit 5 - SETEVENT"]
    #[inline(always)]
    pub fn setevent(&mut self) -> SETEVENT_W<5> {
        SETEVENT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r32_pfic_sctlr](index.html) module"]
pub struct R32_PFIC_SCTLR_SPEC;
impl crate::RegisterSpec for R32_PFIC_SCTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r32_pfic_sctlr::R](R) reader structure"]
impl crate::Readable for R32_PFIC_SCTLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r32_pfic_sctlr::W](W) writer structure"]
impl crate::Writable for R32_PFIC_SCTLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R32_PFIC_SCTLR to value 0"]
impl crate::Resettable for R32_PFIC_SCTLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
