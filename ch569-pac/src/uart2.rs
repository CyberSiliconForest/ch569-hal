#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UART2 modem control"]
    pub r8_uart2_mcr: R8_UART2_MCR,
    #[doc = "0x01 - UART2 interrupt enable"]
    pub r8_uart2_ier: R8_UART2_IER,
    #[doc = "0x02 - UART2 FIFO control"]
    pub r8_uart2_fcr: R8_UART2_FCR,
    #[doc = "0x03 - UART2 line control"]
    pub r8_uart2_lcr: R8_UART2_LCR,
    #[doc = "0x04 - UART2 interrupt identification"]
    pub r8_uart2_iir: R8_UART2_IIR,
    #[doc = "0x05 - UART2 line status"]
    pub r8_uart2_lsr: R8_UART2_LSR,
    _reserved6: [u8; 0x02],
    #[doc = "0x08 - UART2 receiver buffer, receiving byte _ UART2 transmitter holding, transmittal byte"]
    pub r8_uart2_rbr_r8_uart2_thr: R8_UART2_RBR_R8_UART2_THR,
    _reserved7: [u8; 0x01],
    #[doc = "0x0a - UART2 receiver FIFO count"]
    pub r8_uart2_rfc: R8_UART2_RFC,
    #[doc = "0x0b - UART2 transmitter FIFO count"]
    pub r8_uart2_tfc: R8_UART2_TFC,
    #[doc = "0x0c - UART2 divisor latch"]
    pub r16_uart2_dl: R16_UART2_DL,
    #[doc = "0x0e - UART2 pre-divisor latch byte"]
    pub r8_uart2_div: R8_UART2_DIV,
}
#[doc = "R8_UART2_MCR (rw) register accessor: an alias for `Reg<R8_UART2_MCR_SPEC>`"]
pub type R8_UART2_MCR = crate::Reg<r8_uart2_mcr::R8_UART2_MCR_SPEC>;
#[doc = "UART2 modem control"]
pub mod r8_uart2_mcr;
#[doc = "R8_UART2_IER (rw) register accessor: an alias for `Reg<R8_UART2_IER_SPEC>`"]
pub type R8_UART2_IER = crate::Reg<r8_uart2_ier::R8_UART2_IER_SPEC>;
#[doc = "UART2 interrupt enable"]
pub mod r8_uart2_ier;
#[doc = "R8_UART2_FCR (rw) register accessor: an alias for `Reg<R8_UART2_FCR_SPEC>`"]
pub type R8_UART2_FCR = crate::Reg<r8_uart2_fcr::R8_UART2_FCR_SPEC>;
#[doc = "UART2 FIFO control"]
pub mod r8_uart2_fcr;
#[doc = "R8_UART2_LCR (rw) register accessor: an alias for `Reg<R8_UART2_LCR_SPEC>`"]
pub type R8_UART2_LCR = crate::Reg<r8_uart2_lcr::R8_UART2_LCR_SPEC>;
#[doc = "UART2 line control"]
pub mod r8_uart2_lcr;
#[doc = "R8_UART2_IIR (r) register accessor: an alias for `Reg<R8_UART2_IIR_SPEC>`"]
pub type R8_UART2_IIR = crate::Reg<r8_uart2_iir::R8_UART2_IIR_SPEC>;
#[doc = "UART2 interrupt identification"]
pub mod r8_uart2_iir;
#[doc = "R8_UART2_LSR (r) register accessor: an alias for `Reg<R8_UART2_LSR_SPEC>`"]
pub type R8_UART2_LSR = crate::Reg<r8_uart2_lsr::R8_UART2_LSR_SPEC>;
#[doc = "UART2 line status"]
pub mod r8_uart2_lsr;
#[doc = "R8_UART2_RBR_R8_UART2_THR (rw) register accessor: an alias for `Reg<R8_UART2_RBR_R8_UART2_THR_SPEC>`"]
pub type R8_UART2_RBR_R8_UART2_THR =
    crate::Reg<r8_uart2_rbr_r8_uart2_thr::R8_UART2_RBR_R8_UART2_THR_SPEC>;
#[doc = "UART2 receiver buffer, receiving byte _ UART2 transmitter holding, transmittal byte"]
pub mod r8_uart2_rbr_r8_uart2_thr;
#[doc = "R8_UART2_RFC (r) register accessor: an alias for `Reg<R8_UART2_RFC_SPEC>`"]
pub type R8_UART2_RFC = crate::Reg<r8_uart2_rfc::R8_UART2_RFC_SPEC>;
#[doc = "UART2 receiver FIFO count"]
pub mod r8_uart2_rfc;
#[doc = "R8_UART2_TFC (r) register accessor: an alias for `Reg<R8_UART2_TFC_SPEC>`"]
pub type R8_UART2_TFC = crate::Reg<r8_uart2_tfc::R8_UART2_TFC_SPEC>;
#[doc = "UART2 transmitter FIFO count"]
pub mod r8_uart2_tfc;
#[doc = "R16_UART2_DL (rw) register accessor: an alias for `Reg<R16_UART2_DL_SPEC>`"]
pub type R16_UART2_DL = crate::Reg<r16_uart2_dl::R16_UART2_DL_SPEC>;
#[doc = "UART2 divisor latch"]
pub mod r16_uart2_dl;
#[doc = "R8_UART2_DIV (rw) register accessor: an alias for `Reg<R8_UART2_DIV_SPEC>`"]
pub type R8_UART2_DIV = crate::Reg<r8_uart2_div::R8_UART2_DIV_SPEC>;
#[doc = "UART2 pre-divisor latch byte"]
pub mod r8_uart2_div;
