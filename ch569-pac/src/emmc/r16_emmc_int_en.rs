#[doc = "Register `R16_EMMC_INT_EN` reader"]
pub struct R(crate::R<R16_EMMC_INT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R16_EMMC_INT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R16_EMMC_INT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R16_EMMC_INT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R16_EMMC_INT_EN` writer"]
pub struct W(crate::W<R16_EMMC_INT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R16_EMMC_INT_EN_SPEC>;
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
impl From<crate::W<R16_EMMC_INT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R16_EMMC_INT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_EMMC_IE_RE_TMOUT` reader - command response timeout interrupt enable"]
pub type RB_EMMC_IE_RE_TMOUT_R = crate::BitReader<bool>;
#[doc = "Field `RB_EMMC_IE_RE_TMOUT` writer - command response timeout interrupt enable"]
pub type RB_EMMC_IE_RE_TMOUT_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, R16_EMMC_INT_EN_SPEC, bool, O>;
#[doc = "Field `RB_EMMC_IE_RECRC_WR` reader - response CRC check error interrupt enable"]
pub type RB_EMMC_IE_RECRC_WR_R = crate::BitReader<bool>;
#[doc = "Field `RB_EMMC_IE_RECRC_WR` writer - response CRC check error interrupt enable"]
pub type RB_EMMC_IE_RECRC_WR_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, R16_EMMC_INT_EN_SPEC, bool, O>;
#[doc = "Field `RB_EMMC_IE_REIDX_ER` reader - response index check error interrupt enable"]
pub type RB_EMMC_IE_REIDX_ER_R = crate::BitReader<bool>;
#[doc = "Field `RB_EMMC_IE_REIDX_ER` writer - response index check error interrupt enable"]
pub type RB_EMMC_IE_REIDX_ER_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, R16_EMMC_INT_EN_SPEC, bool, O>;
#[doc = "Field `RB_EMMC_IE_CMDDONE` reader - command completion interrupt enable"]
pub type RB_EMMC_IE_CMDDONE_R = crate::BitReader<bool>;
#[doc = "Field `RB_EMMC_IE_CMDDONE` writer - command completion interrupt enable"]
pub type RB_EMMC_IE_CMDDONE_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, R16_EMMC_INT_EN_SPEC, bool, O>;
#[doc = "Field `RB_EMMC_IE_DATTMO` reader - data timeout interrupt enable"]
pub type RB_EMMC_IE_DATTMO_R = crate::BitReader<bool>;
#[doc = "Field `RB_EMMC_IE_DATTMO` writer - data timeout interrupt enable"]
pub type RB_EMMC_IE_DATTMO_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, R16_EMMC_INT_EN_SPEC, bool, O>;
#[doc = "Field `RB_EMMC_IE_TRANERR` reader - blocks transfer CRC error interrupt enable"]
pub type RB_EMMC_IE_TRANERR_R = crate::BitReader<bool>;
#[doc = "Field `RB_EMMC_IE_TRANERR` writer - blocks transfer CRC error interrupt enable"]
pub type RB_EMMC_IE_TRANERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, R16_EMMC_INT_EN_SPEC, bool, O>;
#[doc = "Field `RB_EMMC_IE_TRANDONE` reader - all blocks transfer complete interrupt enable"]
pub type RB_EMMC_IE_TRANDONE_R = crate::BitReader<bool>;
#[doc = "Field `RB_EMMC_IE_TRANDONE` writer - all blocks transfer complete interrupt enable"]
pub type RB_EMMC_IE_TRANDONE_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, R16_EMMC_INT_EN_SPEC, bool, O>;
#[doc = "Field `RB_EMMC_IE_BKGAP` reader - single block transmission completion interrupt enable"]
pub type RB_EMMC_IE_BKGAP_R = crate::BitReader<bool>;
#[doc = "Field `RB_EMMC_IE_BKGAP` writer - single block transmission completion interrupt enable"]
pub type RB_EMMC_IE_BKGAP_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, R16_EMMC_INT_EN_SPEC, bool, O>;
#[doc = "Field `RB_EMMC_IE_FIFO_OV` reader - FIFO overflow interrupt enable"]
pub type RB_EMMC_IE_FIFO_OV_R = crate::BitReader<bool>;
#[doc = "Field `RB_EMMC_IE_FIFO_OV` writer - FIFO overflow interrupt enable"]
pub type RB_EMMC_IE_FIFO_OV_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, R16_EMMC_INT_EN_SPEC, bool, O>;
#[doc = "Field `RB_EMMC_IE_SDIOINT` reader - SDIO card interrupt enable"]
pub type RB_EMMC_IE_SDIOINT_R = crate::BitReader<bool>;
#[doc = "Field `RB_EMMC_IE_SDIOINT` writer - SDIO card interrupt enable"]
pub type RB_EMMC_IE_SDIOINT_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, R16_EMMC_INT_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - command response timeout interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_re_tmout(&self) -> RB_EMMC_IE_RE_TMOUT_R {
        RB_EMMC_IE_RE_TMOUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - response CRC check error interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_recrc_wr(&self) -> RB_EMMC_IE_RECRC_WR_R {
        RB_EMMC_IE_RECRC_WR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - response index check error interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_reidx_er(&self) -> RB_EMMC_IE_REIDX_ER_R {
        RB_EMMC_IE_REIDX_ER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - command completion interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_cmddone(&self) -> RB_EMMC_IE_CMDDONE_R {
        RB_EMMC_IE_CMDDONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - data timeout interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_dattmo(&self) -> RB_EMMC_IE_DATTMO_R {
        RB_EMMC_IE_DATTMO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - blocks transfer CRC error interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_tranerr(&self) -> RB_EMMC_IE_TRANERR_R {
        RB_EMMC_IE_TRANERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - all blocks transfer complete interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_trandone(&self) -> RB_EMMC_IE_TRANDONE_R {
        RB_EMMC_IE_TRANDONE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - single block transmission completion interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_bkgap(&self) -> RB_EMMC_IE_BKGAP_R {
        RB_EMMC_IE_BKGAP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FIFO overflow interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_fifo_ov(&self) -> RB_EMMC_IE_FIFO_OV_R {
        RB_EMMC_IE_FIFO_OV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SDIO card interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_sdioint(&self) -> RB_EMMC_IE_SDIOINT_R {
        RB_EMMC_IE_SDIOINT_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - command response timeout interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_re_tmout(&mut self) -> RB_EMMC_IE_RE_TMOUT_W<0> {
        RB_EMMC_IE_RE_TMOUT_W::new(self)
    }
    #[doc = "Bit 1 - response CRC check error interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_recrc_wr(&mut self) -> RB_EMMC_IE_RECRC_WR_W<1> {
        RB_EMMC_IE_RECRC_WR_W::new(self)
    }
    #[doc = "Bit 2 - response index check error interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_reidx_er(&mut self) -> RB_EMMC_IE_REIDX_ER_W<2> {
        RB_EMMC_IE_REIDX_ER_W::new(self)
    }
    #[doc = "Bit 3 - command completion interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_cmddone(&mut self) -> RB_EMMC_IE_CMDDONE_W<3> {
        RB_EMMC_IE_CMDDONE_W::new(self)
    }
    #[doc = "Bit 4 - data timeout interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_dattmo(&mut self) -> RB_EMMC_IE_DATTMO_W<4> {
        RB_EMMC_IE_DATTMO_W::new(self)
    }
    #[doc = "Bit 5 - blocks transfer CRC error interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_tranerr(&mut self) -> RB_EMMC_IE_TRANERR_W<5> {
        RB_EMMC_IE_TRANERR_W::new(self)
    }
    #[doc = "Bit 6 - all blocks transfer complete interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_trandone(&mut self) -> RB_EMMC_IE_TRANDONE_W<6> {
        RB_EMMC_IE_TRANDONE_W::new(self)
    }
    #[doc = "Bit 7 - single block transmission completion interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_bkgap(&mut self) -> RB_EMMC_IE_BKGAP_W<7> {
        RB_EMMC_IE_BKGAP_W::new(self)
    }
    #[doc = "Bit 8 - FIFO overflow interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_fifo_ov(&mut self) -> RB_EMMC_IE_FIFO_OV_W<8> {
        RB_EMMC_IE_FIFO_OV_W::new(self)
    }
    #[doc = "Bit 9 - SDIO card interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_sdioint(&mut self) -> RB_EMMC_IE_SDIOINT_W<9> {
        RB_EMMC_IE_SDIOINT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SD 16bits interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r16_emmc_int_en](index.html) module"]
pub struct R16_EMMC_INT_EN_SPEC;
impl crate::RegisterSpec for R16_EMMC_INT_EN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [r16_emmc_int_en::R](R) reader structure"]
impl crate::Readable for R16_EMMC_INT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r16_emmc_int_en::W](W) writer structure"]
impl crate::Writable for R16_EMMC_INT_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R16_EMMC_INT_EN to value 0"]
impl crate::Resettable for R16_EMMC_INT_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
