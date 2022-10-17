#[doc = "Register `R8_UART1_MCR` reader"]
pub struct R(crate::R<R8_UART1_MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_UART1_MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_UART1_MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_UART1_MCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_UART1_MCR` writer"]
pub struct W(crate::W<R8_UART1_MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_UART1_MCR_SPEC>;
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
impl From<crate::W<R8_UART1_MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_UART1_MCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_MCR_OUT2` reader - UART1 control OUT2"]
pub type RB_MCR_OUT2_R = crate::BitReader<bool>;
#[doc = "Field `RB_MCR_OUT2` writer - UART1 control OUT2"]
pub type RB_MCR_OUT2_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_UART1_MCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 3 - UART1 control OUT2"]
    #[inline(always)]
    pub fn rb_mcr_out2(&self) -> RB_MCR_OUT2_R {
        RB_MCR_OUT2_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - UART1 control OUT2"]
    #[inline(always)]
    pub fn rb_mcr_out2(&mut self) -> RB_MCR_OUT2_W<3> {
        RB_MCR_OUT2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART1 modem control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_uart1_mcr](index.html) module"]
pub struct R8_UART1_MCR_SPEC;
impl crate::RegisterSpec for R8_UART1_MCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_uart1_mcr::R](R) reader structure"]
impl crate::Readable for R8_UART1_MCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_uart1_mcr::W](W) writer structure"]
impl crate::Writable for R8_UART1_MCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_UART1_MCR to value 0"]
impl crate::Resettable for R8_UART1_MCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
