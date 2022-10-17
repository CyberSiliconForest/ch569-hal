#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB base control"]
    pub r8_usb_ctrl: R8_USB_CTRL,
    #[doc = "0x01 - USB host control register"]
    pub r8_uhost_ctrl: R8_UHOST_CTRL,
    #[doc = "0x02 - USB interrupt enable"]
    pub r8_usb_int_en: R8_USB_INT_EN,
    #[doc = "0x03 - USB device address"]
    pub r8_usb_dev_ad: R8_USB_DEV_AD,
    #[doc = "0x04 - USB frame number register"]
    pub r16_usb_frame_no: R16_USB_FRAME_NO,
    #[doc = "0x06 - USB suspend register"]
    pub r8_usb_suspend: R8_USB_SUSPEND,
    _reserved6: [u8; 0x01],
    #[doc = "0x08 - USB actual speed register"]
    pub r8_usb_spd_type: R8_USB_SPD_TYPE,
    #[doc = "0x09 - USB miscellaneous status"]
    pub r8_usb_mis_st: R8_USB_MIS_ST,
    #[doc = "0x0a - USB interrupt flag"]
    pub r8_usb_int_fg: R8_USB_INT_FG,
    #[doc = "0x0b - USB interrupt status"]
    pub r8_usb_int_st: R8_USB_INT_ST,
    #[doc = "0x0c - USB receiving length"]
    pub r6_usb_rx_len: R6_USB_RX_LEN,
    _reserved11: [u8; 0x02],
    #[doc = "0x10 - endpoint 1(9) 4(8,12) mode"]
    pub r8_uep4_1_mod: R8_UEP4_1_MOD,
    #[doc = "0x11 - endpoint 2(10) 3(11) mode and USB host endpoint mode control register"]
    pub r8_uep2_3_mod_r8_uh_ep_mod: R8_UEP2_3_MOD_R8_UH_EP_MOD,
    #[doc = "0x12 - endpoint 5(13) 6(14) mode"]
    pub r8_uep5_6_mod: R8_UEP5_6_MOD,
    #[doc = "0x13 - endpoint 7(15) mode"]
    pub r8_uep7_mod: R8_UEP7_MOD,
    #[doc = "0x14 - endpoint 0 DMA buffer address"]
    pub r32_uep0_rt_dma: R32_UEP0_RT_DMA,
    #[doc = "0x18 - endpoint 1 DMA buffer address"]
    pub r32_uep1_rx_dma: R32_UEP1_RX_DMA,
    #[doc = "0x1c - endpoint 2 DMA buffer address _ host rx endpoint buffer start address"]
    pub r32_uep2_rx_dma_r32_uh_rx_dma: R32_UEP2_RX_DMA_R32_UH_RX_DMA,
    #[doc = "0x20 - endpoint 3 DMA buffer address;host tx endpoint buffer high address"]
    pub r32_uep3_rx_dma: R32_UEP3_RX_DMA,
    #[doc = "0x24 - endpoint 4 DMA buffer address"]
    pub r32_uep4_rx_dma: R32_UEP4_RX_DMA,
    #[doc = "0x28 - endpoint 5 DMA buffer address"]
    pub r32_uep5_rx_dma: R32_UEP5_RX_DMA,
    #[doc = "0x2c - endpoint 6 DMA buffer address"]
    pub r32_uep6_rx_dma: R32_UEP6_RX_DMA,
    #[doc = "0x30 - endpoint 7 DMA buffer address"]
    pub r32_uep7_rx_dma: R32_UEP7_RX_DMA,
    #[doc = "0x34 - endpoint 1 DMA TX buffer address"]
    pub r32_uep1_tx_dma: R32_UEP1_TX_DMA,
    #[doc = "0x38 - endpoint 2 DMA TX buffer address"]
    pub r32_uep2_tx_dma: R32_UEP2_TX_DMA,
    #[doc = "0x3c - endpoint 3 DMA TX buffer address and host tx endpoint buffer start address"]
    pub r32_uep3_tx_dma_r32_uh_tx_dma: R32_UEP3_TX_DMA_R32_UH_TX_DMA,
    #[doc = "0x40 - endpoint 4 DMA TX buffer address"]
    pub r32_uep4_tx_dma: R32_UEP4_TX_DMA,
    #[doc = "0x44 - endpoint 5 DMA TX buffer address"]
    pub r32_uep5_tx_dma: R32_UEP5_TX_DMA,
    #[doc = "0x48 - endpoint 4 DMA TX buffer address"]
    pub r32_uep6_tx_dma: R32_UEP6_TX_DMA,
    #[doc = "0x4c - endpoint 7 DMA TX buffer address"]
    pub r32_uep7_tx_dma: R32_UEP7_TX_DMA,
    #[doc = "0x50 - endpoint 0 receive max length"]
    pub r16_uep0_max_len: R16_UEP0_MAX_LEN,
    _reserved31: [u8; 0x02],
    #[doc = "0x54 - endpoint 1 receive max length"]
    pub r16_uep1_max_len: R16_UEP1_MAX_LEN,
    _reserved32: [u8; 0x02],
    #[doc = "0x58 - endpoint 2 receive max length and USB host receive max packet length register"]
    pub r16_uep2_max_len_r16_uh_max_len: R16_UEP2_MAX_LEN_R16_UH_MAX_LEN,
    _reserved33: [u8; 0x02],
    #[doc = "0x5c - endpoint 3 receive max length"]
    pub r16_uep3_max_len: R16_UEP3_MAX_LEN,
    _reserved34: [u8; 0x02],
    #[doc = "0x60 - endpoint 4 receive max length"]
    pub r16_uep4_max_len: R16_UEP4_MAX_LEN,
    _reserved35: [u8; 0x02],
    #[doc = "0x64 - endpoint 5 receive max length"]
    pub r16_uep5_max_len: R16_UEP5_MAX_LEN,
    _reserved36: [u8; 0x02],
    #[doc = "0x68 - endpoint 6 receive max length"]
    pub r16_uep6_max_len: R16_UEP6_MAX_LEN,
    _reserved37: [u8; 0x02],
    #[doc = "0x6c - endpoint 7 receive max length"]
    pub r16_uep7_max_len: R16_UEP7_MAX_LEN,
    _reserved38: [u8; 0x02],
    #[doc = "0x70 - endpoint 0 transmittal length"]
    pub r16_uep0_t_len: R16_UEP0_T_LEN,
    #[doc = "0x72 - endpoint 0 tx control"]
    pub r8_uep0_tx_ctrl: R8_UEP0_TX_CTRL,
    #[doc = "0x73 - endpoint 0 rx control"]
    pub r8_uep0_rx_ctrl: R8_UEP0_RX_CTRL,
    #[doc = "0x74 - endpoint 1 transmittal length"]
    pub r16_uep1_t_len: R16_UEP1_T_LEN,
    #[doc = "0x76 - endpoint 1 tx control"]
    pub r8_uep1_tx_ctrl: R8_UEP1_TX_CTRL,
    #[doc = "0x77 - endpoint 1 rx control"]
    pub r8_uep1_rx_ctrl: R8_UEP1_RX_CTRL,
    #[doc = "0x78 - endpoint 2 transmittal length and Set usb host token register"]
    pub r16_uep2_t_len_r16_uh_ep_pid: R16_UEP2_T_LEN_R16_UH_EP_PID,
    #[doc = "0x7a - endpoint 2 tx control"]
    pub r8_uep2_tx_ctrl: R8_UEP2_TX_CTRL,
    #[doc = "0x7b - endpoint 2 rx control and USb host receive endpoint control register"]
    pub r8_uep2_rx_ctrl_r8_uh_rx_ctrl: R8_UEP2_RX_CTRL_R8_UH_RX_CTRL,
    #[doc = "0x7c - endpoint 3 transmittal length and host transmittal endpoint transmittal length"]
    pub r16_uep3_t_len_r16_uh_tx_len: R16_UEP3_T_LEN_R16_UH_TX_LEN,
    #[doc = "0x7e - endpoint 3 tx control and host transmittal endpoint control"]
    pub r8_uep3_tx_ctrl_r8_uh_tx_ctrl: R8_UEP3_TX_CTRL_R8_UH_TX_CTRL,
    #[doc = "0x7f - endpoint 3 rx control"]
    pub r8_uep3_rx_ctrl: R8_UEP3_RX_CTRL,
    #[doc = "0x80 - endpoint 4 transmittal length and USB host Tx SPLIT packet data"]
    pub r16_uep4_t_len_r16_uh_split_data: R16_UEP4_T_LEN_R16_UH_SPLIT_DATA,
    #[doc = "0x82 - endpoint 4 tx control"]
    pub r8_uep4_tx_ctrl: R8_UEP4_TX_CTRL,
    #[doc = "0x83 - endpoint 4 rx control"]
    pub r8_uep4_rx_ctrl: R8_UEP4_RX_CTRL,
    #[doc = "0x84 - endpoint 5 transmittal length"]
    pub r16_uep5_t_len: R16_UEP5_T_LEN,
    #[doc = "0x86 - endpoint 5 tx control"]
    pub r8_uep5_tx_ctrl: R8_UEP5_TX_CTRL,
    #[doc = "0x87 - endpoint 5 rx control"]
    pub r8_uep5_rx_ctrl: R8_UEP5_RX_CTRL,
    #[doc = "0x88 - endpoint 6 transmittal length"]
    pub r16_uep6_t_len: R16_UEP6_T_LEN,
    #[doc = "0x8a - endpoint 6 tx control"]
    pub r8_uep6_tx_ctrl: R8_UEP6_TX_CTRL,
    #[doc = "0x8b - endpoint 6 rx control"]
    pub r8_uep6_rx_ctrl: R8_UEP6_RX_CTRL,
    #[doc = "0x8c - endpoint 7 transmittal length"]
    pub r16_uep7_t_len: R16_UEP7_T_LEN,
    #[doc = "0x8e - endpoint 7 tx control"]
    pub r8_uep7_tx_ctrl: R8_UEP7_TX_CTRL,
    #[doc = "0x8f - endpoint 7 rx control"]
    pub r8_uep7_rx_ctrl: R8_UEP7_RX_CTRL,
}
#[doc = "R8_USB_CTRL (rw) register accessor: an alias for `Reg<R8_USB_CTRL_SPEC>`"]
pub type R8_USB_CTRL = crate::Reg<r8_usb_ctrl::R8_USB_CTRL_SPEC>;
#[doc = "USB base control"]
pub mod r8_usb_ctrl;
#[doc = "R8_UHOST_CTRL (rw) register accessor: an alias for `Reg<R8_UHOST_CTRL_SPEC>`"]
pub type R8_UHOST_CTRL = crate::Reg<r8_uhost_ctrl::R8_UHOST_CTRL_SPEC>;
#[doc = "USB host control register"]
pub mod r8_uhost_ctrl;
#[doc = "R8_USB_INT_EN (rw) register accessor: an alias for `Reg<R8_USB_INT_EN_SPEC>`"]
pub type R8_USB_INT_EN = crate::Reg<r8_usb_int_en::R8_USB_INT_EN_SPEC>;
#[doc = "USB interrupt enable"]
pub mod r8_usb_int_en;
#[doc = "R8_USB_DEV_AD (rw) register accessor: an alias for `Reg<R8_USB_DEV_AD_SPEC>`"]
pub type R8_USB_DEV_AD = crate::Reg<r8_usb_dev_ad::R8_USB_DEV_AD_SPEC>;
#[doc = "USB device address"]
pub mod r8_usb_dev_ad;
#[doc = "R16_USB_FRAME_NO (r) register accessor: an alias for `Reg<R16_USB_FRAME_NO_SPEC>`"]
pub type R16_USB_FRAME_NO = crate::Reg<r16_usb_frame_no::R16_USB_FRAME_NO_SPEC>;
#[doc = "USB frame number register"]
pub mod r16_usb_frame_no;
#[doc = "R8_USB_SUSPEND (rw) register accessor: an alias for `Reg<R8_USB_SUSPEND_SPEC>`"]
pub type R8_USB_SUSPEND = crate::Reg<r8_usb_suspend::R8_USB_SUSPEND_SPEC>;
#[doc = "USB suspend register"]
pub mod r8_usb_suspend;
#[doc = "R8_USB_SPD_TYPE (r) register accessor: an alias for `Reg<R8_USB_SPD_TYPE_SPEC>`"]
pub type R8_USB_SPD_TYPE = crate::Reg<r8_usb_spd_type::R8_USB_SPD_TYPE_SPEC>;
#[doc = "USB actual speed register"]
pub mod r8_usb_spd_type;
#[doc = "R8_USB_MIS_ST (r) register accessor: an alias for `Reg<R8_USB_MIS_ST_SPEC>`"]
pub type R8_USB_MIS_ST = crate::Reg<r8_usb_mis_st::R8_USB_MIS_ST_SPEC>;
#[doc = "USB miscellaneous status"]
pub mod r8_usb_mis_st;
#[doc = "R8_USB_INT_FG (rw) register accessor: an alias for `Reg<R8_USB_INT_FG_SPEC>`"]
pub type R8_USB_INT_FG = crate::Reg<r8_usb_int_fg::R8_USB_INT_FG_SPEC>;
#[doc = "USB interrupt flag"]
pub mod r8_usb_int_fg;
#[doc = "R8_USB_INT_ST (r) register accessor: an alias for `Reg<R8_USB_INT_ST_SPEC>`"]
pub type R8_USB_INT_ST = crate::Reg<r8_usb_int_st::R8_USB_INT_ST_SPEC>;
#[doc = "USB interrupt status"]
pub mod r8_usb_int_st;
#[doc = "R6_USB_RX_LEN (r) register accessor: an alias for `Reg<R6_USB_RX_LEN_SPEC>`"]
pub type R6_USB_RX_LEN = crate::Reg<r6_usb_rx_len::R6_USB_RX_LEN_SPEC>;
#[doc = "USB receiving length"]
pub mod r6_usb_rx_len;
#[doc = "R8_UEP4_1_MOD (rw) register accessor: an alias for `Reg<R8_UEP4_1_MOD_SPEC>`"]
pub type R8_UEP4_1_MOD = crate::Reg<r8_uep4_1_mod::R8_UEP4_1_MOD_SPEC>;
#[doc = "endpoint 1(9) 4(8,12) mode"]
pub mod r8_uep4_1_mod;
#[doc = "R8_UEP2_3_MOD_R8_UH_EP_MOD (rw) register accessor: an alias for `Reg<R8_UEP2_3_MOD_R8_UH_EP_MOD_SPEC>`"]
pub type R8_UEP2_3_MOD_R8_UH_EP_MOD =
    crate::Reg<r8_uep2_3_mod_r8_uh_ep_mod::R8_UEP2_3_MOD_R8_UH_EP_MOD_SPEC>;
