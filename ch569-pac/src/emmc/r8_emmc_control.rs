#[doc = "Register `R8_EMMC_CONTROL` reader"]
pub struct R(crate::R<R8_EMMC_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_EMMC_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_EMMC_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_EMMC_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_EMMC_CONTROL` writer"]
pub struct W(crate::W<R8_EMMC_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_EMMC_CONTROL_SPEC>;
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
impl From<crate::W<R8_EMMC_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_EMMC_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_EMMC_LW_MASK` reader - effctive data width for sending or receiving data"]
pub type RB_EMMC_LW_MASK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RB_EMMC_LW_MASK` writer - effctive data width for sending or receiving data"]
pub type RB_EMMC_LW_MASK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, R8_EMMC_CONTROL_SPEC, u8, u8, 2, O>;
#[doc = "Field `RB_EMMC_ALL_CLR` reader - reset all the inner logic, default is valid"]
pub type RB_EMMC_ALL_CLR_R = crate::BitReader<bool>;
#[doc = "Field `RB_EMMC_ALL_CLR` writer - reset all the inner logic, default is valid"]
pub type RB_EMMC_ALL_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_EMMC_CONTROL_SPEC, bool, O>;
#[doc = "Field `RB_EMMC_DMAEN` reader - enable the dma"]
pub type RB_EMMC_DMAEN_R = crate::BitReader<bool>;
#[doc = "Field `RB_EMMC_DMAEN` writer - enable the dma"]
pub type RB_EMMC_DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_EMMC_CONTROL_SPEC, bool, O>;
#[doc = "Field `RB_EMMC_RST_LGC` reader - reset the data tran/recv logic"]
pub type RB_EMMC_RST_LGC_R = crate::BitReader<bool>;
#[doc = "Field `RB_EMMC_RST_LGC` writer - reset the data tran/recv logic"]
pub type RB_EMMC_RST_LGC_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_EMMC_CONTROL_SPEC, bool, O>;
#[doc = "Field `RB_EMMC_NEGSMP` reader - controller use nagedge sample cmd"]
pub type RB_EMMC_NEGSMP_R = crate::BitReader<bool>;
#[doc = "Field `RB_EMMC_NEGSMP` writer - controller use nagedge sample cmd"]
pub type RB_EMMC_NEGSMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_EMMC_CONTROL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - effctive data width for sending or receiving data"]
    #[inline(always)]
    pub fn rb_emmc_lw_mask(&self) -> RB_EMMC_LW_MASK_R {
        RB_EMMC_LW_MASK_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - reset all the inner logic, default is valid"]
    #[inline(always)]
    pub fn rb_emmc_all_clr(&self) -> RB_EMMC_ALL_CLR_R {
        RB_EMMC_ALL_CLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable the dma"]
    #[inline(always)]
    pub fn rb_emmc_dmaen(&self) -> RB_EMMC_DMAEN_R {
        RB_EMMC_DMAEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - reset the data tran/recv logic"]
    #[inline(always)]
    pub fn rb_emmc_rst_lgc(&self) -> RB_EMMC_RST_LGC_R {
        RB_EMMC_RST_LGC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - controller use nagedge sample cmd"]
    #[inline(always)]
    pub fn rb_emmc_negsmp(&self) -> RB_EMMC_NEGSMP_R {
        RB_EMMC_NEGSMP_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - effctive data width for sending or receiving data"]
    #[inline(always)]
    pub fn rb_emmc_lw_mask(&mut self) -> RB_EMMC_LW_MASK_W<0> {
        RB_EMMC_LW_MASK_W::new(self)
    }
    #[doc = "Bit 2 - reset all the inner logic, default is valid"]
    #[inline(always)]
    pub fn rb_emmc_all_clr(&mut self) -> RB_EMMC_ALL_CLR_W<2> {
        RB_EMMC_ALL_CLR_W::new(self)
    }
    #[doc = "Bit 3 - enable the dma"]
    #[inline(always)]
    pub fn rb_emmc_dmaen(&mut self) -> RB_EMMC_DMAEN_W<3> {
        RB_EMMC_DMAEN_W::new(self)
    }
    #[doc = "Bit 4 - reset the data tran/recv logic"]
    #[inline(always)]
    pub fn rb_emmc_rst_lgc(&mut self) -> RB_EMMC_RST_LGC_W<4> {
        RB_EMMC_RST_LGC_W::new(self)
    }
    #[doc = "Bit 5 - controller use nagedge sample cmd"]
    #[inline(always)]
    pub fn rb_emmc_negsmp(&mut self) -> RB_EMMC_NEGSMP_W<5> {
        RB_EMMC_NEGSMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SD 8bits control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_emmc_control](index.html) module"]
pub struct R8_EMMC_CONTROL_SPEC;
impl crate::RegisterSpec for R8_EMMC_CONTROL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_emmc_control::R](R) reader structure"]
impl crate::Readable for R8_EMMC_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_emmc_control::W](W) writer structure"]
impl crate::Writable for R8_EMMC_CONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_EMMC_CONTROL to value 0x15"]
impl crate::Resettable for R8_EMMC_CONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x15
    }
}
