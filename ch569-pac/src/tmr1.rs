#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TMR1 mode control"]
    pub r8_tmr1_ctrl_mod: R8_TMR1_CTRL_MOD,
    #[doc = "0x01 - TMR1 DMA control"]
    pub r8_tmr1_ctrl_dma: R8_TMR1_CTRL_DMA,
    #[doc = "0x02 - TMR1 interrupt enable"]
    pub r8_tmr1_inter_en: R8_TMR1_INTER_EN,
    _reserved3: [u8; 0x03],
    #[doc = "0x06 - TMR1 interrupt flag"]
    pub r8_tmr1_int_flag: R8_TMR1_INT_FLAG,
    #[doc = "0x07 - TMR1 FIFO count status"]
    pub r8_tmr1_fifo_count: R8_TMR1_FIFO_COUNT,
    #[doc = "0x08 - TMR1 current count"]
    pub r32_tmr1_count: R32_TMR1_COUNT,
    #[doc = "0x0c - TMR1 end count value, only low 26 bit"]
    pub r32_tmr1_cnt_end: R32_TMR1_CNT_END,
    #[doc = "0x10 - TMR1 FIFO only low 26 bit"]
    pub r32_tmr1_fifo: R32_TMR1_FIFO,
    #[doc = "0x14 - TMR1 DMA current address"]
    pub r32_tmr1_dma_now: R32_TMR1_DMA_NOW,
    #[doc = "0x18 - TMR1 DMA begin address"]
    pub r32_tmr1_dma_beg: R32_TMR1_DMA_BEG,
    #[doc = "0x1c - TMR1 DMA end address"]
    pub r32_tmr1_dma_end: R32_TMR1_DMA_END,
}
#[doc = "R8_TMR1_CTRL_MOD (rw) register accessor: an alias for `Reg<R8_TMR1_CTRL_MOD_SPEC>`"]
pub type R8_TMR1_CTRL_MOD = crate::Reg<r8_tmr1_ctrl_mod::R8_TMR1_CTRL_MOD_SPEC>;
#[doc = "TMR1 mode control"]
pub mod r8_tmr1_ctrl_mod;
#[doc = "R8_TMR1_INTER_EN (rw) register accessor: an alias for `Reg<R8_TMR1_INTER_EN_SPEC>`"]
pub type R8_TMR1_INTER_EN = crate::Reg<r8_tmr1_inter_en::R8_TMR1_INTER_EN_SPEC>;
#[doc = "TMR1 interrupt enable"]
pub mod r8_tmr1_inter_en;
#[doc = "R8_TMR1_INT_FLAG (rw) register accessor: an alias for `Reg<R8_TMR1_INT_FLAG_SPEC>`"]
pub type R8_TMR1_INT_FLAG = crate::Reg<r8_tmr1_int_flag::R8_TMR1_INT_FLAG_SPEC>;
#[doc = "TMR1 interrupt flag"]
pub mod r8_tmr1_int_flag;
#[doc = "R8_TMR1_FIFO_COUNT (r) register accessor: an alias for `Reg<R8_TMR1_FIFO_COUNT_SPEC>`"]
pub type R8_TMR1_FIFO_COUNT = crate::Reg<r8_tmr1_fifo_count::R8_TMR1_FIFO_COUNT_SPEC>;
#[doc = "TMR1 FIFO count status"]
pub mod r8_tmr1_fifo_count;
#[doc = "R32_TMR1_COUNT (r) register accessor: an alias for `Reg<R32_TMR1_COUNT_SPEC>`"]
pub type R32_TMR1_COUNT = crate::Reg<r32_tmr1_count::R32_TMR1_COUNT_SPEC>;
#[doc = "TMR1 current count"]
pub mod r32_tmr1_count;
#[doc = "R32_TMR1_CNT_END (rw) register accessor: an alias for `Reg<R32_TMR1_CNT_END_SPEC>`"]
pub type R32_TMR1_CNT_END = crate::Reg<r32_tmr1_cnt_end::R32_TMR1_CNT_END_SPEC>;
#[doc = "TMR1 end count value, only low 26 bit"]
pub mod r32_tmr1_cnt_end;
#[doc = "R32_TMR1_FIFO (rw) register accessor: an alias for `Reg<R32_TMR1_FIFO_SPEC>`"]
pub type R32_TMR1_FIFO = crate::Reg<r32_tmr1_fifo::R32_TMR1_FIFO_SPEC>;
#[doc = "TMR1 FIFO only low 26 bit"]
pub mod r32_tmr1_fifo;
#[doc = "R8_TMR1_CTRL_DMA (rw) register accessor: an alias for `Reg<R8_TMR1_CTRL_DMA_SPEC>`"]
pub type R8_TMR1_CTRL_DMA = crate::Reg<r8_tmr1_ctrl_dma::R8_TMR1_CTRL_DMA_SPEC>;
#[doc = "TMR1 DMA control"]
pub mod r8_tmr1_ctrl_dma;
#[doc = "R32_TMR1_DMA_NOW (rw) register accessor: an alias for `Reg<R32_TMR1_DMA_NOW_SPEC>`"]
pub type R32_TMR1_DMA_NOW = crate::Reg<r32_tmr1_dma_now::R32_TMR1_DMA_NOW_SPEC>;
#[doc = "TMR1 DMA current address"]
pub mod r32_tmr1_dma_now;
#[doc = "R32_TMR1_DMA_BEG (rw) register accessor: an alias for `Reg<R32_TMR1_DMA_BEG_SPEC>`"]
pub type R32_TMR1_DMA_BEG = crate::Reg<r32_tmr1_dma_beg::R32_TMR1_DMA_BEG_SPEC>;
#[doc = "TMR1 DMA begin address"]
pub mod r32_tmr1_dma_beg;
#[doc = "R32_TMR1_DMA_END (rw) register accessor: an alias for `Reg<R32_TMR1_DMA_END_SPEC>`"]
pub type R32_TMR1_DMA_END = crate::Reg<r32_tmr1_dma_end::R32_TMR1_DMA_END_SPEC>;
#[doc = "TMR1 DMA end address"]
pub mod r32_tmr1_dma_end;
