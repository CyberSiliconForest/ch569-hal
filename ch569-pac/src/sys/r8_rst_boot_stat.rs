#[doc = "Register `R8_RST_BOOT_STAT` reader"]
pub struct R(crate::R<R8_RST_BOOT_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_RST_BOOT_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_RST_BOOT_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_RST_BOOT_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RB_RESET_FLAG` reader - recent reset flag"]
pub type RB_RESET_FLAG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RB_CFG_RESET_EN` reader - manual reset input enable status"]
pub type RB_CFG_RESET_EN_R = crate::BitReader<bool>;
#[doc = "Field `RB_CFG_BOOT_EN` reader - boot-loader enable status"]
pub type RB_CFG_BOOT_EN_R = crate::BitReader<bool>;
#[doc = "Field `RB_CFG_DEBUG_EN` reader - debug enable status"]
pub type RB_CFG_DEBUG_EN_R = crate::BitReader<bool>;
#[doc = "Field `RB_BOOT_LOADER` reader - indicate boot loader status"]
pub type RB_BOOT_LOADER_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:1 - recent reset flag"]
    #[inline(always)]
    pub fn rb_reset_flag(&self) -> RB_RESET_FLAG_R {
        RB_RESET_FLAG_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - manual reset input enable status"]
    #[inline(always)]
    pub fn rb_cfg_reset_en(&self) -> RB_CFG_RESET_EN_R {
        RB_CFG_RESET_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - boot-loader enable status"]
    #[inline(always)]
    pub fn rb_cfg_boot_en(&self) -> RB_CFG_BOOT_EN_R {
        RB_CFG_BOOT_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - debug enable status"]
    #[inline(always)]
    pub fn rb_cfg_debug_en(&self) -> RB_CFG_DEBUG_EN_R {
        RB_CFG_DEBUG_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - indicate boot loader status"]
    #[inline(always)]
    pub fn rb_boot_loader(&self) -> RB_BOOT_LOADER_R {
        RB_BOOT_LOADER_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "reset status and boot/debug status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_rst_boot_stat](index.html) module"]
pub struct R8_RST_BOOT_STAT_SPEC;
impl crate::RegisterSpec for R8_RST_BOOT_STAT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_rst_boot_stat::R](R) reader structure"]
impl crate::Readable for R8_RST_BOOT_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets R8_RST_BOOT_STAT to value 0xc8"]
impl crate::Resettable for R8_RST_BOOT_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc8
    }
}
