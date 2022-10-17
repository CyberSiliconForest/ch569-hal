#[doc = "Register `R8_UHOST_CTRL` reader"]
pub struct R(crate::R<R8_UHOST_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_UHOST_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_UHOST_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_UHOST_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_UHOST_CTRL` writer"]
pub struct W(crate::W<R8_UHOST_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_UHOST_CTRL_SPEC>;
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
impl From<crate::W<R8_UHOST_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_UHOST_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_UH_BUS_RESET` reader - USB host send bus reset signal"]
pub type RB_UH_BUS_RESET_R = crate::BitReader<bool>;
#[doc = "Field `RB_UH_BUS_RESET` writer - USB host send bus reset signal"]
pub type RB_UH_BUS_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_UHOST_CTRL_SPEC, bool, O>;
#[doc = "Field `RB_UH_BUS_SUSPEND` reader - USB host send bus suspend signal"]
pub type RB_UH_BUS_SUSPEND_R = crate::BitReader<bool>;
#[doc = "Field `RB_UH_BUS_SUSPEND` writer - USB host send bus suspend signal"]
pub type RB_UH_BUS_SUSPEND_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_UHOST_CTRL_SPEC, bool, O>;
#[doc = "Field `RB_UH_BUS_RESUME` reader - USB host suspend state and wake up device"]
pub type RB_UH_BUS_RESUME_R = crate::BitReader<bool>;
#[doc = "Field `RB_UH_BUS_RESUME` writer - USB host suspend state and wake up device"]
pub type RB_UH_BUS_RESUME_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_UHOST_CTRL_SPEC, bool, O>;
#[doc = "Field `RB_UH_AUTOSOF_EN` reader - Automatically generate sof packet enable control"]
pub type RB_UH_AUTOSOF_EN_R = crate::BitReader<bool>;
#[doc = "Field `RB_UH_AUTOSOF_EN` writer - Automatically generate sof packet enable control"]
pub type RB_UH_AUTOSOF_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_UHOST_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - USB host send bus reset signal"]
    #[inline(always)]
    pub fn rb_uh_bus_reset(&self) -> RB_UH_BUS_RESET_R {
        RB_UH_BUS_RESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB host send bus suspend signal"]
    #[inline(always)]
    pub fn rb_uh_bus_suspend(&self) -> RB_UH_BUS_SUSPEND_R {
        RB_UH_BUS_SUSPEND_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB host suspend state and wake up device"]
    #[inline(always)]
    pub fn rb_uh_bus_resume(&self) -> RB_UH_BUS_RESUME_R {
        RB_UH_BUS_RESUME_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Automatically generate sof packet enable control"]
    #[inline(always)]
    pub fn rb_uh_autosof_en(&self) -> RB_UH_AUTOSOF_EN_R {
        RB_UH_AUTOSOF_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB host send bus reset signal"]
    #[inline(always)]
    pub fn rb_uh_bus_reset(&mut self) -> RB_UH_BUS_RESET_W<0> {
        RB_UH_BUS_RESET_W::new(self)
    }
    #[doc = "Bit 1 - USB host send bus suspend signal"]
    #[inline(always)]
    pub fn rb_uh_bus_suspend(&mut self) -> RB_UH_BUS_SUSPEND_W<1> {
        RB_UH_BUS_SUSPEND_W::new(self)
    }
    #[doc = "Bit 2 - USB host suspend state and wake up device"]
    #[inline(always)]
    pub fn rb_uh_bus_resume(&mut self) -> RB_UH_BUS_RESUME_W<2> {
        RB_UH_BUS_RESUME_W::new(self)
    }
    #[doc = "Bit 7 - Automatically generate sof packet enable control"]
    #[inline(always)]
    pub fn rb_uh_autosof_en(&mut self) -> RB_UH_AUTOSOF_EN_W<7> {
        RB_UH_AUTOSOF_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB host control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_uhost_ctrl](index.html) module"]
pub struct R8_UHOST_CTRL_SPEC;
impl crate::RegisterSpec for R8_UHOST_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_uhost_ctrl::R](R) reader structure"]
impl crate::Readable for R8_UHOST_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_uhost_ctrl::W](W) writer structure"]
impl crate::Writable for R8_UHOST_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_UHOST_CTRL to value 0"]
impl crate::Resettable for R8_UHOST_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
