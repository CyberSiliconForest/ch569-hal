#[doc = "Register `R8_TMR1_INT_FLAG` reader"]
pub struct R(crate::R<R8_TMR1_INT_FLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_TMR1_INT_FLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_TMR1_INT_FLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_TMR1_INT_FLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_TMR1_INT_FLAG` writer"]
pub struct W(crate::W<R8_TMR1_INT_FLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_TMR1_INT_FLAG_SPEC>;
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
impl From<crate::W<R8_TMR1_INT_FLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_TMR1_INT_FLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_TMR_IF_CYC_END` reader - interrupt flag for timer capture count timeout or PWM cycle end"]
pub type RB_TMR_IF_CYC_END_R = crate::BitReader<bool>;
#[doc = "Field `RB_TMR_IF_CYC_END` writer - interrupt flag for timer capture count timeout or PWM cycle end"]
pub type RB_TMR_IF_CYC_END_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_TMR1_INT_FLAG_SPEC, bool, O>;
#[doc = "Field `RB_TMR_IF_DATA_ACT` reader - interrupt flag for timer capture input action or PWM trigger"]
pub type RB_TMR_IF_DATA_ACT_R = crate::BitReader<bool>;
#[doc = "Field `RB_TMR_IF_DATA_ACT` writer - interrupt flag for timer capture input action or PWM trigger"]
pub type RB_TMR_IF_DATA_ACT_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_TMR1_INT_FLAG_SPEC, bool, O>;
#[doc = "Field `RB_TMR_IF_FIFO_HF` reader - interrupt flag for timer FIFO half (capture fifo >=4 or PWM fifo lower than 3)"]
pub type RB_TMR_IF_FIFO_HF_R = crate::BitReader<bool>;
#[doc = "Field `RB_TMR_IF_FIFO_HF` writer - interrupt flag for timer FIFO half (capture fifo >=4 or PWM fifo lower than 3)"]
pub type RB_TMR_IF_FIFO_HF_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_TMR1_INT_FLAG_SPEC, bool, O>;
#[doc = "Field `RB_TMR_IF_DMA_END` reader - interrupt flag for timer1_2 DMA completion"]
pub type RB_TMR_IF_DMA_END_R = crate::BitReader<bool>;
#[doc = "Field `RB_TMR_IF_DMA_END` writer - interrupt flag for timer1_2 DMA completion"]
pub type RB_TMR_IF_DMA_END_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_TMR1_INT_FLAG_SPEC, bool, O>;
#[doc = "Field `RB_TMR_IF_FIFO_OV` reader - interrupt flag for timer FIFO overflow"]
pub type RB_TMR_IF_FIFO_OV_R = crate::BitReader<bool>;
#[doc = "Field `RB_TMR_IF_FIFO_OV` writer - interrupt flag for timer FIFO overflow"]
pub type RB_TMR_IF_FIFO_OV_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_TMR1_INT_FLAG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - interrupt flag for timer capture count timeout or PWM cycle end"]
    #[inline(always)]
    pub fn rb_tmr_if_cyc_end(&self) -> RB_TMR_IF_CYC_END_R {
        RB_TMR_IF_CYC_END_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - interrupt flag for timer capture input action or PWM trigger"]
    #[inline(always)]
    pub fn rb_tmr_if_data_act(&self) -> RB_TMR_IF_DATA_ACT_R {
        RB_TMR_IF_DATA_ACT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - interrupt flag for timer FIFO half (capture fifo >=4 or PWM fifo lower than 3)"]
    #[inline(always)]
    pub fn rb_tmr_if_fifo_hf(&self) -> RB_TMR_IF_FIFO_HF_R {
        RB_TMR_IF_FIFO_HF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - interrupt flag for timer1_2 DMA completion"]
    #[inline(always)]
    pub fn rb_tmr_if_dma_end(&self) -> RB_TMR_IF_DMA_END_R {
        RB_TMR_IF_DMA_END_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - interrupt flag for timer FIFO overflow"]
    #[inline(always)]
    pub fn rb_tmr_if_fifo_ov(&self) -> RB_TMR_IF_FIFO_OV_R {
        RB_TMR_IF_FIFO_OV_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - interrupt flag for timer capture count timeout or PWM cycle end"]
    #[inline(always)]
    pub fn rb_tmr_if_cyc_end(&mut self) -> RB_TMR_IF_CYC_END_W<0> {
        RB_TMR_IF_CYC_END_W::new(self)
    }
    #[doc = "Bit 1 - interrupt flag for timer capture input action or PWM trigger"]
    #[inline(always)]
    pub fn rb_tmr_if_data_act(&mut self) -> RB_TMR_IF_DATA_ACT_W<1> {
        RB_TMR_IF_DATA_ACT_W::new(self)
    }
    #[doc = "Bit 2 - interrupt flag for timer FIFO half (capture fifo >=4 or PWM fifo lower than 3)"]
    #[inline(always)]
    pub fn rb_tmr_if_fifo_hf(&mut self) -> RB_TMR_IF_FIFO_HF_W<2> {
        RB_TMR_IF_FIFO_HF_W::new(self)
    }
    #[doc = "Bit 3 - interrupt flag for timer1_2 DMA completion"]
    #[inline(always)]
    pub fn rb_tmr_if_dma_end(&mut self) -> RB_TMR_IF_DMA_END_W<3> {
        RB_TMR_IF_DMA_END_W::new(self)
    }
    #[doc = "Bit 4 - interrupt flag for timer FIFO overflow"]
    #[inline(always)]
    pub fn rb_tmr_if_fifo_ov(&mut self) -> RB_TMR_IF_FIFO_OV_W<4> {
        RB_TMR_IF_FIFO_OV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TMR1 interrupt flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_tmr1_int_flag](index.html) module"]
pub struct R8_TMR1_INT_FLAG_SPEC;
impl crate::RegisterSpec for R8_TMR1_INT_FLAG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_tmr1_int_flag::R](R) reader structure"]
impl crate::Readable for R8_TMR1_INT_FLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_tmr1_int_flag::W](W) writer structure"]
impl crate::Writable for R8_TMR1_INT_FLAG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_TMR1_INT_FLAG to value 0"]
impl crate::Resettable for R8_TMR1_INT_FLAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
