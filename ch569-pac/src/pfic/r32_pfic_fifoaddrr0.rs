#[doc = "Register `R32_PFIC_FIFOADDRR0` reader"]
pub struct R(crate::R<R32_PFIC_FIFOADDRR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R32_PFIC_FIFOADDRR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R32_PFIC_FIFOADDRR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R32_PFIC_FIFOADDRR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R32_PFIC_FIFOADDRR0` writer"]
pub struct W(crate::W<R32_PFIC_FIFOADDRR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R32_PFIC_FIFOADDRR0_SPEC>;
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
impl From<crate::W<R32_PFIC_FIFOADDRR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R32_PFIC_FIFOADDRR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFADDR0` reader - OFFADDR0"]
pub type OFFADDR0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `OFFADDR0` writer - OFFADDR0"]
pub type OFFADDR0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, R32_PFIC_FIFOADDRR0_SPEC, u32, u32, 24, O>;
#[doc = "Field `IRQID0` reader - IRQID0"]
pub type IRQID0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IRQID0` writer - IRQID0"]
pub type IRQID0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, R32_PFIC_FIFOADDRR0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:23 - OFFADDR0"]
    #[inline(always)]
    pub fn offaddr0(&self) -> OFFADDR0_R {
        OFFADDR0_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 24:31 - IRQID0"]
    #[inline(always)]
    pub fn irqid0(&self) -> IRQID0_R {
        IRQID0_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - OFFADDR0"]
    #[inline(always)]
    pub fn offaddr0(&mut self) -> OFFADDR0_W<0> {
        OFFADDR0_W::new(self)
    }
    #[doc = "Bits 24:31 - IRQID0"]
    #[inline(always)]
    pub fn irqid0(&mut self) -> IRQID0_W<24> {
        IRQID0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt 0 address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r32_pfic_fifoaddrr0](index.html) module"]
pub struct R32_PFIC_FIFOADDRR0_SPEC;
impl crate::RegisterSpec for R32_PFIC_FIFOADDRR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r32_pfic_fifoaddrr0::R](R) reader structure"]
impl crate::Readable for R32_PFIC_FIFOADDRR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r32_pfic_fifoaddrr0::W](W) writer structure"]
impl crate::Writable for R32_PFIC_FIFOADDRR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R32_PFIC_FIFOADDRR0 to value 0"]
impl crate::Resettable for R32_PFIC_FIFOADDRR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
