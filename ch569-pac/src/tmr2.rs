#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TMR2 mode control"]
    pub r8_tmr2_ctrl_mod: R8_TMR2_CTRL_MOD,
    #[doc = "0x01 - TMR2 DMA control"]
    pub r8_tmr2_ctrl_dma: R8_TMR2_CTRL_DMA,
    #[doc = "0x02 - TMR2 interrupt enable"]
    pub r8_tmr2_inter_en: R8_TMR2_INTER_EN,
    _reserved3: [u8; 0x03],
    #[doc = "0x06 - TMR2 interrupt flag"]
    pub r8_tmr2_int_flag: R8_TMR2_INT_FLAG,
    #[doc = "0x07 - TMR2 FIFO count status"]
    pub r8_tmr2_fifo_count: R8_TMR2_FIFO_COUNT,
    #[doc = "0x08 - TMR2 current count"]
    pub r32_tmr2_count: R32_TMR2_COUNT,
    #[doc = "0x0c - TMR2 end count value, only low 26 bit"]
    pub r32_tmr2_cnt_end: R32_TMR2_CNT_END,
    #[doc = "0x10 - TMR2 end count value, only low 26 bit"]
    pub r32_tmr2_fifo: R32_TMR2_FIFO,
    #[doc = "0x14 - TMR2 DMA current address"]
    pub r32_tmr2_dma_now: R32_TMR2_DMA_NOW,
    #[doc = "0x18 - TMR2 DMA begin address"]
    pub r32_tmr2_dma_beg: R32_TMR2_DMA_BEG,
    #[doc = "0x1c - TMR2 DMA end address"]
    pub r32_tmr2_dma_end: R32_TMR2_DMA_END,
}
#[doc = "R8_TMR2_CTRL_MOD (rw) register accessor: an alias for `Reg<R8_TMR2_CTRL_MOD_SPEC>`"]
pub type R8_TMR2_CTRL_MOD = crate::Reg<r8_tmr2_ctrl_mod::R8_TMR2_CTRL_MOD_SPEC>;
#[doc = "TMR2 mode control"]
pub mod r8_tmr2_ctrl_mod;
#[doc = "R8_TMR2_INTER_EN (rw) register accessor: an alias for `Reg<R8_TMR2_INTER_EN_SPEC>`"]
pub type R8_TMR2_INTER_EN = crate::Reg<r8_tmr2_inter_en::R8_TMR2_INTER_EN_SPEC>;
#[doc = "TMR2 interrupt enable"]
pub mod r8_tmr2_inter_en;
#[doc = "R8_TMR2_INT_FLAG (rw) register accessor: an alias for `Reg<R8_TMR2_INT_FLAG_SPEC>`"]
pub type R8_TMR2_INT_FLAG = crate::Reg<r8_tmr2_int_flag::R8_TMR2_INT_FLAG_SPEC>;
#[doc = "TMR2 interrupt flag"]
pub mod r8_tmr2_int_flag;
#[doc = "R8_TMR2_FIFO_COUNT (r) register accessor: an alias for `Reg<R8_TMR2_FIFO_COUNT_SPEC>`"]
pub type R8_TMR2_FIFO_COUNT = crate::Reg<r8_tmr2_fifo_count::R8_TMR2_FIFO_COUNT_SPEC>;
#[doc = "TMR2 FIFO count status"]
pub mod r8_tmr2_fifo_count;
#[doc = "R32_TMR2_COUNT (r) register accessor: an alias for `Reg<R32_TMR2_COUNT_SPEC>`"]
pub type R32_TMR2_COUNT = crate::Reg<r32_tmr2_count::R32_TMR2_COUNT_SPEC>;
#[doc = "TMR2 current count"]
pub mod r32_tmr2_count;
#[doc = "R32_TMR2_CNT_END (rw) register accessor: an alias for `Reg<R32_TMR2_CNT_END_SPEC>`"]
pub type R32_TMR2_CNT_END = crate::Reg<r32_tmr2_cnt_end::R32_TMR2_CNT_END_SPEC>;
#[doc = "TMR2 end count value, only low 26 bit"]
pub mod r32_tmr2_cnt_end;
#[doc = "R32_TMR2_FIFO (rw) register accessor: an alias for `Reg<R32_TMR2_FIFO_SPEC>`"]
pub type R32_TMR2_FIFO = crate::Reg<r32_tmr2_fifo::R32_TMR2_FIFO_SPEC>;
#[doc = "TMR2 end count value, only low 26 bit"]
pub mod r32_tmr2_fifo;
#[doc = "R8_TMR2_CTRL_DMA (rw) register accessor: an alias for `Reg<R8_TMR2_CTRL_DMA_SPEC>`"]
pub type R8_TMR2_CTRL_DMA = crate::Reg<r8_tmr2_ctrl_dma::R8_TMR2_CTRL_DMA_SPEC>;
#[doc = "TMR2 DMA control"]
pub mod r8_tmr2_ctrl_dma;
#[doc = "R32_TMR2_DMA_NOW (rw) register accessor: an alias for `Reg<R32_TMR2_DMA_NOW_SPEC>`"]
pub type R32_TMR2_DMA_NOW = crate::Reg<r32_tmr2_dma_now::R32_TMR2_DMA_NOW_SPEC>;
#[doc = "TMR2 DMA current address"]
pub mod r32_tmr2_dma_now;
#[doc = "R32_TMR2_DMA_BEG (rw) register accessor: an alias for `Reg<R32_TMR2_DMA_BEG_SPEC>`"]
pub type R32_TMR2_DMA_BEG = crate::Reg<r32_tmr2_dma_beg::R32_TMR2_DMA_BEG_SPEC>;
#[doc = "TMR2 DMA begin address"]
pub mod r32_tmr2_dma_beg;
#[doc = "R32_TMR2_DMA_END (rw) register accessor: an alias for `Reg<R32_TMR2_DMA_END_SPEC>`"]
pub type R32_TMR2_DMA_END = crate::Reg<r32_tmr2_dma_end::R32_TMR2_DMA_END_SPEC>;
#[doc = "TMR2 DMA end address"]
pub mod r32_tmr2_dma_end;
