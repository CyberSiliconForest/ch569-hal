#[doc = "Register `R8_USB_MIS_ST` reader"]
pub struct R(crate::R<R8_USB_MIS_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_USB_MIS_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_USB_MIS_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_USB_MIS_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RB_USB_SPLIT_EN` reader - RO,indicate host allow SPLIT packet"]
pub type RB_USB_SPLIT_EN_R = crate::BitReader<bool>;
#[doc = "Field `RB_USB_ATTACH` reader - RO, indicate device attached status on USB host"]
pub type RB_USB_ATTACH_R = crate::BitReader<bool>;
#[doc = "Field `RB_USBBUS_SUSPEND` reader - RO, indicate USB suspend status"]
pub type RB_USBBUS_SUSPEND_R = crate::BitReader<bool>;
#[doc = "Field `RB_USBBUS_RESET` reader - RO, indicate USB bus reset status"]
pub type RB_USBBUS_RESET_R = crate::BitReader<bool>;
#[doc = "Field `RB_USB_FIFO_RDY` reader - RO, indicate USB receiving FIFO ready status (not empty)"]
pub type RB_USB_FIFO_RDY_R = crate::BitReader<bool>;
#[doc = "Field `RB_USB_SIE_FREE` reader - RO, indicate USB SIE free status"]
pub type RB_USB_SIE_FREE_R = crate::BitReader<bool>;
#[doc = "Field `RB_USB_SOF_ACT` reader - RO, indicate host SOF timer action status for USB host"]
pub type RB_USB_SOF_ACT_R = crate::BitReader<bool>;
#[doc = "Field `RB_USB_SOF_PRES` reader - RO, indicate host SOF timer presage status"]
pub type RB_USB_SOF_PRES_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - RO,indicate host allow SPLIT packet"]
    #[inline(always)]
    pub fn rb_usb_split_en(&self) -> RB_USB_SPLIT_EN_R {
        RB_USB_SPLIT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RO, indicate device attached status on USB host"]
    #[inline(always)]
    pub fn rb_usb_attach(&self) -> RB_USB_ATTACH_R {
        RB_USB_ATTACH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RO, indicate USB suspend status"]
    #[inline(always)]
    pub fn rb_usbbus_suspend(&self) -> RB_USBBUS_SUSPEND_R {
        RB_USBBUS_SUSPEND_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RO, indicate USB bus reset status"]
    #[inline(always)]
    pub fn rb_usbbus_reset(&self) -> RB_USBBUS_RESET_R {
        RB_USBBUS_RESET_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RO, indicate USB receiving FIFO ready status (not empty)"]
    #[inline(always)]
    pub fn rb_usb_fifo_rdy(&self) -> RB_USB_FIFO_RDY_R {
        RB_USB_FIFO_RDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RO, indicate USB SIE free status"]
    #[inline(always)]
    pub fn rb_usb_sie_free(&self) -> RB_USB_SIE_FREE_R {
        RB_USB_SIE_FREE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RO, indicate host SOF timer action status for USB host"]
    #[inline(always)]
    pub fn rb_usb_sof_act(&self) -> RB_USB_SOF_ACT_R {
        RB_USB_SOF_ACT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RO, indicate host SOF timer presage status"]
    #[inline(always)]
    pub fn rb_usb_sof_pres(&self) -> RB_USB_SOF_PRES_R {
        RB_USB_SOF_PRES_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "USB miscellaneous status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_usb_mis_st](index.html) module"]
pub struct R8_USB_MIS_ST_SPEC;
impl crate::RegisterSpec for R8_USB_MIS_ST_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_usb_mis_st::R](R) reader structure"]
impl crate::Readable for R8_USB_MIS_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets R8_USB_MIS_ST to value 0x20"]
impl crate::Resettable for R8_USB_MIS_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}
