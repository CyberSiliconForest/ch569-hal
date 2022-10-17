#[doc = "Register `R32_STK_CTLR` reader"]
pub struct R(crate::R<R32_STK_CTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R32_STK_CTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R32_STK_CTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R32_STK_CTLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R32_STK_CTLR` writer"]
pub struct W(crate::W<R32_STK_CTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R32_STK_CTLR_SPEC>;
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
impl From<crate::W<R32_STK_CTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R32_STK_CTLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STE` reader - Systick counter enable"]
pub type STE_R = crate::BitReader<bool>;
#[doc = "Field `STE` writer - Systick counter enable"]
pub type STE_W<'a, const O: u8> = crate::BitWriter<'a, u32, R32_STK_CTLR_SPEC, bool, O>;
#[doc = "Field `STIE` reader - Systick counter interrupt enable"]
pub type STIE_R = crate::BitReader<bool>;
#[doc = "Field `STIE` writer - Systick counter interrupt enable"]
pub type STIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, R32_STK_CTLR_SPEC, bool, O>;
#[doc = "Field `STCLK` reader - System counter clock Source selection"]
pub type STCLK_R = crate::BitReader<bool>;
#[doc = "Field `STCLK` writer - System counter clock Source selection"]
pub type STCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, R32_STK_CTLR_SPEC, bool, O>;
#[doc = "Field `STRELOAD` reader - System counter reload control"]
pub type STRELOAD_R = crate::BitReader<bool>;
#[doc = "Field `STRELOAD` writer - System counter reload control"]
pub type STRELOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, R32_STK_CTLR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Systick counter enable"]
    #[inline(always)]
    pub fn ste(&self) -> STE_R {
        STE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Systick counter interrupt enable"]
    #[inline(always)]
    pub fn stie(&self) -> STIE_R {
        STIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - System counter clock Source selection"]
    #[inline(always)]
    pub fn stclk(&self) -> STCLK_R {
        STCLK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - System counter reload control"]
    #[inline(always)]
    pub fn streload(&self) -> STRELOAD_R {
        STRELOAD_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Systick counter enable"]
    #[inline(always)]
    pub fn ste(&mut self) -> STE_W<0> {
        STE_W::new(self)
    }
    #[doc = "Bit 1 - Systick counter interrupt enable"]
    #[inline(always)]
    pub fn stie(&mut self) -> STIE_W<1> {
        STIE_W::new(self)
    }
    #[doc = "Bit 2 - System counter clock Source selection"]
    #[inline(always)]
    pub fn stclk(&mut self) -> STCLK_W<2> {
        STCLK_W::new(self)
    }
    #[doc = "Bit 8 - System counter reload control"]
    #[inline(always)]
    pub fn streload(&mut self) -> STRELOAD_W<8> {
        STRELOAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Systick counter control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r32_stk_ctlr](index.html) module"]
pub struct R32_STK_CTLR_SPEC;
impl crate::RegisterSpec for R32_STK_CTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r32_stk_ctlr::R](R) reader structure"]
impl crate::Readable for R32_STK_CTLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r32_stk_ctlr::W](W) writer structure"]
impl crate::Writable for R32_STK_CTLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R32_STK_CTLR to value 0"]
impl crate::Resettable for R32_STK_CTLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
