#[doc = "Register `R8_UEP2_3_MOD_R8_UH_EP_MOD` reader"]
pub struct R(crate::R<R8_UEP2_3_MOD_R8_UH_EP_MOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_UEP2_3_MOD_R8_UH_EP_MOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_UEP2_3_MOD_R8_UH_EP_MOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_UEP2_3_MOD_R8_UH_EP_MOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_UEP2_3_MOD_R8_UH_EP_MOD` writer"]
pub struct W(crate::W<R8_UEP2_3_MOD_R8_UH_EP_MOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_UEP2_3_MOD_R8_UH_EP_MOD_SPEC>;
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
impl From<crate::W<R8_UEP2_3_MOD_R8_UH_EP_MOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_UEP2_3_MOD_R8_UH_EP_MOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_UEP2_BUF_MOD_RB_UH_RX_EN` reader - buffer mode of USB endpoint 2(10) and USB host receive endpoint (IN) enable"]
pub type RB_UEP2_BUF_MOD_RB_UH_RX_EN_R = crate::BitReader<bool>;
#[doc = "Field `RB_UEP2_BUF_MOD_RB_UH_RX_EN` writer - buffer mode of USB endpoint 2(10) and USB host receive endpoint (IN) enable"]
pub type RB_UEP2_BUF_MOD_RB_UH_RX_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_UEP2_3_MOD_R8_UH_EP_MOD_SPEC, bool, O>;
#[doc = "Field `RB_UEP2_TX_EN` reader - enable USB endpoint 2(10) transmittal (IN)"]
pub type RB_UEP2_TX_EN_R = crate::BitReader<bool>;
#[doc = "Field `RB_UEP2_TX_EN` writer - enable USB endpoint 2(10) transmittal (IN)"]
pub type RB_UEP2_TX_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_UEP2_3_MOD_R8_UH_EP_MOD_SPEC, bool, O>;
#[doc = "Field `RB_UEP2_RX_EN` reader - enable USB endpoint 2(10) receiving (OUT)"]
pub type RB_UEP2_RX_EN_R = crate::BitReader<bool>;
#[doc = "Field `RB_UEP2_RX_EN` writer - enable USB endpoint 2(10) receiving (OUT)"]
pub type RB_UEP2_RX_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_UEP2_3_MOD_R8_UH_EP_MOD_SPEC, bool, O>;
#[doc = "Field `RB_UEP3_BUF_MOD` reader - buffer mode of USB endpoint 3(11)"]
pub type RB_UEP3_BUF_MOD_R = crate::BitReader<bool>;
#[doc = "Field `RB_UEP3_BUF_MOD` writer - buffer mode of USB endpoint 3(11)"]
pub type RB_UEP3_BUF_MOD_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_UEP2_3_MOD_R8_UH_EP_MOD_SPEC, bool, O>;
#[doc = "Field `RB_UEP3_TX_EN_RB_UH_TX_EN` reader - enable USB endpoint 3(11) transmittal (IN) and USB host send endpoint (SETUP/OUT) enable"]
pub type RB_UEP3_TX_EN_RB_UH_TX_EN_R = crate::BitReader<bool>;
#[doc = "Field `RB_UEP3_TX_EN_RB_UH_TX_EN` writer - enable USB endpoint 3(11) transmittal (IN) and USB host send endpoint (SETUP/OUT) enable"]
pub type RB_UEP3_TX_EN_RB_UH_TX_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_UEP2_3_MOD_R8_UH_EP_MOD_SPEC, bool, O>;
#[doc = "Field `RB_UEP3_RX_EN` reader - enable USB endpoint 3(11) receiving (OUT)"]
pub type RB_UEP3_RX_EN_R = crate::BitReader<bool>;
#[doc = "Field `RB_UEP3_RX_EN` writer - enable USB endpoint 3(11) receiving (OUT)"]
pub type RB_UEP3_RX_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_UEP2_3_MOD_R8_UH_EP_MOD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - buffer mode of USB endpoint 2(10) and USB host receive endpoint (IN) enable"]
    #[inline(always)]
    pub fn rb_uep2_buf_mod_rb_uh_rx_en(&self) -> RB_UEP2_BUF_MOD_RB_UH_RX_EN_R {
        RB_UEP2_BUF_MOD_RB_UH_RX_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - enable USB endpoint 2(10) transmittal (IN)"]
    #[inline(always)]
    pub fn rb_uep2_tx_en(&self) -> RB_UEP2_TX_EN_R {
        RB_UEP2_TX_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable USB endpoint 2(10) receiving (OUT)"]
    #[inline(always)]
    pub fn rb_uep2_rx_en(&self) -> RB_UEP2_RX_EN_R {
        RB_UEP2_RX_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - buffer mode of USB endpoint 3(11)"]
    #[inline(always)]
    pub fn rb_uep3_buf_mod(&self) -> RB_UEP3_BUF_MOD_R {
        RB_UEP3_BUF_MOD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - enable USB endpoint 3(11) transmittal (IN) and USB host send endpoint (SETUP/OUT) enable"]
    #[inline(always)]
    pub fn rb_uep3_tx_en_rb_uh_tx_en(&self) -> RB_UEP3_TX_EN_RB_UH_TX_EN_R {
        RB_UEP3_TX_EN_RB_UH_TX_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - enable USB endpoint 3(11) receiving (OUT)"]
    #[inline(always)]
    pub fn rb_uep3_rx_en(&self) -> RB_UEP3_RX_EN_R {
        RB_UEP3_RX_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - buffer mode of USB endpoint 2(10) and USB host receive endpoint (IN) enable"]
    #[inline(always)]
    pub fn rb_uep2_buf_mod_rb_uh_rx_en(&mut self) -> RB_UEP2_BUF_MOD_RB_UH_RX_EN_W<0> {
        RB_UEP2_BUF_MOD_RB_UH_RX_EN_W::new(self)
    }
    #[doc = "Bit 2 - enable USB endpoint 2(10) transmittal (IN)"]
    #[inline(always)]
    pub fn rb_uep2_tx_en(&mut self) -> RB_UEP2_TX_EN_W<2> {
        RB_UEP2_TX_EN_W::new(self)
    }
    #[doc = "Bit 3 - enable USB endpoint 2(10) receiving (OUT)"]
    #[inline(always)]
    pub fn rb_uep2_rx_en(&mut self) -> RB_UEP2_RX_EN_W<3> {
        RB_UEP2_RX_EN_W::new(self)
    }
    #[doc = "Bit 4 - buffer mode of USB endpoint 3(11)"]
    #[inline(always)]
    pub fn rb_uep3_buf_mod(&mut self) -> RB_UEP3_BUF_MOD_W<4> {
        RB_UEP3_BUF_MOD_W::new(self)
    }
    #[doc = "Bit 6 - enable USB endpoint 3(11) transmittal (IN) and USB host send endpoint (SETUP/OUT) enable"]
    #[inline(always)]
    pub fn rb_uep3_tx_en_rb_uh_tx_en(&mut self) -> RB_UEP3_TX_EN_RB_UH_TX_EN_W<6> {
        RB_UEP3_TX_EN_RB_UH_TX_EN_W::new(self)
    }
    #[doc = "Bit 7 - enable USB endpoint 3(11) receiving (OUT)"]
    #[inline(always)]
    pub fn rb_uep3_rx_en(&mut self) -> RB_UEP3_RX_EN_W<7> {
        RB_UEP3_RX_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "endpoint 2(10) 3(11) mode and USB host endpoint mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_uep2_3_mod_r8_uh_ep_mod](index.html) module"]
pub struct R8_UEP2_3_MOD_R8_UH_EP_MOD_SPEC;
impl crate::RegisterSpec for R8_UEP2_3_MOD_R8_UH_EP_MOD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_uep2_3_mod_r8_uh_ep_mod::R](R) reader structure"]
impl crate::Readable for R8_UEP2_3_MOD_R8_UH_EP_MOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_uep2_3_mod_r8_uh_ep_mod::W](W) writer structure"]
impl crate::Writable for R8_UEP2_3_MOD_R8_UH_EP_MOD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_UEP2_3_MOD_R8_UH_EP_MOD to value 0"]
impl crate::Resettable for R8_UEP2_3_MOD_R8_UH_EP_MOD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
