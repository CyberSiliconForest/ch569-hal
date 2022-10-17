#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UART1 modem control"]
    pub r8_uart1_mcr: R8_UART1_MCR,
    #[doc = "0x01 - UART1 interrupt enable"]
    pub r8_uart1_ier: R8_UART1_IER,
    #[doc = "0x02 - UART1 FIFO control"]
    pub r8_uart1_fcr: R8_UART1_FCR,
    #[doc = "0x03 - UART1 line control"]
    pub r8_uart1_lcr: R8_UART1_LCR,
    #[doc = "0x04 - UART1 interrupt identification"]
    pub r8_uart1_iir: R8_UART1_IIR,
    #[doc = "0x05 - UART1 line status"]
    pub r8_uart1_lsr: R8_UART1_LSR,
    _reserved6: [u8; 0x02],
    #[doc = "0x08 - UART1 receiver buffer, receiving byte _ UART1 transmitter holding, transmittal byte"]
    pub r8_uart1_rbr_r8_uart1_thr: R8_UART1_RBR_R8_UART1_THR,
    _reserved7: [u8; 0x01],
    #[doc = "0x0a - UART1 receiver FIFO count"]
    pub r8_uart1_rfc: R8_UART1_RFC,
    #[doc = "0x0b - UART1 transmitter FIFO count"]
    pub r8_uart1_tfc: R8_UART1_TFC,
    #[doc = "0x0c - UART1 divisor latch"]
    pub r16_uart1_dl: R16_UART1_DL,
    #[doc = "0x0e - UART1 pre-divisor latch byte"]
    pub r8_uart1_div: R8_UART1_DIV,
}
#[doc = "R8_UART1_MCR (rw) register accessor: an alias for `Reg<R8_UART1_MCR_SPEC>`"]
pub type R8_UART1_MCR = crate::Reg<r8_uart1_mcr::R8_UART1_MCR_SPEC>;
#[doc = "UART1 modem control"]
pub mod r8_uart1_mcr;
#[doc = "R8_UART1_IER (rw) register accessor: an alias for `Reg<R8_UART1_IER_SPEC>`"]
pub type R8_UART1_IER = crate::Reg<r8_uart1_ier::R8_UART1_IER_SPEC>;
#[doc = "UART1 interrupt enable"]
pub mod r8_uart1_ier;
#[doc = "R8_UART1_FCR (rw) register accessor: an alias for `Reg<R8_UART1_FCR_SPEC>`"]
pub type R8_UART1_FCR = crate::Reg<r8_uart1_fcr::R8_UART1_FCR_SPEC>;
#[doc = "UART1 FIFO control"]
pub mod r8_uart1_fcr;
#[doc = "R8_UART1_LCR (rw) register accessor: an alias for `Reg<R8_UART1_LCR_SPEC>`"]
pub type R8_UART1_LCR = crate::Reg<r8_uart1_lcr::R8_UART1_LCR_SPEC>;
#[doc = "UART1 line control"]
pub mod r8_uart1_lcr;
#[doc = "R8_UART1_IIR (r) register accessor: an alias for `Reg<R8_UART1_IIR_SPEC>`"]
pub type R8_UART1_IIR = crate::Reg<r8_uart1_iir::R8_UART1_IIR_SPEC>;
#[doc = "UART1 interrupt identification"]
pub mod r8_uart1_iir;
#[doc = "R8_UART1_LSR (r) register accessor: an alias for `Reg<R8_UART1_LSR_SPEC>`"]
pub type R8_UART1_LSR = crate::Reg<r8_uart1_lsr::R8_UART1_LSR_SPEC>;
#[doc = "UART1 line status"]
pub mod r8_uart1_lsr;
#[doc = "R8_UART1_RBR_R8_UART1_THR (rw) register accessor: an alias for `Reg<R8_UART1_RBR_R8_UART1_THR_SPEC>`"]
pub type R8_UART1_RBR_R8_UART1_THR =
    crate::Reg<r8_uart1_rbr_r8_uart1_thr::R8_UART1_RBR_R8_UART1_THR_SPEC>;
#[doc = "UART1 receiver buffer, receiving byte _ UART1 transmitter holding, transmittal byte"]
pub mod r8_uart1_rbr_r8_uart1_thr;
#[doc = "R8_UART1_RFC (r) register accessor: an alias for `Reg<R8_UART1_RFC_SPEC>`"]
pub type R8_UART1_RFC = crate::Reg<r8_uart1_rfc::R8_UART1_RFC_SPEC>;
#[doc = "UART1 receiver FIFO count"]
pub mod r8_uart1_rfc;
#[doc = "R8_UART1_TFC (r) register accessor: an alias for `Reg<R8_UART1_TFC_SPEC>`"]
pub type R8_UART1_TFC = crate::Reg<r8_uart1_tfc::R8_UART1_TFC_SPEC>;
#[doc = "UART1 transmitter FIFO count"]
pub mod r8_uart1_tfc;
#[doc = "R16_UART1_DL (rw) register accessor: an alias for `Reg<R16_UART1_DL_SPEC>`"]
pub type R16_UART1_DL = crate::Reg<r16_uart1_dl::R16_UART1_DL_SPEC>;
#[doc = "UART1 divisor latch"]
pub mod r16_uart1_dl;
#[doc = "R8_UART1_DIV (rw) register accessor: an alias for `Reg<R8_UART1_DIV_SPEC>`"]
pub type R8_UART1_DIV = crate::Reg<r8_uart1_div::R8_UART1_DIV_SPEC>;
#[doc = "UART1 pre-divisor latch byte"]
pub mod r8_uart1_div;
