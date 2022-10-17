#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UART0 modem control"]
    pub r8_uart0_mcr: R8_UART0_MCR,
    #[doc = "0x01 - UART0 interrupt enable"]
    pub r8_uart0_ier: R8_UART0_IER,
    #[doc = "0x02 - UART0 FIFO control"]
    pub r8_uart0_fcr: R8_UART0_FCR,
    #[doc = "0x03 - UART0 line control"]
    pub r8_uart0_lcr: R8_UART0_LCR,
    #[doc = "0x04 - UART0 interrupt identification"]
    pub r8_uart0_iir: R8_UART0_IIR,
    #[doc = "0x05 - UART0 line status"]
    pub r8_uart0_lsr: R8_UART0_LSR,
    #[doc = "0x06 - UART0 modem status"]
    pub r8_uart0_msr: R8_UART0_MSR,
    _reserved7: [u8; 0x01],
    #[doc = "0x08 - UART0 receiver buffer, receiving byte _ UART0 transmitter holding, transmittal byte"]
    pub r8_uart0_rbr_r8_uart0_thr: R8_UART0_RBR_R8_UART0_THR,
    _reserved8: [u8; 0x01],
    #[doc = "0x0a - UART0 receiver FIFO count"]
    pub r8_uart0_rfc: R8_UART0_RFC,
    #[doc = "0x0b - UART0 transmitter FIFO count"]
    pub r8_uart0_tfc: R8_UART0_TFC,
    #[doc = "0x0c - UART0 divisor latch"]
    pub r16_uart0_dl: R16_UART0_DL,
    #[doc = "0x0e - UART0 pre-divisor latch byte"]
    pub r8_uart0_div: R8_UART0_DIV,
    #[doc = "0x0f - UART0 slave address"]
    pub r8_uart0_adr: R8_UART0_ADR,
}
#[doc = "R8_UART0_MCR (rw) register accessor: an alias for `Reg<R8_UART0_MCR_SPEC>`"]
pub type R8_UART0_MCR = crate::Reg<r8_uart0_mcr::R8_UART0_MCR_SPEC>;
#[doc = "UART0 modem control"]
pub mod r8_uart0_mcr;
#[doc = "R8_UART0_IER (rw) register accessor: an alias for `Reg<R8_UART0_IER_SPEC>`"]
pub type R8_UART0_IER = crate::Reg<r8_uart0_ier::R8_UART0_IER_SPEC>;
#[doc = "UART0 interrupt enable"]
pub mod r8_uart0_ier;
#[doc = "R8_UART0_FCR (rw) register accessor: an alias for `Reg<R8_UART0_FCR_SPEC>`"]
pub type R8_UART0_FCR = crate::Reg<r8_uart0_fcr::R8_UART0_FCR_SPEC>;
#[doc = "UART0 FIFO control"]
pub mod r8_uart0_fcr;
#[doc = "R8_UART0_LCR (rw) register accessor: an alias for `Reg<R8_UART0_LCR_SPEC>`"]
pub type R8_UART0_LCR = crate::Reg<r8_uart0_lcr::R8_UART0_LCR_SPEC>;
#[doc = "UART0 line control"]
pub mod r8_uart0_lcr;
#[doc = "R8_UART0_IIR (r) register accessor: an alias for `Reg<R8_UART0_IIR_SPEC>`"]
pub type R8_UART0_IIR = crate::Reg<r8_uart0_iir::R8_UART0_IIR_SPEC>;
#[doc = "UART0 interrupt identification"]
pub mod r8_uart0_iir;
#[doc = "R8_UART0_LSR (r) register accessor: an alias for `Reg<R8_UART0_LSR_SPEC>`"]
pub type R8_UART0_LSR = crate::Reg<r8_uart0_lsr::R8_UART0_LSR_SPEC>;
#[doc = "UART0 line status"]
pub mod r8_uart0_lsr;
#[doc = "R8_UART0_MSR (r) register accessor: an alias for `Reg<R8_UART0_MSR_SPEC>`"]
pub type R8_UART0_MSR = crate::Reg<r8_uart0_msr::R8_UART0_MSR_SPEC>;
#[doc = "UART0 modem status"]
pub mod r8_uart0_msr;
#[doc = "R8_UART0_RBR_R8_UART0_THR (rw) register accessor: an alias for `Reg<R8_UART0_RBR_R8_UART0_THR_SPEC>`"]
pub type R8_UART0_RBR_R8_UART0_THR =
    crate::Reg<r8_uart0_rbr_r8_uart0_thr::R8_UART0_RBR_R8_UART0_THR_SPEC>;
#[doc = "UART0 receiver buffer, receiving byte _ UART0 transmitter holding, transmittal byte"]
pub mod r8_uart0_rbr_r8_uart0_thr;
#[doc = "R8_UART0_RFC (r) register accessor: an alias for `Reg<R8_UART0_RFC_SPEC>`"]
pub type R8_UART0_RFC = crate::Reg<r8_uart0_rfc::R8_UART0_RFC_SPEC>;
#[doc = "UART0 receiver FIFO count"]
pub mod r8_uart0_rfc;
#[doc = "R8_UART0_TFC (r) register accessor: an alias for `Reg<R8_UART0_TFC_SPEC>`"]
pub type R8_UART0_TFC = crate::Reg<r8_uart0_tfc::R8_UART0_TFC_SPEC>;
#[doc = "UART0 transmitter FIFO count"]
pub mod r8_uart0_tfc;
#[doc = "R16_UART0_DL (rw) register accessor: an alias for `Reg<R16_UART0_DL_SPEC>`"]
pub type R16_UART0_DL = crate::Reg<r16_uart0_dl::R16_UART0_DL_SPEC>;
#[doc = "UART0 divisor latch"]
pub mod r16_uart0_dl;
#[doc = "R8_UART0_DIV (rw) register accessor: an alias for `Reg<R8_UART0_DIV_SPEC>`"]
pub type R8_UART0_DIV = crate::Reg<r8_uart0_div::R8_UART0_DIV_SPEC>;
#[doc = "UART0 pre-divisor latch byte"]
pub mod r8_uart0_div;
#[doc = "R8_UART0_ADR (rw) register accessor: an alias for `Reg<R8_UART0_ADR_SPEC>`"]
pub type R8_UART0_ADR = crate::Reg<r8_uart0_adr::R8_UART0_ADR_SPEC>;
#[doc = "UART0 slave address"]
pub mod r8_uart0_adr;