#[doc = "endpoint 2(10) 3(11) mode and USB host endpoint mode control register"]
pub mod r8_uep2_3_mod_r8_uh_ep_mod;
#[doc = "R8_UEP5_6_MOD (rw) register accessor: an alias for `Reg<R8_UEP5_6_MOD_SPEC>`"]
pub type R8_UEP5_6_MOD = crate::Reg<r8_uep5_6_mod::R8_UEP5_6_MOD_SPEC>;
#[doc = "endpoint 5(13) 6(14) mode"]
pub mod r8_uep5_6_mod;
#[doc = "R8_UEP7_MOD (rw) register accessor: an alias for `Reg<R8_UEP7_MOD_SPEC>`"]
pub type R8_UEP7_MOD = crate::Reg<r8_uep7_mod::R8_UEP7_MOD_SPEC>;
#[doc = "endpoint 7(15) mode"]
pub mod r8_uep7_mod;
#[doc = "R32_UEP0_RT_DMA (rw) register accessor: an alias for `Reg<R32_UEP0_RT_DMA_SPEC>`"]
pub type R32_UEP0_RT_DMA = crate::Reg<r32_uep0_rt_dma::R32_UEP0_RT_DMA_SPEC>;
#[doc = "endpoint 0 DMA buffer address"]
pub mod r32_uep0_rt_dma;
#[doc = "R32_UEP1_RX_DMA (rw) register accessor: an alias for `Reg<R32_UEP1_RX_DMA_SPEC>`"]
pub type R32_UEP1_RX_DMA = crate::Reg<r32_uep1_rx_dma::R32_UEP1_RX_DMA_SPEC>;
#[doc = "endpoint 1 DMA buffer address"]
pub mod r32_uep1_rx_dma;
#[doc = "R32_UEP2_RX_DMA_R32_UH_RX_DMA (rw) register accessor: an alias for `Reg<R32_UEP2_RX_DMA_R32_UH_RX_DMA_SPEC>`"]
pub type R32_UEP2_RX_DMA_R32_UH_RX_DMA =
    crate::Reg<r32_uep2_rx_dma_r32_uh_rx_dma::R32_UEP2_RX_DMA_R32_UH_RX_DMA_SPEC>;
