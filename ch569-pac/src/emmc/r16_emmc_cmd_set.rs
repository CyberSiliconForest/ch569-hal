#[doc = "Register `R16_EMMC_CMD_SET` reader"]
pub struct R(crate::R<R16_EMMC_CMD_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R16_EMMC_CMD_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R16_EMMC_CMD_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R16_EMMC_CMD_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R16_EMMC_CMD_SET` writer"]
pub struct W(crate::W<R16_EMMC_CMD_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R16_EMMC_CMD_SET_SPEC>;
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
impl From<crate::W<R16_EMMC_CMD_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R16_EMMC_CMD_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_EMMC_CMDIDX_MASK` reader - the index number of the currently sent command"]
pub type RB_EMMC_CMDIDX_MASK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RB_EMMC_CMDIDX_MASK` writer - the index number of the currently sent command"]
pub type RB_EMMC_CMDIDX_MASK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, R16_EMMC_CMD_SET_SPEC, u8, u8, 6, O>;
#[doc = "Field `RB_EMMC_RPTY_MASK` reader - current respone type"]
pub type RB_EMMC_RPTY_MASK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RB_EMMC_RPTY_MASK` writer - current respone type"]
pub type RB_EMMC_RPTY_MASK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, R16_EMMC_CMD_SET_SPEC, u8, u8, 2, O>;
#[doc = "Field `RB_EMMC_CKCRC` reader - check the response CRC"]
pub type RB_EMMC_CKCRC_R = crate::BitReader<bool>;
#[doc = "Field `RB_EMMC_CKCRC` writer - check the response CRC"]
pub type RB_EMMC_CKCRC_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, R16_EMMC_CMD_SET_SPEC, bool, O>;
#[doc = "Field `RB_EMMC_CKIDX` reader - check the response command index"]
pub type RB_EMMC_CKIDX_R = crate::BitReader<bool>;
#[doc = "Field `RB_EMMC_CKIDX` writer - check the response command index"]
pub type RB_EMMC_CKIDX_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, R16_EMMC_CMD_SET_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - the index number of the currently sent command"]
    #[inline(always)]
    pub fn rb_emmc_cmdidx_mask(&self) -> RB_EMMC_CMDIDX_MASK_R {
        RB_EMMC_CMDIDX_MASK_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:9 - current respone type"]
    #[inline(always)]
    pub fn rb_emmc_rpty_mask(&self) -> RB_EMMC_RPTY_MASK_R {
        RB_EMMC_RPTY_MASK_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - check the response CRC"]
    #[inline(always)]
    pub fn rb_emmc_ckcrc(&self) -> RB_EMMC_CKCRC_R {
        RB_EMMC_CKCRC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - check the response command index"]
    #[inline(always)]
    pub fn rb_emmc_ckidx(&self) -> RB_EMMC_CKIDX_R {
        RB_EMMC_CKIDX_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - the index number of the currently sent command"]
    #[inline(always)]
    pub fn rb_emmc_cmdidx_mask(&mut self) -> RB_EMMC_CMDIDX_MASK_W<0> {
        RB_EMMC_CMDIDX_MASK_W::new(self)
    }
    #[doc = "Bits 8:9 - current respone type"]
    #[inline(always)]
    pub fn rb_emmc_rpty_mask(&mut self) -> RB_EMMC_RPTY_MASK_W<8> {
        RB_EMMC_RPTY_MASK_W::new(self)
    }
    #[doc = "Bit 10 - check the response CRC"]
    #[inline(always)]
    pub fn rb_emmc_ckcrc(&mut self) -> RB_EMMC_CKCRC_W<10> {
        RB_EMMC_CKCRC_W::new(self)
    }
    #[doc = "Bit 11 - check the response command index"]
    #[inline(always)]
    pub fn rb_emmc_ckidx(&mut self) -> RB_EMMC_CKIDX_W<11> {
        RB_EMMC_CKIDX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SD 16bits cmd setting register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r16_emmc_cmd_set](index.html) module"]
pub struct R16_EMMC_CMD_SET_SPEC;
impl crate::RegisterSpec for R16_EMMC_CMD_SET_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [r16_emmc_cmd_set::R](R) reader structure"]
impl crate::Readable for R16_EMMC_CMD_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r16_emmc_cmd_set::W](W) writer structure"]
impl crate::Writable for R16_EMMC_CMD_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R16_EMMC_CMD_SET to value 0"]
impl crate::Resettable for R16_EMMC_CMD_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
