#[doc = "Register `R16_UEP2_T_LEN_R16_UH_EP_PID` reader"]
pub struct R(crate::R<R16_UEP2_T_LEN_R16_UH_EP_PID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R16_UEP2_T_LEN_R16_UH_EP_PID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R16_UEP2_T_LEN_R16_UH_EP_PID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R16_UEP2_T_LEN_R16_UH_EP_PID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R16_UEP2_T_LEN_R16_UH_EP_PID` writer"]
pub struct W(crate::W<R16_UEP2_T_LEN_R16_UH_EP_PID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R16_UEP2_T_LEN_R16_UH_EP_PID_SPEC>;
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
impl From<crate::W<R16_UEP2_T_LEN_R16_UH_EP_PID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R16_UEP2_T_LEN_R16_UH_EP_PID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_UH_EPNUM_MASK` reader - The endpoint number of the target of this operation"]
pub type RB_UH_EPNUM_MASK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RB_UH_EPNUM_MASK` writer - The endpoint number of the target of this operation"]
pub type RB_UH_EPNUM_MASK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, R16_UEP2_T_LEN_R16_UH_EP_PID_SPEC, u8, u8, 4, O>;
#[doc = "Field `UEP2_T_LEN` reader - endpoint 2 transmittal length"]
pub type UEP2_T_LEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `UEP2_T_LEN` writer - endpoint 2 transmittal length"]
pub type UEP2_T_LEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, R16_UEP2_T_LEN_R16_UH_EP_PID_SPEC, u16, u16, 16, O>;
#[doc = "Field `RB_UH_TOKEN_MASK` reader - The token PID packet identification of this USB transfer transaction"]
pub type RB_UH_TOKEN_MASK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RB_UH_TOKEN_MASK` writer - The token PID packet identification of this USB transfer transaction"]
pub type RB_UH_TOKEN_MASK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, R16_UEP2_T_LEN_R16_UH_EP_PID_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - The endpoint number of the target of this operation"]
    #[inline(always)]
    pub fn rb_uh_epnum_mask(&self) -> RB_UH_EPNUM_MASK_R {
        RB_UH_EPNUM_MASK_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 0:15 - endpoint 2 transmittal length"]
    #[inline(always)]
    pub fn uep2_t_len(&self) -> UEP2_T_LEN_R {
        UEP2_T_LEN_R::new(self.bits)
    }
    #[doc = "Bits 4:7 - The token PID packet identification of this USB transfer transaction"]
    #[inline(always)]
    pub fn rb_uh_token_mask(&self) -> RB_UH_TOKEN_MASK_R {
        RB_UH_TOKEN_MASK_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - The endpoint number of the target of this operation"]
    #[inline(always)]
    pub fn rb_uh_epnum_mask(&mut self) -> RB_UH_EPNUM_MASK_W<0> {
        RB_UH_EPNUM_MASK_W::new(self)
    }
    #[doc = "Bits 0:15 - endpoint 2 transmittal length"]
    #[inline(always)]
    pub fn uep2_t_len(&mut self) -> UEP2_T_LEN_W<0> {
        UEP2_T_LEN_W::new(self)
    }
    #[doc = "Bits 4:7 - The token PID packet identification of this USB transfer transaction"]
    #[inline(always)]
    pub fn rb_uh_token_mask(&mut self) -> RB_UH_TOKEN_MASK_W<4> {
        RB_UH_TOKEN_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "endpoint 2 transmittal length and Set usb host token register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r16_uep2_t_len_r16_uh_ep_pid](index.html) module"]
pub struct R16_UEP2_T_LEN_R16_UH_EP_PID_SPEC;
impl crate::RegisterSpec for R16_UEP2_T_LEN_R16_UH_EP_PID_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [r16_uep2_t_len_r16_uh_ep_pid::R](R) reader structure"]
impl crate::Readable for R16_UEP2_T_LEN_R16_UH_EP_PID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r16_uep2_t_len_r16_uh_ep_pid::W](W) writer structure"]
impl crate::Writable for R16_UEP2_T_LEN_R16_UH_EP_PID_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R16_UEP2_T_LEN_R16_UH_EP_PID to value 0"]
impl crate::Resettable for R16_UEP2_T_LEN_R16_UH_EP_PID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
