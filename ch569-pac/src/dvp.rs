#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DVP control register0"]
    pub r8_dvp_cr0: R8_DVP_CR0,
    #[doc = "0x01 - DVP control register1"]
    pub r8_dvp_cr1: R8_DVP_CR1,
    #[doc = "0x02 - DVP interrupt enable register"]
    pub r8_dvp_int_en: R8_DVP_INT_EN,
    _reserved3: [u8; 0x01],
    #[doc = "0x04 - DVP row number of a frame indicator register"]
    pub r16_dvp_row_num: R16_DVP_ROW_NUM,
    #[doc = "0x06 - DVP row number of a frame indicator register"]
    pub r16_dvp_col_num: R16_DVP_COL_NUM,
    #[doc = "0x08 - DVP dma buffer0 addr"]
    pub r32_dvp_dma_buf0: R32_DVP_DMA_BUF0,
    #[doc = "0x0c - DVP dma buffer1 addr"]
    pub r32_dvp_dma_buf1: R32_DVP_DMA_BUF1,
    _reserved_7_r8_dvp: [u8; 0x04],
    #[doc = "0x14 - DVP row count value"]
    pub r16_dvp_row_cnt: R16_DVP_ROW_CNT,
    #[doc = "0x16 - DVP col count value"]
    pub r16_dvp_col_cnt: R16_DVP_COL_CNT,
}
impl RegisterBlock {
    #[doc = "0x10 - DVP interrupt flag register"]
    #[inline(always)]
    pub fn r8_dvp_int_flag(&self) -> &R8_DVP_INT_FLAG {
        unsafe { &*(((self as *const Self) as *const u8).add(16usize) as *const R8_DVP_INT_FLAG) }
    }
    #[doc = "0x11 - DVP receive fifo status"]
    #[inline(always)]
    pub fn r8_dvp_fifo_st(&self) -> &R8_DVP_FIFO_ST {
        unsafe { &*(((self as *const Self) as *const u8).add(17usize) as *const R8_DVP_FIFO_ST) }
    }
}
#[doc = "R8_DVP_CR0 (rw) register accessor: an alias for `Reg<R8_DVP_CR0_SPEC>`"]
pub type R8_DVP_CR0 = crate::Reg<r8_dvp_cr0::R8_DVP_CR0_SPEC>;
#[doc = "DVP control register0"]
pub mod r8_dvp_cr0;
#[doc = "R8_DVP_CR1 (rw) register accessor: an alias for `Reg<R8_DVP_CR1_SPEC>`"]
pub type R8_DVP_CR1 = crate::Reg<r8_dvp_cr1::R8_DVP_CR1_SPEC>;
#[doc = "DVP control register1"]
pub mod r8_dvp_cr1;
#[doc = "R8_DVP_INT_EN (rw) register accessor: an alias for `Reg<R8_DVP_INT_EN_SPEC>`"]
pub type R8_DVP_INT_EN = crate::Reg<r8_dvp_int_en::R8_DVP_INT_EN_SPEC>;
#[doc = "DVP interrupt enable register"]
pub mod r8_dvp_int_en;
#[doc = "R16_DVP_ROW_NUM (rw) register accessor: an alias for `Reg<R16_DVP_ROW_NUM_SPEC>`"]
pub type R16_DVP_ROW_NUM = crate::Reg<r16_dvp_row_num::R16_DVP_ROW_NUM_SPEC>;
#[doc = "DVP row number of a frame indicator register"]
pub mod r16_dvp_row_num;
#[doc = "R16_DVP_COL_NUM (rw) register accessor: an alias for `Reg<R16_DVP_COL_NUM_SPEC>`"]
pub type R16_DVP_COL_NUM = crate::Reg<r16_dvp_col_num::R16_DVP_COL_NUM_SPEC>;
#[doc = "DVP row number of a frame indicator register"]
pub mod r16_dvp_col_num;
#[doc = "R32_DVP_DMA_BUF0 (rw) register accessor: an alias for `Reg<R32_DVP_DMA_BUF0_SPEC>`"]
pub type R32_DVP_DMA_BUF0 = crate::Reg<r32_dvp_dma_buf0::R32_DVP_DMA_BUF0_SPEC>;
#[doc = "DVP dma buffer0 addr"]
pub mod r32_dvp_dma_buf0;
#[doc = "R32_DVP_DMA_BUF1 (rw) register accessor: an alias for `Reg<R32_DVP_DMA_BUF1_SPEC>`"]
pub type R32_DVP_DMA_BUF1 = crate::Reg<r32_dvp_dma_buf1::R32_DVP_DMA_BUF1_SPEC>;
#[doc = "DVP dma buffer1 addr"]
pub mod r32_dvp_dma_buf1;
#[doc = "R8_DVP_INT_FLAG (rw) register accessor: an alias for `Reg<R8_DVP_INT_FLAG_SPEC>`"]
pub type R8_DVP_INT_FLAG = crate::Reg<r8_dvp_int_flag::R8_DVP_INT_FLAG_SPEC>;
#[doc = "DVP interrupt flag register"]
pub mod r8_dvp_int_flag;
#[doc = "R8_DVP_FIFO_ST (r) register accessor: an alias for `Reg<R8_DVP_FIFO_ST_SPEC>`"]
pub type R8_DVP_FIFO_ST = crate::Reg<r8_dvp_fifo_st::R8_DVP_FIFO_ST_SPEC>;
#[doc = "DVP receive fifo status"]
pub mod r8_dvp_fifo_st;
#[doc = "R16_DVP_ROW_CNT (r) register accessor: an alias for `Reg<R16_DVP_ROW_CNT_SPEC>`"]
pub type R16_DVP_ROW_CNT = crate::Reg<r16_dvp_row_cnt::R16_DVP_ROW_CNT_SPEC>;
#[doc = "DVP row count value"]
pub mod r16_dvp_row_cnt;
#[doc = "R16_DVP_COL_CNT (r) register accessor: an alias for `Reg<R16_DVP_COL_CNT_SPEC>`"]
pub type R16_DVP_COL_CNT = crate::Reg<r16_dvp_col_cnt::R16_DVP_COL_CNT_SPEC>;
#[doc = "DVP col count value"]
pub mod r16_dvp_col_cnt;
