#[doc = "Register `R32_EMMC_STATUS` reader"]
pub struct R(crate::R<R32_EMMC_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R32_EMMC_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R32_EMMC_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R32_EMMC_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MASK_BLOCK_NUM` reader - the number of blocks successfully transmitted in the current multi-block transmission"]
pub type MASK_BLOCK_NUM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RB_EMMC_CMDSTA` reader - indicate cmd line is high level now"]
pub type RB_EMMC_CMDSTA_R = crate::BitReader<bool>;
#[doc = "Field `RB_EMMC_DAT0STA` reader - indicate dat\\[0\\]
line is high level now"]
pub type RB_EMMC_DAT0STA_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:15 - the number of blocks successfully transmitted in the current multi-block transmission"]
    #[inline(always)]
    pub fn mask_block_num(&self) -> MASK_BLOCK_NUM_R {
        MASK_BLOCK_NUM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - indicate cmd line is high level now"]
    #[inline(always)]
    pub fn rb_emmc_cmdsta(&self) -> RB_EMMC_CMDSTA_R {
        RB_EMMC_CMDSTA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - indicate dat\\[0\\]
line is high level now"]
    #[inline(always)]
    pub fn rb_emmc_dat0sta(&self) -> RB_EMMC_DAT0STA_R {
        RB_EMMC_DAT0STA_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "SD status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r32_emmc_status](index.html) module"]
pub struct R32_EMMC_STATUS_SPEC;
impl crate::RegisterSpec for R32_EMMC_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r32_emmc_status::R](R) reader structure"]
impl crate::Readable for R32_EMMC_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets R32_EMMC_STATUS to value 0"]
impl crate::Resettable for R32_EMMC_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
