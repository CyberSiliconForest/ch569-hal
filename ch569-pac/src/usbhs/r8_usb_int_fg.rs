#[doc = "Register `R8_USB_INT_FG` reader"]
pub struct R(crate::R<R8_USB_INT_FG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_USB_INT_FG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_USB_INT_FG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_USB_INT_FG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_USB_INT_FG` writer"]
pub struct W(crate::W<R8_USB_INT_FG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_USB_INT_FG_SPEC>;
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
impl From<crate::W<R8_USB_INT_FG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_USB_INT_FG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_USB_IF_BUSRST_RB_USB_IF_DETECT` reader - bus reset event interrupt flag for USB device mode, direct bit address clear or write 1 to clear;device detected event interrupt flag for USB host mode, direct bit address clear or write 1 to clear"]
pub type RB_USB_IF_BUSRST_RB_USB_IF_DETECT_R = crate::BitReader<bool>;
#[doc = "Field `RB_USB_IF_BUSRST_RB_USB_IF_DETECT` writer - bus reset event interrupt flag for USB device mode, direct bit address clear or write 1 to clear;device detected event interrupt flag for USB host mode, direct bit address clear or write 1 to clear"]
pub type RB_USB_IF_BUSRST_RB_USB_IF_DETECT_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_USB_INT_FG_SPEC, bool, O>;
#[doc = "Field `RB_USB_IF_TRANSFER` reader - USB transfer completion interrupt flag, direct bit address clear or write 1 to clear"]
pub type RB_USB_IF_TRANSFER_R = crate::BitReader<bool>;
#[doc = "Field `RB_USB_IF_TRANSFER` writer - USB transfer completion interrupt flag, direct bit address clear or write 1 to clear"]
pub type RB_USB_IF_TRANSFER_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_USB_INT_FG_SPEC, bool, O>;
#[doc = "Field `RB_USB_IF_SUSPEND` reader - USB suspend or resume event interrupt flag, direct bit address clear or write 1 to clear"]
pub type RB_USB_IF_SUSPEND_R = crate::BitReader<bool>;
#[doc = "Field `RB_USB_IF_SUSPEND` writer - USB suspend or resume event interrupt flag, direct bit address clear or write 1 to clear"]
pub type RB_USB_IF_SUSPEND_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_USB_INT_FG_SPEC, bool, O>;
#[doc = "Field `RB_USB_IF_HST_SOF` reader - host SOF timer interrupt flag for USB host, direct bit address clear or write 1 to clear"]
pub type RB_USB_IF_HST_SOF_R = crate::BitReader<bool>;
#[doc = "Field `RB_USB_IF_HST_SOF` writer - host SOF timer interrupt flag for USB host, direct bit address clear or write 1 to clear"]
pub type RB_USB_IF_HST_SOF_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_USB_INT_FG_SPEC, bool, O>;
#[doc = "Field `RB_USB_IF_FIFOOV` reader - FIFO overflow interrupt flag for USB, direct bit address clear or write 1 to clear"]
pub type RB_USB_IF_FIFOOV_R = crate::BitReader<bool>;
#[doc = "Field `RB_USB_IF_FIFOOV` writer - FIFO overflow interrupt flag for USB, direct bit address clear or write 1 to clear"]
pub type RB_USB_IF_FIFOOV_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_USB_INT_FG_SPEC, bool, O>;
#[doc = "Field `RB_USB_IF_SETUOACT` reader - RO, Setup transaction end interrupt flag"]
pub type RB_USB_IF_SETUOACT_R = crate::BitReader<bool>;
#[doc = "Field `RB_USB_IF_SETUOACT` writer - RO, Setup transaction end interrupt flag"]
pub type RB_USB_IF_SETUOACT_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_USB_INT_FG_SPEC, bool, O>;
#[doc = "Field `RB_USB_IF_ISOACT` reader - RO, Synchronous transmission received control token packet interrupt flag"]
pub type RB_USB_IF_ISOACT_R = crate::BitReader<bool>;
#[doc = "Field `RB_USB_IF_ISOACT` writer - RO, Synchronous transmission received control token packet interrupt flag"]
pub type RB_USB_IF_ISOACT_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_USB_INT_FG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - bus reset event interrupt flag for USB device mode, direct bit address clear or write 1 to clear;device detected event interrupt flag for USB host mode, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_usb_if_busrst_rb_usb_if_detect(&self) -> RB_USB_IF_BUSRST_RB_USB_IF_DETECT_R {
        RB_USB_IF_BUSRST_RB_USB_IF_DETECT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB transfer completion interrupt flag, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_usb_if_transfer(&self) -> RB_USB_IF_TRANSFER_R {
        RB_USB_IF_TRANSFER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB suspend or resume event interrupt flag, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_usb_if_suspend(&self) -> RB_USB_IF_SUSPEND_R {
        RB_USB_IF_SUSPEND_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - host SOF timer interrupt flag for USB host, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_usb_if_hst_sof(&self) -> RB_USB_IF_HST_SOF_R {
        RB_USB_IF_HST_SOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FIFO overflow interrupt flag for USB, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_usb_if_fifoov(&self) -> RB_USB_IF_FIFOOV_R {
        RB_USB_IF_FIFOOV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RO, Setup transaction end interrupt flag"]
    #[inline(always)]
    pub fn rb_usb_if_setuoact(&self) -> RB_USB_IF_SETUOACT_R {
        RB_USB_IF_SETUOACT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RO, Synchronous transmission received control token packet interrupt flag"]
    #[inline(always)]
    pub fn rb_usb_if_isoact(&self) -> RB_USB_IF_ISOACT_R {
        RB_USB_IF_ISOACT_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - bus reset event interrupt flag for USB device mode, direct bit address clear or write 1 to clear;device detected event interrupt flag for USB host mode, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_usb_if_busrst_rb_usb_if_detect(&mut self) -> RB_USB_IF_BUSRST_RB_USB_IF_DETECT_W<0> {
        RB_USB_IF_BUSRST_RB_USB_IF_DETECT_W::new(self)
    }
    #[doc = "Bit 1 - USB transfer completion interrupt flag, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_usb_if_transfer(&mut self) -> RB_USB_IF_TRANSFER_W<1> {
        RB_USB_IF_TRANSFER_W::new(self)
    }
    #[doc = "Bit 2 - USB suspend or resume event interrupt flag, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_usb_if_suspend(&mut self) -> RB_USB_IF_SUSPEND_W<2> {
        RB_USB_IF_SUSPEND_W::new(self)
    }
    #[doc = "Bit 3 - host SOF timer interrupt flag for USB host, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_usb_if_hst_sof(&mut self) -> RB_USB_IF_HST_SOF_W<3> {
        RB_USB_IF_HST_SOF_W::new(self)
    }
    #[doc = "Bit 4 - FIFO overflow interrupt flag for USB, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_usb_if_fifoov(&mut self) -> RB_USB_IF_FIFOOV_W<4> {
        RB_USB_IF_FIFOOV_W::new(self)
    }
    #[doc = "Bit 5 - RO, Setup transaction end interrupt flag"]
    #[inline(always)]
    pub fn rb_usb_if_setuoact(&mut self) -> RB_USB_IF_SETUOACT_W<5> {
        RB_USB_IF_SETUOACT_W::new(self)
    }
    #[doc = "Bit 6 - RO, Synchronous transmission received control token packet interrupt flag"]
    #[inline(always)]
    pub fn rb_usb_if_isoact(&mut self) -> RB_USB_IF_ISOACT_W<6> {
        RB_USB_IF_ISOACT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB interrupt flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_usb_int_fg](index.html) module"]
pub struct R8_USB_INT_FG_SPEC;
impl crate::RegisterSpec for R8_USB_INT_FG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_usb_int_fg::R](R) reader structure"]
impl crate::Readable for R8_USB_INT_FG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_usb_int_fg::W](W) writer structure"]
impl crate::Writable for R8_USB_INT_FG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_USB_INT_FG to value 0"]
impl crate::Resettable for R8_USB_INT_FG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
