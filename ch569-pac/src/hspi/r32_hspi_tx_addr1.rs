#[doc = "Register `R32_HSPI_TX_ADDR1` reader"]
pub struct R(crate::R<R32_HSPI_TX_ADDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R32_HSPI_TX_ADDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R32_HSPI_TX_ADDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R32_HSPI_TX_ADDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R32_HSPI_TX_ADDR1` writer"]
pub struct W(crate::W<R32_HSPI_TX_ADDR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R32_HSPI_TX_ADDR1_SPEC>;
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
impl From<crate::W<R32_HSPI_TX_ADDR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R32_HSPI_TX_ADDR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_HSPI_TX_ADDR1` reader - parallel if dma tx addr1"]
pub type RB_HSPI_TX_ADDR1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RB_HSPI_TX_ADDR1` writer - parallel if dma tx addr1"]
pub type RB_HSPI_TX_ADDR1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, R32_HSPI_TX_ADDR1_SPEC, u32, u32, 17, O>;
impl R {
    #[doc = "Bits 0:16 - parallel if dma tx addr1"]
    #[inline(always)]
    pub fn rb_hspi_tx_addr1(&self) -> RB_HSPI_TX_ADDR1_R {
        RB_HSPI_TX_ADDR1_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:16 - parallel if dma tx addr1"]
    #[inline(always)]
    pub fn rb_hspi_tx_addr1(&mut self) -> RB_HSPI_TX_ADDR1_W<0> {
        RB_HSPI_TX_ADDR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "parallel if dma tx addr1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r32_hspi_tx_addr1](index.html) module"]
pub struct R32_HSPI_TX_ADDR1_SPEC;
impl crate::RegisterSpec for R32_HSPI_TX_ADDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r32_hspi_tx_addr1::R](R) reader structure"]
impl crate::Readable for R32_HSPI_TX_ADDR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r32_hspi_tx_addr1::W](W) writer structure"]
impl crate::Writable for R32_HSPI_TX_ADDR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R32_HSPI_TX_ADDR1 to value 0"]
impl crate::Resettable for R32_HSPI_TX_ADDR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
