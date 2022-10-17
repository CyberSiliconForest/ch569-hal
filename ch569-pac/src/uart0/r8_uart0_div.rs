#[doc = "Register `R8_UART0_DIV` reader"]
pub struct R(crate::R<R8_UART0_DIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_UART0_DIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_UART0_DIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_UART0_DIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_UART0_DIV` writer"]
pub struct W(crate::W<R8_UART0_DIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_UART0_DIV_SPEC>;
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
impl From<crate::W<R8_UART0_DIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_UART0_DIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `R8_UART0_ADR` reader - UART pre-divisor latch byte"]
pub type R8_UART0_ADR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `R8_UART0_ADR` writer - UART pre-divisor latch byte"]
pub type R8_UART0_ADR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, R8_UART0_DIV_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - UART pre-divisor latch byte"]
    #[inline(always)]
    pub fn r8_uart0_adr(&self) -> R8_UART0_ADR_R {
        R8_UART0_ADR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - UART pre-divisor latch byte"]
    #[inline(always)]
    pub fn r8_uart0_adr(&mut self) -> R8_UART0_ADR_W<0> {
        R8_UART0_ADR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART0 pre-divisor latch byte\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_uart0_div](index.html) module"]
pub struct R8_UART0_DIV_SPEC;
impl crate::RegisterSpec for R8_UART0_DIV_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_uart0_div::R](R) reader structure"]
impl crate::Readable for R8_UART0_DIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_uart0_div::W](W) writer structure"]
impl crate::Writable for R8_UART0_DIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_UART0_DIV to value 0"]
impl crate::Resettable for R8_UART0_DIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
