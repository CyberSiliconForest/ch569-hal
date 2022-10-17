#[doc = "Register `R8_UART0_MCR` reader"]
pub struct R(crate::R<R8_UART0_MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_UART0_MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_UART0_MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_UART0_MCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_UART0_MCR` writer"]
pub struct W(crate::W<R8_UART0_MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_UART0_MCR_SPEC>;
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
impl From<crate::W<R8_UART0_MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_UART0_MCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_MCR_DTR` reader - UART0 control DTR"]
pub type RB_MCR_DTR_R = crate::BitReader<bool>;
#[doc = "Field `RB_MCR_DTR` writer - UART0 control DTR"]
pub type RB_MCR_DTR_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_UART0_MCR_SPEC, bool, O>;
#[doc = "Field `RB_MCR_RTS` reader - UART0 control RTS"]
pub type RB_MCR_RTS_R = crate::BitReader<bool>;
#[doc = "Field `RB_MCR_RTS` writer - UART0 control RTS"]
pub type RB_MCR_RTS_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_UART0_MCR_SPEC, bool, O>;
#[doc = "Field `RB_MCR_OUT1` reader - UART0 control OUT1"]
pub type RB_MCR_OUT1_R = crate::BitReader<bool>;
#[doc = "Field `RB_MCR_OUT1` writer - UART0 control OUT1"]
pub type RB_MCR_OUT1_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_UART0_MCR_SPEC, bool, O>;
#[doc = "Field `RB_MCR_OUT2` reader - UART control OUT2"]
pub type RB_MCR_OUT2_R = crate::BitReader<bool>;
#[doc = "Field `RB_MCR_OUT2` writer - UART control OUT2"]
pub type RB_MCR_OUT2_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_UART0_MCR_SPEC, bool, O>;
#[doc = "Field `RB_MCR_LOOP` reader - UART0 enable local loop back"]
pub type RB_MCR_LOOP_R = crate::BitReader<bool>;
#[doc = "Field `RB_MCR_LOOP` writer - UART0 enable local loop back"]
pub type RB_MCR_LOOP_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_UART0_MCR_SPEC, bool, O>;
#[doc = "Field `RB_MCR_AU_FLOW_EN` reader - UART0 enable autoflow control"]
pub type RB_MCR_AU_FLOW_EN_R = crate::BitReader<bool>;
#[doc = "Field `RB_MCR_AU_FLOW_EN` writer - UART0 enable autoflow control"]
pub type RB_MCR_AU_FLOW_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_UART0_MCR_SPEC, bool, O>;
#[doc = "Field `RB_MCR_TNOW` reader - UART0 enable TNOW output on DTR pin"]
pub type RB_MCR_TNOW_R = crate::BitReader<bool>;
#[doc = "Field `RB_MCR_TNOW` writer - UART0 enable TNOW output on DTR pin"]
pub type RB_MCR_TNOW_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_UART0_MCR_SPEC, bool, O>;
#[doc = "Field `RB_MCR_HALF` reader - UART0 enable half-duplex"]
pub type RB_MCR_HALF_R = crate::BitReader<bool>;
#[doc = "Field `RB_MCR_HALF` writer - UART0 enable half-duplex"]
pub type RB_MCR_HALF_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_UART0_MCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - UART0 control DTR"]
    #[inline(always)]
    pub fn rb_mcr_dtr(&self) -> RB_MCR_DTR_R {
        RB_MCR_DTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART0 control RTS"]
    #[inline(always)]
    pub fn rb_mcr_rts(&self) -> RB_MCR_RTS_R {
        RB_MCR_RTS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART0 control OUT1"]
    #[inline(always)]
    pub fn rb_mcr_out1(&self) -> RB_MCR_OUT1_R {
        RB_MCR_OUT1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART control OUT2"]
    #[inline(always)]
    pub fn rb_mcr_out2(&self) -> RB_MCR_OUT2_R {
        RB_MCR_OUT2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART0 enable local loop back"]
    #[inline(always)]
    pub fn rb_mcr_loop(&self) -> RB_MCR_LOOP_R {
        RB_MCR_LOOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART0 enable autoflow control"]
    #[inline(always)]
    pub fn rb_mcr_au_flow_en(&self) -> RB_MCR_AU_FLOW_EN_R {
        RB_MCR_AU_FLOW_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART0 enable TNOW output on DTR pin"]
    #[inline(always)]
    pub fn rb_mcr_tnow(&self) -> RB_MCR_TNOW_R {
        RB_MCR_TNOW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART0 enable half-duplex"]
    #[inline(always)]
    pub fn rb_mcr_half(&self) -> RB_MCR_HALF_R {
        RB_MCR_HALF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART0 control DTR"]
    #[inline(always)]
    pub fn rb_mcr_dtr(&mut self) -> RB_MCR_DTR_W<0> {
        RB_MCR_DTR_W::new(self)
    }
    #[doc = "Bit 1 - UART0 control RTS"]
    #[inline(always)]
    pub fn rb_mcr_rts(&mut self) -> RB_MCR_RTS_W<1> {
        RB_MCR_RTS_W::new(self)
    }
    #[doc = "Bit 2 - UART0 control OUT1"]
    #[inline(always)]
    pub fn rb_mcr_out1(&mut self) -> RB_MCR_OUT1_W<2> {
        RB_MCR_OUT1_W::new(self)
    }
    #[doc = "Bit 3 - UART control OUT2"]
    #[inline(always)]
    pub fn rb_mcr_out2(&mut self) -> RB_MCR_OUT2_W<3> {
        RB_MCR_OUT2_W::new(self)
    }
    #[doc = "Bit 4 - UART0 enable local loop back"]
    #[inline(always)]
    pub fn rb_mcr_loop(&mut self) -> RB_MCR_LOOP_W<4> {
        RB_MCR_LOOP_W::new(self)
    }
    #[doc = "Bit 5 - UART0 enable autoflow control"]
    #[inline(always)]
    pub fn rb_mcr_au_flow_en(&mut self) -> RB_MCR_AU_FLOW_EN_W<5> {
        RB_MCR_AU_FLOW_EN_W::new(self)
    }
    #[doc = "Bit 6 - UART0 enable TNOW output on DTR pin"]
    #[inline(always)]
    pub fn rb_mcr_tnow(&mut self) -> RB_MCR_TNOW_W<6> {
        RB_MCR_TNOW_W::new(self)
    }
    #[doc = "Bit 7 - UART0 enable half-duplex"]
    #[inline(always)]
    pub fn rb_mcr_half(&mut self) -> RB_MCR_HALF_W<7> {
        RB_MCR_HALF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART0 modem control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_uart0_mcr](index.html) module"]
pub struct R8_UART0_MCR_SPEC;
impl crate::RegisterSpec for R8_UART0_MCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_uart0_mcr::R](R) reader structure"]
impl crate::Readable for R8_UART0_MCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_uart0_mcr::W](W) writer structure"]
impl crate::Writable for R8_UART0_MCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_UART0_MCR to value 0"]
impl crate::Resettable for R8_UART0_MCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
