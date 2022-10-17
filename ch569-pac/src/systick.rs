#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Systick counter control register"]
    pub r32_stk_ctlr: R32_STK_CTLR,
    #[doc = "0x04 - Systick counter low register"]
    pub r32_stk_cntl: R32_STK_CNTL,
    #[doc = "0x08 - Systick counter high register"]
    pub r32_stk_cnth: R32_STK_CNTH,
    #[doc = "0x0c - Systick compare low register"]
    pub r32_stk_cmplr: R32_STK_CMPLR,
    #[doc = "0x10 - Systick compare high register"]
    pub r32_stk_cmphr: R32_STK_CMPHR,
    #[doc = "0x14 - Systick counter flag"]
    pub r32_stk_cntfg: R32_STK_CNTFG,
}
#[doc = "R32_STK_CTLR (rw) register accessor: an alias for `Reg<R32_STK_CTLR_SPEC>`"]
pub type R32_STK_CTLR = crate::Reg<r32_stk_ctlr::R32_STK_CTLR_SPEC>;
#[doc = "Systick counter control register"]
pub mod r32_stk_ctlr;
#[doc = "R32_STK_CNTL (rw) register accessor: an alias for `Reg<R32_STK_CNTL_SPEC>`"]
pub type R32_STK_CNTL = crate::Reg<r32_stk_cntl::R32_STK_CNTL_SPEC>;
#[doc = "Systick counter low register"]
pub mod r32_stk_cntl;
#[doc = "R32_STK_CNTH (rw) register accessor: an alias for `Reg<R32_STK_CNTH_SPEC>`"]
pub type R32_STK_CNTH = crate::Reg<r32_stk_cnth::R32_STK_CNTH_SPEC>;
#[doc = "Systick counter high register"]
pub mod r32_stk_cnth;
#[doc = "R32_STK_CMPLR (rw) register accessor: an alias for `Reg<R32_STK_CMPLR_SPEC>`"]
pub type R32_STK_CMPLR = crate::Reg<r32_stk_cmplr::R32_STK_CMPLR_SPEC>;
#[doc = "Systick compare low register"]
pub mod r32_stk_cmplr;
#[doc = "R32_STK_CMPHR (rw) register accessor: an alias for `Reg<R32_STK_CMPHR_SPEC>`"]
pub type R32_STK_CMPHR = crate::Reg<r32_stk_cmphr::R32_STK_CMPHR_SPEC>;
#[doc = "Systick compare high register"]
pub mod r32_stk_cmphr;
#[doc = "R32_STK_CNTFG (rw) register accessor: an alias for `Reg<R32_STK_CNTFG_SPEC>`"]
pub type R32_STK_CNTFG = crate::Reg<r32_stk_cntfg::R32_STK_CNTFG_SPEC>;
#[doc = "Systick counter flag"]
pub mod r32_stk_cntfg;
