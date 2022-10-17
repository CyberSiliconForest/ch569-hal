#[doc = "Register `R8_SLP_CLK_OFF1` reader"]
pub struct R(crate::R<R8_SLP_CLK_OFF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_SLP_CLK_OFF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_SLP_CLK_OFF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_SLP_CLK_OFF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_SLP_CLK_OFF1` writer"]
pub struct W(crate::W<R8_SLP_CLK_OFF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_SLP_CLK_OFF1_SPEC>;
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
impl From<crate::W<R8_SLP_CLK_OFF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_SLP_CLK_OFF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_SLP_CLK_SPI0` reader - sleep SPI0 clock"]
pub type RB_SLP_CLK_SPI0_R = crate::BitReader<bool>;
#[doc = "Field `RB_SLP_CLK_SPI0` writer - sleep SPI0 clock"]
pub type RB_SLP_CLK_SPI0_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SLP_CLK_OFF1_SPEC, bool, O>;
#[doc = "Field `RB_SLP_CLK_SPI1` reader - sleep SPI1 clock"]
pub type RB_SLP_CLK_SPI1_R = crate::BitReader<bool>;
#[doc = "Field `RB_SLP_CLK_SPI1` writer - sleep SPI1 clock"]
pub type RB_SLP_CLK_SPI1_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SLP_CLK_OFF1_SPEC, bool, O>;
#[doc = "Field `RB_SLP_CLK_EMMC` reader - sleep EMMC clock"]
pub type RB_SLP_CLK_EMMC_R = crate::BitReader<bool>;
#[doc = "Field `RB_SLP_CLK_EMMC` writer - sleep EMMC clock"]
pub type RB_SLP_CLK_EMMC_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SLP_CLK_OFF1_SPEC, bool, O>;
#[doc = "Field `RB_SLP_CLK_HSPI` reader - sleep HSPI clock"]
pub type RB_SLP_CLK_HSPI_R = crate::BitReader<bool>;
#[doc = "Field `RB_SLP_CLK_HSPI` writer - sleep HSPI clock"]
pub type RB_SLP_CLK_HSPI_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SLP_CLK_OFF1_SPEC, bool, O>;
#[doc = "Field `RB_SLP_CLK_USBHS` reader - sleep USBHS clock"]
pub type RB_SLP_CLK_USBHS_R = crate::BitReader<bool>;
#[doc = "Field `RB_SLP_CLK_USBHS` writer - sleep USBHS clock"]
pub type RB_SLP_CLK_USBHS_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SLP_CLK_OFF1_SPEC, bool, O>;
#[doc = "Field `RB_SLP_CLK_USBSS` reader - sleep USBSS clock"]
pub type RB_SLP_CLK_USBSS_R = crate::BitReader<bool>;
#[doc = "Field `RB_SLP_CLK_USBSS` writer - sleep USBSS clock"]
pub type RB_SLP_CLK_USBSS_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SLP_CLK_OFF1_SPEC, bool, O>;
#[doc = "Field `RB_SLP_CLK_SERD` reader - sleep SERD clock"]
pub type RB_SLP_CLK_SERD_R = crate::BitReader<bool>;
#[doc = "Field `RB_SLP_CLK_SERD` writer - sleep SERD clock"]
pub type RB_SLP_CLK_SERD_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SLP_CLK_OFF1_SPEC, bool, O>;
#[doc = "Field `RB_SLP_CLK_DVP` reader - sleep DVP clock"]
pub type RB_SLP_CLK_DVP_R = crate::BitReader<bool>;
#[doc = "Field `RB_SLP_CLK_DVP` writer - sleep DVP clock"]
pub type RB_SLP_CLK_DVP_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SLP_CLK_OFF1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - sleep SPI0 clock"]
    #[inline(always)]
    pub fn rb_slp_clk_spi0(&self) -> RB_SLP_CLK_SPI0_R {
        RB_SLP_CLK_SPI0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - sleep SPI1 clock"]
    #[inline(always)]
    pub fn rb_slp_clk_spi1(&self) -> RB_SLP_CLK_SPI1_R {
        RB_SLP_CLK_SPI1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - sleep EMMC clock"]
    #[inline(always)]
    pub fn rb_slp_clk_emmc(&self) -> RB_SLP_CLK_EMMC_R {
        RB_SLP_CLK_EMMC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - sleep HSPI clock"]
    #[inline(always)]
    pub fn rb_slp_clk_hspi(&self) -> RB_SLP_CLK_HSPI_R {
        RB_SLP_CLK_HSPI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - sleep USBHS clock"]
    #[inline(always)]
    pub fn rb_slp_clk_usbhs(&self) -> RB_SLP_CLK_USBHS_R {
        RB_SLP_CLK_USBHS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - sleep USBSS clock"]
    #[inline(always)]
    pub fn rb_slp_clk_usbss(&self) -> RB_SLP_CLK_USBSS_R {
        RB_SLP_CLK_USBSS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - sleep SERD clock"]
    #[inline(always)]
    pub fn rb_slp_clk_serd(&self) -> RB_SLP_CLK_SERD_R {
        RB_SLP_CLK_SERD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - sleep DVP clock"]
    #[inline(always)]
    pub fn rb_slp_clk_dvp(&self) -> RB_SLP_CLK_DVP_R {
        RB_SLP_CLK_DVP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - sleep SPI0 clock"]
    #[inline(always)]
    pub fn rb_slp_clk_spi0(&mut self) -> RB_SLP_CLK_SPI0_W<0> {
        RB_SLP_CLK_SPI0_W::new(self)
    }
    #[doc = "Bit 1 - sleep SPI1 clock"]
    #[inline(always)]
    pub fn rb_slp_clk_spi1(&mut self) -> RB_SLP_CLK_SPI1_W<1> {
        RB_SLP_CLK_SPI1_W::new(self)
    }
    #[doc = "Bit 2 - sleep EMMC clock"]
    #[inline(always)]
    pub fn rb_slp_clk_emmc(&mut self) -> RB_SLP_CLK_EMMC_W<2> {
        RB_SLP_CLK_EMMC_W::new(self)
    }
    #[doc = "Bit 3 - sleep HSPI clock"]
    #[inline(always)]
    pub fn rb_slp_clk_hspi(&mut self) -> RB_SLP_CLK_HSPI_W<3> {
        RB_SLP_CLK_HSPI_W::new(self)
    }
    #[doc = "Bit 4 - sleep USBHS clock"]
    #[inline(always)]
    pub fn rb_slp_clk_usbhs(&mut self) -> RB_SLP_CLK_USBHS_W<4> {
        RB_SLP_CLK_USBHS_W::new(self)
    }
    #[doc = "Bit 5 - sleep USBSS clock"]
    #[inline(always)]
    pub fn rb_slp_clk_usbss(&mut self) -> RB_SLP_CLK_USBSS_W<5> {
        RB_SLP_CLK_USBSS_W::new(self)
    }
    #[doc = "Bit 6 - sleep SERD clock"]
    #[inline(always)]
    pub fn rb_slp_clk_serd(&mut self) -> RB_SLP_CLK_SERD_W<6> {
        RB_SLP_CLK_SERD_W::new(self)
    }
    #[doc = "Bit 7 - sleep DVP clock"]
    #[inline(always)]
    pub fn rb_slp_clk_dvp(&mut self) -> RB_SLP_CLK_DVP_W<7> {
        RB_SLP_CLK_DVP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sleep clock off control byte 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_slp_clk_off1](index.html) module"]
pub struct R8_SLP_CLK_OFF1_SPEC;
impl crate::RegisterSpec for R8_SLP_CLK_OFF1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_slp_clk_off1::R](R) reader structure"]
impl crate::Readable for R8_SLP_CLK_OFF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_slp_clk_off1::W](W) writer structure"]
impl crate::Writable for R8_SLP_CLK_OFF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_SLP_CLK_OFF1 to value 0"]
impl crate::Resettable for R8_SLP_CLK_OFF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
