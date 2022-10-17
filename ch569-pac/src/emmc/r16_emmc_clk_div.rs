#[doc = "Register `R16_EMMC_CLK_DIV` reader"]
pub struct R(crate::R<R16_EMMC_CLK_DIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R16_EMMC_CLK_DIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R16_EMMC_CLK_DIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R16_EMMC_CLK_DIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R16_EMMC_CLK_DIV` writer"]
pub struct W(crate::W<R16_EMMC_CLK_DIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R16_EMMC_CLK_DIV_SPEC>;
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
impl From<crate::W<R16_EMMC_CLK_DIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R16_EMMC_CLK_DIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_EMMC_DIV_MASK` reader - clk div"]
pub type RB_EMMC_DIV_MASK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RB_EMMC_DIV_MASK` writer - clk div"]
pub type RB_EMMC_DIV_MASK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, R16_EMMC_CLK_DIV_SPEC, u8, u8, 5, O>;
#[doc = "Field `RB_EMMC_CLKOE` reader - chip output sdclk oe"]
pub type RB_EMMC_CLKOE_R = crate::BitReader<bool>;
#[doc = "Field `RB_EMMC_CLKOE` writer - chip output sdclk oe"]
pub type RB_EMMC_CLKOE_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, R16_EMMC_CLK_DIV_SPEC, bool, O>;
#[doc = "Field `RB_EMMC_CLKMode` reader - EMMC clock frequency mode selection bit"]
pub type RB_EMMC_CLKMODE_R = crate::BitReader<bool>;
#[doc = "Field `RB_EMMC_CLKMode` writer - EMMC clock frequency mode selection bit"]
pub type RB_EMMC_CLKMODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, R16_EMMC_CLK_DIV_SPEC, bool, O>;
#[doc = "Field `RB_EMMC_PHASEINV` reader - invert chip output sdclk phase"]
pub type RB_EMMC_PHASEINV_R = crate::BitReader<bool>;
#[doc = "Field `RB_EMMC_PHASEINV` writer - invert chip output sdclk phase"]
pub type RB_EMMC_PHASEINV_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, R16_EMMC_CLK_DIV_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - clk div"]
    #[inline(always)]
    pub fn rb_emmc_div_mask(&self) -> RB_EMMC_DIV_MASK_R {
        RB_EMMC_DIV_MASK_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - chip output sdclk oe"]
    #[inline(always)]
    pub fn rb_emmc_clkoe(&self) -> RB_EMMC_CLKOE_R {
        RB_EMMC_CLKOE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EMMC clock frequency mode selection bit"]
    #[inline(always)]
    pub fn rb_emmc_clkmode(&self) -> RB_EMMC_CLKMODE_R {
        RB_EMMC_CLKMODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - invert chip output sdclk phase"]
    #[inline(always)]
    pub fn rb_emmc_phaseinv(&self) -> RB_EMMC_PHASEINV_R {
        RB_EMMC_PHASEINV_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - clk div"]
    #[inline(always)]
    pub fn rb_emmc_div_mask(&mut self) -> RB_EMMC_DIV_MASK_W<0> {
        RB_EMMC_DIV_MASK_W::new(self)
    }
    #[doc = "Bit 8 - chip output sdclk oe"]
    #[inline(always)]
    pub fn rb_emmc_clkoe(&mut self) -> RB_EMMC_CLKOE_W<8> {
        RB_EMMC_CLKOE_W::new(self)
    }
    #[doc = "Bit 9 - EMMC clock frequency mode selection bit"]
    #[inline(always)]
    pub fn rb_emmc_clkmode(&mut self) -> RB_EMMC_CLKMODE_W<9> {
        RB_EMMC_CLKMODE_W::new(self)
    }
    #[doc = "Bit 10 - invert chip output sdclk phase"]
    #[inline(always)]
    pub fn rb_emmc_phaseinv(&mut self) -> RB_EMMC_PHASEINV_W<10> {
        RB_EMMC_PHASEINV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SD clock divider register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r16_emmc_clk_div](index.html) module"]
pub struct R16_EMMC_CLK_DIV_SPEC;
impl crate::RegisterSpec for R16_EMMC_CLK_DIV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [r16_emmc_clk_div::R](R) reader structure"]
impl crate::Readable for R16_EMMC_CLK_DIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r16_emmc_clk_div::W](W) writer structure"]
impl crate::Writable for R16_EMMC_CLK_DIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R16_EMMC_CLK_DIV to value 0x0213"]
impl crate::Resettable for R16_EMMC_CLK_DIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0213
    }
}
