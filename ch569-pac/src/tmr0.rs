#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TMR0 mode control"]
    pub r8_tmr0_ctrl_mod: R8_TMR0_CTRL_MOD,
    _reserved1: [u8; 0x01],
    #[doc = "0x02 - TMR0 interrupt enable"]
    pub r8_tmr0_inter_en: R8_TMR0_INTER_EN,
    _reserved2: [u8; 0x03],
    #[doc = "0x06 - TMR0 interrupt flag"]
    pub r8_tmr0_int_flag: R8_TMR0_INT_FLAG,
    #[doc = "0x07 - TMR0 FIFO count status"]
    pub r8_tmr0_fifo_count: R8_TMR0_FIFO_COUNT,
    #[doc = "0x08 - TMR0 current count"]
    pub r32_tmr0_count: R32_TMR0_COUNT,
    #[doc = "0x0c - TMR0 end count value, only low 26 bit"]
    pub r32_tmr0_cnt_end: R32_TMR0_CNT_END,
    #[doc = "0x10 - TMR0 FIFO register, only low 26 bit"]
    pub r32_tmr0_fifo: R32_TMR0_FIFO,
}
#[doc = "R8_TMR0_CTRL_MOD (rw) register accessor: an alias for `Reg<R8_TMR0_CTRL_MOD_SPEC>`"]
pub type R8_TMR0_CTRL_MOD = crate::Reg<r8_tmr0_ctrl_mod::R8_TMR0_CTRL_MOD_SPEC>;
#[doc = "TMR0 mode control"]
pub mod r8_tmr0_ctrl_mod;
#[doc = "R8_TMR0_INTER_EN (rw) register accessor: an alias for `Reg<R8_TMR0_INTER_EN_SPEC>`"]
pub type R8_TMR0_INTER_EN = crate::Reg<r8_tmr0_inter_en::R8_TMR0_INTER_EN_SPEC>;
#[doc = "TMR0 interrupt enable"]
pub mod r8_tmr0_inter_en;
#[doc = "R8_TMR0_INT_FLAG (rw) register accessor: an alias for `Reg<R8_TMR0_INT_FLAG_SPEC>`"]
pub type R8_TMR0_INT_FLAG = crate::Reg<r8_tmr0_int_flag::R8_TMR0_INT_FLAG_SPEC>;
#[doc = "TMR0 interrupt flag"]
pub mod r8_tmr0_int_flag;
#[doc = "R8_TMR0_FIFO_COUNT (r) register accessor: an alias for `Reg<R8_TMR0_FIFO_COUNT_SPEC>`"]
pub type R8_TMR0_FIFO_COUNT = crate::Reg<r8_tmr0_fifo_count::R8_TMR0_FIFO_COUNT_SPEC>;
#[doc = "TMR0 FIFO count status"]
pub mod r8_tmr0_fifo_count;
#[doc = "R32_TMR0_COUNT (r) register accessor: an alias for `Reg<R32_TMR0_COUNT_SPEC>`"]
pub type R32_TMR0_COUNT = crate::Reg<r32_tmr0_count::R32_TMR0_COUNT_SPEC>;
#[doc = "TMR0 current count"]
pub mod r32_tmr0_count;
#[doc = "R32_TMR0_CNT_END (rw) register accessor: an alias for `Reg<R32_TMR0_CNT_END_SPEC>`"]
pub type R32_TMR0_CNT_END = crate::Reg<r32_tmr0_cnt_end::R32_TMR0_CNT_END_SPEC>;
#[doc = "TMR0 end count value, only low 26 bit"]
pub mod r32_tmr0_cnt_end;
#[doc = "R32_TMR0_FIFO (rw) register accessor: an alias for `Reg<R32_TMR0_FIFO_SPEC>`"]
pub type R32_TMR0_FIFO = crate::Reg<r32_tmr0_fifo::R32_TMR0_FIFO_SPEC>;
#[doc = "TMR0 FIFO register, only low 26 bit"]
pub mod r32_tmr0_fifo;
