#[doc = "Register `R8_HSPI_RTX_STATUS` reader"]
pub struct R(crate::R<R8_HSPI_RTX_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_HSPI_RTX_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_HSPI_RTX_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_HSPI_RTX_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_HSPI_RTX_STATUS` writer"]
pub struct W(crate::W<R8_HSPI_RTX_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_HSPI_RTX_STATUS_SPEC>;
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
impl From<crate::W<R8_HSPI_RTX_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_HSPI_RTX_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_HSPI_CRC_ERR` reader - CRC error occur"]
pub type RB_HSPI_CRC_ERR_R = crate::BitReader<bool>;
#[doc = "Field `RB_HSPI_CRC_ERR` writer - CRC error occur"]
pub type RB_HSPI_CRC_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_HSPI_RTX_STATUS_SPEC, bool, O>;
#[doc = "Field `RB_HSPI_NUM_MIS` reader - rx and tx sequence number mismatch"]
pub type RB_HSPI_NUM_MIS_R = crate::BitReader<bool>;
#[doc = "Field `RB_HSPI_NUM_MIS` writer - rx and tx sequence number mismatch"]
pub type RB_HSPI_NUM_MIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_HSPI_RTX_STATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - CRC error occur"]
    #[inline(always)]
    pub fn rb_hspi_crc_err(&self) -> RB_HSPI_CRC_ERR_R {
        RB_HSPI_CRC_ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - rx and tx sequence number mismatch"]
    #[inline(always)]
    pub fn rb_hspi_num_mis(&self) -> RB_HSPI_NUM_MIS_R {
        RB_HSPI_NUM_MIS_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - CRC error occur"]
    #[inline(always)]
    pub fn rb_hspi_crc_err(&mut self) -> RB_HSPI_CRC_ERR_W<1> {
        RB_HSPI_CRC_ERR_W::new(self)
    }
    #[doc = "Bit 2 - rx and tx sequence number mismatch"]
    #[inline(always)]
    pub fn rb_hspi_num_mis(&mut self) -> RB_HSPI_NUM_MIS_W<2> {
        RB_HSPI_NUM_MIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "parallel rtx status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_hspi_rtx_status](index.html) module"]
pub struct R8_HSPI_RTX_STATUS_SPEC;
impl crate::RegisterSpec for R8_HSPI_RTX_STATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_hspi_rtx_status::R](R) reader structure"]
impl crate::Readable for R8_HSPI_RTX_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_hspi_rtx_status::W](W) writer structure"]
impl crate::Writable for R8_HSPI_RTX_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_HSPI_RTX_STATUS to value 0"]
impl crate::Resettable for R8_HSPI_RTX_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
