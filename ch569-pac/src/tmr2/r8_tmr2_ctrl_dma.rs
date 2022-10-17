#[doc = "Register `R8_TMR2_CTRL_DMA` reader"]
pub struct R(crate::R<R8_TMR2_CTRL_DMA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_TMR2_CTRL_DMA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_TMR2_CTRL_DMA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_TMR2_CTRL_DMA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_TMR2_CTRL_DMA` writer"]
pub struct W(crate::W<R8_TMR2_CTRL_DMA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_TMR2_CTRL_DMA_SPEC>;
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
impl From<crate::W<R8_TMR2_CTRL_DMA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_TMR2_CTRL_DMA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_TMR_DMA_ENABLE` reader - timer1_2 DMA enable"]
pub type RB_TMR_DMA_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `RB_TMR_DMA_ENABLE` writer - timer1_2 DMA enable"]
pub type RB_TMR_DMA_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_TMR2_CTRL_DMA_SPEC, bool, O>;
#[doc = "Field `RB_TMR_DMA_LOOP` reader - timer1_2 DMA address loop enable"]
pub type RB_TMR_DMA_LOOP_R = crate::BitReader<bool>;
#[doc = "Field `RB_TMR_DMA_LOOP` writer - timer1_2 DMA address loop enable"]
pub type RB_TMR_DMA_LOOP_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_TMR2_CTRL_DMA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - timer1_2 DMA enable"]
    #[inline(always)]
    pub fn rb_tmr_dma_enable(&self) -> RB_TMR_DMA_ENABLE_R {
        RB_TMR_DMA_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - timer1_2 DMA address loop enable"]
    #[inline(always)]
    pub fn rb_tmr_dma_loop(&self) -> RB_TMR_DMA_LOOP_R {
        RB_TMR_DMA_LOOP_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - timer1_2 DMA enable"]
    #[inline(always)]
    pub fn rb_tmr_dma_enable(&mut self) -> RB_TMR_DMA_ENABLE_W<0> {
        RB_TMR_DMA_ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - timer1_2 DMA address loop enable"]
    #[inline(always)]
    pub fn rb_tmr_dma_loop(&mut self) -> RB_TMR_DMA_LOOP_W<2> {
        RB_TMR_DMA_LOOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TMR2 DMA control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_tmr2_ctrl_dma](index.html) module"]
pub struct R8_TMR2_CTRL_DMA_SPEC;
impl crate::RegisterSpec for R8_TMR2_CTRL_DMA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_tmr2_ctrl_dma::R](R) reader structure"]
impl crate::Readable for R8_TMR2_CTRL_DMA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_tmr2_ctrl_dma::W](W) writer structure"]
impl crate::Writable for R8_TMR2_CTRL_DMA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_TMR2_CTRL_DMA to value 0"]
impl crate::Resettable for R8_TMR2_CTRL_DMA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
