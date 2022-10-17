#[doc = "Register `R8_SAFE_ACCESS_SIG` reader"]
pub struct R(crate::R<R8_SAFE_ACCESS_SIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_SAFE_ACCESS_SIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_SAFE_ACCESS_SIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_SAFE_ACCESS_SIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_SAFE_ACCESS_SIG` writer"]
pub struct W(crate::W<R8_SAFE_ACCESS_SIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_SAFE_ACCESS_SIG_SPEC>;
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
impl From<crate::W<R8_SAFE_ACCESS_SIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_SAFE_ACCESS_SIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_SAFE_ACC_MODE` reader - current safe accessing mode"]
pub type RB_SAFE_ACC_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RB_SAFE_ACC_MODE` writer - current safe accessing mode"]
pub type RB_SAFE_ACC_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, R8_SAFE_ACCESS_SIG_SPEC, u8, u8, 2, O>;
#[doc = "Field `RB_SAFE_ACC_TIMER` reader - safe accessing timer bit mask"]
pub type RB_SAFE_ACC_TIMER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RB_SAFE_ACC_TIMER` writer - safe accessing timer bit mask"]
pub type RB_SAFE_ACC_TIMER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, R8_SAFE_ACCESS_SIG_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:1 - current safe accessing mode"]
    #[inline(always)]
    pub fn rb_safe_acc_mode(&self) -> RB_SAFE_ACC_MODE_R {
        RB_SAFE_ACC_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6 - safe accessing timer bit mask"]
    #[inline(always)]
    pub fn rb_safe_acc_timer(&self) -> RB_SAFE_ACC_TIMER_R {
        RB_SAFE_ACC_TIMER_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - current safe accessing mode"]
    #[inline(always)]
    pub fn rb_safe_acc_mode(&mut self) -> RB_SAFE_ACC_MODE_W<0> {
        RB_SAFE_ACC_MODE_W::new(self)
    }
    #[doc = "Bits 4:6 - safe accessing timer bit mask"]
    #[inline(always)]
    pub fn rb_safe_acc_timer(&mut self) -> RB_SAFE_ACC_TIMER_W<4> {
        RB_SAFE_ACC_TIMER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "safe accessing sign register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_safe_access_sig](index.html) module"]
pub struct R8_SAFE_ACCESS_SIG_SPEC;
impl crate::RegisterSpec for R8_SAFE_ACCESS_SIG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_safe_access_sig::R](R) reader structure"]
impl crate::Readable for R8_SAFE_ACCESS_SIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_safe_access_sig::W](W) writer structure"]
impl crate::Writable for R8_SAFE_ACCESS_SIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_SAFE_ACCESS_SIG to value 0"]
impl crate::Resettable for R8_SAFE_ACCESS_SIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
