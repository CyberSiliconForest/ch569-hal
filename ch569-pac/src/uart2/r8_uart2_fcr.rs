#[doc = "Register `R8_UART2_FCR` reader"]
pub struct R(crate::R<R8_UART2_FCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_UART2_FCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_UART2_FCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_UART2_FCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_UART2_FCR` writer"]
pub struct W(crate::W<R8_UART2_FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_UART2_FCR_SPEC>;
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
impl From<crate::W<R8_UART2_FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_UART2_FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_FCR_FIFO_EN` reader - UART FIFO enable"]
pub type RB_FCR_FIFO_EN_R = crate::BitReader<bool>;
#[doc = "Field `RB_FCR_FIFO_EN` writer - UART FIFO enable"]
pub type RB_FCR_FIFO_EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_UART2_FCR_SPEC, bool, O>;
#[doc = "Field `RB_FCR_RX_FIFO_CLR` reader - clear UART receiver FIFO, high action, auto clear"]
pub type RB_FCR_RX_FIFO_CLR_R = crate::BitReader<bool>;
#[doc = "Field `RB_FCR_RX_FIFO_CLR` writer - clear UART receiver FIFO, high action, auto clear"]
pub type RB_FCR_RX_FIFO_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_UART2_FCR_SPEC, bool, O>;
#[doc = "Field `RB_FCR_TX_FIFO_CLR` reader - clear UART transmitter FIFO, high action, auto clear"]
pub type RB_FCR_TX_FIFO_CLR_R = crate::BitReader<bool>;
#[doc = "Field `RB_FCR_TX_FIFO_CLR` writer - clear UART transmitter FIFO, high action, auto clear"]
pub type RB_FCR_TX_FIFO_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_UART2_FCR_SPEC, bool, O>;
#[doc = "Field `RB_FCR_FIFO_TRIG` reader - UART receiver FIFO trigger level"]
pub type RB_FCR_FIFO_TRIG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RB_FCR_FIFO_TRIG` writer - UART receiver FIFO trigger level"]
pub type RB_FCR_FIFO_TRIG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, R8_UART2_FCR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - UART FIFO enable"]
    #[inline(always)]
    pub fn rb_fcr_fifo_en(&self) -> RB_FCR_FIFO_EN_R {
        RB_FCR_FIFO_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - clear UART receiver FIFO, high action, auto clear"]
    #[inline(always)]
    pub fn rb_fcr_rx_fifo_clr(&self) -> RB_FCR_RX_FIFO_CLR_R {
        RB_FCR_RX_FIFO_CLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - clear UART transmitter FIFO, high action, auto clear"]
    #[inline(always)]
    pub fn rb_fcr_tx_fifo_clr(&self) -> RB_FCR_TX_FIFO_CLR_R {
        RB_FCR_TX_FIFO_CLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 6:7 - UART receiver FIFO trigger level"]
    #[inline(always)]
    pub fn rb_fcr_fifo_trig(&self) -> RB_FCR_FIFO_TRIG_R {
        RB_FCR_FIFO_TRIG_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - UART FIFO enable"]
    #[inline(always)]
    pub fn rb_fcr_fifo_en(&mut self) -> RB_FCR_FIFO_EN_W<0> {
        RB_FCR_FIFO_EN_W::new(self)
    }
    #[doc = "Bit 1 - clear UART receiver FIFO, high action, auto clear"]
    #[inline(always)]
    pub fn rb_fcr_rx_fifo_clr(&mut self) -> RB_FCR_RX_FIFO_CLR_W<1> {
        RB_FCR_RX_FIFO_CLR_W::new(self)
    }
    #[doc = "Bit 2 - clear UART transmitter FIFO, high action, auto clear"]
    #[inline(always)]
    pub fn rb_fcr_tx_fifo_clr(&mut self) -> RB_FCR_TX_FIFO_CLR_W<2> {
        RB_FCR_TX_FIFO_CLR_W::new(self)
    }
    #[doc = "Bits 6:7 - UART receiver FIFO trigger level"]
    #[inline(always)]
    pub fn rb_fcr_fifo_trig(&mut self) -> RB_FCR_FIFO_TRIG_W<6> {
        RB_FCR_FIFO_TRIG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART2 FIFO control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_uart2_fcr](index.html) module"]
pub struct R8_UART2_FCR_SPEC;
impl crate::RegisterSpec for R8_UART2_FCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_uart2_fcr::R](R) reader structure"]
impl crate::Readable for R8_UART2_FCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_uart2_fcr::W](W) writer structure"]
impl crate::Writable for R8_UART2_FCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_UART2_FCR to value 0"]
impl crate::Resettable for R8_UART2_FCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
