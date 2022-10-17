#[doc = "Register `R8_TMR1_CTRL_MOD` reader"]
pub struct R(crate::R<R8_TMR1_CTRL_MOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_TMR1_CTRL_MOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_TMR1_CTRL_MOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_TMR1_CTRL_MOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_TMR1_CTRL_MOD` writer"]
pub struct W(crate::W<R8_TMR1_CTRL_MOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_TMR1_CTRL_MOD_SPEC>;
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
impl From<crate::W<R8_TMR1_CTRL_MOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_TMR1_CTRL_MOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_TMR_MODE_IN` reader - timer in mode"]
pub type RB_TMR_MODE_IN_R = crate::BitReader<bool>;
#[doc = "Field `RB_TMR_MODE_IN` writer - timer in mode"]
pub type RB_TMR_MODE_IN_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_TMR1_CTRL_MOD_SPEC, bool, O>;
#[doc = "Field `RB_TMR_ALL_CLEAR` reader - force clear timer FIFO and count"]
pub type RB_TMR_ALL_CLEAR_R = crate::BitReader<bool>;
#[doc = "Field `RB_TMR_ALL_CLEAR` writer - force clear timer FIFO and count"]
pub type RB_TMR_ALL_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_TMR1_CTRL_MOD_SPEC, bool, O>;
#[doc = "Field `RB_TMR_COUNT_EN` reader - timer count enable"]
pub type RB_TMR_COUNT_EN_R = crate::BitReader<bool>;
#[doc = "Field `RB_TMR_COUNT_EN` writer - timer count enable"]
pub type RB_TMR_COUNT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_TMR1_CTRL_MOD_SPEC, bool, O>;
#[doc = "Field `RB_TMR_OUT_EN` reader - timer output enable"]
pub type RB_TMR_OUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `RB_TMR_OUT_EN` writer - timer output enable"]
pub type RB_TMR_OUT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_TMR1_CTRL_MOD_SPEC, bool, O>;
#[doc = "Field `RB_TMR_OUT_POLAR_RB_TMR_CAP_COUNT` reader - timer PWM output polarity _ Count sub-mode"]
pub type RB_TMR_OUT_POLAR_RB_TMR_CAP_COUNT_R = crate::BitReader<bool>;
#[doc = "Field `RB_TMR_OUT_POLAR_RB_TMR_CAP_COUNT` writer - timer PWM output polarity _ Count sub-mode"]
pub type RB_TMR_OUT_POLAR_RB_TMR_CAP_COUNT_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_TMR1_CTRL_MOD_SPEC, bool, O>;
#[doc = "Field `RB_TMR_PWM_REPEAT_RB_TMR_CAP_EDGE` reader - timer PWM repeat mode _ timer capture edge mode"]
pub type RB_TMR_PWM_REPEAT_RB_TMR_CAP_EDGE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RB_TMR_PWM_REPEAT_RB_TMR_CAP_EDGE` writer - timer PWM repeat mode _ timer capture edge mode"]
pub type RB_TMR_PWM_REPEAT_RB_TMR_CAP_EDGE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, R8_TMR1_CTRL_MOD_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - timer in mode"]
    #[inline(always)]
    pub fn rb_tmr_mode_in(&self) -> RB_TMR_MODE_IN_R {
        RB_TMR_MODE_IN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - force clear timer FIFO and count"]
    #[inline(always)]
    pub fn rb_tmr_all_clear(&self) -> RB_TMR_ALL_CLEAR_R {
        RB_TMR_ALL_CLEAR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - timer count enable"]
    #[inline(always)]
    pub fn rb_tmr_count_en(&self) -> RB_TMR_COUNT_EN_R {
        RB_TMR_COUNT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - timer output enable"]
    #[inline(always)]
    pub fn rb_tmr_out_en(&self) -> RB_TMR_OUT_EN_R {
        RB_TMR_OUT_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - timer PWM output polarity _ Count sub-mode"]
    #[inline(always)]
    pub fn rb_tmr_out_polar_rb_tmr_cap_count(&self) -> RB_TMR_OUT_POLAR_RB_TMR_CAP_COUNT_R {
        RB_TMR_OUT_POLAR_RB_TMR_CAP_COUNT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 6:7 - timer PWM repeat mode _ timer capture edge mode"]
    #[inline(always)]
    pub fn rb_tmr_pwm_repeat_rb_tmr_cap_edge(&self) -> RB_TMR_PWM_REPEAT_RB_TMR_CAP_EDGE_R {
        RB_TMR_PWM_REPEAT_RB_TMR_CAP_EDGE_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - timer in mode"]
    #[inline(always)]
    pub fn rb_tmr_mode_in(&mut self) -> RB_TMR_MODE_IN_W<0> {
        RB_TMR_MODE_IN_W::new(self)
    }
    #[doc = "Bit 1 - force clear timer FIFO and count"]
    #[inline(always)]
    pub fn rb_tmr_all_clear(&mut self) -> RB_TMR_ALL_CLEAR_W<1> {
        RB_TMR_ALL_CLEAR_W::new(self)
    }
    #[doc = "Bit 2 - timer count enable"]
    #[inline(always)]
    pub fn rb_tmr_count_en(&mut self) -> RB_TMR_COUNT_EN_W<2> {
        RB_TMR_COUNT_EN_W::new(self)
    }
    #[doc = "Bit 3 - timer output enable"]
    #[inline(always)]
    pub fn rb_tmr_out_en(&mut self) -> RB_TMR_OUT_EN_W<3> {
        RB_TMR_OUT_EN_W::new(self)
    }
    #[doc = "Bit 4 - timer PWM output polarity _ Count sub-mode"]
    #[inline(always)]
    pub fn rb_tmr_out_polar_rb_tmr_cap_count(&mut self) -> RB_TMR_OUT_POLAR_RB_TMR_CAP_COUNT_W<4> {
        RB_TMR_OUT_POLAR_RB_TMR_CAP_COUNT_W::new(self)
    }
    #[doc = "Bits 6:7 - timer PWM repeat mode _ timer capture edge mode"]
    #[inline(always)]
    pub fn rb_tmr_pwm_repeat_rb_tmr_cap_edge(&mut self) -> RB_TMR_PWM_REPEAT_RB_TMR_CAP_EDGE_W<6> {
        RB_TMR_PWM_REPEAT_RB_TMR_CAP_EDGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TMR1 mode control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_tmr1_ctrl_mod](index.html) module"]
pub struct R8_TMR1_CTRL_MOD_SPEC;
impl crate::RegisterSpec for R8_TMR1_CTRL_MOD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_tmr1_ctrl_mod::R](R) reader structure"]
impl crate::Readable for R8_TMR1_CTRL_MOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_tmr1_ctrl_mod::W](W) writer structure"]
impl crate::Writable for R8_TMR1_CTRL_MOD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_TMR1_CTRL_MOD to value 0x02"]
impl crate::Resettable for R8_TMR1_CTRL_MOD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
