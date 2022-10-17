#[doc = "Register `R8_USB_INT_EN` reader"]
pub struct R(crate::R<R8_USB_INT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_USB_INT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_USB_INT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_USB_INT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_USB_INT_EN` writer"]
pub struct W(crate::W<R8_USB_INT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_USB_INT_EN_SPEC>;
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
impl From<crate::W<R8_USB_INT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_USB_INT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_USB_IE_BUSRST_RB_USB_IE_DETECT` reader - enable interrupt for USB bus reset event for USB device mode _ enable interrupt for USB device detected event for USB host mode"]
pub type RB_USB_IE_BUSRST_RB_USB_IE_DETECT_R = crate::BitReader<bool>;
#[doc = "Field `RB_USB_IE_BUSRST_RB_USB_IE_DETECT` writer - enable interrupt for USB bus reset event for USB device mode _ enable interrupt for USB device detected event for USB host mode"]
pub type RB_USB_IE_BUSRST_RB_USB_IE_DETECT_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_USB_INT_EN_SPEC, bool, O>;
#[doc = "Field `RB_USB_IE_TRANS` reader - enable interrupt for USB transfer completion"]
pub type RB_USB_IE_TRANS_R = crate::BitReader<bool>;
#[doc = "Field `RB_USB_IE_TRANS` writer - enable interrupt for USB transfer completion"]
pub type RB_USB_IE_TRANS_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_USB_INT_EN_SPEC, bool, O>;
#[doc = "Field `RB_USB_IE_SUSPEND` reader - enable interrupt for USB suspend or resume event"]
pub type RB_USB_IE_SUSPEND_R = crate::BitReader<bool>;
#[doc = "Field `RB_USB_IE_SUSPEND` writer - enable interrupt for USB suspend or resume event"]
pub type RB_USB_IE_SUSPEND_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_USB_INT_EN_SPEC, bool, O>;
#[doc = "Field `RB_USB_IE_SOF` reader - enable interrupt for host SOF timer action for USB host mode"]
pub type RB_USB_IE_SOF_R = crate::BitReader<bool>;
#[doc = "Field `RB_USB_IE_SOF` writer - enable interrupt for host SOF timer action for USB host mode"]
pub type RB_USB_IE_SOF_W<'a, const O: u8> = crate::BitWriter<'a, u8, R8_USB_INT_EN_SPEC, bool, O>;
#[doc = "Field `RB_USB_IE_FIFOOV` reader - enable interrupt for FIFO overflow"]
pub type RB_USB_IE_FIFOOV_R = crate::BitReader<bool>;
#[doc = "Field `RB_USB_IE_FIFOOV` writer - enable interrupt for FIFO overflow"]
pub type RB_USB_IE_FIFOOV_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_USB_INT_EN_SPEC, bool, O>;
#[doc = "Field `RB_USB_IE_SETUPACT` reader - Setup packet end interrupt"]
pub type RB_USB_IE_SETUPACT_R = crate::BitReader<bool>;
#[doc = "Field `RB_USB_IE_SETUPACT` writer - Setup packet end interrupt"]
pub type RB_USB_IE_SETUPACT_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_USB_INT_EN_SPEC, bool, O>;
#[doc = "Field `RB_USB_IE_ISOACT` reader - Synchronous transmission received control token packet interrupt"]
pub type RB_USB_IE_ISOACT_R = crate::BitReader<bool>;
#[doc = "Field `RB_USB_IE_ISOACT` writer - Synchronous transmission received control token packet interrupt"]
pub type RB_USB_IE_ISOACT_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_USB_INT_EN_SPEC, bool, O>;
#[doc = "Field `RB_USB_IE_DEV_NAK` reader - enable interrupt for NAK responded for USB device mode"]
pub type RB_USB_IE_DEV_NAK_R = crate::BitReader<bool>;
#[doc = "Field `RB_USB_IE_DEV_NAK` writer - enable interrupt for NAK responded for USB device mode"]
pub type RB_USB_IE_DEV_NAK_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_USB_INT_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - enable interrupt for USB bus reset event for USB device mode _ enable interrupt for USB device detected event for USB host mode"]
    #[inline(always)]
    pub fn rb_usb_ie_busrst_rb_usb_ie_detect(&self) -> RB_USB_IE_BUSRST_RB_USB_IE_DETECT_R {
        RB_USB_IE_BUSRST_RB_USB_IE_DETECT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable interrupt for USB transfer completion"]
    #[inline(always)]
    pub fn rb_usb_ie_trans(&self) -> RB_USB_IE_TRANS_R {
        RB_USB_IE_TRANS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable interrupt for USB suspend or resume event"]
    #[inline(always)]
    pub fn rb_usb_ie_suspend(&self) -> RB_USB_IE_SUSPEND_R {
        RB_USB_IE_SUSPEND_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable interrupt for host SOF timer action for USB host mode"]
    #[inline(always)]
    pub fn rb_usb_ie_sof(&self) -> RB_USB_IE_SOF_R {
        RB_USB_IE_SOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - enable interrupt for FIFO overflow"]
    #[inline(always)]
    pub fn rb_usb_ie_fifoov(&self) -> RB_USB_IE_FIFOOV_R {
        RB_USB_IE_FIFOOV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Setup packet end interrupt"]
    #[inline(always)]
    pub fn rb_usb_ie_setupact(&self) -> RB_USB_IE_SETUPACT_R {
        RB_USB_IE_SETUPACT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Synchronous transmission received control token packet interrupt"]
    #[inline(always)]
    pub fn rb_usb_ie_isoact(&self) -> RB_USB_IE_ISOACT_R {
        RB_USB_IE_ISOACT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - enable interrupt for NAK responded for USB device mode"]
    #[inline(always)]
    pub fn rb_usb_ie_dev_nak(&self) -> RB_USB_IE_DEV_NAK_R {
        RB_USB_IE_DEV_NAK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable interrupt for USB bus reset event for USB device mode _ enable interrupt for USB device detected event for USB host mode"]
    #[inline(always)]
    pub fn rb_usb_ie_busrst_rb_usb_ie_detect(&mut self) -> RB_USB_IE_BUSRST_RB_USB_IE_DETECT_W<0> {
        RB_USB_IE_BUSRST_RB_USB_IE_DETECT_W::new(self)
    }
    #[doc = "Bit 1 - enable interrupt for USB transfer completion"]
    #[inline(always)]
    pub fn rb_usb_ie_trans(&mut self) -> RB_USB_IE_TRANS_W<1> {
        RB_USB_IE_TRANS_W::new(self)
    }
    #[doc = "Bit 2 - enable interrupt for USB suspend or resume event"]
    #[inline(always)]
    pub fn rb_usb_ie_suspend(&mut self) -> RB_USB_IE_SUSPEND_W<2> {
        RB_USB_IE_SUSPEND_W::new(self)
    }
    #[doc = "Bit 3 - enable interrupt for host SOF timer action for USB host mode"]
    #[inline(always)]
    pub fn rb_usb_ie_sof(&mut self) -> RB_USB_IE_SOF_W<3> {
        RB_USB_IE_SOF_W::new(self)
    }
    #[doc = "Bit 4 - enable interrupt for FIFO overflow"]
    #[inline(always)]
    pub fn rb_usb_ie_fifoov(&mut self) -> RB_USB_IE_FIFOOV_W<4> {
        RB_USB_IE_FIFOOV_W::new(self)
    }
    #[doc = "Bit 5 - Setup packet end interrupt"]
    #[inline(always)]
    pub fn rb_usb_ie_setupact(&mut self) -> RB_USB_IE_SETUPACT_W<5> {
        RB_USB_IE_SETUPACT_W::new(self)
    }
    #[doc = "Bit 6 - Synchronous transmission received control token packet interrupt"]
    #[inline(always)]
    pub fn rb_usb_ie_isoact(&mut self) -> RB_USB_IE_ISOACT_W<6> {
        RB_USB_IE_ISOACT_W::new(self)
    }
    #[doc = "Bit 7 - enable interrupt for NAK responded for USB device mode"]
    #[inline(always)]
    pub fn rb_usb_ie_dev_nak(&mut self) -> RB_USB_IE_DEV_NAK_W<7> {
        RB_USB_IE_DEV_NAK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB interrupt enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_usb_int_en](index.html) module"]
pub struct R8_USB_INT_EN_SPEC;
impl crate::RegisterSpec for R8_USB_INT_EN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_usb_int_en::R](R) reader structure"]
impl crate::Readable for R8_USB_INT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_usb_int_en::W](W) writer structure"]
impl crate::Writable for R8_USB_INT_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_USB_INT_EN to value 0"]
impl crate::Resettable for R8_USB_INT_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
