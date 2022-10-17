#[doc = "Register `R8_UART0_IER` reader"]
pub struct R(crate::R<R8_UART0_IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_UART0_IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_UART0_IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_UART0_IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_UART0_IER` writer"]
pub struct W(crate::W<R8_UART0_IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_UART0_IER_SPEC>;
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
impl From<crate::W<R8_UART0_IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_UART0_IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_IER_RECV_RDY` reader - UART interrupt enable for receiver data ready"]
pub type RB_IER_RECV_RDY_R = crate::BitReader<bool>;
#[doc = "Field `RB_IER_RECV_RDY` writer - UART interrupt enable for receiver data ready"]
pub type RB_IER_RECV_RDY_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_UART0_IER_SPEC, bool, O>;
#[doc = "Field `RB_IER_THR_EMPTY` reader - UART interrupt enable for THR empty"]
pub type RB_IER_THR_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `RB_IER_THR_EMPTY` writer - UART interrupt enable for THR empty"]
pub type RB_IER_THR_EMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_UART0_IER_SPEC, bool, O>;
#[doc = "Field `RB_IER_LINE_STAT` reader - UART interrupt enable for receiver line status"]
pub type RB_IER_LINE_STAT_R = crate::BitReader<bool>;
#[doc = "Field `RB_IER_LINE_STAT` writer - UART interrupt enable for receiver line status"]
pub type RB_IER_LINE_STAT_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_UART0_IER_SPEC, bool, O>;
#[doc = "Field `RB_IER_MODEM_CHG` reader - UART0 interrupt enable for modem status change"]
pub type RB_IER_MODEM_CHG_R = crate::BitReader<bool>;
#[doc = "Field `RB_IER_MODEM_CHG` writer - UART0 interrupt enable for modem status change"]
pub type RB_IER_MODEM_CHG_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_UART0_IER_SPEC, bool, O>;
#[doc = "Field `RB_IER_DTR_EN` reader - UART0 DTR/TNOW output pin enable"]
pub type RB_IER_DTR_EN_R = crate::BitReader<bool>;
#[doc = "Field `RB_IER_DTR_EN` writer - UART0 DTR/TNOW output pin enable"]
pub type RB_IER_DTR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_UART0_IER_SPEC, bool, O>;
#[doc = "Field `RB_IER_RTS_EN` reader - UART0 RTS output pin enable"]
pub type RB_IER_RTS_EN_R = crate::BitReader<bool>;
#[doc = "Field `RB_IER_RTS_EN` writer - UART0 RTS output pin enable"]
pub type RB_IER_RTS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_UART0_IER_SPEC, bool, O>;
#[doc = "Field `RB_IER_TXD_EN` reader - UART TXD pin enable"]
pub type RB_IER_TXD_EN_R = crate::BitReader<bool>;
#[doc = "Field `RB_IER_TXD_EN` writer - UART TXD pin enable"]
pub type RB_IER_TXD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_UART0_IER_SPEC, bool, O>;
#[doc = "Field `RB_IER_RESET` reader - UART software reset control, high action, auto clear"]
pub type RB_IER_RESET_R = crate::BitReader<bool>;
#[doc = "Field `RB_IER_RESET` writer - UART software reset control, high action, auto clear"]
pub type RB_IER_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_UART0_IER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - UART interrupt enable for receiver data ready"]
    #[inline(always)]
    pub fn rb_ier_recv_rdy(&self) -> RB_IER_RECV_RDY_R {
        RB_IER_RECV_RDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART interrupt enable for THR empty"]
    #[inline(always)]
    pub fn rb_ier_thr_empty(&self) -> RB_IER_THR_EMPTY_R {
        RB_IER_THR_EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART interrupt enable for receiver line status"]
    #[inline(always)]
    pub fn rb_ier_line_stat(&self) -> RB_IER_LINE_STAT_R {
        RB_IER_LINE_STAT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART0 interrupt enable for modem status change"]
    #[inline(always)]
    pub fn rb_ier_modem_chg(&self) -> RB_IER_MODEM_CHG_R {
        RB_IER_MODEM_CHG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART0 DTR/TNOW output pin enable"]
    #[inline(always)]
    pub fn rb_ier_dtr_en(&self) -> RB_IER_DTR_EN_R {
        RB_IER_DTR_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART0 RTS output pin enable"]
    #[inline(always)]
    pub fn rb_ier_rts_en(&self) -> RB_IER_RTS_EN_R {
        RB_IER_RTS_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART TXD pin enable"]
    #[inline(always)]
    pub fn rb_ier_txd_en(&self) -> RB_IER_TXD_EN_R {
        RB_IER_TXD_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART software reset control, high action, auto clear"]
    #[inline(always)]
    pub fn rb_ier_reset(&self) -> RB_IER_RESET_R {
        RB_IER_RESET_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART interrupt enable for receiver data ready"]
    #[inline(always)]
    pub fn rb_ier_recv_rdy(&mut self) -> RB_IER_RECV_RDY_W<0> {
        RB_IER_RECV_RDY_W::new(self)
    }
    #[doc = "Bit 1 - UART interrupt enable for THR empty"]
    #[inline(always)]
    pub fn rb_ier_thr_empty(&mut self) -> RB_IER_THR_EMPTY_W<1> {
        RB_IER_THR_EMPTY_W::new(self)
    }
    #[doc = "Bit 2 - UART interrupt enable for receiver line status"]
    #[inline(always)]
    pub fn rb_ier_line_stat(&mut self) -> RB_IER_LINE_STAT_W<2> {
        RB_IER_LINE_STAT_W::new(self)
    }
    #[doc = "Bit 3 - UART0 interrupt enable for modem status change"]
    #[inline(always)]
    pub fn rb_ier_modem_chg(&mut self) -> RB_IER_MODEM_CHG_W<3> {
        RB_IER_MODEM_CHG_W::new(self)
    }
    #[doc = "Bit 4 - UART0 DTR/TNOW output pin enable"]
    #[inline(always)]
    pub fn rb_ier_dtr_en(&mut self) -> RB_IER_DTR_EN_W<4> {
        RB_IER_DTR_EN_W::new(self)
    }
    #[doc = "Bit 5 - UART0 RTS output pin enable"]
    #[inline(always)]
    pub fn rb_ier_rts_en(&mut self) -> RB_IER_RTS_EN_W<5> {
        RB_IER_RTS_EN_W::new(self)
    }
    #[doc = "Bit 6 - UART TXD pin enable"]
    #[inline(always)]
    pub fn rb_ier_txd_en(&mut self) -> RB_IER_TXD_EN_W<6> {
        RB_IER_TXD_EN_W::new(self)
    }
    #[doc = "Bit 7 - UART software reset control, high action, auto clear"]
    #[inline(always)]
    pub fn rb_ier_reset(&mut self) -> RB_IER_RESET_W<7> {
        RB_IER_RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART0 interrupt enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_uart0_ier](index.html) module"]
pub struct R8_UART0_IER_SPEC;
impl crate::RegisterSpec for R8_UART0_IER_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_uart0_ier::R](R) reader structure"]
impl crate::Readable for R8_UART0_IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_uart0_ier::W](W) writer structure"]
impl crate::Writable for R8_UART0_IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_UART0_IER to value 0"]
impl crate::Resettable for R8_UART0_IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
