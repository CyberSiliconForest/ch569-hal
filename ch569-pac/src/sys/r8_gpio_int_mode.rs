#[doc = "Register `R8_GPIO_INT_MODE` reader"]
pub struct R(crate::R<R8_GPIO_INT_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_GPIO_INT_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_GPIO_INT_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_GPIO_INT_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_GPIO_INT_MODE` writer"]
pub struct W(crate::W<R8_GPIO_INT_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_GPIO_INT_MODE_SPEC>;
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
impl From<crate::W<R8_GPIO_INT_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_GPIO_INT_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_GPIO_PA2_IM` reader - PA2 pin interrupt mode"]
pub type RB_GPIO_PA2_IM_R = crate::BitReader<bool>;
#[doc = "Field `RB_GPIO_PA2_IM` writer - PA2 pin interrupt mode"]
pub type RB_GPIO_PA2_IM_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_GPIO_INT_MODE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - PA2 pin interrupt mode"]
    #[inline(always)]
    pub fn rb_gpio_pa2_im(&self) -> RB_GPIO_PA2_IM_R {
        RB_GPIO_PA2_IM_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PA2 pin interrupt mode"]
    #[inline(always)]
    pub fn rb_gpio_pa2_im(&mut self) -> RB_GPIO_PA2_IM_W<0> {
        RB_GPIO_PA2_IM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO interrupt mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_gpio_int_mode](index.html) module"]
pub struct R8_GPIO_INT_MODE_SPEC;
impl crate::RegisterSpec for R8_GPIO_INT_MODE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_gpio_int_mode::R](R) reader structure"]
impl crate::Readable for R8_GPIO_INT_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_gpio_int_mode::W](W) writer structure"]
impl crate::Writable for R8_GPIO_INT_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_GPIO_INT_MODE to value 0"]
impl crate::Resettable for R8_GPIO_INT_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
