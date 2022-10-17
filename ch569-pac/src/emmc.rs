#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SD 32bits command argument register"]
    pub r32_emmc_argument: R32_EMMC_ARGUMENT,
    #[doc = "0x04 - SD 16bits cmd setting register"]
    pub r16_emmc_cmd_set: R16_EMMC_CMD_SET,
    _reserved2: [u8; 0x02],
    #[doc = "0x08 - SD 128bits response register, \\[31:0\\]
32bits"]
    pub r32_emmc_response0: R32_EMMC_RESPONSE0,
    #[doc = "0x0c - SD 128bits response register, \\[63:32\\]
32bits"]
    pub r32_emmc_response1: R32_EMMC_RESPONSE1,
    #[doc = "0x10 - SD 128bits response register, \\[95:64\\]
32bits"]
    pub r32_emmc_response2: R32_EMMC_RESPONSE2,
    _reserved_5_r32_emmc: [u8; 0x04],
    #[doc = "0x18 - SD 8bits control register"]
    pub r8_emmc_control: R8_EMMC_CONTROL,
    _reserved7: [u8; 0x03],
    #[doc = "0x1c - SD 8bits data timeout value"]
    pub r8_emmc_timeout: R8_EMMC_TIMEOUT,
    _reserved8: [u8; 0x03],
    #[doc = "0x20 - SD status"]
    pub r32_emmc_status: R32_EMMC_STATUS,
    #[doc = "0x24 - SD 16bits interrupt flag register"]
    pub r16_emmc_int_fg: R16_EMMC_INT_FG,
    _reserved10: [u8; 0x02],
    #[doc = "0x28 - SD 16bits interrupt enable register"]
    pub r16_emmc_int_en: R16_EMMC_INT_EN,
    _reserved11: [u8; 0x02],
    #[doc = "0x2c - SD 16bits DMA start address register when to operate"]
    pub r32_emmc_dma_beg1: R32_EMMC_DMA_BEG1,
    #[doc = "0x30 - SD 32bits data counter, \\[15:0\\]
number of blocks this time will tran/recv, \\[27:16\\]
block sise(byte number) of every block in this time tran/recv"]
    pub r32_emmc_block_cfg: R32_EMMC_BLOCK_CFG,
    #[doc = "0x34 - SD TRANSFER MODE register"]
    pub r32_emmc_tran_mode: R32_EMMC_TRAN_MODE,
    #[doc = "0x38 - SD clock divider register"]
    pub r16_emmc_clk_div: R16_EMMC_CLK_DIV,
    _reserved15: [u8; 0x02],
    #[doc = "0x3c - SD 16bits DMA start address register when to operate"]
    pub r32_emmc_dma_beg2: R32_EMMC_DMA_BEG2,
}
impl RegisterBlock {
    #[doc = "0x14 - Multiplexing register of the EMMC_RESPONSE3,\\[127:96\\]
32bits"]
    #[inline(always)]
    pub fn r32_emmc_write_cont(&self) -> &R32_EMMC_WRITE_CONT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(20usize) as *const R32_EMMC_WRITE_CONT)
        }
    }
    #[doc = "0x14 - SD 128bits response register, \\[127:96\\]
