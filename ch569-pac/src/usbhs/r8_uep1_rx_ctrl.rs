#[doc = "Register `R8_UEP1_RX_CTRL` reader"]
pub struct R(crate::R<R8_UEP1_RX_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_UEP1_RX_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_UEP1_RX_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_UEP1_RX_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_UEP1_RX_CTRL` writer"]
pub struct W(crate::W<R8_UEP1_RX_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_UEP1_RX_CTRL_SPEC>;
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
impl From<crate::W<R8_UEP1_RX_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_UEP1_RX_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_UEP_RRES_MASK` reader - bit mask of handshake response type for USB endpoint X receiving (OUT)"]
pub type RB_UEP_RRES_MASK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RB_UEP_RRES_MASK` writer - bit mask of handshake response type for USB endpoint X receiving (OUT)"]
pub type RB_UEP_RRES_MASK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, R8_UEP1_RX_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `RB_UEP_RRES_NO` reader - prepared no response"]
pub type RB_UEP_RRES_NO_R = crate::BitReader<bool>;
#[doc = "Field `RB_UEP_RRES_NO` writer - prepared no response"]
pub type RB_UEP_RRES_NO_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_UEP1_RX_CTRL_SPEC, bool, O>;
#[doc = "Field `RB_UEP_R_TOG_MASK` reader - expected data toggle flag of USB endpoint X receiving"]
pub type RB_UEP_R_TOG_MASK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RB_UEP_R_TOG_MASK` writer - expected data toggle flag of USB endpoint X receiving"]
pub type RB_UEP_R_TOG_MASK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, R8_UEP1_RX_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `RB_UEP_R_AUTOTOG` reader - enable automatic toggle after successful transfer completion on endpoint"]
pub type RB_UEP_R_AUTOTOG_R = crate::BitReader<bool>;
#[doc = "Field `RB_UEP_R_AUTOTOG` writer - enable automatic toggle after successful transfer completion on endpoint"]
pub type RB_UEP_R_AUTOTOG_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_UEP1_RX_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - bit mask of handshake response type for USB endpoint X receiving (OUT)"]
    #[inline(always)]
    pub fn rb_uep_rres_mask(&self) -> RB_UEP_RRES_MASK_R {
        RB_UEP_RRES_MASK_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - prepared no response"]
    #[inline(always)]
    pub fn rb_uep_rres_no(&self) -> RB_UEP_RRES_NO_R {
        RB_UEP_RRES_NO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - expected data toggle flag of USB endpoint X receiving"]
    #[inline(always)]
    pub fn rb_uep_r_tog_mask(&self) -> RB_UEP_R_TOG_MASK_R {
        RB_UEP_R_TOG_MASK_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - enable automatic toggle after successful transfer completion on endpoint"]
    #[inline(always)]
    pub fn rb_uep_r_autotog(&self) -> RB_UEP_R_AUTOTOG_R {
        RB_UEP_R_AUTOTOG_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - bit mask of handshake response type for USB endpoint X receiving (OUT)"]
    #[inline(always)]
    pub fn rb_uep_rres_mask(&mut self) -> RB_UEP_RRES_MASK_W<0> {
        RB_UEP_RRES_MASK_W::new(self)
    }
    #[doc = "Bit 2 - prepared no response"]
    #[inline(always)]
    pub fn rb_uep_rres_no(&mut self) -> RB_UEP_RRES_NO_W<2> {
        RB_UEP_RRES_NO_W::new(self)
    }
    #[doc = "Bits 3:4 - expected data toggle flag of USB endpoint X receiving"]
    #[inline(always)]
    pub fn rb_uep_r_tog_mask(&mut self) -> RB_UEP_R_TOG_MASK_W<3> {
        RB_UEP_R_TOG_MASK_W::new(self)
    }
    #[doc = "Bit 5 - enable automatic toggle after successful transfer completion on endpoint"]
    #[inline(always)]
    pub fn rb_uep_r_autotog(&mut self) -> RB_UEP_R_AUTOTOG_W<5> {
        RB_UEP_R_AUTOTOG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "endpoint 1 rx control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_uep1_rx_ctrl](index.html) module"]
pub struct R8_UEP1_RX_CTRL_SPEC;
impl crate::RegisterSpec for R8_UEP1_RX_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_uep1_rx_ctrl::R](R) reader structure"]
impl crate::Readable for R8_UEP1_RX_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_uep1_rx_ctrl::W](W) writer structure"]
impl crate::Writable for R8_UEP1_RX_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_UEP1_RX_CTRL to value 0"]
impl crate::Resettable for R8_UEP1_RX_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
