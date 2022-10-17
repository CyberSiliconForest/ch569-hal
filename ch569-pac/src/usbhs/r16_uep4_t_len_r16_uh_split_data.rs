#[doc = "Register `R16_UEP4_T_LEN_R16_UH_SPLIT_DATA` reader"]
pub struct R(crate::R<R16_UEP4_T_LEN_R16_UH_SPLIT_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R16_UEP4_T_LEN_R16_UH_SPLIT_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R16_UEP4_T_LEN_R16_UH_SPLIT_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R16_UEP4_T_LEN_R16_UH_SPLIT_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R16_UEP4_T_LEN_R16_UH_SPLIT_DATA` writer"]
pub struct W(crate::W<R16_UEP4_T_LEN_R16_UH_SPLIT_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R16_UEP4_T_LEN_R16_UH_SPLIT_DATA_SPEC>;
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
impl From<crate::W<R16_UEP4_T_LEN_R16_UH_SPLIT_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R16_UEP4_T_LEN_R16_UH_SPLIT_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UEP4_T_LEN_UH_SPLIT_DATA` reader - endpoint 4 transmittal length and USB host Tx SPLIT packet data"]
pub type UEP4_T_LEN_UH_SPLIT_DATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `UEP4_T_LEN_UH_SPLIT_DATA` writer - endpoint 4 transmittal length and USB host Tx SPLIT packet data"]
pub type UEP4_T_LEN_UH_SPLIT_DATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, R16_UEP4_T_LEN_R16_UH_SPLIT_DATA_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - endpoint 4 transmittal length and USB host Tx SPLIT packet data"]
    #[inline(always)]
    pub fn uep4_t_len_uh_split_data(&self) -> UEP4_T_LEN_UH_SPLIT_DATA_R {
        UEP4_T_LEN_UH_SPLIT_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - endpoint 4 transmittal length and USB host Tx SPLIT packet data"]
    #[inline(always)]
    pub fn uep4_t_len_uh_split_data(&mut self) -> UEP4_T_LEN_UH_SPLIT_DATA_W<0> {
        UEP4_T_LEN_UH_SPLIT_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "endpoint 4 transmittal length and USB host Tx SPLIT packet data\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r16_uep4_t_len_r16_uh_split_data](index.html) module"]
pub struct R16_UEP4_T_LEN_R16_UH_SPLIT_DATA_SPEC;
impl crate::RegisterSpec for R16_UEP4_T_LEN_R16_UH_SPLIT_DATA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [r16_uep4_t_len_r16_uh_split_data::R](R) reader structure"]
impl crate::Readable for R16_UEP4_T_LEN_R16_UH_SPLIT_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r16_uep4_t_len_r16_uh_split_data::W](W) writer structure"]
impl crate::Writable for R16_UEP4_T_LEN_R16_UH_SPLIT_DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R16_UEP4_T_LEN_R16_UH_SPLIT_DATA to value 0"]
impl crate::Resettable for R16_UEP4_T_LEN_R16_UH_SPLIT_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