#[doc = "endpoint 2 DMA buffer address _ host rx endpoint buffer start address"]
pub mod r32_uep2_rx_dma_r32_uh_rx_dma;
#[doc = "R32_UEP3_RX_DMA (rw) register accessor: an alias for `Reg<R32_UEP3_RX_DMA_SPEC>`"]
pub type R32_UEP3_RX_DMA = crate::Reg<r32_uep3_rx_dma::R32_UEP3_RX_DMA_SPEC>;
#[doc = "endpoint 3 DMA buffer address;host tx endpoint buffer high address"]
pub mod r32_uep3_rx_dma;
#[doc = "R32_UEP4_RX_DMA (rw) register accessor: an alias for `Reg<R32_UEP4_RX_DMA_SPEC>`"]
pub type R32_UEP4_RX_DMA = crate::Reg<r32_uep4_rx_dma::R32_UEP4_RX_DMA_SPEC>;
#[doc = "endpoint 4 DMA buffer address"]
pub mod r32_uep4_rx_dma;
#[doc = "R32_UEP5_RX_DMA (rw) register accessor: an alias for `Reg<R32_UEP5_RX_DMA_SPEC>`"]
pub type R32_UEP5_RX_DMA = crate::Reg<r32_uep5_rx_dma::R32_UEP5_RX_DMA_SPEC>;
#[doc = "endpoint 5 DMA buffer address"]
pub mod r32_uep5_rx_dma;
#[doc = "R32_UEP6_RX_DMA (rw) register accessor: an alias for `Reg<R32_UEP6_RX_DMA_SPEC>`"]
pub type R32_UEP6_RX_DMA = crate::Reg<r32_uep6_rx_dma::R32_UEP6_RX_DMA_SPEC>;
#[doc = "endpoint 6 DMA buffer address"]
pub mod r32_uep6_rx_dma;
#[doc = "R32_UEP7_RX_DMA (rw) register accessor: an alias for `Reg<R32_UEP7_RX_DMA_SPEC>`"]
pub type R32_UEP7_RX_DMA = crate::Reg<r32_uep7_rx_dma::R32_UEP7_RX_DMA_SPEC>;
#[doc = "endpoint 7 DMA buffer address"]
pub mod r32_uep7_rx_dma;
#[doc = "R32_UEP1_TX_DMA (rw) register accessor: an alias for `Reg<R32_UEP1_TX_DMA_SPEC>`"]
pub type R32_UEP1_TX_DMA = crate::Reg<r32_uep1_tx_dma::R32_UEP1_TX_DMA_SPEC>;
#[doc = "endpoint 1 DMA TX buffer address"]
pub mod r32_uep1_tx_dma;
#[doc = "R32_UEP2_TX_DMA (rw) register accessor: an alias for `Reg<R32_UEP2_TX_DMA_SPEC>`"]
pub type R32_UEP2_TX_DMA = crate::Reg<r32_uep2_tx_dma::R32_UEP2_TX_DMA_SPEC>;
#[doc = "endpoint 2 DMA TX buffer address"]
pub mod r32_uep2_tx_dma;
#[doc = "R32_UEP3_TX_DMA_R32_UH_TX_DMA (rw) register accessor: an alias for `Reg<R32_UEP3_TX_DMA_R32_UH_TX_DMA_SPEC>`"]
pub type R32_UEP3_TX_DMA_R32_UH_TX_DMA =
    crate::Reg<r32_uep3_tx_dma_r32_uh_tx_dma::R32_UEP3_TX_DMA_R32_UH_TX_DMA_SPEC>;
