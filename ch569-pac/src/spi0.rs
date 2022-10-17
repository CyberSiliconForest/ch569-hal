#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI0 mode control"]
    pub r8_spi0_ctrl_mod: R8_SPI0_CTRL_MOD,
    #[doc = "0x01 - SPI0 configuration control"]
    pub r8_spi0_ctrl_cfg: R8_SPI0_CTRL_CFG,
    #[doc = "0x02 - SPI0 interrupt enable"]
    pub r8_spi0_inter_en: R8_SPI0_INTER_EN,
    #[doc = "0x03 - SPI0 master clock divisor_ SPI0 slave preset value"]
    pub r8_spi0_clock_div_r8_spi0_slave_pre: R8_SPI0_CLOCK_DIV_R8_SPI0_SLAVE_PRE,
    #[doc = "0x04 - SPI0 data buffer"]
    pub r8_spi0_buffer: R8_SPI0_BUFFER,
    #[doc = "0x05 - SPI0 work flag"]
    pub r8_spi0_run_flag: R8_SPI0_RUN_FLAG,
    #[doc = "0x06 - SPI0 interrupt flag"]
    pub r8_spi0_int_flag: R8_SPI0_INT_FLAG,
    #[doc = "0x07 - SPI0 FIFO count status"]
    pub r8_spi0_fifo_count: R8_SPI0_FIFO_COUNT,
    _reserved8: [u8; 0x04],
    #[doc = "0x0c - SPI0 total byte count, only low 12 bit"]
    pub r16_spi0_total_cnt: R16_SPI0_TOTAL_CNT,
    _reserved9: [u8; 0x02],
    #[doc = "0x10 - SPI0 FIFO register"]
    pub r8_spi0_fifo: R8_SPI0_FIFO,
    _reserved10: [u8; 0x02],
    #[doc = "0x13 - SPI0 FIFO count status"]
    pub r8_spi0_fifo_count1: R8_SPI0_FIFO_COUNT1,
    #[doc = "0x14 - SPI0 DMA current address"]
    pub r32_spi0_dma_now: R32_SPI0_DMA_NOW,
    #[doc = "0x18 - SPI0 DMA begin address"]
    pub r32_spi0_dma_beg: R32_SPI0_DMA_BEG,
    #[doc = "0x1c - SPI0 DMA end address"]
    pub r32_spi0_dma_end: R32_SPI0_DMA_END,
}
#[doc = "R8_SPI0_CTRL_MOD (rw) register accessor: an alias for `Reg<R8_SPI0_CTRL_MOD_SPEC>`"]
pub type R8_SPI0_CTRL_MOD = crate::Reg<r8_spi0_ctrl_mod::R8_SPI0_CTRL_MOD_SPEC>;
#[doc = "SPI0 mode control"]
pub mod r8_spi0_ctrl_mod;
#[doc = "R8_SPI0_CTRL_CFG (rw) register accessor: an alias for `Reg<R8_SPI0_CTRL_CFG_SPEC>`"]
pub type R8_SPI0_CTRL_CFG = crate::Reg<r8_spi0_ctrl_cfg::R8_SPI0_CTRL_CFG_SPEC>;
#[doc = "SPI0 configuration control"]
pub mod r8_spi0_ctrl_cfg;
#[doc = "R8_SPI0_INTER_EN (rw) register accessor: an alias for `Reg<R8_SPI0_INTER_EN_SPEC>`"]
pub type R8_SPI0_INTER_EN = crate::Reg<r8_spi0_inter_en::R8_SPI0_INTER_EN_SPEC>;
#[doc = "SPI0 interrupt enable"]
pub mod r8_spi0_inter_en;
#[doc = "R8_SPI0_CLOCK_DIV_R8_SPI0_SLAVE_PRE (rw) register accessor: an alias for `Reg<R8_SPI0_CLOCK_DIV_R8_SPI0_SLAVE_PRE_SPEC>`"]
pub type R8_SPI0_CLOCK_DIV_R8_SPI0_SLAVE_PRE =
    crate::Reg<r8_spi0_clock_div_r8_spi0_slave_pre::R8_SPI0_CLOCK_DIV_R8_SPI0_SLAVE_PRE_SPEC>;
