#[doc = "Register `R32_PFIC_FIFOADDRR1` reader"]
pub struct R(crate::R<R32_PFIC_FIFOADDRR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R32_PFIC_FIFOADDRR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R32_PFIC_FIFOADDRR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R32_PFIC_FIFOADDRR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R32_PFIC_FIFOADDRR1` writer"]
pub struct W(crate::W<R32_PFIC_FIFOADDRR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R32_PFIC_FIFOADDRR1_SPEC>;
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
impl From<crate::W<R32_PFIC_FIFOADDRR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R32_PFIC_FIFOADDRR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFADDR1` reader - OFFADDR1"]
pub type OFFADDR1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `OFFADDR1` writer - OFFADDR1"]
pub type OFFADDR1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, R32_PFIC_FIFOADDRR1_SPEC, u32, u32, 24, O>;
#[doc = "Field `IRQID1` reader - IRQID1"]
pub type IRQID1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IRQID1` writer - IRQID1"]
pub type IRQID1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, R32_PFIC_FIFOADDRR1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:23 - OFFADDR1"]
    #[inline(always)]
    pub fn offaddr1(&self) -> OFFADDR1_R {
        OFFADDR1_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 24:31 - IRQID1"]
    #[inline(always)]
    pub fn irqid1(&self) -> IRQID1_R {
        IRQID1_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - OFFADDR1"]
    #[inline(always)]
    pub fn offaddr1(&mut self) -> OFFADDR1_W<0> {
        OFFADDR1_W::new(self)
    }
    #[doc = "Bits 24:31 - IRQID1"]
    #[inline(always)]
    pub fn irqid1(&mut self) -> IRQID1_W<24> {
        IRQID1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt 1 address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r32_pfic_fifoaddrr1](index.html) module"]
pub struct R32_PFIC_FIFOADDRR1_SPEC;
impl crate::RegisterSpec for R32_PFIC_FIFOADDRR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r32_pfic_fifoaddrr1::R](R) reader structure"]
impl crate::Readable for R32_PFIC_FIFOADDRR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r32_pfic_fifoaddrr1::W](W) writer structure"]
impl crate::Writable for R32_PFIC_FIFOADDRR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R32_PFIC_FIFOADDRR1 to value 0"]
impl crate::Resettable for R32_PFIC_FIFOADDRR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
