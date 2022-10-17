#[doc = "Register `R8_SPI1_CTRL_MOD` reader"]
pub struct R(crate::R<R8_SPI1_CTRL_MOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_SPI1_CTRL_MOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_SPI1_CTRL_MOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_SPI1_CTRL_MOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_SPI1_CTRL_MOD` writer"]
pub struct W(crate::W<R8_SPI1_CTRL_MOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_SPI1_CTRL_MOD_SPEC>;
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
impl From<crate::W<R8_SPI1_CTRL_MOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_SPI1_CTRL_MOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_SPI_MODE_SLAVE` reader - SPI slave mode"]
pub type RB_SPI_MODE_SLAVE_R = crate::BitReader<bool>;
#[doc = "Field `RB_SPI_MODE_SLAVE` writer - SPI slave mode"]
pub type RB_SPI_MODE_SLAVE_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SPI1_CTRL_MOD_SPEC, bool, O>;
#[doc = "Field `RB_SPI_ALL_CLEAR` reader - force clear SPI FIFO and count"]
pub type RB_SPI_ALL_CLEAR_R = crate::BitReader<bool>;
#[doc = "Field `RB_SPI_ALL_CLEAR` writer - force clear SPI FIFO and count"]
pub type RB_SPI_ALL_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SPI1_CTRL_MOD_SPEC, bool, O>;
#[doc = "Field `RB_SPI_2WIRE_MOD` reader - SPI enable 2 wire mode"]
pub type RB_SPI_2WIRE_MOD_R = crate::BitReader<bool>;
#[doc = "Field `RB_SPI_2WIRE_MOD` writer - SPI enable 2 wire mode"]
pub type RB_SPI_2WIRE_MOD_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SPI1_CTRL_MOD_SPEC, bool, O>;
#[doc = "Field `RB_SPI_MST_SCK_MOD_RB_SPI_SLV_CMD_MOD` reader - SPI master clock mode _ SPI slave command mode"]
pub type RB_SPI_MST_SCK_MOD_RB_SPI_SLV_CMD_MOD_R = crate::BitReader<bool>;
#[doc = "Field `RB_SPI_MST_SCK_MOD_RB_SPI_SLV_CMD_MOD` writer - SPI master clock mode _ SPI slave command mode"]
pub type RB_SPI_MST_SCK_MOD_RB_SPI_SLV_CMD_MOD_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SPI1_CTRL_MOD_SPEC, bool, O>;
#[doc = "Field `RB_SPI_FIFO_DIR` reader - SPI FIFO direction"]
pub type RB_SPI_FIFO_DIR_R = crate::BitReader<bool>;
#[doc = "Field `RB_SPI_FIFO_DIR` writer - SPI FIFO direction"]
pub type RB_SPI_FIFO_DIR_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SPI1_CTRL_MOD_SPEC, bool, O>;
#[doc = "Field `RB_SPI_SCK_OE` reader - SPI SCK output enable"]
pub type RB_SPI_SCK_OE_R = crate::BitReader<bool>;
#[doc = "Field `RB_SPI_SCK_OE` writer - SPI SCK output enable"]
pub type RB_SPI_SCK_OE_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SPI1_CTRL_MOD_SPEC, bool, O>;
#[doc = "Field `RB_SPI_MOSI_OE` reader - SPI MOSI output enable"]
pub type RB_SPI_MOSI_OE_R = crate::BitReader<bool>;
#[doc = "Field `RB_SPI_MOSI_OE` writer - SPI MOSI output enable"]
pub type RB_SPI_MOSI_OE_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SPI1_CTRL_MOD_SPEC, bool, O>;
#[doc = "Field `RB_SPI_MISO_OE` reader - SPI MISO output enable"]
pub type RB_SPI_MISO_OE_R = crate::BitReader<bool>;
#[doc = "Field `RB_SPI_MISO_OE` writer - SPI MISO output enable"]
pub type RB_SPI_MISO_OE_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SPI1_CTRL_MOD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SPI slave mode"]
    #[inline(always)]
    pub fn rb_spi_mode_slave(&self) -> RB_SPI_MODE_SLAVE_R {
        RB_SPI_MODE_SLAVE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - force clear SPI FIFO and count"]
    #[inline(always)]
    pub fn rb_spi_all_clear(&self) -> RB_SPI_ALL_CLEAR_R {
        RB_SPI_ALL_CLEAR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SPI enable 2 wire mode"]
    #[inline(always)]
    pub fn rb_spi_2wire_mod(&self) -> RB_SPI_2WIRE_MOD_R {
        RB_SPI_2WIRE_MOD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SPI master clock mode _ SPI slave command mode"]
    #[inline(always)]
    pub fn rb_spi_mst_sck_mod_rb_spi_slv_cmd_mod(&self) -> RB_SPI_MST_SCK_MOD_RB_SPI_SLV_CMD_MOD_R {
        RB_SPI_MST_SCK_MOD_RB_SPI_SLV_CMD_MOD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SPI FIFO direction"]
    #[inline(always)]
    pub fn rb_spi_fifo_dir(&self) -> RB_SPI_FIFO_DIR_R {
        RB_SPI_FIFO_DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI SCK output enable"]
    #[inline(always)]
    pub fn rb_spi_sck_oe(&self) -> RB_SPI_SCK_OE_R {
        RB_SPI_SCK_OE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SPI MOSI output enable"]
    #[inline(always)]
    pub fn rb_spi_mosi_oe(&self) -> RB_SPI_MOSI_OE_R {
        RB_SPI_MOSI_OE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SPI MISO output enable"]
    #[inline(always)]
    pub fn rb_spi_miso_oe(&self) -> RB_SPI_MISO_OE_R {
        RB_SPI_MISO_OE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI slave mode"]
    #[inline(always)]
    pub fn rb_spi_mode_slave(&mut self) -> RB_SPI_MODE_SLAVE_W<0> {
        RB_SPI_MODE_SLAVE_W::new(self)
    }
    #[doc = "Bit 1 - force clear SPI FIFO and count"]
    #[inline(always)]
    pub fn rb_spi_all_clear(&mut self) -> RB_SPI_ALL_CLEAR_W<1> {
        RB_SPI_ALL_CLEAR_W::new(self)
    }
    #[doc = "Bit 2 - SPI enable 2 wire mode"]
    #[inline(always)]
    pub fn rb_spi_2wire_mod(&mut self) -> RB_SPI_2WIRE_MOD_W<2> {
        RB_SPI_2WIRE_MOD_W::new(self)
    }
    #[doc = "Bit 3 - SPI master clock mode _ SPI slave command mode"]
    #[inline(always)]
    pub fn rb_spi_mst_sck_mod_rb_spi_slv_cmd_mod(
        &mut self,
    ) -> RB_SPI_MST_SCK_MOD_RB_SPI_SLV_CMD_MOD_W<3> {
        RB_SPI_MST_SCK_MOD_RB_SPI_SLV_CMD_MOD_W::new(self)
    }
    #[doc = "Bit 4 - SPI FIFO direction"]
    #[inline(always)]
    pub fn rb_spi_fifo_dir(&mut self) -> RB_SPI_FIFO_DIR_W<4> {
        RB_SPI_FIFO_DIR_W::new(self)
    }
    #[doc = "Bit 5 - SPI SCK output enable"]
    #[inline(always)]
    pub fn rb_spi_sck_oe(&mut self) -> RB_SPI_SCK_OE_W<5> {
        RB_SPI_SCK_OE_W::new(self)
    }
    #[doc = "Bit 6 - SPI MOSI output enable"]
    #[inline(always)]
    pub fn rb_spi_mosi_oe(&mut self) -> RB_SPI_MOSI_OE_W<6> {
        RB_SPI_MOSI_OE_W::new(self)
    }
    #[doc = "Bit 7 - SPI MISO output enable"]
    #[inline(always)]
    pub fn rb_spi_miso_oe(&mut self) -> RB_SPI_MISO_OE_W<7> {
        RB_SPI_MISO_OE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 mode control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_spi1_ctrl_mod](index.html) module"]
pub struct R8_SPI1_CTRL_MOD_SPEC;
impl crate::RegisterSpec for R8_SPI1_CTRL_MOD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_spi1_ctrl_mod::R](R) reader structure"]
impl crate::Readable for R8_SPI1_CTRL_MOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_spi1_ctrl_mod::W](W) writer structure"]
impl crate::Writable for R8_SPI1_CTRL_MOD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_SPI1_CTRL_MOD to value 0x02"]
impl crate::Resettable for R8_SPI1_CTRL_MOD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
