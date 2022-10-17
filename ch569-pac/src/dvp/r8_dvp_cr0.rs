#[doc = "Register `R8_DVP_CR0` reader"]
pub struct R(crate::R<R8_DVP_CR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_DVP_CR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_DVP_CR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_DVP_CR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_DVP_CR0` writer"]
pub struct W(crate::W<R8_DVP_CR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_DVP_CR0_SPEC>;
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
impl From<crate::W<R8_DVP_CR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_DVP_CR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_DVP_ENABLE` reader - DVP enable"]
pub type RB_DVP_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `RB_DVP_ENABLE` writer - DVP enable"]
pub type RB_DVP_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_DVP_CR0_SPEC, bool, O>;
#[doc = "Field `RB_DVP_V_POLAR` reader - DVP VSYNC polarity control"]
pub type RB_DVP_V_POLAR_R = crate::BitReader<bool>;
#[doc = "Field `RB_DVP_V_POLAR` writer - DVP VSYNC polarity control"]
pub type RB_DVP_V_POLAR_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_DVP_CR0_SPEC, bool, O>;
#[doc = "Field `RB_DVP_H_POLAR` reader - DVP HSYNC polarity control"]
pub type RB_DVP_H_POLAR_R = crate::BitReader<bool>;
#[doc = "Field `RB_DVP_H_POLAR` writer - DVP HSYNC polarity control"]
pub type RB_DVP_H_POLAR_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_DVP_CR0_SPEC, bool, O>;
#[doc = "Field `RB_DVP_P_POLAR` reader - DVP PCLK polarity control"]
pub type RB_DVP_P_POLAR_R = crate::BitReader<bool>;
#[doc = "Field `RB_DVP_P_POLAR` writer - DVP PCLK polarity control"]
pub type RB_DVP_P_POLAR_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_DVP_CR0_SPEC, bool, O>;
#[doc = "Field `RB_DVP_MSK_DAT_MOD` reader - DVP data bit width confguration"]
pub type RB_DVP_MSK_DAT_MOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RB_DVP_MSK_DAT_MOD` writer - DVP data bit width confguration"]
pub type RB_DVP_MSK_DAT_MOD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, R8_DVP_CR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `RB_DVP_JPEG` reader - DVP JPEG mode"]
pub type RB_DVP_JPEG_R = crate::BitReader<bool>;
#[doc = "Field `RB_DVP_JPEG` writer - DVP JPEG mode"]
pub type RB_DVP_JPEG_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_DVP_CR0_SPEC, bool, O>;
#[doc = "Field `RB_DVP_RAW_CM` reader - DVP row count mode"]
pub type RB_DVP_RAW_CM_R = crate::BitReader<bool>;
#[doc = "Field `RB_DVP_RAW_CM` writer - DVP row count mode"]
pub type RB_DVP_RAW_CM_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_DVP_CR0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DVP enable"]
    #[inline(always)]
    pub fn rb_dvp_enable(&self) -> RB_DVP_ENABLE_R {
        RB_DVP_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DVP VSYNC polarity control"]
    #[inline(always)]
    pub fn rb_dvp_v_polar(&self) -> RB_DVP_V_POLAR_R {
        RB_DVP_V_POLAR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DVP HSYNC polarity control"]
    #[inline(always)]
    pub fn rb_dvp_h_polar(&self) -> RB_DVP_H_POLAR_R {
        RB_DVP_H_POLAR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DVP PCLK polarity control"]
    #[inline(always)]
    pub fn rb_dvp_p_polar(&self) -> RB_DVP_P_POLAR_R {
        RB_DVP_P_POLAR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - DVP data bit width confguration"]
    #[inline(always)]
    pub fn rb_dvp_msk_dat_mod(&self) -> RB_DVP_MSK_DAT_MOD_R {
        RB_DVP_MSK_DAT_MOD_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - DVP JPEG mode"]
    #[inline(always)]
    pub fn rb_dvp_jpeg(&self) -> RB_DVP_JPEG_R {
        RB_DVP_JPEG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DVP row count mode"]
    #[inline(always)]
    pub fn rb_dvp_raw_cm(&self) -> RB_DVP_RAW_CM_R {
        RB_DVP_RAW_CM_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DVP enable"]
    #[inline(always)]
    pub fn rb_dvp_enable(&mut self) -> RB_DVP_ENABLE_W<0> {
        RB_DVP_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - DVP VSYNC polarity control"]
    #[inline(always)]
    pub fn rb_dvp_v_polar(&mut self) -> RB_DVP_V_POLAR_W<1> {
        RB_DVP_V_POLAR_W::new(self)
    }
    #[doc = "Bit 2 - DVP HSYNC polarity control"]
    #[inline(always)]
    pub fn rb_dvp_h_polar(&mut self) -> RB_DVP_H_POLAR_W<2> {
        RB_DVP_H_POLAR_W::new(self)
    }
    #[doc = "Bit 3 - DVP PCLK polarity control"]
    #[inline(always)]
    pub fn rb_dvp_p_polar(&mut self) -> RB_DVP_P_POLAR_W<3> {
        RB_DVP_P_POLAR_W::new(self)
    }
    #[doc = "Bits 4:5 - DVP data bit width confguration"]
    #[inline(always)]
    pub fn rb_dvp_msk_dat_mod(&mut self) -> RB_DVP_MSK_DAT_MOD_W<4> {
        RB_DVP_MSK_DAT_MOD_W::new(self)
    }
    #[doc = "Bit 6 - DVP JPEG mode"]
    #[inline(always)]
    pub fn rb_dvp_jpeg(&mut self) -> RB_DVP_JPEG_W<6> {
        RB_DVP_JPEG_W::new(self)
    }
    #[doc = "Bit 7 - DVP row count mode"]
    #[inline(always)]
    pub fn rb_dvp_raw_cm(&mut self) -> RB_DVP_RAW_CM_W<7> {
        RB_DVP_RAW_CM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DVP control register0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_dvp_cr0](index.html) module"]
pub struct R8_DVP_CR0_SPEC;
impl crate::RegisterSpec for R8_DVP_CR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_dvp_cr0::R](R) reader structure"]
impl crate::Readable for R8_DVP_CR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_dvp_cr0::W](W) writer structure"]
impl crate::Writable for R8_DVP_CR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_DVP_CR0 to value 0"]
impl crate::Resettable for R8_DVP_CR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
