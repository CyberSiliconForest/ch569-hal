#[doc = "Register `R8_ECDC_INT_EN` reader"]
pub struct R(crate::R<R8_ECDC_INT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_ECDC_INT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_ECDC_INT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_ECDC_INT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_ECDC_INT_EN` writer"]
pub struct W(crate::W<R8_ECDC_INT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_ECDC_INT_EN_SPEC>;
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
impl From<crate::W<R8_ECDC_INT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_ECDC_INT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_ECDC_IE_EKDONE` reader - Key extension completion interrupt enable"]
pub type RB_ECDC_IE_EKDONE_R = crate::BitReader<bool>;
#[doc = "Field `RB_ECDC_IE_EKDONE` writer - Key extension completion interrupt enable"]
pub type RB_ECDC_IE_EKDONE_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_ECDC_INT_EN_SPEC, bool, O>;
#[doc = "Field `RB_ECDC_IE_SINGLE` reader - Single encryption and decryption completion interrupt enable"]
pub type RB_ECDC_IE_SINGLE_R = crate::BitReader<bool>;
#[doc = "Field `RB_ECDC_IE_SINGLE` writer - Single encryption and decryption completion interrupt enable"]
pub type RB_ECDC_IE_SINGLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_ECDC_INT_EN_SPEC, bool, O>;
#[doc = "Field `RB_ECDC_IE_WRSRAM` reader - Memory to memory encryption and decryption completion interrupt enable"]
pub type RB_ECDC_IE_WRSRAM_R = crate::BitReader<bool>;
#[doc = "Field `RB_ECDC_IE_WRSRAM` writer - Memory to memory encryption and decryption completion interrupt enable"]
pub type RB_ECDC_IE_WRSRAM_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_ECDC_INT_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Key extension completion interrupt enable"]
    #[inline(always)]
    pub fn rb_ecdc_ie_ekdone(&self) -> RB_ECDC_IE_EKDONE_R {
        RB_ECDC_IE_EKDONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Single encryption and decryption completion interrupt enable"]
    #[inline(always)]
    pub fn rb_ecdc_ie_single(&self) -> RB_ECDC_IE_SINGLE_R {
        RB_ECDC_IE_SINGLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Memory to memory encryption and decryption completion interrupt enable"]
    #[inline(always)]
    pub fn rb_ecdc_ie_wrsram(&self) -> RB_ECDC_IE_WRSRAM_R {
        RB_ECDC_IE_WRSRAM_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Key extension completion interrupt enable"]
    #[inline(always)]
    pub fn rb_ecdc_ie_ekdone(&mut self) -> RB_ECDC_IE_EKDONE_W<0> {
        RB_ECDC_IE_EKDONE_W::new(self)
    }
    #[doc = "Bit 1 - Single encryption and decryption completion interrupt enable"]
    #[inline(always)]
    pub fn rb_ecdc_ie_single(&mut self) -> RB_ECDC_IE_SINGLE_W<1> {
        RB_ECDC_IE_SINGLE_W::new(self)
    }
    #[doc = "Bit 2 - Memory to memory encryption and decryption completion interrupt enable"]
    #[inline(always)]
    pub fn rb_ecdc_ie_wrsram(&mut self) -> RB_ECDC_IE_WRSRAM_W<2> {
        RB_ECDC_IE_WRSRAM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_ecdc_int_en](index.html) module"]
pub struct R8_ECDC_INT_EN_SPEC;
impl crate::RegisterSpec for R8_ECDC_INT_EN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_ecdc_int_en::R](R) reader structure"]
impl crate::Readable for R8_ECDC_INT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_ecdc_int_en::W](W) writer structure"]
impl crate::Writable for R8_ECDC_INT_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_ECDC_INT_EN to value 0"]
impl crate::Resettable for R8_ECDC_INT_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
