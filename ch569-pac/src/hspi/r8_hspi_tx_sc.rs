#[doc = "Register `R8_HSPI_TX_SC` reader"]
pub struct R(crate::R<R8_HSPI_TX_SC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_HSPI_TX_SC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_HSPI_TX_SC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_HSPI_TX_SC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_HSPI_TX_SC` writer"]
pub struct W(crate::W<R8_HSPI_TX_SC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_HSPI_TX_SC_SPEC>;
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
impl From<crate::W<R8_HSPI_TX_SC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_HSPI_TX_SC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_HSPI_TX_NUM` reader - parallel if tx sequence num"]
pub type RB_HSPI_TX_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RB_HSPI_TX_NUM` writer - parallel if tx sequence num"]
pub type RB_HSPI_TX_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, R8_HSPI_TX_SC_SPEC, u8, u8, 4, O>;
#[doc = "Field `RB_HSPI_TX_TOG` reader - parallel if tx addr toggle flag"]
pub type RB_HSPI_TX_TOG_R = crate::BitReader<bool>;
#[doc = "Field `RB_HSPI_TX_TOG` writer - parallel if tx addr toggle flag"]
pub type RB_HSPI_TX_TOG_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_HSPI_TX_SC_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - parallel if tx sequence num"]
    #[inline(always)]
    pub fn rb_hspi_tx_num(&self) -> RB_HSPI_TX_NUM_R {
        RB_HSPI_TX_NUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - parallel if tx addr toggle flag"]
    #[inline(always)]
    pub fn rb_hspi_tx_tog(&self) -> RB_HSPI_TX_TOG_R {
        RB_HSPI_TX_TOG_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - parallel if tx sequence num"]
    #[inline(always)]
    pub fn rb_hspi_tx_num(&mut self) -> RB_HSPI_TX_NUM_W<0> {
        RB_HSPI_TX_NUM_W::new(self)
    }
    #[doc = "Bit 4 - parallel if tx addr toggle flag"]
    #[inline(always)]
    pub fn rb_hspi_tx_tog(&mut self) -> RB_HSPI_TX_TOG_W<4> {
        RB_HSPI_TX_TOG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "parallel TX sequence ctrl\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_hspi_tx_sc](index.html) module"]
pub struct R8_HSPI_TX_SC_SPEC;
impl crate::RegisterSpec for R8_HSPI_TX_SC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_hspi_tx_sc::R](R) reader structure"]
impl crate::Readable for R8_HSPI_TX_SC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_hspi_tx_sc::W](W) writer structure"]
impl crate::Writable for R8_HSPI_TX_SC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_HSPI_TX_SC to value 0"]
impl crate::Resettable for R8_HSPI_TX_SC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
