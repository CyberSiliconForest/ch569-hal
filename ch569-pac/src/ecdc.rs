#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ECED AES/SM4 register"]
    pub r16_ecec_ctrl: R16_ECEC_CTRL,
    #[doc = "0x02 - Interupt enable register"]
    pub r8_ecdc_int_en: R8_ECDC_INT_EN,
    _reserved2: [u8; 0x03],
    #[doc = "0x06 - Interupt flag register"]
    pub r8_ecdc_int_fg: R8_ECDC_INT_FG,
    _reserved3: [u8; 0x01],
    #[doc = "0x08 - User key 224-255 register"]
    pub r32_ecdc_key_255t224: R32_ECDC_KEY_255T224,
    #[doc = "0x0c - User key 192-223 register"]
    pub r32_ecdc_key_223t192: R32_ECDC_KEY_223T192,
    #[doc = "0x10 - User key 160-191 register"]
    pub r32_ecdc_key_191t160: R32_ECDC_KEY_191T160,
    #[doc = "0x14 - User key 128-159 register"]
    pub r32_ecdc_key_159t128: R32_ECDC_KEY_159T128,
    #[doc = "0x18 - User key 96-127 register"]
    pub r32_ecdc_key_127t96: R32_ECDC_KEY_127T96,
    #[doc = "0x1c - User key 64-95 register"]
    pub r32_ecdc_key_95t64: R32_ECDC_KEY_95T64,
    #[doc = "0x20 - User key 32-63 register"]
    pub r32_ecdc_key_63t32: R32_ECDC_KEY_63T32,
    #[doc = "0x24 - User key 0-31 register"]
    pub r32_ecdc_key_31t0: R32_ECDC_KEY_31T0,
    #[doc = "0x28 - CTR mode count 96-127 register"]
    pub r32_ecdc_iv_127t96: R32_ECDC_IV_127T96,
    #[doc = "0x2c - CTR mode count 64-95 register"]
    pub r32_ecdc_iv_95t64: R32_ECDC_IV_95T64,
    #[doc = "0x30 - CTR mode count 32-63 register"]
    pub r32_ecdc_iv_63t32: R32_ECDC_IV_63T32,
    #[doc = "0x34 - CTR mode count 0-31 register"]
    pub r32_ecdc_iv_31t0: R32_ECDC_IV_31T0,
    _reserved15: [u8; 0x08],
    #[doc = "0x40 - Single encryption and decryption of original data 96-127 register"]
    pub r32_ecdc_sgsd_127t96: R32_ECDC_SGSD_127T96,
    #[doc = "0x44 - Single encryption and decryption of original data 64-95 register"]
    pub r32_ecdc_sgsd_95t64: R32_ECDC_SGSD_95T64,
    #[doc = "0x48 - Single encryption and decryption of original data 32-63 register"]
    pub r32_ecdc_sgsd_63t32: R32_ECDC_SGSD_63T32,
    #[doc = "0x4c - Single encryption and decryption of original data 0-31 register"]
    pub r32_ecdc_sgsd_31t0: R32_ECDC_SGSD_31T0,
    #[doc = "0x50 - Single encryption and decryption result 96-127 register"]
    pub r32_ecdc_sgrt_127t96: R32_ECDC_SGRT_127T96,
    #[doc = "0x54 - Single encryption and decryption result 64-95 register"]
    pub r32_ecdc_sgrt_95t64: R32_ECDC_SGRT_95T64,
    #[doc = "0x58 - Single encryption and decryption result 0-31 register"]
    pub r32_ecdc_sgrt_63t32: R32_ECDC_SGRT_63T32,
    #[doc = "0x5c - Single encryption and decryption result 0-31 register"]
    pub rb_ecdc_sgrt_31t0: RB_ECDC_SGRT_31T0,
    #[doc = "0x60 - encryption and decryption sram start address register"]
    pub r32_ecdc_sram_addr: R32_ECDC_SRAM_ADDR,
    #[doc = "0x64 - encryption and decryption sram size register"]
    pub r32_ecdc_sram_len: R32_ECDC_SRAM_LEN,
}
#[doc = "R16_ECEC_CTRL (rw) register accessor: an alias for `Reg<R16_ECEC_CTRL_SPEC>`"]
pub type R16_ECEC_CTRL = crate::Reg<r16_ecec_ctrl::R16_ECEC_CTRL_SPEC>;
#[doc = "ECED AES/SM4 register"]
pub mod r16_ecec_ctrl;
#[doc = "R8_ECDC_INT_EN (rw) register accessor: an alias for `Reg<R8_ECDC_INT_EN_SPEC>`"]
pub type R8_ECDC_INT_EN = crate::Reg<r8_ecdc_int_en::R8_ECDC_INT_EN_SPEC>;
#[doc = "Interupt enable register"]
pub mod r8_ecdc_int_en;
#[doc = "R8_ECDC_INT_FG (rw) register accessor: an alias for `Reg<R8_ECDC_INT_FG_SPEC>`"]
pub type R8_ECDC_INT_FG = crate::Reg<r8_ecdc_int_fg::R8_ECDC_INT_FG_SPEC>;
#[doc = "Interupt flag register"]
pub mod r8_ecdc_int_fg;
#[doc = "R32_ECDC_KEY_255T224 (rw) register accessor: an alias for `Reg<R32_ECDC_KEY_255T224_SPEC>`"]
pub type R32_ECDC_KEY_255T224 = crate::Reg<r32_ecdc_key_255t224::R32_ECDC_KEY_255T224_SPEC>;
#[doc = "User key 224-255 register"]
pub mod r32_ecdc_key_255t224;
#[doc = "R32_ECDC_KEY_223T192 (rw) register accessor: an alias for `Reg<R32_ECDC_KEY_223T192_SPEC>`"]
pub type R32_ECDC_KEY_223T192 = crate::Reg<r32_ecdc_key_223t192::R32_ECDC_KEY_223T192_SPEC>;
#[doc = "User key 192-223 register"]
pub mod r32_ecdc_key_223t192;
#[doc = "R32_ECDC_KEY_191T160 (rw) register accessor: an alias for `Reg<R32_ECDC_KEY_191T160_SPEC>`"]
pub type R32_ECDC_KEY_191T160 = crate::Reg<r32_ecdc_key_191t160::R32_ECDC_KEY_191T160_SPEC>;
#[doc = "User key 160-191 register"]
pub mod r32_ecdc_key_191t160;
#[doc = "R32_ECDC_KEY_159T128 (rw) register accessor: an alias for `Reg<R32_ECDC_KEY_159T128_SPEC>`"]
pub type R32_ECDC_KEY_159T128 = crate::Reg<r32_ecdc_key_159t128::R32_ECDC_KEY_159T128_SPEC>;
#[doc = "User key 128-159 register"]
pub mod r32_ecdc_key_159t128;
#[doc = "R32_ECDC_KEY_127T96 (rw) register accessor: an alias for `Reg<R32_ECDC_KEY_127T96_SPEC>`"]
pub type R32_ECDC_KEY_127T96 = crate::Reg<r32_ecdc_key_127t96::R32_ECDC_KEY_127T96_SPEC>;
#[doc = "User key 96-127 register"]
pub mod r32_ecdc_key_127t96;
#[doc = "R32_ECDC_KEY_95T64 (rw) register accessor: an alias for `Reg<R32_ECDC_KEY_95T64_SPEC>`"]
pub type R32_ECDC_KEY_95T64 = crate::Reg<r32_ecdc_key_95t64::R32_ECDC_KEY_95T64_SPEC>;
#[doc = "User key 64-95 register"]
pub mod r32_ecdc_key_95t64;
#[doc = "R32_ECDC_KEY_63T32 (rw) register accessor: an alias for `Reg<R32_ECDC_KEY_63T32_SPEC>`"]
pub type R32_ECDC_KEY_63T32 = crate::Reg<r32_ecdc_key_63t32::R32_ECDC_KEY_63T32_SPEC>;
#[doc = "User key 32-63 register"]
pub mod r32_ecdc_key_63t32;
#[doc = "R32_ECDC_KEY_31T0 (rw) register accessor: an alias for `Reg<R32_ECDC_KEY_31T0_SPEC>`"]
pub type R32_ECDC_KEY_31T0 = crate::Reg<r32_ecdc_key_31t0::R32_ECDC_KEY_31T0_SPEC>;
#[doc = "User key 0-31 register"]
pub mod r32_ecdc_key_31t0;
#[doc = "R32_ECDC_IV_127T96 (rw) register accessor: an alias for `Reg<R32_ECDC_IV_127T96_SPEC>`"]
pub type R32_ECDC_IV_127T96 = crate::Reg<r32_ecdc_iv_127t96::R32_ECDC_IV_127T96_SPEC>;
#[doc = "CTR mode count 96-127 register"]
pub mod r32_ecdc_iv_127t96;
#[doc = "R32_ECDC_IV_95T64 (rw) register accessor: an alias for `Reg<R32_ECDC_IV_95T64_SPEC>`"]
pub type R32_ECDC_IV_95T64 = crate::Reg<r32_ecdc_iv_95t64::R32_ECDC_IV_95T64_SPEC>;
#[doc = "CTR mode count 64-95 register"]
pub mod r32_ecdc_iv_95t64;
#[doc = "R32_ECDC_IV_63T32 (rw) register accessor: an alias for `Reg<R32_ECDC_IV_63T32_SPEC>`"]
pub type R32_ECDC_IV_63T32 = crate::Reg<r32_ecdc_iv_63t32::R32_ECDC_IV_63T32_SPEC>;
#[doc = "CTR mode count 32-63 register"]
pub mod r32_ecdc_iv_63t32;
#[doc = "R32_ECDC_IV_31T0 (rw) register accessor: an alias for `Reg<R32_ECDC_IV_31T0_SPEC>`"]
pub type R32_ECDC_IV_31T0 = crate::Reg<r32_ecdc_iv_31t0::R32_ECDC_IV_31T0_SPEC>;
#[doc = "CTR mode count 0-31 register"]
pub mod r32_ecdc_iv_31t0;
#[doc = "R32_ECDC_SGSD_127T96 (rw) register accessor: an alias for `Reg<R32_ECDC_SGSD_127T96_SPEC>`"]
pub type R32_ECDC_SGSD_127T96 = crate::Reg<r32_ecdc_sgsd_127t96::R32_ECDC_SGSD_127T96_SPEC>;
#[doc = "Single encryption and decryption of original data 96-127 register"]
pub mod r32_ecdc_sgsd_127t96;
#[doc = "R32_ECDC_SGSD_95T64 (rw) register accessor: an alias for `Reg<R32_ECDC_SGSD_95T64_SPEC>`"]
pub type R32_ECDC_SGSD_95T64 = crate::Reg<r32_ecdc_sgsd_95t64::R32_ECDC_SGSD_95T64_SPEC>;
#[doc = "Single encryption and decryption of original data 64-95 register"]
pub mod r32_ecdc_sgsd_95t64;
#[doc = "R32_ECDC_SGSD_63T32 (rw) register accessor: an alias for `Reg<R32_ECDC_SGSD_63T32_SPEC>`"]
pub type R32_ECDC_SGSD_63T32 = crate::Reg<r32_ecdc_sgsd_63t32::R32_ECDC_SGSD_63T32_SPEC>;
#[doc = "Single encryption and decryption of original data 32-63 register"]
pub mod r32_ecdc_sgsd_63t32;
#[doc = "R32_ECDC_SGSD_31T0 (rw) register accessor: an alias for `Reg<R32_ECDC_SGSD_31T0_SPEC>`"]
pub type R32_ECDC_SGSD_31T0 = crate::Reg<r32_ecdc_sgsd_31t0::R32_ECDC_SGSD_31T0_SPEC>;
#[doc = "Single encryption and decryption of original data 0-31 register"]
pub mod r32_ecdc_sgsd_31t0;
#[doc = "R32_ECDC_SGRT_127T96 (rw) register accessor: an alias for `Reg<R32_ECDC_SGRT_127T96_SPEC>`"]
pub type R32_ECDC_SGRT_127T96 = crate::Reg<r32_ecdc_sgrt_127t96::R32_ECDC_SGRT_127T96_SPEC>;
#[doc = "Single encryption and decryption result 96-127 register"]
pub mod r32_ecdc_sgrt_127t96;
#[doc = "R32_ECDC_SGRT_95T64 (rw) register accessor: an alias for `Reg<R32_ECDC_SGRT_95T64_SPEC>`"]
pub type R32_ECDC_SGRT_95T64 = crate::Reg<r32_ecdc_sgrt_95t64::R32_ECDC_SGRT_95T64_SPEC>;
#[doc = "Single encryption and decryption result 64-95 register"]
pub mod r32_ecdc_sgrt_95t64;
#[doc = "R32_ECDC_SGRT_63T32 (rw) register accessor: an alias for `Reg<R32_ECDC_SGRT_63T32_SPEC>`"]
pub type R32_ECDC_SGRT_63T32 = crate::Reg<r32_ecdc_sgrt_63t32::R32_ECDC_SGRT_63T32_SPEC>;
#[doc = "Single encryption and decryption result 0-31 register"]
pub mod r32_ecdc_sgrt_63t32;
#[doc = "RB_ECDC_SGRT_31T0 (rw) register accessor: an alias for `Reg<RB_ECDC_SGRT_31T0_SPEC>`"]
pub type RB_ECDC_SGRT_31T0 = crate::Reg<rb_ecdc_sgrt_31t0::RB_ECDC_SGRT_31T0_SPEC>;
#[doc = "Single encryption and decryption result 0-31 register"]
pub mod rb_ecdc_sgrt_31t0;
#[doc = "R32_ECDC_SRAM_ADDR (rw) register accessor: an alias for `Reg<R32_ECDC_SRAM_ADDR_SPEC>`"]
pub type R32_ECDC_SRAM_ADDR = crate::Reg<r32_ecdc_sram_addr::R32_ECDC_SRAM_ADDR_SPEC>;
#[doc = "encryption and decryption sram start address register"]
pub mod r32_ecdc_sram_addr;
#[doc = "R32_ECDC_SRAM_LEN (rw) register accessor: an alias for `Reg<R32_ECDC_SRAM_LEN_SPEC>`"]
pub type R32_ECDC_SRAM_LEN = crate::Reg<r32_ecdc_sram_len::R32_ECDC_SRAM_LEN_SPEC>;
#[doc = "encryption and decryption sram size register"]
pub mod r32_ecdc_sram_len;
