#[doc = "Register `R32_EMMC_WRITE_CONT` writer"]
pub struct W(crate::W<R32_EMMC_WRITE_CONT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R32_EMMC_WRITE_CONT_SPEC>;
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
impl From<crate::W<R32_EMMC_WRITE_CONT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R32_EMMC_WRITE_CONT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `R32_EMMC_WRITE_CONT` writer - response parameter register"]
pub type R32_EMMC_WRITE_CONT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, R32_EMMC_WRITE_CONT_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - response parameter register"]
    #[inline(always)]
    pub fn r32_emmc_write_cont(&mut self) -> R32_EMMC_WRITE_CONT_W<0> {
        R32_EMMC_WRITE_CONT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Multiplexing register of the EMMC_RESPONSE3,\\[127:96\\]
32bits\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r32_emmc_write_cont](index.html) module"]
pub struct R32_EMMC_WRITE_CONT_SPEC;
impl crate::RegisterSpec for R32_EMMC_WRITE_CONT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [r32_emmc_write_cont::W](W) writer structure"]
impl crate::Writable for R32_EMMC_WRITE_CONT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R32_EMMC_WRITE_CONT to value 0"]
impl crate::Resettable for R32_EMMC_WRITE_CONT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
