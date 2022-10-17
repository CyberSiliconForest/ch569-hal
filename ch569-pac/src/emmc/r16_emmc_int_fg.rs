#[doc = "Register `R16_EMMC_INT_FG` reader"]
pub struct R(crate::R<R16_EMMC_INT_FG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R16_EMMC_INT_FG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R16_EMMC_INT_FG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R16_EMMC_INT_FG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R16_EMMC_INT_FG` writer"]
pub struct W(crate::W<R16_EMMC_INT_FG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R16_EMMC_INT_FG_SPEC>;
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
impl From<crate::W<R16_EMMC_INT_FG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R16_EMMC_INT_FG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_EMMC_IF_RE_TMOUT` reader - indicate when expect the response, timeout"]
pub type RB_EMMC_IF_RE_TMOUT_R = crate::BitReader<bool>;
#[doc = "Field `RB_EMMC_IF_RE_TMOUT` writer - indicate when expect the response, timeout"]
pub type RB_EMMC_IF_RE_TMOUT_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, R16_EMMC_INT_FG_SPEC, bool, O>;
#[doc = "Field `RB_EMMC_IF_RECRC_WR` reader - indicate CRC error of the response"]
pub type RB_EMMC_IF_RECRC_WR_R = crate::BitReader<bool>;
#[doc = "Field `RB_EMMC_IF_RECRC_WR` writer - indicate CRC error of the response"]
pub type RB_EMMC_IF_RECRC_WR_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, R16_EMMC_INT_FG_SPEC, bool, O>;
#[doc = "Field `RB_EMMC_IF_REIDX_ER` reader - indicate INDEX error of the response"]
pub type RB_EMMC_IF_REIDX_ER_R = crate::BitReader<bool>;
#[doc = "Field `RB_EMMC_IF_REIDX_ER` writer - indicate INDEX error of the response"]
pub type RB_EMMC_IF_REIDX_ER_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, R16_EMMC_INT_FG_SPEC, bool, O>;
#[doc = "Field `RB_EMMC_IF_CMDDONE` reader - when cmd hasn't response, indicate cmd has been sent, when cmd has a response, indicate cmd has bee sent and has received the response"]
pub type RB_EMMC_IF_CMDDONE_R = crate::BitReader<bool>;
#[doc = "Field `RB_EMMC_IF_CMDDONE` writer - when cmd hasn't response, indicate cmd has been sent, when cmd has a response, indicate cmd has bee sent and has received the response"]
pub type RB_EMMC_IF_CMDDONE_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, R16_EMMC_INT_FG_SPEC, bool, O>;
#[doc = "Field `RB_EMMC_IF_DATTMO` reader - data line busy timeout"]
pub type RB_EMMC_IF_DATTMO_R = crate::BitReader<bool>;
#[doc = "Field `RB_EMMC_IF_DATTMO` writer - data line busy timeout"]
pub type RB_EMMC_IF_DATTMO_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, R16_EMMC_INT_FG_SPEC, bool, O>;
#[doc = "Field `RB_EMMC_IF_TRANERR` reader - last block have encountered a CRC error"]
pub type RB_EMMC_IF_TRANERR_R = crate::BitReader<bool>;
#[doc = "Field `RB_EMMC_IF_TRANERR` writer - last block have encountered a CRC error"]
pub type RB_EMMC_IF_TRANERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, R16_EMMC_INT_FG_SPEC, bool, O>;
#[doc = "Field `RB_EMMC_IF_TRANDONE` reader - all the blocks have been tran/recv successfully"]
pub type RB_EMMC_IF_TRANDONE_R = crate::BitReader<bool>;
#[doc = "Field `RB_EMMC_IF_TRANDONE` writer - all the blocks have been tran/recv successfully"]
pub type RB_EMMC_IF_TRANDONE_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, R16_EMMC_INT_FG_SPEC, bool, O>;
#[doc = "Field `RB_EMMC_IF_BKGAP` reader - every block gap interrupt when multiple read or write, allow drive change the DMA address at this moment"]
pub type RB_EMMC_IF_BKGAP_R = crate::BitReader<bool>;
#[doc = "Field `RB_EMMC_IF_BKGAP` writer - every block gap interrupt when multiple read or write, allow drive change the DMA address at this moment"]
pub type RB_EMMC_IF_BKGAP_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, R16_EMMC_INT_FG_SPEC, bool, O>;
#[doc = "Field `RB_EMMC_IF_FIFO_OV` reader - fifo overflow, when write sd, indicate empty overflow, when read sd, indicate full overflow"]
pub type RB_EMMC_IF_FIFO_OV_R = crate::BitReader<bool>;
#[doc = "Field `RB_EMMC_IF_FIFO_OV` writer - fifo overflow, when write sd, indicate empty overflow, when read sd, indicate full overflow"]
pub type RB_EMMC_IF_FIFO_OV_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, R16_EMMC_INT_FG_SPEC, bool, O>;
#[doc = "Field `RB_EMMC_IF_SDIOINT` reader - interrupt from SDIO card inside"]
pub type RB_EMMC_IF_SDIOINT_R = crate::BitReader<bool>;
#[doc = "Field `RB_EMMC_IF_SDIOINT` writer - interrupt from SDIO card inside"]
pub type RB_EMMC_IF_SDIOINT_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, R16_EMMC_INT_FG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - indicate when expect the response, timeout"]
    #[inline(always)]
    pub fn rb_emmc_if_re_tmout(&self) -> RB_EMMC_IF_RE_TMOUT_R {
        RB_EMMC_IF_RE_TMOUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - indicate CRC error of the response"]
    #[inline(always)]
    pub fn rb_emmc_if_recrc_wr(&self) -> RB_EMMC_IF_RECRC_WR_R {
        RB_EMMC_IF_RECRC_WR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - indicate INDEX error of the response"]
    #[inline(always)]
    pub fn rb_emmc_if_reidx_er(&self) -> RB_EMMC_IF_REIDX_ER_R {
        RB_EMMC_IF_REIDX_ER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - when cmd hasn't response, indicate cmd has been sent, when cmd has a response, indicate cmd has bee sent and has received the response"]
    #[inline(always)]
    pub fn rb_emmc_if_cmddone(&self) -> RB_EMMC_IF_CMDDONE_R {
        RB_EMMC_IF_CMDDONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - data line busy timeout"]
    #[inline(always)]
    pub fn rb_emmc_if_dattmo(&self) -> RB_EMMC_IF_DATTMO_R {
        RB_EMMC_IF_DATTMO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - last block have encountered a CRC error"]
    #[inline(always)]
    pub fn rb_emmc_if_tranerr(&self) -> RB_EMMC_IF_TRANERR_R {
        RB_EMMC_IF_TRANERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - all the blocks have been tran/recv successfully"]
    #[inline(always)]
    pub fn rb_emmc_if_trandone(&self) -> RB_EMMC_IF_TRANDONE_R {
        RB_EMMC_IF_TRANDONE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - every block gap interrupt when multiple read or write, allow drive change the DMA address at this moment"]
    #[inline(always)]
    pub fn rb_emmc_if_bkgap(&self) -> RB_EMMC_IF_BKGAP_R {
        RB_EMMC_IF_BKGAP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - fifo overflow, when write sd, indicate empty overflow, when read sd, indicate full overflow"]
    #[inline(always)]
    pub fn rb_emmc_if_fifo_ov(&self) -> RB_EMMC_IF_FIFO_OV_R {
        RB_EMMC_IF_FIFO_OV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - interrupt from SDIO card inside"]
    #[inline(always)]
    pub fn rb_emmc_if_sdioint(&self) -> RB_EMMC_IF_SDIOINT_R {
        RB_EMMC_IF_SDIOINT_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - indicate when expect the response, timeout"]
    #[inline(always)]
    pub fn rb_emmc_if_re_tmout(&mut self) -> RB_EMMC_IF_RE_TMOUT_W<0> {
        RB_EMMC_IF_RE_TMOUT_W::new(self)
    }
    #[doc = "Bit 1 - indicate CRC error of the response"]
    #[inline(always)]
    pub fn rb_emmc_if_recrc_wr(&mut self) -> RB_EMMC_IF_RECRC_WR_W<1> {
        RB_EMMC_IF_RECRC_WR_W::new(self)
    }
    #[doc = "Bit 2 - indicate INDEX error of the response"]
    #[inline(always)]
    pub fn rb_emmc_if_reidx_er(&mut self) -> RB_EMMC_IF_REIDX_ER_W<2> {
        RB_EMMC_IF_REIDX_ER_W::new(self)
    }
    #[doc = "Bit 3 - when cmd hasn't response, indicate cmd has been sent, when cmd has a response, indicate cmd has bee sent and has received the response"]
    #[inline(always)]
    pub fn rb_emmc_if_cmddone(&mut self) -> RB_EMMC_IF_CMDDONE_W<3> {
        RB_EMMC_IF_CMDDONE_W::new(self)
    }
    #[doc = "Bit 4 - data line busy timeout"]
    #[inline(always)]
    pub fn rb_emmc_if_dattmo(&mut self) -> RB_EMMC_IF_DATTMO_W<4> {
        RB_EMMC_IF_DATTMO_W::new(self)
    }
    #[doc = "Bit 5 - last block have encountered a CRC error"]
    #[inline(always)]
    pub fn rb_emmc_if_tranerr(&mut self) -> RB_EMMC_IF_TRANERR_W<5> {
        RB_EMMC_IF_TRANERR_W::new(self)
    }
    #[doc = "Bit 6 - all the blocks have been tran/recv successfully"]
    #[inline(always)]
    pub fn rb_emmc_if_trandone(&mut self) -> RB_EMMC_IF_TRANDONE_W<6> {
        RB_EMMC_IF_TRANDONE_W::new(self)
    }
    #[doc = "Bit 7 - every block gap interrupt when multiple read or write, allow drive change the DMA address at this moment"]
    #[inline(always)]
    pub fn rb_emmc_if_bkgap(&mut self) -> RB_EMMC_IF_BKGAP_W<7> {
        RB_EMMC_IF_BKGAP_W::new(self)
    }
    #[doc = "Bit 8 - fifo overflow, when write sd, indicate empty overflow, when read sd, indicate full overflow"]
    #[inline(always)]
    pub fn rb_emmc_if_fifo_ov(&mut self) -> RB_EMMC_IF_FIFO_OV_W<8> {
        RB_EMMC_IF_FIFO_OV_W::new(self)
    }
    #[doc = "Bit 9 - interrupt from SDIO card inside"]
    #[inline(always)]
    pub fn rb_emmc_if_sdioint(&mut self) -> RB_EMMC_IF_SDIOINT_W<9> {
        RB_EMMC_IF_SDIOINT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SD 16bits interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r16_emmc_int_fg](index.html) module"]
pub struct R16_EMMC_INT_FG_SPEC;
impl crate::RegisterSpec for R16_EMMC_INT_FG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [r16_emmc_int_fg::R](R) reader structure"]
impl crate::Readable for R16_EMMC_INT_FG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r16_emmc_int_fg::W](W) writer structure"]
impl crate::Writable for R16_EMMC_INT_FG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R16_EMMC_INT_FG to value 0"]
impl crate::Resettable for R16_EMMC_INT_FG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