#[doc = "endpoint 3 DMA TX buffer address and host tx endpoint buffer start address"]
pub mod r32_uep3_tx_dma_r32_uh_tx_dma;
#[doc = "R32_UEP4_TX_DMA (rw) register accessor: an alias for `Reg<R32_UEP4_TX_DMA_SPEC>`"]
pub type R32_UEP4_TX_DMA = crate::Reg<r32_uep4_tx_dma::R32_UEP4_TX_DMA_SPEC>;
#[doc = "endpoint 4 DMA TX buffer address"]
pub mod r32_uep4_tx_dma;
#[doc = "R32_UEP5_TX_DMA (rw) register accessor: an alias for `Reg<R32_UEP5_TX_DMA_SPEC>`"]
pub type R32_UEP5_TX_DMA = crate::Reg<r32_uep5_tx_dma::R32_UEP5_TX_DMA_SPEC>;
#[doc = "endpoint 5 DMA TX buffer address"]
pub mod r32_uep5_tx_dma;
#[doc = "R32_UEP6_TX_DMA (rw) register accessor: an alias for `Reg<R32_UEP6_TX_DMA_SPEC>`"]
pub type R32_UEP6_TX_DMA = crate::Reg<r32_uep6_tx_dma::R32_UEP6_TX_DMA_SPEC>;
#[doc = "endpoint 4 DMA TX buffer address"]
pub mod r32_uep6_tx_dma;
#[doc = "R32_UEP7_TX_DMA (rw) register accessor: an alias for `Reg<R32_UEP7_TX_DMA_SPEC>`"]
pub type R32_UEP7_TX_DMA = crate::Reg<r32_uep7_tx_dma::R32_UEP7_TX_DMA_SPEC>;
#[doc = "endpoint 7 DMA TX buffer address"]
pub mod r32_uep7_tx_dma;
#[doc = "R16_UEP0_MAX_LEN (rw) register accessor: an alias for `Reg<R16_UEP0_MAX_LEN_SPEC>`"]
pub type R16_UEP0_MAX_LEN = crate::Reg<r16_uep0_max_len::R16_UEP0_MAX_LEN_SPEC>;
#[doc = "endpoint 0 receive max length"]
pub mod r16_uep0_max_len;
#[doc = "R16_UEP1_MAX_LEN (rw) register accessor: an alias for `Reg<R16_UEP1_MAX_LEN_SPEC>`"]
pub type R16_UEP1_MAX_LEN = crate::Reg<r16_uep1_max_len::R16_UEP1_MAX_LEN_SPEC>;
#[doc = "endpoint 1 receive max length"]
pub mod r16_uep1_max_len;
#[doc = "R16_UEP2_MAX_LEN_R16_UH_MAX_LEN (rw) register accessor: an alias for `Reg<R16_UEP2_MAX_LEN_R16_UH_MAX_LEN_SPEC>`"]
pub type R16_UEP2_MAX_LEN_R16_UH_MAX_LEN =
    crate::Reg<r16_uep2_max_len_r16_uh_max_len::R16_UEP2_MAX_LEN_R16_UH_MAX_LEN_SPEC>;
