#[doc = "Register `R8_DVP_INT_EN` reader"]
pub struct R(crate::R<R8_DVP_INT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_DVP_INT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_DVP_INT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_DVP_INT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_DVP_INT_EN` writer"]
pub struct W(crate::W<R8_DVP_INT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_DVP_INT_EN_SPEC>;
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
impl From<crate::W<R8_DVP_INT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_DVP_INT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_DVP_IE_STR_FRM` reader - DVP frame start interrupt enable"]
pub type RB_DVP_IE_STR_FRM_R = crate::BitReader<bool>;
#[doc = "Field `RB_DVP_IE_STR_FRM` writer - DVP frame start interrupt enable"]
pub type RB_DVP_IE_STR_FRM_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_DVP_INT_EN_SPEC, bool, O>;
#[doc = "Field `RB_DVP_IE_ROW_DONE` reader - DVP row received done interrupt enable"]
pub type RB_DVP_IE_ROW_DONE_R = crate::BitReader<bool>;
#[doc = "Field `RB_DVP_IE_ROW_DONE` writer - DVP row received done interrupt enable"]
pub type RB_DVP_IE_ROW_DONE_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_DVP_INT_EN_SPEC, bool, O>;
#[doc = "Field `RB_DVP_IE_FRM_DONE` reader - DVP frame received done interrupt enable"]
pub type RB_DVP_IE_FRM_DONE_R = crate::BitReader<bool>;
#[doc = "Field `RB_DVP_IE_FRM_DONE` writer - DVP frame received done interrupt enable"]
pub type RB_DVP_IE_FRM_DONE_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_DVP_INT_EN_SPEC, bool, O>;
#[doc = "Field `RB_DVP_IE_FIFO_OV` reader - DVP receive fifo overflow interrupt enable"]
pub type RB_DVP_IE_FIFO_OV_R = crate::BitReader<bool>;
#[doc = "Field `RB_DVP_IE_FIFO_OV` writer - DVP receive fifo overflow interrupt enable"]
pub type RB_DVP_IE_FIFO_OV_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_DVP_INT_EN_SPEC, bool, O>;
#[doc = "Field `RB_DVP_IE_STP_FRM` reader - DVP frame stop interrupt enable"]
pub type RB_DVP_IE_STP_FRM_R = crate::BitReader<bool>;
#[doc = "Field `RB_DVP_IE_STP_FRM` writer - DVP frame stop interrupt enable"]
pub type RB_DVP_IE_STP_FRM_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_DVP_INT_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DVP frame start interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_str_frm(&self) -> RB_DVP_IE_STR_FRM_R {
        RB_DVP_IE_STR_FRM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DVP row received done interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_row_done(&self) -> RB_DVP_IE_ROW_DONE_R {
        RB_DVP_IE_ROW_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DVP frame received done interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_frm_done(&self) -> RB_DVP_IE_FRM_DONE_R {
        RB_DVP_IE_FRM_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DVP receive fifo overflow interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_fifo_ov(&self) -> RB_DVP_IE_FIFO_OV_R {
        RB_DVP_IE_FIFO_OV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DVP frame stop interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_stp_frm(&self) -> RB_DVP_IE_STP_FRM_R {
        RB_DVP_IE_STP_FRM_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DVP frame start interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_str_frm(&mut self) -> RB_DVP_IE_STR_FRM_W<0> {
        RB_DVP_IE_STR_FRM_W::new(self)
    }
    #[doc = "Bit 1 - DVP row received done interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_row_done(&mut self) -> RB_DVP_IE_ROW_DONE_W<1> {
        RB_DVP_IE_ROW_DONE_W::new(self)
    }
    #[doc = "Bit 2 - DVP frame received done interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_frm_done(&mut self) -> RB_DVP_IE_FRM_DONE_W<2> {
        RB_DVP_IE_FRM_DONE_W::new(self)
    }
    #[doc = "Bit 3 - DVP receive fifo overflow interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_fifo_ov(&mut self) -> RB_DVP_IE_FIFO_OV_W<3> {
        RB_DVP_IE_FIFO_OV_W::new(self)
    }
    #[doc = "Bit 4 - DVP frame stop interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_stp_frm(&mut self) -> RB_DVP_IE_STP_FRM_W<4> {
        RB_DVP_IE_STP_FRM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DVP interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_dvp_int_en](index.html) module"]
pub struct R8_DVP_INT_EN_SPEC;
impl crate::RegisterSpec for R8_DVP_INT_EN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_dvp_int_en::R](R) reader structure"]
impl crate::Readable for R8_DVP_INT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_dvp_int_en::W](W) writer structure"]
impl crate::Writable for R8_DVP_INT_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_DVP_INT_EN to value 0"]
impl crate::Resettable for R8_DVP_INT_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
