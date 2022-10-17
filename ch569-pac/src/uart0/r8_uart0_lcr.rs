#[doc = "Register `R8_UART0_LCR` reader"]
pub struct R(crate::R<R8_UART0_LCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_UART0_LCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_UART0_LCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_UART0_LCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_UART0_LCR` writer"]
pub struct W(crate::W<R8_UART0_LCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_UART0_LCR_SPEC>;
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
impl From<crate::W<R8_UART0_LCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_UART0_LCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_LCR_WORD_SZ` reader - UART word bit length"]
pub type RB_LCR_WORD_SZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RB_LCR_WORD_SZ` writer - UART word bit length"]
pub type RB_LCR_WORD_SZ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, R8_UART0_LCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `RB_LCR_STOP_BIT` reader - UART stop bit length"]
pub type RB_LCR_STOP_BIT_R = crate::BitReader<bool>;
#[doc = "Field `RB_LCR_STOP_BIT` writer - UART stop bit length"]
pub type RB_LCR_STOP_BIT_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_UART0_LCR_SPEC, bool, O>;
#[doc = "Field `RB_LCR_PAR_EN` reader - UART parity enable"]
pub type RB_LCR_PAR_EN_R = crate::BitReader<bool>;
#[doc = "Field `RB_LCR_PAR_EN` writer - UART parity enable"]
pub type RB_LCR_PAR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_UART0_LCR_SPEC, bool, O>;
#[doc = "Field `RB_LCR_PAR_MOD` reader - UART parity mode"]
pub type RB_LCR_PAR_MOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RB_LCR_PAR_MOD` writer - UART parity mode"]
pub type RB_LCR_PAR_MOD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, R8_UART0_LCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `RB_LCR_BREAK_EN` reader - UART break control enable"]
pub type RB_LCR_BREAK_EN_R = crate::BitReader<bool>;
#[doc = "Field `RB_LCR_BREAK_EN` writer - UART break control enable"]
pub type RB_LCR_BREAK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_UART0_LCR_SPEC, bool, O>;
#[doc = "Field `RB_LCR_DLAB_RB_LCR_GP_BIT` reader - UART reserved bit _UART general purpose bit"]
pub type RB_LCR_DLAB_RB_LCR_GP_BIT_R = crate::BitReader<bool>;
#[doc = "Field `RB_LCR_DLAB_RB_LCR_GP_BIT` writer - UART reserved bit _UART general purpose bit"]
pub type RB_LCR_DLAB_RB_LCR_GP_BIT_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_UART0_LCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - UART word bit length"]
    #[inline(always)]
    pub fn rb_lcr_word_sz(&self) -> RB_LCR_WORD_SZ_R {
        RB_LCR_WORD_SZ_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - UART stop bit length"]
    #[inline(always)]
    pub fn rb_lcr_stop_bit(&self) -> RB_LCR_STOP_BIT_R {
        RB_LCR_STOP_BIT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART parity enable"]
    #[inline(always)]
    pub fn rb_lcr_par_en(&self) -> RB_LCR_PAR_EN_R {
        RB_LCR_PAR_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - UART parity mode"]
    #[inline(always)]
    pub fn rb_lcr_par_mod(&self) -> RB_LCR_PAR_MOD_R {
        RB_LCR_PAR_MOD_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - UART break control enable"]
    #[inline(always)]
    pub fn rb_lcr_break_en(&self) -> RB_LCR_BREAK_EN_R {
        RB_LCR_BREAK_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART reserved bit _UART general purpose bit"]
    #[inline(always)]
    pub fn rb_lcr_dlab_rb_lcr_gp_bit(&self) -> RB_LCR_DLAB_RB_LCR_GP_BIT_R {
        RB_LCR_DLAB_RB_LCR_GP_BIT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART word bit length"]
    #[inline(always)]
    pub fn rb_lcr_word_sz(&mut self) -> RB_LCR_WORD_SZ_W<0> {
        RB_LCR_WORD_SZ_W::new(self)
    }
    #[doc = "Bit 2 - UART stop bit length"]
    #[inline(always)]
    pub fn rb_lcr_stop_bit(&mut self) -> RB_LCR_STOP_BIT_W<2> {
        RB_LCR_STOP_BIT_W::new(self)
    }
    #[doc = "Bit 3 - UART parity enable"]
    #[inline(always)]
    pub fn rb_lcr_par_en(&mut self) -> RB_LCR_PAR_EN_W<3> {
        RB_LCR_PAR_EN_W::new(self)
    }
    #[doc = "Bits 4:5 - UART parity mode"]
    #[inline(always)]
    pub fn rb_lcr_par_mod(&mut self) -> RB_LCR_PAR_MOD_W<4> {
        RB_LCR_PAR_MOD_W::new(self)
    }
    #[doc = "Bit 6 - UART break control enable"]
    #[inline(always)]
    pub fn rb_lcr_break_en(&mut self) -> RB_LCR_BREAK_EN_W<6> {
        RB_LCR_BREAK_EN_W::new(self)
    }
    #[doc = "Bit 7 - UART reserved bit _UART general purpose bit"]
    #[inline(always)]
    pub fn rb_lcr_dlab_rb_lcr_gp_bit(&mut self) -> RB_LCR_DLAB_RB_LCR_GP_BIT_W<7> {
        RB_LCR_DLAB_RB_LCR_GP_BIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART0 line control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_uart0_lcr](index.html) module"]
pub struct R8_UART0_LCR_SPEC;
impl crate::RegisterSpec for R8_UART0_LCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_uart0_lcr::R](R) reader structure"]
impl crate::Readable for R8_UART0_LCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_uart0_lcr::W](W) writer structure"]
impl crate::Writable for R8_UART0_LCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_UART0_LCR to value 0"]
impl crate::Resettable for R8_UART0_LCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
