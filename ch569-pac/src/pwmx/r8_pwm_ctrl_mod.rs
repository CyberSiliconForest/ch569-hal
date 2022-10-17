#[doc = "Register `R8_PWM_CTRL_MOD` reader"]
pub struct R(crate::R<R8_PWM_CTRL_MOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_PWM_CTRL_MOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_PWM_CTRL_MOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_PWM_CTRL_MOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_PWM_CTRL_MOD` writer"]
pub struct W(crate::W<R8_PWM_CTRL_MOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_PWM_CTRL_MOD_SPEC>;
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
impl From<crate::W<R8_PWM_CTRL_MOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_PWM_CTRL_MOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_PWM0_OUT_EN` reader - PWM0 output enable"]
pub type RB_PWM0_OUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `RB_PWM0_OUT_EN` writer - PWM0 output enable"]
pub type RB_PWM0_OUT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_PWM_CTRL_MOD_SPEC, bool, O>;
#[doc = "Field `RB_PWM1_OUT_EN` reader - PWM1 output enable"]
pub type RB_PWM1_OUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `RB_PWM1_OUT_EN` writer - PWM1 output enable"]
pub type RB_PWM1_OUT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_PWM_CTRL_MOD_SPEC, bool, O>;
#[doc = "Field `RB_PWM2_OUT_EN` reader - PWM2 output enable"]
pub type RB_PWM2_OUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `RB_PWM2_OUT_EN` writer - PWM2 output enable"]
pub type RB_PWM2_OUT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_PWM_CTRL_MOD_SPEC, bool, O>;
#[doc = "Field `RB_PWM3_OUT_EN` reader - PWM3 output enable"]
pub type RB_PWM3_OUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `RB_PWM3_OUT_EN` writer - PWM3 output enable"]
pub type RB_PWM3_OUT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_PWM_CTRL_MOD_SPEC, bool, O>;
#[doc = "Field `RB_PWM0_POLAR` reader - PWM0 output polarity"]
pub type RB_PWM0_POLAR_R = crate::BitReader<bool>;
#[doc = "Field `RB_PWM0_POLAR` writer - PWM0 output polarity"]
pub type RB_PWM0_POLAR_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_PWM_CTRL_MOD_SPEC, bool, O>;
#[doc = "Field `RB_PWM1_POLAR` reader - PWM1 output polarity"]
pub type RB_PWM1_POLAR_R = crate::BitReader<bool>;
#[doc = "Field `RB_PWM1_POLAR` writer - PWM1 output polarity"]
pub type RB_PWM1_POLAR_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_PWM_CTRL_MOD_SPEC, bool, O>;
#[doc = "Field `RB_PWM2_POLAR` reader - PWM2 output polarity"]
pub type RB_PWM2_POLAR_R = crate::BitReader<bool>;
#[doc = "Field `RB_PWM2_POLAR` writer - PWM2 output polarity"]
pub type RB_PWM2_POLAR_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_PWM_CTRL_MOD_SPEC, bool, O>;
#[doc = "Field `RB_PWM3_POLAR` reader - PWM3 output polarity"]
pub type RB_PWM3_POLAR_R = crate::BitReader<bool>;
#[doc = "Field `RB_PWM3_POLAR` writer - PWM3 output polarity"]
pub type RB_PWM3_POLAR_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_PWM_CTRL_MOD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - PWM0 output enable"]
    #[inline(always)]
    pub fn rb_pwm0_out_en(&self) -> RB_PWM0_OUT_EN_R {
        RB_PWM0_OUT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PWM1 output enable"]
    #[inline(always)]
    pub fn rb_pwm1_out_en(&self) -> RB_PWM1_OUT_EN_R {
        RB_PWM1_OUT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PWM2 output enable"]
    #[inline(always)]
    pub fn rb_pwm2_out_en(&self) -> RB_PWM2_OUT_EN_R {
        RB_PWM2_OUT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PWM3 output enable"]
    #[inline(always)]
    pub fn rb_pwm3_out_en(&self) -> RB_PWM3_OUT_EN_R {
        RB_PWM3_OUT_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PWM0 output polarity"]
    #[inline(always)]
    pub fn rb_pwm0_polar(&self) -> RB_PWM0_POLAR_R {
        RB_PWM0_POLAR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PWM1 output polarity"]
    #[inline(always)]
    pub fn rb_pwm1_polar(&self) -> RB_PWM1_POLAR_R {
        RB_PWM1_POLAR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PWM2 output polarity"]
    #[inline(always)]
    pub fn rb_pwm2_polar(&self) -> RB_PWM2_POLAR_R {
        RB_PWM2_POLAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PWM3 output polarity"]
    #[inline(always)]
    pub fn rb_pwm3_polar(&self) -> RB_PWM3_POLAR_R {
        RB_PWM3_POLAR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM0 output enable"]
    #[inline(always)]
    pub fn rb_pwm0_out_en(&mut self) -> RB_PWM0_OUT_EN_W<0> {
        RB_PWM0_OUT_EN_W::new(self)
    }
    #[doc = "Bit 1 - PWM1 output enable"]
    #[inline(always)]
    pub fn rb_pwm1_out_en(&mut self) -> RB_PWM1_OUT_EN_W<1> {
        RB_PWM1_OUT_EN_W::new(self)
    }
    #[doc = "Bit 2 - PWM2 output enable"]
    #[inline(always)]
    pub fn rb_pwm2_out_en(&mut self) -> RB_PWM2_OUT_EN_W<2> {
        RB_PWM2_OUT_EN_W::new(self)
    }
    #[doc = "Bit 3 - PWM3 output enable"]
    #[inline(always)]
    pub fn rb_pwm3_out_en(&mut self) -> RB_PWM3_OUT_EN_W<3> {
        RB_PWM3_OUT_EN_W::new(self)
    }
    #[doc = "Bit 4 - PWM0 output polarity"]
    #[inline(always)]
    pub fn rb_pwm0_polar(&mut self) -> RB_PWM0_POLAR_W<4> {
        RB_PWM0_POLAR_W::new(self)
    }
    #[doc = "Bit 5 - PWM1 output polarity"]
    #[inline(always)]
    pub fn rb_pwm1_polar(&mut self) -> RB_PWM1_POLAR_W<5> {
        RB_PWM1_POLAR_W::new(self)
    }
    #[doc = "Bit 6 - PWM2 output polarity"]
    #[inline(always)]
    pub fn rb_pwm2_polar(&mut self) -> RB_PWM2_POLAR_W<6> {
        RB_PWM2_POLAR_W::new(self)
    }
    #[doc = "Bit 7 - PWM3 output polarity"]
    #[inline(always)]
    pub fn rb_pwm3_polar(&mut self) -> RB_PWM3_POLAR_W<7> {
        RB_PWM3_POLAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM mode control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_pwm_ctrl_mod](index.html) module"]
pub struct R8_PWM_CTRL_MOD_SPEC;
impl crate::RegisterSpec for R8_PWM_CTRL_MOD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_pwm_ctrl_mod::R](R) reader structure"]
impl crate::Readable for R8_PWM_CTRL_MOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_pwm_ctrl_mod::W](W) writer structure"]
impl crate::Writable for R8_PWM_CTRL_MOD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_PWM_CTRL_MOD to value 0"]
impl crate::Resettable for R8_PWM_CTRL_MOD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
