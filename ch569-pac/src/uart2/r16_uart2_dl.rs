#[doc = "Register `R16_UART2_DL` reader"]
pub struct R(crate::R<R16_UART2_DL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R16_UART2_DL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R16_UART2_DL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R16_UART2_DL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R16_UART2_DL` writer"]
pub struct W(crate::W<R16_UART2_DL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R16_UART2_DL_SPEC>;
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
impl From<crate::W<R16_UART2_DL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R16_UART2_DL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `R16_UART2_DL` reader - UART divisor latch"]
pub type R16_UART2_DL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `R16_UART2_DL` writer - UART divisor latch"]
pub type R16_UART2_DL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, R16_UART2_DL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - UART divisor latch"]
    #[inline(always)]
    pub fn r16_uart2_dl(&self) -> R16_UART2_DL_R {
        R16_UART2_DL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART divisor latch"]
    #[inline(always)]
    pub fn r16_uart2_dl(&mut self) -> R16_UART2_DL_W<0> {
        R16_UART2_DL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART2 divisor latch\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r16_uart2_dl](index.html) module"]
pub struct R16_UART2_DL_SPEC;
impl crate::RegisterSpec for R16_UART2_DL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [r16_uart2_dl::R](R) reader structure"]
impl crate::Readable for R16_UART2_DL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r16_uart2_dl::W](W) writer structure"]
impl crate::Writable for R16_UART2_DL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R16_UART2_DL to value 0"]
impl crate::Resettable for R16_UART2_DL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
