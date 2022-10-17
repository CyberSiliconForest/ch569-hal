#[doc = "Register `R8_SPI0_INT_FLAG` reader"]
pub struct R(crate::R<R8_SPI0_INT_FLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_SPI0_INT_FLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_SPI0_INT_FLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_SPI0_INT_FLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_SPI0_INT_FLAG` writer"]
pub struct W(crate::W<R8_SPI0_INT_FLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_SPI0_INT_FLAG_SPEC>;
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
impl From<crate::W<R8_SPI0_INT_FLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_SPI0_INT_FLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_SPI_IF_CNT_END` reader - interrupt flag for SPI total byte count end"]
pub type RB_SPI_IF_CNT_END_R = crate::BitReader<bool>;
#[doc = "Field `RB_SPI_IF_CNT_END` writer - interrupt flag for SPI total byte count end"]
pub type RB_SPI_IF_CNT_END_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SPI0_INT_FLAG_SPEC, bool, O>;
#[doc = "Field `RB_SPI_IF_BYTE_END` reader - interrupt flag for SPI byte exchanged"]
pub type RB_SPI_IF_BYTE_END_R = crate::BitReader<bool>;
#[doc = "Field `RB_SPI_IF_BYTE_END` writer - interrupt flag for SPI byte exchanged"]
pub type RB_SPI_IF_BYTE_END_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SPI0_INT_FLAG_SPEC, bool, O>;
#[doc = "Field `RB_SPI_IF_FIFO_HF` reader - interrupt flag for SPI FIFO half"]
pub type RB_SPI_IF_FIFO_HF_R = crate::BitReader<bool>;
#[doc = "Field `RB_SPI_IF_FIFO_HF` writer - interrupt flag for SPI FIFO half"]
pub type RB_SPI_IF_FIFO_HF_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SPI0_INT_FLAG_SPEC, bool, O>;
#[doc = "Field `RB_SPI_IF_DMA_END` reader - interrupt flag for SPI DMA completion"]
pub type RB_SPI_IF_DMA_END_R = crate::BitReader<bool>;
#[doc = "Field `RB_SPI_IF_DMA_END` writer - interrupt flag for SPI DMA completion"]
pub type RB_SPI_IF_DMA_END_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SPI0_INT_FLAG_SPEC, bool, O>;
#[doc = "Field `RB_SPI_IF_FIFO_OV` reader - interrupt flag for SPI FIFO overflow"]
pub type RB_SPI_IF_FIFO_OV_R = crate::BitReader<bool>;
#[doc = "Field `RB_SPI_IF_FIFO_OV` writer - interrupt flag for SPI FIFO overflow"]
pub type RB_SPI_IF_FIFO_OV_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SPI0_INT_FLAG_SPEC, bool, O>;
#[doc = "Field `RB_SPI_FREE` reader - current SPI free status"]
pub type RB_SPI_FREE_R = crate::BitReader<bool>;
#[doc = "Field `RB_SPI_FREE` writer - current SPI free status"]
pub type RB_SPI_FREE_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_SPI0_INT_FLAG_SPEC, bool, O>;
#[doc = "Field `RB_SPI_IF_FST_BYTE` reader - interrupt flag for SPI slave mode first byte received"]
pub type RB_SPI_IF_FST_BYTE_R = crate::BitReader<bool>;
#[doc = "Field `RB_SPI_IF_FST_BYTE` writer - interrupt flag for SPI slave mode first byte received"]
pub type RB_SPI_IF_FST_BYTE_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SPI0_INT_FLAG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - interrupt flag for SPI total byte count end"]
    #[inline(always)]
    pub fn rb_spi_if_cnt_end(&self) -> RB_SPI_IF_CNT_END_R {
        RB_SPI_IF_CNT_END_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - interrupt flag for SPI byte exchanged"]
    #[inline(always)]
    pub fn rb_spi_if_byte_end(&self) -> RB_SPI_IF_BYTE_END_R {
        RB_SPI_IF_BYTE_END_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - interrupt flag for SPI FIFO half"]
    #[inline(always)]
    pub fn rb_spi_if_fifo_hf(&self) -> RB_SPI_IF_FIFO_HF_R {
        RB_SPI_IF_FIFO_HF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - interrupt flag for SPI DMA completion"]
    #[inline(always)]
    pub fn rb_spi_if_dma_end(&self) -> RB_SPI_IF_DMA_END_R {
        RB_SPI_IF_DMA_END_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - interrupt flag for SPI FIFO overflow"]
    #[inline(always)]
    pub fn rb_spi_if_fifo_ov(&self) -> RB_SPI_IF_FIFO_OV_R {
        RB_SPI_IF_FIFO_OV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - current SPI free status"]
    #[inline(always)]
    pub fn rb_spi_free(&self) -> RB_SPI_FREE_R {
        RB_SPI_FREE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - interrupt flag for SPI slave mode first byte received"]
    #[inline(always)]
    pub fn rb_spi_if_fst_byte(&self) -> RB_SPI_IF_FST_BYTE_R {
        RB_SPI_IF_FST_BYTE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - interrupt flag for SPI total byte count end"]
    #[inline(always)]
    pub fn rb_spi_if_cnt_end(&mut self) -> RB_SPI_IF_CNT_END_W<0> {
        RB_SPI_IF_CNT_END_W::new(self)
    }
    #[doc = "Bit 1 - interrupt flag for SPI byte exchanged"]
    #[inline(always)]
    pub fn rb_spi_if_byte_end(&mut self) -> RB_SPI_IF_BYTE_END_W<1> {
        RB_SPI_IF_BYTE_END_W::new(self)
    }
    #[doc = "Bit 2 - interrupt flag for SPI FIFO half"]
    #[inline(always)]
    pub fn rb_spi_if_fifo_hf(&mut self) -> RB_SPI_IF_FIFO_HF_W<2> {
        RB_SPI_IF_FIFO_HF_W::new(self)
    }
    #[doc = "Bit 3 - interrupt flag for SPI DMA completion"]
    #[inline(always)]
    pub fn rb_spi_if_dma_end(&mut self) -> RB_SPI_IF_DMA_END_W<3> {
        RB_SPI_IF_DMA_END_W::new(self)
    }
    #[doc = "Bit 4 - interrupt flag for SPI FIFO overflow"]
    #[inline(always)]
    pub fn rb_spi_if_fifo_ov(&mut self) -> RB_SPI_IF_FIFO_OV_W<4> {
        RB_SPI_IF_FIFO_OV_W::new(self)
    }
    #[doc = "Bit 6 - current SPI free status"]
    #[inline(always)]
    pub fn rb_spi_free(&mut self) -> RB_SPI_FREE_W<6> {
        RB_SPI_FREE_W::new(self)
    }
    #[doc = "Bit 7 - interrupt flag for SPI slave mode first byte received"]
    #[inline(always)]
    pub fn rb_spi_if_fst_byte(&mut self) -> RB_SPI_IF_FST_BYTE_W<7> {
        RB_SPI_IF_FST_BYTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 interrupt flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_spi0_int_flag](index.html) module"]
pub struct R8_SPI0_INT_FLAG_SPEC;
impl crate::RegisterSpec for R8_SPI0_INT_FLAG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_spi0_int_flag::R](R) reader structure"]
impl crate::Readable for R8_SPI0_INT_FLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_spi0_int_flag::W](W) writer structure"]
impl crate::Writable for R8_SPI0_INT_FLAG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_SPI0_INT_FLAG to value 0"]
impl crate::Resettable for R8_SPI0_INT_FLAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
