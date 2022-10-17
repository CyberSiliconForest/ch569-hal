#[doc = "Register `R16_HSPI_BURST_CFG` reader"]
pub struct R(crate::R<R16_HSPI_BURST_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R16_HSPI_BURST_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R16_HSPI_BURST_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R16_HSPI_BURST_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R16_HSPI_BURST_CFG` writer"]
pub struct W(crate::W<R16_HSPI_BURST_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R16_HSPI_BURST_CFG_SPEC>;
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
impl From<crate::W<R16_HSPI_BURST_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R16_HSPI_BURST_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_HSPI_BURST_EN` reader - burst transmit enable"]
pub type RB_HSPI_BURST_EN_R = crate::BitReader<bool>;
#[doc = "Field `RB_HSPI_BURST_EN` writer - burst transmit enable"]
pub type RB_HSPI_BURST_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, R16_HSPI_BURST_CFG_SPEC, bool, O>;
#[doc = "Field `RB_HSPI_BURST_LEN` reader - burst transmit length"]
pub type RB_HSPI_BURST_LEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RB_HSPI_BURST_LEN` writer - burst transmit length"]
pub type RB_HSPI_BURST_LEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, R16_HSPI_BURST_CFG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - burst transmit enable"]
    #[inline(always)]
    pub fn rb_hspi_burst_en(&self) -> RB_HSPI_BURST_EN_R {
        RB_HSPI_BURST_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - burst transmit length"]
    #[inline(always)]
    pub fn rb_hspi_burst_len(&self) -> RB_HSPI_BURST_LEN_R {
        RB_HSPI_BURST_LEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - burst transmit enable"]
    #[inline(always)]
    pub fn rb_hspi_burst_en(&mut self) -> RB_HSPI_BURST_EN_W<0> {
        RB_HSPI_BURST_EN_W::new(self)
    }
    #[doc = "Bits 8:15 - burst transmit length"]
    #[inline(always)]
    pub fn rb_hspi_burst_len(&mut self) -> RB_HSPI_BURST_LEN_W<8> {
        RB_HSPI_BURST_LEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "parallel if tx burst config register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r16_hspi_burst_cfg](index.html) module"]
pub struct R16_HSPI_BURST_CFG_SPEC;
impl crate::RegisterSpec for R16_HSPI_BURST_CFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [r16_hspi_burst_cfg::R](R) reader structure"]
impl crate::Readable for R16_HSPI_BURST_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r16_hspi_burst_cfg::W](W) writer structure"]
impl crate::Writable for R16_HSPI_BURST_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R16_HSPI_BURST_CFG to value 0"]
impl crate::Resettable for R16_HSPI_BURST_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