#[doc = "endpoint 2 receive max length and USB host receive max packet length register"]
pub mod r16_uep2_max_len_r16_uh_max_len;
#[doc = "R16_UEP3_MAX_LEN (rw) register accessor: an alias for `Reg<R16_UEP3_MAX_LEN_SPEC>`"]
pub type R16_UEP3_MAX_LEN = crate::Reg<r16_uep3_max_len::R16_UEP3_MAX_LEN_SPEC>;
#[doc = "endpoint 3 receive max length"]
pub mod r16_uep3_max_len;
#[doc = "R16_UEP4_MAX_LEN (rw) register accessor: an alias for `Reg<R16_UEP4_MAX_LEN_SPEC>`"]
pub type R16_UEP4_MAX_LEN = crate::Reg<r16_uep4_max_len::R16_UEP4_MAX_LEN_SPEC>;
#[doc = "endpoint 4 receive max length"]
pub mod r16_uep4_max_len;
#[doc = "R16_UEP5_MAX_LEN (rw) register accessor: an alias for `Reg<R16_UEP5_MAX_LEN_SPEC>`"]
pub type R16_UEP5_MAX_LEN = crate::Reg<r16_uep5_max_len::R16_UEP5_MAX_LEN_SPEC>;
#[doc = "endpoint 5 receive max length"]
pub mod r16_uep5_max_len;
#[doc = "R16_UEP6_MAX_LEN (rw) register accessor: an alias for `Reg<R16_UEP6_MAX_LEN_SPEC>`"]
pub type R16_UEP6_MAX_LEN = crate::Reg<r16_uep6_max_len::R16_UEP6_MAX_LEN_SPEC>;
#[doc = "endpoint 6 receive max length"]
pub mod r16_uep6_max_len;
#[doc = "R16_UEP7_MAX_LEN (rw) register accessor: an alias for `Reg<R16_UEP7_MAX_LEN_SPEC>`"]
pub type R16_UEP7_MAX_LEN = crate::Reg<r16_uep7_max_len::R16_UEP7_MAX_LEN_SPEC>;
#[doc = "endpoint 7 receive max length"]
pub mod r16_uep7_max_len;
#[doc = "R16_UEP0_T_LEN (rw) register accessor: an alias for `Reg<R16_UEP0_T_LEN_SPEC>`"]
pub type R16_UEP0_T_LEN = crate::Reg<r16_uep0_t_len::R16_UEP0_T_LEN_SPEC>;
#[doc = "endpoint 0 transmittal length"]
pub mod r16_uep0_t_len;
#[doc = "R8_UEP0_TX_CTRL (rw) register accessor: an alias for `Reg<R8_UEP0_TX_CTRL_SPEC>`"]
pub type R8_UEP0_TX_CTRL = crate::Reg<r8_uep0_tx_ctrl::R8_UEP0_TX_CTRL_SPEC>;
#[doc = "endpoint 0 tx control"]
pub mod r8_uep0_tx_ctrl;
#[doc = "R8_UEP0_RX_CTRL (rw) register accessor: an alias for `Reg<R8_UEP0_RX_CTRL_SPEC>`"]
pub type R8_UEP0_RX_CTRL = crate::Reg<r8_uep0_rx_ctrl::R8_UEP0_RX_CTRL_SPEC>;
#[doc = "endpoint 0 rx control"]
pub mod r8_uep0_rx_ctrl;
#[doc = "R16_UEP1_T_LEN (rw) register accessor: an alias for `Reg<R16_UEP1_T_LEN_SPEC>`"]
pub type R16_UEP1_T_LEN = crate::Reg<r16_uep1_t_len::R16_UEP1_T_LEN_SPEC>;
#[doc = "endpoint 1 transmittal length"]
pub mod r16_uep1_t_len;
#[doc = "R8_UEP1_TX_CTRL (rw) register accessor: an alias for `Reg<R8_UEP1_TX_CTRL_SPEC>`"]
pub type R8_UEP1_TX_CTRL = crate::Reg<r8_uep1_tx_ctrl::R8_UEP1_TX_CTRL_SPEC>;
#[doc = "endpoint 1 tx control"]
pub mod r8_uep1_tx_ctrl;
#[doc = "R8_UEP1_RX_CTRL (rw) register accessor: an alias for `Reg<R8_UEP1_RX_CTRL_SPEC>`"]
pub type R8_UEP1_RX_CTRL = crate::Reg<r8_uep1_rx_ctrl::R8_UEP1_RX_CTRL_SPEC>;
#[doc = "endpoint 1 rx control"]
pub mod r8_uep1_rx_ctrl;
#[doc = "R16_UEP2_T_LEN_R16_UH_EP_PID (rw) register accessor: an alias for `Reg<R16_UEP2_T_LEN_R16_UH_EP_PID_SPEC>`"]
pub type R16_UEP2_T_LEN_R16_UH_EP_PID =
    crate::Reg<r16_uep2_t_len_r16_uh_ep_pid::R16_UEP2_T_LEN_R16_UH_EP_PID_SPEC>;
