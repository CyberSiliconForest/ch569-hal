#[doc = "Register `R32_PB_PD` reader"]
pub struct R(crate::R<R32_PB_PD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R32_PB_PD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R32_PB_PD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R32_PB_PD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R32_PB_PD` writer"]
pub struct W(crate::W<R32_PB_PD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R32_PB_PD_SPEC>;
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
impl From<crate::W<R32_PB_PD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R32_PB_PD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `R32_PB_PD` reader - GPIO PB output open-drain and input pulldown resistance enable"]
pub type R32_PB_PD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `R32_PB_PD` writer - GPIO PB output open-drain and input pulldown resistance enable"]
pub type R32_PB_PD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, R32_PB_PD_SPEC, u32, u32, 25, O>;
impl R {
    #[doc = "Bits 0:24 - GPIO PB output open-drain and input pulldown resistance enable"]
    #[inline(always)]
    pub fn r32_pb_pd(&self) -> R32_PB_PD_R {
        R32_PB_PD_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:24 - GPIO PB output open-drain and input pulldown resistance enable"]
    #[inline(always)]
    pub fn r32_pb_pd(&mut self) -> R32_PB_PD_W<0> {
        R32_PB_PD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO PB output open-drain and input pulldown resistance enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r32_pb_pd](index.html) module"]
pub struct R32_PB_PD_SPEC;
impl crate::RegisterSpec for R32_PB_PD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r32_pb_pd::R](R) reader structure"]
impl crate::Readable for R32_PB_PD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r32_pb_pd::W](W) writer structure"]
impl crate::Writable for R32_PB_PD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R32_PB_PD to value 0"]
impl crate::Resettable for R32_PB_PD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
