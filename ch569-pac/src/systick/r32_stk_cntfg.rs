#[doc = "Register `R32_STK_CNTFG` reader"]
pub struct R(crate::R<R32_STK_CNTFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R32_STK_CNTFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R32_STK_CNTFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R32_STK_CNTFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R32_STK_CNTFG` writer"]
pub struct W(crate::W<R32_STK_CNTFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R32_STK_CNTFG_SPEC>;
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
impl From<crate::W<R32_STK_CNTFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R32_STK_CNTFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWIE` reader - System soft interrupt enable"]
pub type SWIE_R = crate::BitReader<bool>;
#[doc = "Field `SWIE` writer - System soft interrupt enable"]
pub type SWIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, R32_STK_CNTFG_SPEC, bool, O>;
#[doc = "Field `CNTIF` reader - Systick counter clear zero flag"]
pub type CNTIF_R = crate::BitReader<bool>;
#[doc = "Field `CNTIF` writer - Systick counter clear zero flag"]
pub type CNTIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, R32_STK_CNTFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - System soft interrupt enable"]
    #[inline(always)]
    pub fn swie(&self) -> SWIE_R {
        SWIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Systick counter clear zero flag"]
    #[inline(always)]
    pub fn cntif(&self) -> CNTIF_R {
        CNTIF_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System soft interrupt enable"]
    #[inline(always)]
    pub fn swie(&mut self) -> SWIE_W<0> {
        SWIE_W::new(self)
    }
    #[doc = "Bit 1 - Systick counter clear zero flag"]
    #[inline(always)]
    pub fn cntif(&mut self) -> CNTIF_W<1> {
        CNTIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Systick counter flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r32_stk_cntfg](index.html) module"]
pub struct R32_STK_CNTFG_SPEC;
impl crate::RegisterSpec for R32_STK_CNTFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r32_stk_cntfg::R](R) reader structure"]
impl crate::Readable for R32_STK_CNTFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r32_stk_cntfg::W](W) writer structure"]
impl crate::Writable for R32_STK_CNTFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R32_STK_CNTFG to value 0"]
impl crate::Resettable for R32_STK_CNTFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