#[doc = "endpoint 2 transmittal length and Set usb host token register"]
pub mod r16_uep2_t_len_r16_uh_ep_pid;
#[doc = "R8_UEP2_TX_CTRL (rw) register accessor: an alias for `Reg<R8_UEP2_TX_CTRL_SPEC>`"]
pub type R8_UEP2_TX_CTRL = crate::Reg<r8_uep2_tx_ctrl::R8_UEP2_TX_CTRL_SPEC>;
#[doc = "endpoint 2 tx control"]
pub mod r8_uep2_tx_ctrl;
#[doc = "R8_UEP2_RX_CTRL_R8_UH_RX_CTRL (rw) register accessor: an alias for `Reg<R8_UEP2_RX_CTRL_R8_UH_RX_CTRL_SPEC>`"]
pub type R8_UEP2_RX_CTRL_R8_UH_RX_CTRL =
    crate::Reg<r8_uep2_rx_ctrl_r8_uh_rx_ctrl::R8_UEP2_RX_CTRL_R8_UH_RX_CTRL_SPEC>;
#[doc = "endpoint 2 rx control and USb host receive endpoint control register"]
pub mod r8_uep2_rx_ctrl_r8_uh_rx_ctrl;
#[doc = "R16_UEP3_T_LEN_R16_UH_TX_LEN (rw) register accessor: an alias for `Reg<R16_UEP3_T_LEN_R16_UH_TX_LEN_SPEC>`"]
pub type R16_UEP3_T_LEN_R16_UH_TX_LEN =
    crate::Reg<r16_uep3_t_len_r16_uh_tx_len::R16_UEP3_T_LEN_R16_UH_TX_LEN_SPEC>;