#[doc = "SPI0 master clock divisor_ SPI0 slave preset value"]
pub mod r8_spi0_clock_div_r8_spi0_slave_pre;
#[doc = "R8_SPI0_BUFFER (r) register accessor: an alias for `Reg<R8_SPI0_BUFFER_SPEC>`"]
pub type R8_SPI0_BUFFER = crate::Reg<r8_spi0_buffer::R8_SPI0_BUFFER_SPEC>;
#[doc = "SPI0 data buffer"]
pub mod r8_spi0_buffer;
#[doc = "R8_SPI0_RUN_FLAG (r) register accessor: an alias for `Reg<R8_SPI0_RUN_FLAG_SPEC>`"]
pub type R8_SPI0_RUN_FLAG = crate::Reg<r8_spi0_run_flag::R8_SPI0_RUN_FLAG_SPEC>;
#[doc = "SPI0 work flag"]
pub mod r8_spi0_run_flag;
#[doc = "R8_SPI0_INT_FLAG (rw) register accessor: an alias for `Reg<R8_SPI0_INT_FLAG_SPEC>`"]
pub type R8_SPI0_INT_FLAG = crate::Reg<r8_spi0_int_flag::R8_SPI0_INT_FLAG_SPEC>;
#[doc = "SPI0 interrupt flag"]
pub mod r8_spi0_int_flag;
#[doc = "R8_SPI0_FIFO_COUNT (rw) register accessor: an alias for `Reg<R8_SPI0_FIFO_COUNT_SPEC>`"]
pub type R8_SPI0_FIFO_COUNT = crate::Reg<r8_spi0_fifo_count::R8_SPI0_FIFO_COUNT_SPEC>;
#[doc = "SPI0 FIFO count status"]
pub mod r8_spi0_fifo_count;
#[doc = "R16_SPI0_TOTAL_CNT (rw) register accessor: an alias for `Reg<R16_SPI0_TOTAL_CNT_SPEC>`"]
pub type R16_SPI0_TOTAL_CNT = crate::Reg<r16_spi0_total_cnt::R16_SPI0_TOTAL_CNT_SPEC>;
#[doc = "SPI0 total byte count, only low 12 bit"]
pub mod r16_spi0_total_cnt;
#[doc = "R8_SPI0_FIFO (rw) register accessor: an alias for `Reg<R8_SPI0_FIFO_SPEC>`"]
pub type R8_SPI0_FIFO = crate::Reg<r8_spi0_fifo::R8_SPI0_FIFO_SPEC>;
#[doc = "SPI0 FIFO register"]
pub mod r8_spi0_fifo;
#[doc = "R8_SPI0_FIFO_COUNT1 (rw) register accessor: an alias for `Reg<R8_SPI0_FIFO_COUNT1_SPEC>`"]
pub type R8_SPI0_FIFO_COUNT1 = crate::Reg<r8_spi0_fifo_count1::R8_SPI0_FIFO_COUNT1_SPEC>;
#[doc = "SPI0 FIFO count status"]
pub mod r8_spi0_fifo_count1;
#[doc = "R32_SPI0_DMA_NOW (rw) register accessor: an alias for `Reg<R32_SPI0_DMA_NOW_SPEC>`"]
pub type R32_SPI0_DMA_NOW = crate::Reg<r32_spi0_dma_now::R32_SPI0_DMA_NOW_SPEC>;
#[doc = "SPI0 DMA current address"]
pub mod r32_spi0_dma_now;
#[doc = "R32_SPI0_DMA_BEG (rw) register accessor: an alias for `Reg<R32_SPI0_DMA_BEG_SPEC>`"]
pub type R32_SPI0_DMA_BEG = crate::Reg<r32_spi0_dma_beg::R32_SPI0_DMA_BEG_SPEC>;
#[doc = "SPI0 DMA begin address"]
pub mod r32_spi0_dma_beg;
#[doc = "R32_SPI0_DMA_END (rw) register accessor: an alias for `Reg<R32_SPI0_DMA_END_SPEC>`"]
pub type R32_SPI0_DMA_END = crate::Reg<r32_spi0_dma_end::R32_SPI0_DMA_END_SPEC>;
#[doc = "SPI0 DMA end address"]
pub mod r32_spi0_dma_end;
