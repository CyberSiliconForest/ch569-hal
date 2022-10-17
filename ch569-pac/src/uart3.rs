#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UART3 modem control"]
    pub r8_uart3_mcr: R8_UART3_MCR,
    #[doc = "0x01 - UART3 interrupt enable"]
    pub r8_uart3_ier: R8_UART3_IER,
    #[doc = "0x02 - UART3 FIFO control"]
    pub r8_uart3_fcr: R8_UART3_FCR,
    #[doc = "0x03 - UART3 line control"]
    pub r8_uart3_lcr: R8_UART3_LCR,
    #[doc = "0x04 - UART3 interrupt identification"]
    pub r8_uart3_iir: R8_UART3_IIR,
    #[doc = "0x05 - UART3 line status"]
    pub r8_uart3_lsr: R8_UART3_LSR,
    _reserved6: [u8; 0x02],
    #[doc = "0x08 - UART3 receiver buffer, receiving byte _ UART3 transmitter holding, transmittal byte"]
    pub r8_uart3_rbr_r8_uart3_thr: R8_UART3_RBR_R8_UART3_THR,
    _reserved7: [u8; 0x01],
    #[doc = "0x0a - UART3 receiver FIFO count"]
    pub r8_uart3_rfc: R8_UART3_RFC,
    #[doc = "0x0b - UART3 transmitter FIFO count"]
    pub r8_uart3_tfc: R8_UART3_TFC,
    #[doc = "0x0c - UART3 divisor latch"]
    pub r16_uart3_dl: R16_UART3_DL,
    #[doc = "0x0e - UART3 pre-divisor latch byte"]
    pub r8_uart3_div: R8_UART3_DIV,
}
#[doc = "R8_UART3_MCR (rw) register accessor: an alias for `Reg<R8_UART3_MCR_SPEC>`"]
pub type R8_UART3_MCR = crate::Reg<r8_uart3_mcr::R8_UART3_MCR_SPEC>;
#[doc = "UART3 modem control"]
pub mod r8_uart3_mcr;
#[doc = "R8_UART3_IER (rw) register accessor: an alias for `Reg<R8_UART3_IER_SPEC>`"]
pub type R8_UART3_IER = crate::Reg<r8_uart3_ier::R8_UART3_IER_SPEC>;
#[doc = "UART3 interrupt enable"]
pub mod r8_uart3_ier;
#[doc = "R8_UART3_FCR (rw) register accessor: an alias for `Reg<R8_UART3_FCR_SPEC>`"]
pub type R8_UART3_FCR = crate::Reg<r8_uart3_fcr::R8_UART3_FCR_SPEC>;
#[doc = "UART3 FIFO control"]
pub mod r8_uart3_fcr;
#[doc = "R8_UART3_LCR (rw) register accessor: an alias for `Reg<R8_UART3_LCR_SPEC>`"]
pub type R8_UART3_LCR = crate::Reg<r8_uart3_lcr::R8_UART3_LCR_SPEC>;
#[doc = "UART3 line control"]
pub mod r8_uart3_lcr;
#[doc = "R8_UART3_IIR (r) register accessor: an alias for `Reg<R8_UART3_IIR_SPEC>`"]
pub type R8_UART3_IIR = crate::Reg<r8_uart3_iir::R8_UART3_IIR_SPEC>;
#[doc = "UART3 interrupt identification"]
pub mod r8_uart3_iir;
#[doc = "R8_UART3_LSR (r) register accessor: an alias for `Reg<R8_UART3_LSR_SPEC>`"]
pub type R8_UART3_LSR = crate::Reg<r8_uart3_lsr::R8_UART3_LSR_SPEC>;
#[doc = "UART3 line status"]
pub mod r8_uart3_lsr;
#[doc = "R8_UART3_RBR_R8_UART3_THR (rw) register accessor: an alias for `Reg<R8_UART3_RBR_R8_UART3_THR_SPEC>`"]
pub type R8_UART3_RBR_R8_UART3_THR =
    crate::Reg<r8_uart3_rbr_r8_uart3_thr::R8_UART3_RBR_R8_UART3_THR_SPEC>;
#[doc = "UART3 receiver buffer, receiving byte _ UART3 transmitter holding, transmittal byte"]
pub mod r8_uart3_rbr_r8_uart3_thr;
#[doc = "R8_UART3_RFC (r) register accessor: an alias for `Reg<R8_UART3_RFC_SPEC>`"]
pub type R8_UART3_RFC = crate::Reg<r8_uart3_rfc::R8_UART3_RFC_SPEC>;
#[doc = "UART3 receiver FIFO count"]
pub mod r8_uart3_rfc;
#[doc = "R8_UART3_TFC (r) register accessor: an alias for `Reg<R8_UART3_TFC_SPEC>`"]
pub type R8_UART3_TFC = crate::Reg<r8_uart3_tfc::R8_UART3_TFC_SPEC>;
#[doc = "UART3 transmitter FIFO count"]
pub mod r8_uart3_tfc;
#[doc = "R16_UART3_DL (rw) register accessor: an alias for `Reg<R16_UART3_DL_SPEC>`"]
pub type R16_UART3_DL = crate::Reg<r16_uart3_dl::R16_UART3_DL_SPEC>;
#[doc = "UART3 divisor latch"]
pub mod r16_uart3_dl;
#[doc = "R8_UART3_DIV (rw) register accessor: an alias for `Reg<R8_UART3_DIV_SPEC>`"]
pub type R8_UART3_DIV = crate::Reg<r8_uart3_div::R8_UART3_DIV_SPEC>;
#[doc = "UART3 pre-divisor latch byte"]
pub mod r8_uart3_div;