#[doc = "endpoint 3 transmittal length and host transmittal endpoint transmittal length"]
pub mod r16_uep3_t_len_r16_uh_tx_len;
#[doc = "R8_UEP3_TX_CTRL_R8_UH_TX_CTRL (rw) register accessor: an alias for `Reg<R8_UEP3_TX_CTRL_R8_UH_TX_CTRL_SPEC>`"]
pub type R8_UEP3_TX_CTRL_R8_UH_TX_CTRL =
    crate::Reg<r8_uep3_tx_ctrl_r8_uh_tx_ctrl::R8_UEP3_TX_CTRL_R8_UH_TX_CTRL_SPEC>;
#[doc = "endpoint 3 tx control and host transmittal endpoint control"]
pub mod r8_uep3_tx_ctrl_r8_uh_tx_ctrl;
#[doc = "R8_UEP3_RX_CTRL (rw) register accessor: an alias for `Reg<R8_UEP3_RX_CTRL_SPEC>`"]
pub type R8_UEP3_RX_CTRL = crate::Reg<r8_uep3_rx_ctrl::R8_UEP3_RX_CTRL_SPEC>;
#[doc = "endpoint 3 rx control"]
pub mod r8_uep3_rx_ctrl;
#[doc = "R16_UEP4_T_LEN_R16_UH_SPLIT_DATA (rw) register accessor: an alias for `Reg<R16_UEP4_T_LEN_R16_UH_SPLIT_DATA_SPEC>`"]
pub type R16_UEP4_T_LEN_R16_UH_SPLIT_DATA =
    crate::Reg<r16_uep4_t_len_r16_uh_split_data::R16_UEP4_T_LEN_R16_UH_SPLIT_DATA_SPEC>;