32bits"]
    #[inline(always)]
    pub fn r32_emmc_response3(&self) -> &R32_EMMC_RESPONSE3 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(20usize) as *const R32_EMMC_RESPONSE3)
        }
    }
}
#[doc = "R16_EMMC_CLK_DIV (rw) register accessor: an alias for `Reg<R16_EMMC_CLK_DIV_SPEC>`"]
pub type R16_EMMC_CLK_DIV = crate::Reg<r16_emmc_clk_div::R16_EMMC_CLK_DIV_SPEC>;
#[doc = "SD clock divider register"]
pub mod r16_emmc_clk_div;
#[doc = "R32_EMMC_ARGUMENT (rw) register accessor: an alias for `Reg<R32_EMMC_ARGUMENT_SPEC>`"]
pub type R32_EMMC_ARGUMENT = crate::Reg<r32_emmc_argument::R32_EMMC_ARGUMENT_SPEC>;
#[doc = "SD 32bits command argument register"]
pub mod r32_emmc_argument;
#[doc = "R16_EMMC_CMD_SET (rw) register accessor: an alias for `Reg<R16_EMMC_CMD_SET_SPEC>`"]
pub type R16_EMMC_CMD_SET = crate::Reg<r16_emmc_cmd_set::R16_EMMC_CMD_SET_SPEC>;
#[doc = "SD 16bits cmd setting register"]
pub mod r16_emmc_cmd_set;
#[doc = "R32_EMMC_RESPONSE0 (r) register accessor: an alias for `Reg<R32_EMMC_RESPONSE0_SPEC>`"]
pub type R32_EMMC_RESPONSE0 = crate::Reg<r32_emmc_response0::R32_EMMC_RESPONSE0_SPEC>;
#[doc = "SD 128bits response register, \\[31:0\\]
32bits"]
pub mod r32_emmc_response0;
#[doc = "R32_EMMC_RESPONSE1 (r) register accessor: an alias for `Reg<R32_EMMC_RESPONSE1_SPEC>`"]
pub type R32_EMMC_RESPONSE1 = crate::Reg<r32_emmc_response1::R32_EMMC_RESPONSE1_SPEC>;
#[doc = "SD 128bits response register, \\[63:32\\]
32bits"]
pub mod r32_emmc_response1;
#[doc = "R32_EMMC_RESPONSE2 (r) register accessor: an alias for `Reg<R32_EMMC_RESPONSE2_SPEC>`"]
pub type R32_EMMC_RESPONSE2 = crate::Reg<r32_emmc_response2::R32_EMMC_RESPONSE2_SPEC>;
#[doc = "SD 128bits response register, \\[95:64\\]
32bits"]
pub mod r32_emmc_response2;
#[doc = "R32_EMMC_RESPONSE3 (r) register accessor: an alias for `Reg<R32_EMMC_RESPONSE3_SPEC>`"]
pub type R32_EMMC_RESPONSE3 = crate::Reg<r32_emmc_response3::R32_EMMC_RESPONSE3_SPEC>;
#[doc = "SD 128bits response register, \\[127:96\\]
32bits"]
pub mod r32_emmc_response3;
#[doc = "R32_EMMC_WRITE_CONT (w) register accessor: an alias for `Reg<R32_EMMC_WRITE_CONT_SPEC>`"]
pub type R32_EMMC_WRITE_CONT = crate::Reg<r32_emmc_write_cont::R32_EMMC_WRITE_CONT_SPEC>;
#[doc = "Multiplexing register of the EMMC_RESPONSE3,\\[127:96\\]
32bits"]
pub mod r32_emmc_write_cont;
#[doc = "R8_EMMC_CONTROL (rw) register accessor: an alias for `Reg<R8_EMMC_CONTROL_SPEC>`"]
pub type R8_EMMC_CONTROL = crate::Reg<r8_emmc_control::R8_EMMC_CONTROL_SPEC>;
#[doc = "SD 8bits control register"]
pub mod r8_emmc_control;
#[doc = "R8_EMMC_TIMEOUT (rw) register accessor: an alias for `Reg<R8_EMMC_TIMEOUT_SPEC>`"]
pub type R8_EMMC_TIMEOUT = crate::Reg<r8_emmc_timeout::R8_EMMC_TIMEOUT_SPEC>;
#[doc = "SD 8bits data timeout value"]
pub mod r8_emmc_timeout;
#[doc = "R32_EMMC_STATUS (r) register accessor: an alias for `Reg<R32_EMMC_STATUS_SPEC>`"]
pub type R32_EMMC_STATUS = crate::Reg<r32_emmc_status::R32_EMMC_STATUS_SPEC>;
#[doc = "SD status"]
pub mod r32_emmc_status;
#[doc = "R16_EMMC_INT_FG (rw) register accessor: an alias for `Reg<R16_EMMC_INT_FG_SPEC>`"]
pub type R16_EMMC_INT_FG = crate::Reg<r16_emmc_int_fg::R16_EMMC_INT_FG_SPEC>;
#[doc = "SD 16bits interrupt flag register"]
pub mod r16_emmc_int_fg;
#[doc = "R16_EMMC_INT_EN (rw) register accessor: an alias for `Reg<R16_EMMC_INT_EN_SPEC>`"]
pub type R16_EMMC_INT_EN = crate::Reg<r16_emmc_int_en::R16_EMMC_INT_EN_SPEC>;
#[doc = "SD 16bits interrupt enable register"]
pub mod r16_emmc_int_en;
#[doc = "R32_EMMC_DMA_BEG1 (rw) register accessor: an alias for `Reg<R32_EMMC_DMA_BEG1_SPEC>`"]
pub type R32_EMMC_DMA_BEG1 = crate::Reg<r32_emmc_dma_beg1::R32_EMMC_DMA_BEG1_SPEC>;
#[doc = "SD 16bits DMA start address register when to operate"]
pub mod r32_emmc_dma_beg1;
#[doc = "R32_EMMC_BLOCK_CFG (rw) register accessor: an alias for `Reg<R32_EMMC_BLOCK_CFG_SPEC>`"]
pub type R32_EMMC_BLOCK_CFG = crate::Reg<r32_emmc_block_cfg::R32_EMMC_BLOCK_CFG_SPEC>;
#[doc = "SD 32bits data counter, \\[15:0\\]
number of blocks this time will tran/recv, \\[27:16\\]
block sise(byte number) of every block in this time tran/recv"]
pub mod r32_emmc_block_cfg;
#[doc = "R32_EMMC_TRAN_MODE (rw) register accessor: an alias for `Reg<R32_EMMC_TRAN_MODE_SPEC>`"]
pub type R32_EMMC_TRAN_MODE = crate::Reg<r32_emmc_tran_mode::R32_EMMC_TRAN_MODE_SPEC>;
#[doc = "SD TRANSFER MODE register"]
pub mod r32_emmc_tran_mode;
#[doc = "R32_EMMC_DMA_BEG2 (rw) register accessor: an alias for `Reg<R32_EMMC_DMA_BEG2_SPEC>`"]
pub type R32_EMMC_DMA_BEG2 = crate::Reg<r32_emmc_dma_beg2::R32_EMMC_DMA_BEG2_SPEC>;
#[doc = "SD 16bits DMA start address register when to operate"]
pub mod r32_emmc_dma_beg2;
