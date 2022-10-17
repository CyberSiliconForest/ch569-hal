#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - parallel if tx or rx cfg"]
    pub r8_hspi_cfg: R8_HSPI_CFG,
    #[doc = "0x01 - parallel if tx or rx control"]
    pub r8_hspi_ctrl: R8_HSPI_CTRL,
    #[doc = "0x02 - parallel if interrupt enable register"]
    pub r8_hspi_int_en: R8_HSPI_INT_EN,
    #[doc = "0x03 - parallel if aux"]
    pub r8_hspi_aux: R8_HSPI_AUX,
    #[doc = "0x04 - parallel if dma tx addr0"]
    pub r32_hspi_tx_addr0: R32_HSPI_TX_ADDR0,
    #[doc = "0x08 - parallel if dma tx addr1"]
    pub r32_hspi_tx_addr1: R32_HSPI_TX_ADDR1,
    #[doc = "0x0c - parallel if dma rx addr0"]
    pub r32_hspi_rx_addr0: R32_HSPI_RX_ADDR0,
    #[doc = "0x10 - parallel if dma rx addr1"]
    pub r32_hspi_rx_addr1: R32_HSPI_RX_ADDR1,
    #[doc = "0x14 - parallel if dma length0"]
    pub r16_hspi_dma_len0: R16_HSPI_DMA_LEN0,
    #[doc = "0x16 - parallel if receive length0"]
    pub r16_hspi_rx_len0: R16_HSPI_RX_LEN0,
    #[doc = "0x18 - parallel if dma length1"]
    pub r16_hspi_dma_len1: R16_HSPI_DMA_LEN1,
    #[doc = "0x1a - parallel if receive length1"]
    pub r16_hspi_rx_len1: R16_HSPI_RX_LEN1,
    #[doc = "0x1c - parallel if tx burst config register"]
    pub r16_hspi_burst_cfg: R16_HSPI_BURST_CFG,
    #[doc = "0x1e - parallel if tx burst count"]
    pub r8_hspi_burst_cnt: R8_HSPI_BURST_CNT,
    _reserved14: [u8; 0x01],
    #[doc = "0x20 - parallel if user defined field 0 register"]
    pub r32_hspi_udf0: R32_HSPI_UDF0,
    #[doc = "0x24 - parallel if user defined field 1 register"]
    pub r32_hspi_udf1: R32_HSPI_UDF1,
    #[doc = "0x28 - parallel if interrupt flag"]
    pub r8_hspi_int_flag: R8_HSPI_INT_FLAG,
    #[doc = "0x29 - parallel rtx status"]
    pub r8_hspi_rtx_status: R8_HSPI_RTX_STATUS,
    #[doc = "0x2a - parallel TX sequence ctrl"]
    pub r8_hspi_tx_sc: R8_HSPI_TX_SC,
    #[doc = "0x2b - parallel RX sequence ctrl"]
    pub hspi_rx_sc: HSPI_RX_SC,
}
#[doc = "R8_HSPI_CFG (rw) register accessor: an alias for `Reg<R8_HSPI_CFG_SPEC>`"]
pub type R8_HSPI_CFG = crate::Reg<r8_hspi_cfg::R8_HSPI_CFG_SPEC>;
#[doc = "parallel if tx or rx cfg"]
pub mod r8_hspi_cfg;
#[doc = "R8_HSPI_CTRL (rw) register accessor: an alias for `Reg<R8_HSPI_CTRL_SPEC>`"]
pub type R8_HSPI_CTRL = crate::Reg<r8_hspi_ctrl::R8_HSPI_CTRL_SPEC>;
#[doc = "parallel if tx or rx control"]
pub mod r8_hspi_ctrl;
#[doc = "R8_HSPI_INT_EN (rw) register accessor: an alias for `Reg<R8_HSPI_INT_EN_SPEC>`"]
pub type R8_HSPI_INT_EN = crate::Reg<r8_hspi_int_en::R8_HSPI_INT_EN_SPEC>;
#[doc = "parallel if interrupt enable register"]
pub mod r8_hspi_int_en;
#[doc = "R8_HSPI_AUX (rw) register accessor: an alias for `Reg<R8_HSPI_AUX_SPEC>`"]
pub type R8_HSPI_AUX = crate::Reg<r8_hspi_aux::R8_HSPI_AUX_SPEC>;
#[doc = "parallel if aux"]
pub mod r8_hspi_aux;
#[doc = "R32_HSPI_TX_ADDR0 (rw) register accessor: an alias for `Reg<R32_HSPI_TX_ADDR0_SPEC>`"]
pub type R32_HSPI_TX_ADDR0 = crate::Reg<r32_hspi_tx_addr0::R32_HSPI_TX_ADDR0_SPEC>;
#[doc = "parallel if dma tx addr0"]
pub mod r32_hspi_tx_addr0;
#[doc = "R32_HSPI_TX_ADDR1 (rw) register accessor: an alias for `Reg<R32_HSPI_TX_ADDR1_SPEC>`"]
pub type R32_HSPI_TX_ADDR1 = crate::Reg<r32_hspi_tx_addr1::R32_HSPI_TX_ADDR1_SPEC>;
#[doc = "parallel if dma tx addr1"]
pub mod r32_hspi_tx_addr1;
#[doc = "R32_HSPI_RX_ADDR0 (rw) register accessor: an alias for `Reg<R32_HSPI_RX_ADDR0_SPEC>`"]
pub type R32_HSPI_RX_ADDR0 = crate::Reg<r32_hspi_rx_addr0::R32_HSPI_RX_ADDR0_SPEC>;
#[doc = "parallel if dma rx addr0"]
pub mod r32_hspi_rx_addr0;
#[doc = "R32_HSPI_RX_ADDR1 (rw) register accessor: an alias for `Reg<R32_HSPI_RX_ADDR1_SPEC>`"]
pub type R32_HSPI_RX_ADDR1 = crate::Reg<r32_hspi_rx_addr1::R32_HSPI_RX_ADDR1_SPEC>;
#[doc = "parallel if dma rx addr1"]
pub mod r32_hspi_rx_addr1;
#[doc = "R16_HSPI_DMA_LEN0 (rw) register accessor: an alias for `Reg<R16_HSPI_DMA_LEN0_SPEC>`"]
pub type R16_HSPI_DMA_LEN0 = crate::Reg<r16_hspi_dma_len0::R16_HSPI_DMA_LEN0_SPEC>;
#[doc = "parallel if dma length0"]
pub mod r16_hspi_dma_len0;
#[doc = "R16_HSPI_RX_LEN0 (rw) register accessor: an alias for `Reg<R16_HSPI_RX_LEN0_SPEC>`"]
pub type R16_HSPI_RX_LEN0 = crate::Reg<r16_hspi_rx_len0::R16_HSPI_RX_LEN0_SPEC>;
#[doc = "parallel if receive length0"]
pub mod r16_hspi_rx_len0;
#[doc = "R16_HSPI_DMA_LEN1 (rw) register accessor: an alias for `Reg<R16_HSPI_DMA_LEN1_SPEC>`"]
pub type R16_HSPI_DMA_LEN1 = crate::Reg<r16_hspi_dma_len1::R16_HSPI_DMA_LEN1_SPEC>;
#[doc = "parallel if dma length1"]
pub mod r16_hspi_dma_len1;
#[doc = "R16_HSPI_RX_LEN1 (rw) register accessor: an alias for `Reg<R16_HSPI_RX_LEN1_SPEC>`"]
pub type R16_HSPI_RX_LEN1 = crate::Reg<r16_hspi_rx_len1::R16_HSPI_RX_LEN1_SPEC>;
#[doc = "parallel if receive length1"]
pub mod r16_hspi_rx_len1;
#[doc = "R16_HSPI_BURST_CFG (rw) register accessor: an alias for `Reg<R16_HSPI_BURST_CFG_SPEC>`"]
pub type R16_HSPI_BURST_CFG = crate::Reg<r16_hspi_burst_cfg::R16_HSPI_BURST_CFG_SPEC>;
#[doc = "parallel if tx burst config register"]
pub mod r16_hspi_burst_cfg;
#[doc = "R8_HSPI_BURST_CNT (rw) register accessor: an alias for `Reg<R8_HSPI_BURST_CNT_SPEC>`"]
pub type R8_HSPI_BURST_CNT = crate::Reg<r8_hspi_burst_cnt::R8_HSPI_BURST_CNT_SPEC>;
#[doc = "parallel if tx burst count"]
pub mod r8_hspi_burst_cnt;
#[doc = "R32_HSPI_UDF0 (rw) register accessor: an alias for `Reg<R32_HSPI_UDF0_SPEC>`"]
pub type R32_HSPI_UDF0 = crate::Reg<r32_hspi_udf0::R32_HSPI_UDF0_SPEC>;
#[doc = "parallel if user defined field 0 register"]
pub mod r32_hspi_udf0;
#[doc = "R32_HSPI_UDF1 (rw) register accessor: an alias for `Reg<R32_HSPI_UDF1_SPEC>`"]
pub type R32_HSPI_UDF1 = crate::Reg<r32_hspi_udf1::R32_HSPI_UDF1_SPEC>;
#[doc = "parallel if user defined field 1 register"]
pub mod r32_hspi_udf1;
#[doc = "R8_HSPI_INT_FLAG (rw) register accessor: an alias for `Reg<R8_HSPI_INT_FLAG_SPEC>`"]
pub type R8_HSPI_INT_FLAG = crate::Reg<r8_hspi_int_flag::R8_HSPI_INT_FLAG_SPEC>;
#[doc = "parallel if interrupt flag"]
pub mod r8_hspi_int_flag;
#[doc = "R8_HSPI_RTX_STATUS (rw) register accessor: an alias for `Reg<R8_HSPI_RTX_STATUS_SPEC>`"]
pub type R8_HSPI_RTX_STATUS = crate::Reg<r8_hspi_rtx_status::R8_HSPI_RTX_STATUS_SPEC>;
#[doc = "parallel rtx status"]
pub mod r8_hspi_rtx_status;
#[doc = "R8_HSPI_TX_SC (rw) register accessor: an alias for `Reg<R8_HSPI_TX_SC_SPEC>`"]
pub type R8_HSPI_TX_SC = crate::Reg<r8_hspi_tx_sc::R8_HSPI_TX_SC_SPEC>;
#[doc = "parallel TX sequence ctrl"]
pub mod r8_hspi_tx_sc;
#[doc = "HSPI_RX_SC (rw) register accessor: an alias for `Reg<HSPI_RX_SC_SPEC>`"]
pub type HSPI_RX_SC = crate::Reg<hspi_rx_sc::HSPI_RX_SC_SPEC>;
#[doc = "parallel RX sequence ctrl"]
pub mod hspi_rx_sc;