#[doc = "endpoint 4 transmittal length and USB host Tx SPLIT packet data"]
pub mod r16_uep4_t_len_r16_uh_split_data;
#[doc = "R8_UEP4_TX_CTRL (rw) register accessor: an alias for `Reg<R8_UEP4_TX_CTRL_SPEC>`"]
pub type R8_UEP4_TX_CTRL = crate::Reg<r8_uep4_tx_ctrl::R8_UEP4_TX_CTRL_SPEC>;
#[doc = "endpoint 4 tx control"]
pub mod r8_uep4_tx_ctrl;
#[doc = "R8_UEP4_RX_CTRL (rw) register accessor: an alias for `Reg<R8_UEP4_RX_CTRL_SPEC>`"]
pub type R8_UEP4_RX_CTRL = crate::Reg<r8_uep4_rx_ctrl::R8_UEP4_RX_CTRL_SPEC>;
#[doc = "endpoint 4 rx control"]
pub mod r8_uep4_rx_ctrl;
#[doc = "R16_UEP5_T_LEN (rw) register accessor: an alias for `Reg<R16_UEP5_T_LEN_SPEC>`"]
pub type R16_UEP5_T_LEN = crate::Reg<r16_uep5_t_len::R16_UEP5_T_LEN_SPEC>;
#[doc = "endpoint 5 transmittal length"]
pub mod r16_uep5_t_len;
#[doc = "R8_UEP5_TX_CTRL (rw) register accessor: an alias for `Reg<R8_UEP5_TX_CTRL_SPEC>`"]
pub type R8_UEP5_TX_CTRL = crate::Reg<r8_uep5_tx_ctrl::R8_UEP5_TX_CTRL_SPEC>;
#[doc = "endpoint 5 tx control"]
pub mod r8_uep5_tx_ctrl;
#[doc = "R8_UEP5_RX_CTRL (rw) register accessor: an alias for `Reg<R8_UEP5_RX_CTRL_SPEC>`"]
pub type R8_UEP5_RX_CTRL = crate::Reg<r8_uep5_rx_ctrl::R8_UEP5_RX_CTRL_SPEC>;
#[doc = "endpoint 5 rx control"]
pub mod r8_uep5_rx_ctrl;
#[doc = "R16_UEP6_T_LEN (rw) register accessor: an alias for `Reg<R16_UEP6_T_LEN_SPEC>`"]
pub type R16_UEP6_T_LEN = crate::Reg<r16_uep6_t_len::R16_UEP6_T_LEN_SPEC>;
#[doc = "endpoint 6 transmittal length"]
pub mod r16_uep6_t_len;
#[doc = "R8_UEP6_TX_CTRL (rw) register accessor: an alias for `Reg<R8_UEP6_TX_CTRL_SPEC>`"]
pub type R8_UEP6_TX_CTRL = crate::Reg<r8_uep6_tx_ctrl::R8_UEP6_TX_CTRL_SPEC>;
#[doc = "endpoint 6 tx control"]
pub mod r8_uep6_tx_ctrl;
#[doc = "R8_UEP6_RX_CTRL (rw) register accessor: an alias for `Reg<R8_UEP6_RX_CTRL_SPEC>`"]
pub type R8_UEP6_RX_CTRL = crate::Reg<r8_uep6_rx_ctrl::R8_UEP6_RX_CTRL_SPEC>;
#[doc = "endpoint 6 rx control"]
pub mod r8_uep6_rx_ctrl;
#[doc = "R16_UEP7_T_LEN (rw) register accessor: an alias for `Reg<R16_UEP7_T_LEN_SPEC>`"]
pub type R16_UEP7_T_LEN = crate::Reg<r16_uep7_t_len::R16_UEP7_T_LEN_SPEC>;
#[doc = "endpoint 7 transmittal length"]
pub mod r16_uep7_t_len;
#[doc = "R8_UEP7_TX_CTRL (rw) register accessor: an alias for `Reg<R8_UEP7_TX_CTRL_SPEC>`"]
pub type R8_UEP7_TX_CTRL = crate::Reg<r8_uep7_tx_ctrl::R8_UEP7_TX_CTRL_SPEC>;
#[doc = "endpoint 7 tx control"]
pub mod r8_uep7_tx_ctrl;
#[doc = "R8_UEP7_RX_CTRL (rw) register accessor: an alias for `Reg<R8_UEP7_RX_CTRL_SPEC>`"]
pub type R8_UEP7_RX_CTRL = crate::Reg<r8_uep7_rx_ctrl::R8_UEP7_RX_CTRL_SPEC>;
#[doc = "endpoint 7 rx control"]
pub mod r8_uep7_rx_ctrl;
