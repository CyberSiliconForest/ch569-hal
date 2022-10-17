#[doc = "Register `R8_USB_DEV_AD` reader"]
pub struct R(crate::R<R8_USB_DEV_AD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_USB_DEV_AD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_USB_DEV_AD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_USB_DEV_AD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_USB_DEV_AD` writer"]
pub struct W(crate::W<R8_USB_DEV_AD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_USB_DEV_AD_SPEC>;
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
impl From<crate::W<R8_USB_DEV_AD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_USB_DEV_AD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_ADDR_MASK` reader - bit mask for USB device address"]
pub type USB_ADDR_MASK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_ADDR_MASK` writer - bit mask for USB device address"]
pub type USB_ADDR_MASK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, R8_USB_DEV_AD_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - bit mask for USB device address"]
    #[inline(always)]
    pub fn usb_addr_mask(&self) -> USB_ADDR_MASK_R {
        USB_ADDR_MASK_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - bit mask for USB device address"]
    #[inline(always)]
    pub fn usb_addr_mask(&mut self) -> USB_ADDR_MASK_W<0> {
        USB_ADDR_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB device address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_usb_dev_ad](index.html) module"]
pub struct R8_USB_DEV_AD_SPEC;
impl crate::RegisterSpec for R8_USB_DEV_AD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_usb_dev_ad::R](R) reader structure"]
impl crate::Readable for R8_USB_DEV_AD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_usb_dev_ad::W](W) writer structure"]
impl crate::Writable for R8_USB_DEV_AD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_USB_DEV_AD to value 0"]
impl crate::Resettable for R8_USB_DEV_AD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
