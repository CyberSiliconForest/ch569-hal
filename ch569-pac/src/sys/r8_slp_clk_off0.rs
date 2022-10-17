#[doc = "Register `R8_SLP_CLK_OFF0` reader"]
pub struct R(crate::R<R8_SLP_CLK_OFF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_SLP_CLK_OFF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_SLP_CLK_OFF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_SLP_CLK_OFF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_SLP_CLK_OFF0` writer"]
pub struct W(crate::W<R8_SLP_CLK_OFF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_SLP_CLK_OFF0_SPEC>;
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
impl From<crate::W<R8_SLP_CLK_OFF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_SLP_CLK_OFF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_SLP_CLK_TMR0` reader - sleep TMR0 clock"]
pub type RB_SLP_CLK_TMR0_R = crate::BitReader<bool>;
#[doc = "Field `RB_SLP_CLK_TMR0` writer - sleep TMR0 clock"]
pub type RB_SLP_CLK_TMR0_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SLP_CLK_OFF0_SPEC, bool, O>;
#[doc = "Field `RB_SLP_CLK_TMR1` reader - sleep TMR1 clock"]
pub type RB_SLP_CLK_TMR1_R = crate::BitReader<bool>;
#[doc = "Field `RB_SLP_CLK_TMR1` writer - sleep TMR1 clock"]
pub type RB_SLP_CLK_TMR1_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SLP_CLK_OFF0_SPEC, bool, O>;
#[doc = "Field `RB_SLP_CLK_TMR2` reader - sleep TMR2 clock"]
pub type RB_SLP_CLK_TMR2_R = crate::BitReader<bool>;
#[doc = "Field `RB_SLP_CLK_TMR2` writer - sleep TMR2 clock"]
pub type RB_SLP_CLK_TMR2_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SLP_CLK_OFF0_SPEC, bool, O>;
#[doc = "Field `RB_SLP_CLK_PWMX` reader - sleep PWMX clock"]
pub type RB_SLP_CLK_PWMX_R = crate::BitReader<bool>;
#[doc = "Field `RB_SLP_CLK_PWMX` writer - sleep PWMX clock"]
pub type RB_SLP_CLK_PWMX_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SLP_CLK_OFF0_SPEC, bool, O>;
#[doc = "Field `RB_SLP_CLK_UART0` reader - sleep UART0 clock"]
pub type RB_SLP_CLK_UART0_R = crate::BitReader<bool>;
#[doc = "Field `RB_SLP_CLK_UART0` writer - sleep UART0 clock"]
pub type RB_SLP_CLK_UART0_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SLP_CLK_OFF0_SPEC, bool, O>;
#[doc = "Field `RB_SLP_CLK_UART1` reader - sleep UART1 clock"]
pub type RB_SLP_CLK_UART1_R = crate::BitReader<bool>;
#[doc = "Field `RB_SLP_CLK_UART1` writer - sleep UART1 clock"]
pub type RB_SLP_CLK_UART1_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SLP_CLK_OFF0_SPEC, bool, O>;
#[doc = "Field `RB_SLP_CLK_UART2` reader - sleep UART2 clock"]
pub type RB_SLP_CLK_UART2_R = crate::BitReader<bool>;
#[doc = "Field `RB_SLP_CLK_UART2` writer - sleep UART2 clock"]
pub type RB_SLP_CLK_UART2_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SLP_CLK_OFF0_SPEC, bool, O>;
#[doc = "Field `RB_SLP_CLK_UART3` reader - sleep UART3 clock"]
pub type RB_SLP_CLK_UART3_R = crate::BitReader<bool>;
#[doc = "Field `RB_SLP_CLK_UART3` writer - sleep UART3 clock"]
pub type RB_SLP_CLK_UART3_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SLP_CLK_OFF0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - sleep TMR0 clock"]
    #[inline(always)]
    pub fn rb_slp_clk_tmr0(&self) -> RB_SLP_CLK_TMR0_R {
        RB_SLP_CLK_TMR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - sleep TMR1 clock"]
    #[inline(always)]
    pub fn rb_slp_clk_tmr1(&self) -> RB_SLP_CLK_TMR1_R {
        RB_SLP_CLK_TMR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - sleep TMR2 clock"]
    #[inline(always)]
    pub fn rb_slp_clk_tmr2(&self) -> RB_SLP_CLK_TMR2_R {
        RB_SLP_CLK_TMR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - sleep PWMX clock"]
    #[inline(always)]
    pub fn rb_slp_clk_pwmx(&self) -> RB_SLP_CLK_PWMX_R {
        RB_SLP_CLK_PWMX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - sleep UART0 clock"]
    #[inline(always)]
    pub fn rb_slp_clk_uart0(&self) -> RB_SLP_CLK_UART0_R {
        RB_SLP_CLK_UART0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - sleep UART1 clock"]
    #[inline(always)]
    pub fn rb_slp_clk_uart1(&self) -> RB_SLP_CLK_UART1_R {
        RB_SLP_CLK_UART1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - sleep UART2 clock"]
    #[inline(always)]
    pub fn rb_slp_clk_uart2(&self) -> RB_SLP_CLK_UART2_R {
        RB_SLP_CLK_UART2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - sleep UART3 clock"]
    #[inline(always)]
    pub fn rb_slp_clk_uart3(&self) -> RB_SLP_CLK_UART3_R {
        RB_SLP_CLK_UART3_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - sleep TMR0 clock"]
    #[inline(always)]
    pub fn rb_slp_clk_tmr0(&mut self) -> RB_SLP_CLK_TMR0_W<0> {
        RB_SLP_CLK_TMR0_W::new(self)
    }
    #[doc = "Bit 1 - sleep TMR1 clock"]
    #[inline(always)]
    pub fn rb_slp_clk_tmr1(&mut self) -> RB_SLP_CLK_TMR1_W<1> {
        RB_SLP_CLK_TMR1_W::new(self)
    }
    #[doc = "Bit 2 - sleep TMR2 clock"]
    #[inline(always)]
    pub fn rb_slp_clk_tmr2(&mut self) -> RB_SLP_CLK_TMR2_W<2> {
        RB_SLP_CLK_TMR2_W::new(self)
    }
    #[doc = "Bit 3 - sleep PWMX clock"]
    #[inline(always)]
    pub fn rb_slp_clk_pwmx(&mut self) -> RB_SLP_CLK_PWMX_W<3> {
        RB_SLP_CLK_PWMX_W::new(self)
    }
    #[doc = "Bit 4 - sleep UART0 clock"]
    #[inline(always)]
    pub fn rb_slp_clk_uart0(&mut self) -> RB_SLP_CLK_UART0_W<4> {
        RB_SLP_CLK_UART0_W::new(self)
    }
    #[doc = "Bit 5 - sleep UART1 clock"]
    #[inline(always)]
    pub fn rb_slp_clk_uart1(&mut self) -> RB_SLP_CLK_UART1_W<5> {
        RB_SLP_CLK_UART1_W::new(self)
    }
    #[doc = "Bit 6 - sleep UART2 clock"]
    #[inline(always)]
    pub fn rb_slp_clk_uart2(&mut self) -> RB_SLP_CLK_UART2_W<6> {
        RB_SLP_CLK_UART2_W::new(self)
    }
    #[doc = "Bit 7 - sleep UART3 clock"]
    #[inline(always)]
    pub fn rb_slp_clk_uart3(&mut self) -> RB_SLP_CLK_UART3_W<7> {
        RB_SLP_CLK_UART3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sleep clock off control byte 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_slp_clk_off0](index.html) module"]
pub struct R8_SLP_CLK_OFF0_SPEC;
impl crate::RegisterSpec for R8_SLP_CLK_OFF0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_slp_clk_off0::R](R) reader structure"]
impl crate::Readable for R8_SLP_CLK_OFF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_slp_clk_off0::W](W) writer structure"]
impl crate::Writable for R8_SLP_CLK_OFF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_SLP_CLK_OFF0 to value 0"]
impl crate::Resettable for R8_SLP_CLK_OFF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
