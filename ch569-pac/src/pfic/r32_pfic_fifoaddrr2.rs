#[doc = "Register `R32_PFIC_FIFOADDRR2` reader"]
pub struct R(crate::R<R32_PFIC_FIFOADDRR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R32_PFIC_FIFOADDRR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R32_PFIC_FIFOADDRR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R32_PFIC_FIFOADDRR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R32_PFIC_FIFOADDRR2` writer"]
pub struct W(crate::W<R32_PFIC_FIFOADDRR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R32_PFIC_FIFOADDRR2_SPEC>;
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
impl From<crate::W<R32_PFIC_FIFOADDRR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R32_PFIC_FIFOADDRR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFADDR2` reader - OFFADDR2"]
pub type OFFADDR2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `OFFADDR2` writer - OFFADDR2"]
pub type OFFADDR2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, R32_PFIC_FIFOADDRR2_SPEC, u32, u32, 24, O>;
#[doc = "Field `IRQID2` reader - IRQID2"]
pub type IRQID2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IRQID2` writer - IRQID2"]
pub type IRQID2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, R32_PFIC_FIFOADDRR2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:23 - OFFADDR2"]
    #[inline(always)]
    pub fn offaddr2(&self) -> OFFADDR2_R {
        OFFADDR2_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 24:31 - IRQID2"]
    #[inline(always)]
    pub fn irqid2(&self) -> IRQID2_R {
        IRQID2_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - OFFADDR2"]
    #[inline(always)]
    pub fn offaddr2(&mut self) -> OFFADDR2_W<0> {
        OFFADDR2_W::new(self)
    }
    #[doc = "Bits 24:31 - IRQID2"]
    #[inline(always)]
    pub fn irqid2(&mut self) -> IRQID2_W<24> {
        IRQID2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt 2 address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r32_pfic_fifoaddrr2](index.html) module"]
pub struct R32_PFIC_FIFOADDRR2_SPEC;
impl crate::RegisterSpec for R32_PFIC_FIFOADDRR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r32_pfic_fifoaddrr2::R](R) reader structure"]
impl crate::Readable for R32_PFIC_FIFOADDRR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r32_pfic_fifoaddrr2::W](W) writer structure"]
impl crate::Writable for R32_PFIC_FIFOADDRR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R32_PFIC_FIFOADDRR2 to value 0"]
impl crate::Resettable for R32_PFIC_FIFOADDRR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
