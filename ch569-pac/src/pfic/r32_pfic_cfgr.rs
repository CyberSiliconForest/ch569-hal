#[doc = "Register `R32_PFIC_CFGR` reader"]
pub struct R(crate::R<R32_PFIC_CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R32_PFIC_CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R32_PFIC_CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R32_PFIC_CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R32_PFIC_CFGR` writer"]
pub struct W(crate::W<R32_PFIC_CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R32_PFIC_CFGR_SPEC>;
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
impl From<crate::W<R32_PFIC_CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R32_PFIC_CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HWSTKCTRL` reader - HWSTKCTRL"]
pub type HWSTKCTRL_R = crate::BitReader<bool>;
#[doc = "Field `HWSTKCTRL` writer - HWSTKCTRL"]
pub type HWSTKCTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, R32_PFIC_CFGR_SPEC, bool, O>;
#[doc = "Field `NESTCTRL` reader - NESTCTRL"]
pub type NESTCTRL_R = crate::BitReader<bool>;
#[doc = "Field `NESTCTRL` writer - NESTCTRL"]
pub type NESTCTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, R32_PFIC_CFGR_SPEC, bool, O>;
#[doc = "Field `NMISET` writer - NMISET"]
pub type NMISET_W<'a, const O: u8> = crate::BitWriter<'a, u32, R32_PFIC_CFGR_SPEC, bool, O>;
#[doc = "Field `NMIRESET` writer - NMIRESET"]
pub type NMIRESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, R32_PFIC_CFGR_SPEC, bool, O>;
#[doc = "Field `EXCSET` writer - EXCSET"]
pub type EXCSET_W<'a, const O: u8> = crate::BitWriter<'a, u32, R32_PFIC_CFGR_SPEC, bool, O>;
#[doc = "Field `EXCRESET` writer - EXCRESET"]
pub type EXCRESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, R32_PFIC_CFGR_SPEC, bool, O>;
#[doc = "Field `PFICRESET` writer - PFICRSET"]
pub type PFICRESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, R32_PFIC_CFGR_SPEC, bool, O>;
#[doc = "Field `SYSRESET` writer - SYSRESET"]
pub type SYSRESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, R32_PFIC_CFGR_SPEC, bool, O>;
#[doc = "Field `KEYCODE` writer - KEYCODE"]
pub type KEYCODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, R32_PFIC_CFGR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - HWSTKCTRL"]
    #[inline(always)]
    pub fn hwstkctrl(&self) -> HWSTKCTRL_R {
        HWSTKCTRL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NESTCTRL"]
    #[inline(always)]
    pub fn nestctrl(&self) -> NESTCTRL_R {
        NESTCTRL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HWSTKCTRL"]
    #[inline(always)]
    pub fn hwstkctrl(&mut self) -> HWSTKCTRL_W<0> {
        HWSTKCTRL_W::new(self)
    }
    #[doc = "Bit 1 - NESTCTRL"]
    #[inline(always)]
    pub fn nestctrl(&mut self) -> NESTCTRL_W<1> {
        NESTCTRL_W::new(self)
    }
    #[doc = "Bit 2 - NMISET"]
    #[inline(always)]
    pub fn nmiset(&mut self) -> NMISET_W<2> {
        NMISET_W::new(self)
    }
    #[doc = "Bit 3 - NMIRESET"]
    #[inline(always)]
    pub fn nmireset(&mut self) -> NMIRESET_W<3> {
        NMIRESET_W::new(self)
    }
    #[doc = "Bit 4 - EXCSET"]
    #[inline(always)]
    pub fn excset(&mut self) -> EXCSET_W<4> {
        EXCSET_W::new(self)
    }
    #[doc = "Bit 5 - EXCRESET"]
    #[inline(always)]
    pub fn excreset(&mut self) -> EXCRESET_W<5> {
        EXCRESET_W::new(self)
    }
    #[doc = "Bit 6 - PFICRSET"]
    #[inline(always)]
    pub fn pficreset(&mut self) -> PFICRESET_W<6> {
        PFICRESET_W::new(self)
    }
    #[doc = "Bit 7 - SYSRESET"]
    #[inline(always)]
    pub fn sysreset(&mut self) -> SYSRESET_W<7> {
        SYSRESET_W::new(self)
    }
    #[doc = "Bits 16:31 - KEYCODE"]
    #[inline(always)]
    pub fn keycode(&mut self) -> KEYCODE_W<16> {
        KEYCODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Config Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r32_pfic_cfgr](index.html) module"]
pub struct R32_PFIC_CFGR_SPEC;
impl crate::RegisterSpec for R32_PFIC_CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r32_pfic_cfgr::R](R) reader structure"]
impl crate::Readable for R32_PFIC_CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r32_pfic_cfgr::W](W) writer structure"]
impl crate::Writable for R32_PFIC_CFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R32_PFIC_CFGR to value 0"]
impl crate::Resettable for R32_PFIC_CFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
