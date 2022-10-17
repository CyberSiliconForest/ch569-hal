#[doc = "Register `R8_UART0_MSR` reader"]
pub struct R(crate::R<R8_UART0_MSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_UART0_MSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_UART0_MSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_UART0_MSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RB_MSR_CTS_CHG` reader - UART0 CTS changed status, high action"]
pub type RB_MSR_CTS_CHG_R = crate::BitReader<bool>;
#[doc = "Field `RB_MSR_DSR_CHG` reader - UART0 DSR changed status, high action"]
pub type RB_MSR_DSR_CHG_R = crate::BitReader<bool>;
#[doc = "Field `RB_MSR_RI_CHG` reader - UART0 RI changed status, high action"]
pub type RB_MSR_RI_CHG_R = crate::BitReader<bool>;
#[doc = "Field `RB_MSR_DCD_CHG` reader - UART0 DCD changed status, high action"]
pub type RB_MSR_DCD_CHG_R = crate::BitReader<bool>;
#[doc = "Field `RB_MSR_CTS` reader - UART0 CTS action status"]
pub type RB_MSR_CTS_R = crate::BitReader<bool>;
#[doc = "Field `RB_MSR_DSR` reader - UART0 DSR action status"]
pub type RB_MSR_DSR_R = crate::BitReader<bool>;
#[doc = "Field `RB_MSR_RI` reader - UART0 RI action status"]
pub type RB_MSR_RI_R = crate::BitReader<bool>;
#[doc = "Field `RB_MSR_DCD` reader - UART0 DCD action status"]
pub type RB_MSR_DCD_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - UART0 CTS changed status, high action"]
    #[inline(always)]
    pub fn rb_msr_cts_chg(&self) -> RB_MSR_CTS_CHG_R {
        RB_MSR_CTS_CHG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART0 DSR changed status, high action"]
    #[inline(always)]
    pub fn rb_msr_dsr_chg(&self) -> RB_MSR_DSR_CHG_R {
        RB_MSR_DSR_CHG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART0 RI changed status, high action"]
    #[inline(always)]
    pub fn rb_msr_ri_chg(&self) -> RB_MSR_RI_CHG_R {
        RB_MSR_RI_CHG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART0 DCD changed status, high action"]
    #[inline(always)]
    pub fn rb_msr_dcd_chg(&self) -> RB_MSR_DCD_CHG_R {
        RB_MSR_DCD_CHG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART0 CTS action status"]
    #[inline(always)]
    pub fn rb_msr_cts(&self) -> RB_MSR_CTS_R {
        RB_MSR_CTS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART0 DSR action status"]
    #[inline(always)]
    pub fn rb_msr_dsr(&self) -> RB_MSR_DSR_R {
        RB_MSR_DSR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART0 RI action status"]
    #[inline(always)]
    pub fn rb_msr_ri(&self) -> RB_MSR_RI_R {
        RB_MSR_RI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART0 DCD action status"]
    #[inline(always)]
    pub fn rb_msr_dcd(&self) -> RB_MSR_DCD_R {
        RB_MSR_DCD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "UART0 modem status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_uart0_msr](index.html) module"]
pub struct R8_UART0_MSR_SPEC;
impl crate::RegisterSpec for R8_UART0_MSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_uart0_msr::R](R) reader structure"]
impl crate::Readable for R8_UART0_MSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets R8_UART0_MSR to value 0"]
impl crate::Resettable for R8_UART0_MSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
