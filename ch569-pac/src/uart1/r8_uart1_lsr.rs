#[doc = "Register `R8_UART1_LSR` reader"]
pub struct R(crate::R<R8_UART1_LSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_UART1_LSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_UART1_LSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_UART1_LSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RB_LSR_DATA_RDY` reader - UART receiver fifo data ready status"]
pub type RB_LSR_DATA_RDY_R = crate::BitReader<bool>;
#[doc = "Field `RB_LSR_OVER_ERR` reader - UART receiver overrun error"]
pub type RB_LSR_OVER_ERR_R = crate::BitReader<bool>;
#[doc = "Field `RB_LSR_PAR_ERR` reader - UART receiver frame error"]
pub type RB_LSR_PAR_ERR_R = crate::BitReader<bool>;
#[doc = "Field `RB_LSR_FRAME_ERR` reader - UART receiver frame error"]
pub type RB_LSR_FRAME_ERR_R = crate::BitReader<bool>;
#[doc = "Field `RB_LSR_BREAK_ERR` reader - UART receiver break error"]
pub type RB_LSR_BREAK_ERR_R = crate::BitReader<bool>;
#[doc = "Field `RB_LSR_TX_FIFO_EMP` reader - UART transmitter fifo empty status"]
pub type RB_LSR_TX_FIFO_EMP_R = crate::BitReader<bool>;
#[doc = "Field `RB_LSR_TX_ALL_EMP` reader - UART transmitter all empty status"]
pub type RB_LSR_TX_ALL_EMP_R = crate::BitReader<bool>;
#[doc = "Field `RB_LSR_ERR_RX_FIFO` reader - indicate error in UART receiver fifo"]
pub type RB_LSR_ERR_RX_FIFO_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - UART receiver fifo data ready status"]
    #[inline(always)]
    pub fn rb_lsr_data_rdy(&self) -> RB_LSR_DATA_RDY_R {
        RB_LSR_DATA_RDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART receiver overrun error"]
    #[inline(always)]
    pub fn rb_lsr_over_err(&self) -> RB_LSR_OVER_ERR_R {
        RB_LSR_OVER_ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART receiver frame error"]
    #[inline(always)]
    pub fn rb_lsr_par_err(&self) -> RB_LSR_PAR_ERR_R {
        RB_LSR_PAR_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART receiver frame error"]
    #[inline(always)]
    pub fn rb_lsr_frame_err(&self) -> RB_LSR_FRAME_ERR_R {
        RB_LSR_FRAME_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART receiver break error"]
    #[inline(always)]
    pub fn rb_lsr_break_err(&self) -> RB_LSR_BREAK_ERR_R {
        RB_LSR_BREAK_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART transmitter fifo empty status"]
    #[inline(always)]
    pub fn rb_lsr_tx_fifo_emp(&self) -> RB_LSR_TX_FIFO_EMP_R {
        RB_LSR_TX_FIFO_EMP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART transmitter all empty status"]
    #[inline(always)]
    pub fn rb_lsr_tx_all_emp(&self) -> RB_LSR_TX_ALL_EMP_R {
        RB_LSR_TX_ALL_EMP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - indicate error in UART receiver fifo"]
    #[inline(always)]
    pub fn rb_lsr_err_rx_fifo(&self) -> RB_LSR_ERR_RX_FIFO_R {
        RB_LSR_ERR_RX_FIFO_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "UART1 line status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_uart1_lsr](index.html) module"]
pub struct R8_UART1_LSR_SPEC;
impl crate::RegisterSpec for R8_UART1_LSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_uart1_lsr::R](R) reader structure"]
impl crate::Readable for R8_UART1_LSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets R8_UART1_LSR to value 0xc0"]
impl crate::Resettable for R8_UART1_LSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc0
    }
}
