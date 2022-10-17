#[doc = "Register `R8_HSPI_BURST_CNT` reader"]
pub struct R(crate::R<R8_HSPI_BURST_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_HSPI_BURST_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_HSPI_BURST_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_HSPI_BURST_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_HSPI_BURST_CNT` writer"]
pub struct W(crate::W<R8_HSPI_BURST_CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_HSPI_BURST_CNT_SPEC>;
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
impl From<crate::W<R8_HSPI_BURST_CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_HSPI_BURST_CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_HSPI_BURST_CNT` reader - parallel if tx burst count"]
pub type RB_HSPI_BURST_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RB_HSPI_BURST_CNT` writer - parallel if tx burst count"]
pub type RB_HSPI_BURST_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, R8_HSPI_BURST_CNT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - parallel if tx burst count"]
    #[inline(always)]
    pub fn rb_hspi_burst_cnt(&self) -> RB_HSPI_BURST_CNT_R {
        RB_HSPI_BURST_CNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - parallel if tx burst count"]
    #[inline(always)]
    pub fn rb_hspi_burst_cnt(&mut self) -> RB_HSPI_BURST_CNT_W<0> {
        RB_HSPI_BURST_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "parallel if tx burst count\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_hspi_burst_cnt](index.html) module"]
pub struct R8_HSPI_BURST_CNT_SPEC;
impl crate::RegisterSpec for R8_HSPI_BURST_CNT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_hspi_burst_cnt::R](R) reader structure"]
impl crate::Readable for R8_HSPI_BURST_CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_hspi_burst_cnt::W](W) writer structure"]
impl crate::Writable for R8_HSPI_BURST_CNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_HSPI_BURST_CNT to value 0"]
impl crate::Resettable for R8_HSPI_BURST_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
