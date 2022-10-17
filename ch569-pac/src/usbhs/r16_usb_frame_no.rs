#[doc = "Register `R16_USB_FRAME_NO` reader"]
pub struct R(crate::R<R16_USB_FRAME_NO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R16_USB_FRAME_NO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R16_USB_FRAME_NO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R16_USB_FRAME_NO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USB_FRAME_NO` reader - USB frame number"]
pub type USB_FRAME_NO_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - USB frame number"]
    #[inline(always)]
    pub fn usb_frame_no(&self) -> USB_FRAME_NO_R {
        USB_FRAME_NO_R::new(self.bits)
    }
}
#[doc = "USB frame number register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r16_usb_frame_no](index.html) module"]
pub struct R16_USB_FRAME_NO_SPEC;
impl crate::RegisterSpec for R16_USB_FRAME_NO_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [r16_usb_frame_no::R](R) reader structure"]
impl crate::Readable for R16_USB_FRAME_NO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets R16_USB_FRAME_NO to value 0"]
impl crate::Resettable for R16_USB_FRAME_NO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
