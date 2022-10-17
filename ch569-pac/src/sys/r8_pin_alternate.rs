#[doc = "Register `R8_PIN_ALTERNATE` reader"]
pub struct R(crate::R<R8_PIN_ALTERNATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_PIN_ALTERNATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_PIN_ALTERNATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_PIN_ALTERNATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_PIN_ALTERNATE` writer"]
pub struct W(crate::W<R8_PIN_ALTERNATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_PIN_ALTERNATE_SPEC>;
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
impl From<crate::W<R8_PIN_ALTERNATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_PIN_ALTERNATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_PIN_MII` reader - ETH mii interface selection"]
pub type RB_PIN_MII_R = crate::BitReader<bool>;
#[doc = "Field `RB_PIN_MII` writer - ETH mii interface selection"]
pub type RB_PIN_MII_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_PIN_ALTERNATE_SPEC, bool, O>;
#[doc = "Field `RB_PIN_TMR1` reader - TMR1 alternate pin enable"]
pub type RB_PIN_TMR1_R = crate::BitReader<bool>;
#[doc = "Field `RB_PIN_TMR1` writer - TMR1 alternate pin enable"]
pub type RB_PIN_TMR1_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_PIN_ALTERNATE_SPEC, bool, O>;
#[doc = "Field `RB_PIN_TMR2` reader - TMR2 alternate pin enable"]
pub type RB_PIN_TMR2_R = crate::BitReader<bool>;
#[doc = "Field `RB_PIN_TMR2` writer - TMR2 alternate pin enable"]
pub type RB_PIN_TMR2_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_PIN_ALTERNATE_SPEC, bool, O>;
#[doc = "Field `RB_PIN_UART0` reader - RXD0/TXD0 alternate pin enable"]
pub type RB_PIN_UART0_R = crate::BitReader<bool>;
#[doc = "Field `RB_PIN_UART0` writer - RXD0/TXD0 alternate pin enable"]
pub type RB_PIN_UART0_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_PIN_ALTERNATE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ETH mii interface selection"]
    #[inline(always)]
    pub fn rb_pin_mii(&self) -> RB_PIN_MII_R {
        RB_PIN_MII_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TMR1 alternate pin enable"]
    #[inline(always)]
    pub fn rb_pin_tmr1(&self) -> RB_PIN_TMR1_R {
        RB_PIN_TMR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TMR2 alternate pin enable"]
    #[inline(always)]
    pub fn rb_pin_tmr2(&self) -> RB_PIN_TMR2_R {
        RB_PIN_TMR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - RXD0/TXD0 alternate pin enable"]
    #[inline(always)]
    pub fn rb_pin_uart0(&self) -> RB_PIN_UART0_R {
        RB_PIN_UART0_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ETH mii interface selection"]
    #[inline(always)]
    pub fn rb_pin_mii(&mut self) -> RB_PIN_MII_W<0> {
        RB_PIN_MII_W::new(self)
    }
    #[doc = "Bit 1 - TMR1 alternate pin enable"]
    #[inline(always)]
    pub fn rb_pin_tmr1(&mut self) -> RB_PIN_TMR1_W<1> {
        RB_PIN_TMR1_W::new(self)
    }
    #[doc = "Bit 2 - TMR2 alternate pin enable"]
    #[inline(always)]
    pub fn rb_pin_tmr2(&mut self) -> RB_PIN_TMR2_W<2> {
        RB_PIN_TMR2_W::new(self)
    }
    #[doc = "Bit 4 - RXD0/TXD0 alternate pin enable"]
    #[inline(always)]
    pub fn rb_pin_uart0(&mut self) -> RB_PIN_UART0_W<4> {
        RB_PIN_UART0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "alternate pin control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_pin_alternate](index.html) module"]
pub struct R8_PIN_ALTERNATE_SPEC;
impl crate::RegisterSpec for R8_PIN_ALTERNATE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_pin_alternate::R](R) reader structure"]
impl crate::Readable for R8_PIN_ALTERNATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_pin_alternate::W](W) writer structure"]
impl crate::Writable for R8_PIN_ALTERNATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_PIN_ALTERNATE to value 0"]
impl crate::Resettable for R8_PIN_ALTERNATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
