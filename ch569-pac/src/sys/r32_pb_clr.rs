#[doc = "Register `R32_PB_CLR` writer"]
pub struct W(crate::W<R32_PB_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R32_PB_CLR_SPEC>;
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
impl From<crate::W<R32_PB_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R32_PB_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `R32_PB_CLR` writer - GPIO PB clear output"]
pub type R32_PB_CLR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, R32_PB_CLR_SPEC, u32, u32, 25, O>;
impl W {
    #[doc = "Bits 0:24 - GPIO PB clear output"]
    #[inline(always)]
    pub fn r32_pb_clr(&mut self) -> R32_PB_CLR_W<0> {
        R32_PB_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO PB clear output\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r32_pb_clr](index.html) module"]
pub struct R32_PB_CLR_SPEC;
impl crate::RegisterSpec for R32_PB_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [r32_pb_clr::W](W) writer structure"]
impl crate::Writable for R32_PB_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R32_PB_CLR to value 0"]
impl crate::Resettable for R32_PB_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
