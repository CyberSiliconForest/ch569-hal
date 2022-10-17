#[doc = "Register `R32_STK_CMPLR` reader"]
pub struct R(crate::R<R32_STK_CMPLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R32_STK_CMPLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R32_STK_CMPLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R32_STK_CMPLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R32_STK_CMPLR` writer"]
pub struct W(crate::W<R32_STK_CMPLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R32_STK_CMPLR_SPEC>;
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
impl From<crate::W<R32_STK_CMPLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R32_STK_CMPLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPL` reader - CMPL"]
pub type CMPL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CMPL` writer - CMPL"]
pub type CMPL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, R32_STK_CMPLR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CMPL"]
    #[inline(always)]
    pub fn cmpl(&self) -> CMPL_R {
        CMPL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CMPL"]
    #[inline(always)]
    pub fn cmpl(&mut self) -> CMPL_W<0> {
        CMPL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Systick compare low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r32_stk_cmplr](index.html) module"]
pub struct R32_STK_CMPLR_SPEC;
impl crate::RegisterSpec for R32_STK_CMPLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r32_stk_cmplr::R](R) reader structure"]
impl crate::Readable for R32_STK_CMPLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r32_stk_cmplr::W](W) writer structure"]
impl crate::Writable for R32_STK_CMPLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R32_STK_CMPLR to value 0"]
impl crate::Resettable for R32_STK_CMPLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
