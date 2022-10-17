#[doc = "Register `HSPI_RX_SC` reader"]
pub struct R(crate::R<HSPI_RX_SC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSPI_RX_SC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSPI_RX_SC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSPI_RX_SC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSPI_RX_SC` writer"]
pub struct W(crate::W<HSPI_RX_SC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSPI_RX_SC_SPEC>;
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
impl From<crate::W<HSPI_RX_SC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSPI_RX_SC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_HSPI_RX_NUM` reader - parallel if rx sequence num"]
pub type RB_HSPI_RX_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RB_HSPI_RX_NUM` writer - parallel if rx sequence num"]
pub type RB_HSPI_RX_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, HSPI_RX_SC_SPEC, u8, u8, 4, O>;
#[doc = "Field `RB_HSPI_RX_TOG` reader - parallel if rx addr toggle flag"]
pub type RB_HSPI_RX_TOG_R = crate::BitReader<bool>;
#[doc = "Field `RB_HSPI_RX_TOG` writer - parallel if rx addr toggle flag"]
pub type RB_HSPI_RX_TOG_W<'a, const O: u8> = crate::BitWriter<'a, u8, HSPI_RX_SC_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - parallel if rx sequence num"]
    #[inline(always)]
    pub fn rb_hspi_rx_num(&self) -> RB_HSPI_RX_NUM_R {
        RB_HSPI_RX_NUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - parallel if rx addr toggle flag"]
    #[inline(always)]
    pub fn rb_hspi_rx_tog(&self) -> RB_HSPI_RX_TOG_R {
        RB_HSPI_RX_TOG_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - parallel if rx sequence num"]
    #[inline(always)]
    pub fn rb_hspi_rx_num(&mut self) -> RB_HSPI_RX_NUM_W<0> {
        RB_HSPI_RX_NUM_W::new(self)
    }
    #[doc = "Bit 4 - parallel if rx addr toggle flag"]
    #[inline(always)]
    pub fn rb_hspi_rx_tog(&mut self) -> RB_HSPI_RX_TOG_W<4> {
        RB_HSPI_RX_TOG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "parallel RX sequence ctrl\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hspi_rx_sc](index.html) module"]
pub struct HSPI_RX_SC_SPEC;
impl crate::RegisterSpec for HSPI_RX_SC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [hspi_rx_sc::R](R) reader structure"]
impl crate::Readable for HSPI_RX_SC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hspi_rx_sc::W](W) writer structure"]
impl crate::Writable for HSPI_RX_SC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSPI_RX_SC to value 0"]
impl crate::Resettable for HSPI_RX_SC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
