#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Status Register"]
    pub r32_pfic_isr1: R32_PFIC_ISR1,
    #[doc = "0x04 - Interrupt Status Register"]
    pub r32_pfic_isr2: R32_PFIC_ISR2,
    _reserved2: [u8; 0x18],
    #[doc = "0x20 - Interrupt Pending Register"]
    pub r32_pfic_ipr1: R32_PFIC_IPR1,
    #[doc = "0x24 - Interrupt Pending Register"]
    pub r32_pfic_ipr2: R32_PFIC_IPR2,
    _reserved4: [u8; 0x18],
    #[doc = "0x40 - Interrupt Priority Register"]
    pub r32_pfic_ithresdr: R32_PFIC_ITHRESDR,
    #[doc = "0x44 - Interrupt Fast Address Register"]
    pub r32_pfic_fibaddrr: R32_PFIC_FIBADDRR,
    #[doc = "0x48 - Interrupt Config Register"]
    pub r32_pfic_cfgr: R32_PFIC_CFGR,
    #[doc = "0x4c - Interrupt Global Register"]
    pub r32_pfic_gisr: R32_PFIC_GISR,
    _reserved8: [u8; 0x10],
    #[doc = "0x60 - Interrupt 0 address Register"]
    pub r32_pfic_fifoaddrr0: R32_PFIC_FIFOADDRR0,
    #[doc = "0x64 - Interrupt 1 address Register"]
    pub r32_pfic_fifoaddrr1: R32_PFIC_FIFOADDRR1,
    #[doc = "0x68 - Interrupt 2 address Register"]
    pub r32_pfic_fifoaddrr2: R32_PFIC_FIFOADDRR2,
    #[doc = "0x6c - Interrupt 3 address Register"]
    pub r32_pfic_fifoaddrr3: R32_PFIC_FIFOADDRR3,
    _reserved12: [u8; 0x90],
    #[doc = "0x100 - Interrupt Setting Register"]
    pub r32_pfic_ienr1: R32_PFIC_IENR1,
    #[doc = "0x104 - Interrupt Setting Register"]
    pub r32_pfic_ienr2: R32_PFIC_IENR2,
    _reserved14: [u8; 0x78],
    #[doc = "0x180 - Interrupt Clear Register"]
    pub r32_pfic_irer1: R32_PFIC_IRER1,
    #[doc = "0x184 - Interrupt Clear Register"]
    pub r32_pfic_irer2: R32_PFIC_IRER2,
    _reserved16: [u8; 0x78],
    #[doc = "0x200 - Interrupt Pending Register"]
    pub r32_pfic_ipsr1: R32_PFIC_IPSR1,
    #[doc = "0x204 - Interrupt Pending Register"]
    pub r32_pfic_ipsr2: R32_PFIC_IPSR2,
    _reserved18: [u8; 0x78],
    #[doc = "0x280 - Interrupt Pending Clear Register"]
    pub r32_pfic_iprr1: R32_PFIC_IPRR1,
    #[doc = "0x284 - Interrupt Pending Clear Register"]
    pub r32_pfic_iprr2: R32_PFIC_IPRR2,
    _reserved20: [u8; 0x78],
    #[doc = "0x300 - Interrupt ACTIVE Register"]
    pub r32_pfic_iactr1: R32_PFIC_IACTR1,
    #[doc = "0x304 - Interrupt ACTIVE Register"]
    pub r32_pfic_iactr2: R32_PFIC_IACTR2,
    _reserved22: [u8; 0xf8],
    #[doc = "0x400 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior0: R32_PFIC_IPRIOR0,
    _reserved23: [u8; 0x1c],
    #[doc = "0x420 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior1: R32_PFIC_IPRIOR1,
    _reserved24: [u8; 0x1c],
    #[doc = "0x440 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior2: R32_PFIC_IPRIOR2,
    _reserved25: [u8; 0x1c],
    #[doc = "0x460 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior3: R32_PFIC_IPRIOR3,
    _reserved26: [u8; 0x1c],
    #[doc = "0x480 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior4: R32_PFIC_IPRIOR4,
    _reserved27: [u8; 0x1c],
    #[doc = "0x4a0 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior5: R32_PFIC_IPRIOR5,
    _reserved28: [u8; 0x1c],
    #[doc = "0x4c0 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior6: R32_PFIC_IPRIOR6,
    _reserved29: [u8; 0x1c],
    #[doc = "0x4e0 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior7: R32_PFIC_IPRIOR7,
    _reserved30: [u8; 0x1c],
    #[doc = "0x500 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior8: R32_PFIC_IPRIOR8,
    _reserved31: [u8; 0x1c],
    #[doc = "0x520 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior9: R32_PFIC_IPRIOR9,
    _reserved32: [u8; 0x1c],
    #[doc = "0x540 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior10: R32_PFIC_IPRIOR10,
    _reserved33: [u8; 0x1c],
    #[doc = "0x560 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior11: R32_PFIC_IPRIOR11,
    _reserved34: [u8; 0x1c],
    #[doc = "0x580 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior12: R32_PFIC_IPRIOR12,
    _reserved35: [u8; 0x1c],
    #[doc = "0x5a0 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior13: R32_PFIC_IPRIOR13,
    _reserved36: [u8; 0x1c],
    #[doc = "0x5c0 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior14: R32_PFIC_IPRIOR14,
    _reserved37: [u8; 0x1c],
    #[doc = "0x5e0 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior15: R32_PFIC_IPRIOR15,
    _reserved38: [u8; 0x1c],
    #[doc = "0x600 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior16: R32_PFIC_IPRIOR16,
    _reserved39: [u8; 0x1c],
    #[doc = "0x620 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior17: R32_PFIC_IPRIOR17,
    _reserved40: [u8; 0x1c],
    #[doc = "0x640 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior18: R32_PFIC_IPRIOR18,
    _reserved41: [u8; 0x1c],
    #[doc = "0x660 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior19: R32_PFIC_IPRIOR19,
    _reserved42: [u8; 0x1c],
    #[doc = "0x680 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior20: R32_PFIC_IPRIOR20,
    _reserved43: [u8; 0x1c],
    #[doc = "0x6a0 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior21: R32_PFIC_IPRIOR21,
    _reserved44: [u8; 0x1c],
    #[doc = "0x6c0 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior22: R32_PFIC_IPRIOR22,
    _reserved45: [u8; 0x1c],
    #[doc = "0x6e0 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior23: R32_PFIC_IPRIOR23,
    _reserved46: [u8; 0x1c],
    #[doc = "0x700 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior24: R32_PFIC_IPRIOR24,
    _reserved47: [u8; 0x1c],
    #[doc = "0x720 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior25: R32_PFIC_IPRIOR25,
    _reserved48: [u8; 0x1c],
    #[doc = "0x740 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior26: R32_PFIC_IPRIOR26,
    _reserved49: [u8; 0x1c],
    #[doc = "0x760 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior27: R32_PFIC_IPRIOR27,
    _reserved50: [u8; 0x1c],
    #[doc = "0x780 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior28: R32_PFIC_IPRIOR28,
    _reserved51: [u8; 0x1c],
    #[doc = "0x7a0 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior29: R32_PFIC_IPRIOR29,
    _reserved52: [u8; 0x1c],
    #[doc = "0x7c0 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior30: R32_PFIC_IPRIOR30,
    _reserved53: [u8; 0x1c],
    #[doc = "0x7e0 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior31: R32_PFIC_IPRIOR31,
    _reserved54: [u8; 0x1c],
    #[doc = "0x800 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior32: R32_PFIC_IPRIOR32,
    _reserved55: [u8; 0x1c],
    #[doc = "0x820 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior33: R32_PFIC_IPRIOR33,
    _reserved56: [u8; 0x1c],
    #[doc = "0x840 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior34: R32_PFIC_IPRIOR34,
    _reserved57: [u8; 0x1c],
    #[doc = "0x860 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior35: R32_PFIC_IPRIOR35,
    _reserved58: [u8; 0x1c],
    #[doc = "0x880 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior36: R32_PFIC_IPRIOR36,
    _reserved59: [u8; 0x1c],
    #[doc = "0x8a0 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior37: R32_PFIC_IPRIOR37,
    _reserved60: [u8; 0x1c],
    #[doc = "0x8c0 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior38: R32_PFIC_IPRIOR38,
    _reserved61: [u8; 0x1c],
    #[doc = "0x8e0 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior39: R32_PFIC_IPRIOR39,
    _reserved62: [u8; 0x1c],
    #[doc = "0x900 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior40: R32_PFIC_IPRIOR40,
    _reserved63: [u8; 0x1c],
    #[doc = "0x920 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior41: R32_PFIC_IPRIOR41,
    _reserved64: [u8; 0x1c],
    #[doc = "0x940 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior42: R32_PFIC_IPRIOR42,
    _reserved65: [u8; 0x1c],
    #[doc = "0x960 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior43: R32_PFIC_IPRIOR43,
    _reserved66: [u8; 0x1c],
    #[doc = "0x980 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior44: R32_PFIC_IPRIOR44,
    _reserved67: [u8; 0x1c],
    #[doc = "0x9a0 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior45: R32_PFIC_IPRIOR45,
    _reserved68: [u8; 0x1c],
    #[doc = "0x9c0 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior46: R32_PFIC_IPRIOR46,
    _reserved69: [u8; 0x1c],
    #[doc = "0x9e0 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior47: R32_PFIC_IPRIOR47,
    _reserved70: [u8; 0x1c],
    #[doc = "0xa00 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior48: R32_PFIC_IPRIOR48,
    _reserved71: [u8; 0x1c],
    #[doc = "0xa20 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior49: R32_PFIC_IPRIOR49,
    _reserved72: [u8; 0x1c],
    #[doc = "0xa40 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior50: R32_PFIC_IPRIOR50,
    _reserved73: [u8; 0x1c],
    #[doc = "0xa60 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior51: R32_PFIC_IPRIOR51,
    _reserved74: [u8; 0x1c],
    #[doc = "0xa80 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior52: R32_PFIC_IPRIOR52,
    _reserved75: [u8; 0x1c],
    #[doc = "0xaa0 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior53: R32_PFIC_IPRIOR53,
    _reserved76: [u8; 0x2c],
    #[doc = "0xad0 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior54: R32_PFIC_IPRIOR54,
    _reserved77: [u8; 0x0c],
    #[doc = "0xae0 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior55: R32_PFIC_IPRIOR55,
    _reserved78: [u8; 0x1c],
    #[doc = "0xb00 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior56: R32_PFIC_IPRIOR56,
    _reserved79: [u8; 0x1c],
    #[doc = "0xb20 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior57: R32_PFIC_IPRIOR57,
    _reserved80: [u8; 0x1c],
    #[doc = "0xb40 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior58: R32_PFIC_IPRIOR58,
    _reserved81: [u8; 0x1c],
    #[doc = "0xb60 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior59: R32_PFIC_IPRIOR59,
    _reserved82: [u8; 0x1c],
    #[doc = "0xb80 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior60: R32_PFIC_IPRIOR60,
    _reserved83: [u8; 0x1c],
    #[doc = "0xba0 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior61: R32_PFIC_IPRIOR61,
    _reserved84: [u8; 0x3c],
    #[doc = "0xbe0 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior62: R32_PFIC_IPRIOR62,
    _reserved85: [u8; 0x1c],
    #[doc = "0xc00 - Interrupt Priority configuration Register"]
    pub r32_pfic_iprior63: R32_PFIC_IPRIOR63,
    _reserved86: [u8; 0x010c],
    #[doc = "0xd10 - System Control Register"]
    pub r32_pfic_sctlr: R32_PFIC_SCTLR,
}
#[doc = "R32_PFIC_ISR1 (r) register accessor: an alias for `Reg<R32_PFIC_ISR1_SPEC>`"]
pub type R32_PFIC_ISR1 = crate::Reg<r32_pfic_isr1::R32_PFIC_ISR1_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod r32_pfic_isr1;
#[doc = "R32_PFIC_ISR2 (r) register accessor: an alias for `Reg<R32_PFIC_ISR2_SPEC>`"]
pub type R32_PFIC_ISR2 = crate::Reg<r32_pfic_isr2::R32_PFIC_ISR2_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod r32_pfic_isr2;
#[doc = "R32_PFIC_IPR1 (r) register accessor: an alias for `Reg<R32_PFIC_IPR1_SPEC>`"]
pub type R32_PFIC_IPR1 = crate::Reg<r32_pfic_ipr1::R32_PFIC_IPR1_SPEC>;
#[doc = "Interrupt Pending Register"]
pub mod r32_pfic_ipr1;
#[doc = "R32_PFIC_IPR2 (r) register accessor: an alias for `Reg<R32_PFIC_IPR2_SPEC>`"]
pub type R32_PFIC_IPR2 = crate::Reg<r32_pfic_ipr2::R32_PFIC_IPR2_SPEC>;
#[doc = "Interrupt Pending Register"]
pub mod r32_pfic_ipr2;
#[doc = "R32_PFIC_ITHRESDR (rw) register accessor: an alias for `Reg<R32_PFIC_ITHRESDR_SPEC>`"]
pub type R32_PFIC_ITHRESDR = crate::Reg<r32_pfic_ithresdr::R32_PFIC_ITHRESDR_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod r32_pfic_ithresdr;
#[doc = "R32_PFIC_FIBADDRR (rw) register accessor: an alias for `Reg<R32_PFIC_FIBADDRR_SPEC>`"]
pub type R32_PFIC_FIBADDRR = crate::Reg<r32_pfic_fibaddrr::R32_PFIC_FIBADDRR_SPEC>;
#[doc = "Interrupt Fast Address Register"]
pub mod r32_pfic_fibaddrr;
#[doc = "R32_PFIC_CFGR (rw) register accessor: an alias for `Reg<R32_PFIC_CFGR_SPEC>`"]
pub type R32_PFIC_CFGR = crate::Reg<r32_pfic_cfgr::R32_PFIC_CFGR_SPEC>;
#[doc = "Interrupt Config Register"]
pub mod r32_pfic_cfgr;
#[doc = "R32_PFIC_GISR (r) register accessor: an alias for `Reg<R32_PFIC_GISR_SPEC>`"]
pub type R32_PFIC_GISR = crate::Reg<r32_pfic_gisr::R32_PFIC_GISR_SPEC>;
#[doc = "Interrupt Global Register"]
pub mod r32_pfic_gisr;
#[doc = "R32_PFIC_FIFOADDRR0 (rw) register accessor: an alias for `Reg<R32_PFIC_FIFOADDRR0_SPEC>`"]
pub type R32_PFIC_FIFOADDRR0 = crate::Reg<r32_pfic_fifoaddrr0::R32_PFIC_FIFOADDRR0_SPEC>;
#[doc = "Interrupt 0 address Register"]
pub mod r32_pfic_fifoaddrr0;
#[doc = "R32_PFIC_FIFOADDRR1 (rw) register accessor: an alias for `Reg<R32_PFIC_FIFOADDRR1_SPEC>`"]
pub type R32_PFIC_FIFOADDRR1 = crate::Reg<r32_pfic_fifoaddrr1::R32_PFIC_FIFOADDRR1_SPEC>;
#[doc = "Interrupt 1 address Register"]
pub mod r32_pfic_fifoaddrr1;
#[doc = "R32_PFIC_FIFOADDRR2 (rw) register accessor: an alias for `Reg<R32_PFIC_FIFOADDRR2_SPEC>`"]
pub type R32_PFIC_FIFOADDRR2 = crate::Reg<r32_pfic_fifoaddrr2::R32_PFIC_FIFOADDRR2_SPEC>;
#[doc = "Interrupt 2 address Register"]
pub mod r32_pfic_fifoaddrr2;
#[doc = "R32_PFIC_FIFOADDRR3 (rw) register accessor: an alias for `Reg<R32_PFIC_FIFOADDRR3_SPEC>`"]
pub type R32_PFIC_FIFOADDRR3 = crate::Reg<r32_pfic_fifoaddrr3::R32_PFIC_FIFOADDRR3_SPEC>;
#[doc = "Interrupt 3 address Register"]
pub mod r32_pfic_fifoaddrr3;
#[doc = "R32_PFIC_IENR1 (rw) register accessor: an alias for `Reg<R32_PFIC_IENR1_SPEC>`"]
pub type R32_PFIC_IENR1 = crate::Reg<r32_pfic_ienr1::R32_PFIC_IENR1_SPEC>;
#[doc = "Interrupt Setting Register"]
pub mod r32_pfic_ienr1;
#[doc = "R32_PFIC_IENR2 (rw) register accessor: an alias for `Reg<R32_PFIC_IENR2_SPEC>`"]
pub type R32_PFIC_IENR2 = crate::Reg<r32_pfic_ienr2::R32_PFIC_IENR2_SPEC>;
#[doc = "Interrupt Setting Register"]
pub mod r32_pfic_ienr2;
#[doc = "R32_PFIC_IRER1 (rw) register accessor: an alias for `Reg<R32_PFIC_IRER1_SPEC>`"]
pub type R32_PFIC_IRER1 = crate::Reg<r32_pfic_irer1::R32_PFIC_IRER1_SPEC>;
#[doc = "Interrupt Clear Register"]
pub mod r32_pfic_irer1;
#[doc = "R32_PFIC_IRER2 (rw) register accessor: an alias for `Reg<R32_PFIC_IRER2_SPEC>`"]
pub type R32_PFIC_IRER2 = crate::Reg<r32_pfic_irer2::R32_PFIC_IRER2_SPEC>;
#[doc = "Interrupt Clear Register"]
pub mod r32_pfic_irer2;
#[doc = "R32_PFIC_IPSR1 (rw) register accessor: an alias for `Reg<R32_PFIC_IPSR1_SPEC>`"]
pub type R32_PFIC_IPSR1 = crate::Reg<r32_pfic_ipsr1::R32_PFIC_IPSR1_SPEC>;
#[doc = "Interrupt Pending Register"]
pub mod r32_pfic_ipsr1;
#[doc = "R32_PFIC_IPSR2 (rw) register accessor: an alias for `Reg<R32_PFIC_IPSR2_SPEC>`"]
pub type R32_PFIC_IPSR2 = crate::Reg<r32_pfic_ipsr2::R32_PFIC_IPSR2_SPEC>;
#[doc = "Interrupt Pending Register"]
pub mod r32_pfic_ipsr2;
#[doc = "R32_PFIC_IPRR1 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRR1_SPEC>`"]
pub type R32_PFIC_IPRR1 = crate::Reg<r32_pfic_iprr1::R32_PFIC_IPRR1_SPEC>;
#[doc = "Interrupt Pending Clear Register"]
pub mod r32_pfic_iprr1;
#[doc = "R32_PFIC_IPRR2 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRR2_SPEC>`"]
pub type R32_PFIC_IPRR2 = crate::Reg<r32_pfic_iprr2::R32_PFIC_IPRR2_SPEC>;
#[doc = "Interrupt Pending Clear Register"]
pub mod r32_pfic_iprr2;
#[doc = "R32_PFIC_IACTR1 (rw) register accessor: an alias for `Reg<R32_PFIC_IACTR1_SPEC>`"]
pub type R32_PFIC_IACTR1 = crate::Reg<r32_pfic_iactr1::R32_PFIC_IACTR1_SPEC>;
#[doc = "Interrupt ACTIVE Register"]
pub mod r32_pfic_iactr1;
#[doc = "R32_PFIC_IACTR2 (rw) register accessor: an alias for `Reg<R32_PFIC_IACTR2_SPEC>`"]
pub type R32_PFIC_IACTR2 = crate::Reg<r32_pfic_iactr2::R32_PFIC_IACTR2_SPEC>;
#[doc = "Interrupt ACTIVE Register"]
pub mod r32_pfic_iactr2;
#[doc = "R32_PFIC_IPRIOR0 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR0_SPEC>`"]
pub type R32_PFIC_IPRIOR0 = crate::Reg<r32_pfic_iprior0::R32_PFIC_IPRIOR0_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior0;
#[doc = "R32_PFIC_IPRIOR1 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR1_SPEC>`"]
pub type R32_PFIC_IPRIOR1 = crate::Reg<r32_pfic_iprior1::R32_PFIC_IPRIOR1_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior1;
#[doc = "R32_PFIC_IPRIOR2 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR2_SPEC>`"]
pub type R32_PFIC_IPRIOR2 = crate::Reg<r32_pfic_iprior2::R32_PFIC_IPRIOR2_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior2;
#[doc = "R32_PFIC_IPRIOR3 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR3_SPEC>`"]
pub type R32_PFIC_IPRIOR3 = crate::Reg<r32_pfic_iprior3::R32_PFIC_IPRIOR3_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior3;
#[doc = "R32_PFIC_IPRIOR4 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR4_SPEC>`"]
pub type R32_PFIC_IPRIOR4 = crate::Reg<r32_pfic_iprior4::R32_PFIC_IPRIOR4_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior4;
#[doc = "R32_PFIC_IPRIOR5 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR5_SPEC>`"]
pub type R32_PFIC_IPRIOR5 = crate::Reg<r32_pfic_iprior5::R32_PFIC_IPRIOR5_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior5;
#[doc = "R32_PFIC_IPRIOR6 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR6_SPEC>`"]
pub type R32_PFIC_IPRIOR6 = crate::Reg<r32_pfic_iprior6::R32_PFIC_IPRIOR6_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior6;
#[doc = "R32_PFIC_IPRIOR7 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR7_SPEC>`"]
pub type R32_PFIC_IPRIOR7 = crate::Reg<r32_pfic_iprior7::R32_PFIC_IPRIOR7_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior7;
#[doc = "R32_PFIC_IPRIOR8 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR8_SPEC>`"]
pub type R32_PFIC_IPRIOR8 = crate::Reg<r32_pfic_iprior8::R32_PFIC_IPRIOR8_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior8;
#[doc = "R32_PFIC_IPRIOR9 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR9_SPEC>`"]
pub type R32_PFIC_IPRIOR9 = crate::Reg<r32_pfic_iprior9::R32_PFIC_IPRIOR9_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior9;
#[doc = "R32_PFIC_IPRIOR10 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR10_SPEC>`"]
pub type R32_PFIC_IPRIOR10 = crate::Reg<r32_pfic_iprior10::R32_PFIC_IPRIOR10_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior10;
#[doc = "R32_PFIC_IPRIOR11 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR11_SPEC>`"]
pub type R32_PFIC_IPRIOR11 = crate::Reg<r32_pfic_iprior11::R32_PFIC_IPRIOR11_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior11;
#[doc = "R32_PFIC_IPRIOR12 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR12_SPEC>`"]
pub type R32_PFIC_IPRIOR12 = crate::Reg<r32_pfic_iprior12::R32_PFIC_IPRIOR12_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior12;
#[doc = "R32_PFIC_IPRIOR13 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR13_SPEC>`"]
pub type R32_PFIC_IPRIOR13 = crate::Reg<r32_pfic_iprior13::R32_PFIC_IPRIOR13_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior13;
#[doc = "R32_PFIC_IPRIOR14 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR14_SPEC>`"]
pub type R32_PFIC_IPRIOR14 = crate::Reg<r32_pfic_iprior14::R32_PFIC_IPRIOR14_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior14;
#[doc = "R32_PFIC_IPRIOR15 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR15_SPEC>`"]
pub type R32_PFIC_IPRIOR15 = crate::Reg<r32_pfic_iprior15::R32_PFIC_IPRIOR15_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior15;
#[doc = "R32_PFIC_IPRIOR16 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR16_SPEC>`"]
pub type R32_PFIC_IPRIOR16 = crate::Reg<r32_pfic_iprior16::R32_PFIC_IPRIOR16_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior16;
#[doc = "R32_PFIC_IPRIOR17 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR17_SPEC>`"]
pub type R32_PFIC_IPRIOR17 = crate::Reg<r32_pfic_iprior17::R32_PFIC_IPRIOR17_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior17;
#[doc = "R32_PFIC_IPRIOR18 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR18_SPEC>`"]
pub type R32_PFIC_IPRIOR18 = crate::Reg<r32_pfic_iprior18::R32_PFIC_IPRIOR18_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior18;
#[doc = "R32_PFIC_IPRIOR19 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR19_SPEC>`"]
pub type R32_PFIC_IPRIOR19 = crate::Reg<r32_pfic_iprior19::R32_PFIC_IPRIOR19_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior19;
#[doc = "R32_PFIC_IPRIOR20 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR20_SPEC>`"]
pub type R32_PFIC_IPRIOR20 = crate::Reg<r32_pfic_iprior20::R32_PFIC_IPRIOR20_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior20;
#[doc = "R32_PFIC_IPRIOR21 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR21_SPEC>`"]
pub type R32_PFIC_IPRIOR21 = crate::Reg<r32_pfic_iprior21::R32_PFIC_IPRIOR21_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior21;
#[doc = "R32_PFIC_IPRIOR22 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR22_SPEC>`"]
pub type R32_PFIC_IPRIOR22 = crate::Reg<r32_pfic_iprior22::R32_PFIC_IPRIOR22_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior22;
#[doc = "R32_PFIC_IPRIOR23 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR23_SPEC>`"]
pub type R32_PFIC_IPRIOR23 = crate::Reg<r32_pfic_iprior23::R32_PFIC_IPRIOR23_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior23;
#[doc = "R32_PFIC_IPRIOR24 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR24_SPEC>`"]
pub type R32_PFIC_IPRIOR24 = crate::Reg<r32_pfic_iprior24::R32_PFIC_IPRIOR24_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior24;
#[doc = "R32_PFIC_IPRIOR25 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR25_SPEC>`"]
pub type R32_PFIC_IPRIOR25 = crate::Reg<r32_pfic_iprior25::R32_PFIC_IPRIOR25_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior25;
#[doc = "R32_PFIC_IPRIOR26 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR26_SPEC>`"]
pub type R32_PFIC_IPRIOR26 = crate::Reg<r32_pfic_iprior26::R32_PFIC_IPRIOR26_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior26;
#[doc = "R32_PFIC_IPRIOR27 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR27_SPEC>`"]
pub type R32_PFIC_IPRIOR27 = crate::Reg<r32_pfic_iprior27::R32_PFIC_IPRIOR27_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior27;
#[doc = "R32_PFIC_IPRIOR28 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR28_SPEC>`"]
pub type R32_PFIC_IPRIOR28 = crate::Reg<r32_pfic_iprior28::R32_PFIC_IPRIOR28_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior28;
#[doc = "R32_PFIC_IPRIOR29 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR29_SPEC>`"]
pub type R32_PFIC_IPRIOR29 = crate::Reg<r32_pfic_iprior29::R32_PFIC_IPRIOR29_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior29;
#[doc = "R32_PFIC_IPRIOR30 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR30_SPEC>`"]
pub type R32_PFIC_IPRIOR30 = crate::Reg<r32_pfic_iprior30::R32_PFIC_IPRIOR30_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior30;
#[doc = "R32_PFIC_IPRIOR31 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR31_SPEC>`"]
pub type R32_PFIC_IPRIOR31 = crate::Reg<r32_pfic_iprior31::R32_PFIC_IPRIOR31_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior31;
#[doc = "R32_PFIC_IPRIOR32 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR32_SPEC>`"]
pub type R32_PFIC_IPRIOR32 = crate::Reg<r32_pfic_iprior32::R32_PFIC_IPRIOR32_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior32;
#[doc = "R32_PFIC_IPRIOR33 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR33_SPEC>`"]
pub type R32_PFIC_IPRIOR33 = crate::Reg<r32_pfic_iprior33::R32_PFIC_IPRIOR33_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior33;
#[doc = "R32_PFIC_IPRIOR34 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR34_SPEC>`"]
pub type R32_PFIC_IPRIOR34 = crate::Reg<r32_pfic_iprior34::R32_PFIC_IPRIOR34_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior34;
#[doc = "R32_PFIC_IPRIOR35 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR35_SPEC>`"]
pub type R32_PFIC_IPRIOR35 = crate::Reg<r32_pfic_iprior35::R32_PFIC_IPRIOR35_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior35;
#[doc = "R32_PFIC_IPRIOR36 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR36_SPEC>`"]
pub type R32_PFIC_IPRIOR36 = crate::Reg<r32_pfic_iprior36::R32_PFIC_IPRIOR36_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior36;
#[doc = "R32_PFIC_IPRIOR37 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR37_SPEC>`"]
pub type R32_PFIC_IPRIOR37 = crate::Reg<r32_pfic_iprior37::R32_PFIC_IPRIOR37_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior37;
#[doc = "R32_PFIC_IPRIOR38 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR38_SPEC>`"]
pub type R32_PFIC_IPRIOR38 = crate::Reg<r32_pfic_iprior38::R32_PFIC_IPRIOR38_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior38;
#[doc = "R32_PFIC_IPRIOR39 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR39_SPEC>`"]
pub type R32_PFIC_IPRIOR39 = crate::Reg<r32_pfic_iprior39::R32_PFIC_IPRIOR39_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior39;
#[doc = "R32_PFIC_IPRIOR40 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR40_SPEC>`"]
pub type R32_PFIC_IPRIOR40 = crate::Reg<r32_pfic_iprior40::R32_PFIC_IPRIOR40_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior40;
#[doc = "R32_PFIC_IPRIOR41 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR41_SPEC>`"]
pub type R32_PFIC_IPRIOR41 = crate::Reg<r32_pfic_iprior41::R32_PFIC_IPRIOR41_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior41;
#[doc = "R32_PFIC_IPRIOR42 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR42_SPEC>`"]
pub type R32_PFIC_IPRIOR42 = crate::Reg<r32_pfic_iprior42::R32_PFIC_IPRIOR42_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior42;
#[doc = "R32_PFIC_IPRIOR43 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR43_SPEC>`"]
pub type R32_PFIC_IPRIOR43 = crate::Reg<r32_pfic_iprior43::R32_PFIC_IPRIOR43_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior43;
#[doc = "R32_PFIC_IPRIOR44 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR44_SPEC>`"]
pub type R32_PFIC_IPRIOR44 = crate::Reg<r32_pfic_iprior44::R32_PFIC_IPRIOR44_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior44;
#[doc = "R32_PFIC_IPRIOR45 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR45_SPEC>`"]
pub type R32_PFIC_IPRIOR45 = crate::Reg<r32_pfic_iprior45::R32_PFIC_IPRIOR45_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior45;
#[doc = "R32_PFIC_IPRIOR46 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR46_SPEC>`"]
pub type R32_PFIC_IPRIOR46 = crate::Reg<r32_pfic_iprior46::R32_PFIC_IPRIOR46_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior46;
#[doc = "R32_PFIC_IPRIOR47 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR47_SPEC>`"]
pub type R32_PFIC_IPRIOR47 = crate::Reg<r32_pfic_iprior47::R32_PFIC_IPRIOR47_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior47;
#[doc = "R32_PFIC_IPRIOR48 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR48_SPEC>`"]
pub type R32_PFIC_IPRIOR48 = crate::Reg<r32_pfic_iprior48::R32_PFIC_IPRIOR48_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior48;
#[doc = "R32_PFIC_IPRIOR49 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR49_SPEC>`"]
pub type R32_PFIC_IPRIOR49 = crate::Reg<r32_pfic_iprior49::R32_PFIC_IPRIOR49_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior49;
#[doc = "R32_PFIC_IPRIOR50 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR50_SPEC>`"]
pub type R32_PFIC_IPRIOR50 = crate::Reg<r32_pfic_iprior50::R32_PFIC_IPRIOR50_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior50;
#[doc = "R32_PFIC_IPRIOR51 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR51_SPEC>`"]
pub type R32_PFIC_IPRIOR51 = crate::Reg<r32_pfic_iprior51::R32_PFIC_IPRIOR51_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior51;
#[doc = "R32_PFIC_IPRIOR52 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR52_SPEC>`"]
pub type R32_PFIC_IPRIOR52 = crate::Reg<r32_pfic_iprior52::R32_PFIC_IPRIOR52_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior52;
#[doc = "R32_PFIC_IPRIOR53 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR53_SPEC>`"]
pub type R32_PFIC_IPRIOR53 = crate::Reg<r32_pfic_iprior53::R32_PFIC_IPRIOR53_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior53;
#[doc = "R32_PFIC_IPRIOR54 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR54_SPEC>`"]
pub type R32_PFIC_IPRIOR54 = crate::Reg<r32_pfic_iprior54::R32_PFIC_IPRIOR54_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior54;
#[doc = "R32_PFIC_IPRIOR55 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR55_SPEC>`"]
pub type R32_PFIC_IPRIOR55 = crate::Reg<r32_pfic_iprior55::R32_PFIC_IPRIOR55_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior55;
#[doc = "R32_PFIC_IPRIOR56 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR56_SPEC>`"]
pub type R32_PFIC_IPRIOR56 = crate::Reg<r32_pfic_iprior56::R32_PFIC_IPRIOR56_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior56;
#[doc = "R32_PFIC_IPRIOR57 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR57_SPEC>`"]
pub type R32_PFIC_IPRIOR57 = crate::Reg<r32_pfic_iprior57::R32_PFIC_IPRIOR57_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior57;
#[doc = "R32_PFIC_IPRIOR58 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR58_SPEC>`"]
pub type R32_PFIC_IPRIOR58 = crate::Reg<r32_pfic_iprior58::R32_PFIC_IPRIOR58_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior58;
#[doc = "R32_PFIC_IPRIOR59 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR59_SPEC>`"]
pub type R32_PFIC_IPRIOR59 = crate::Reg<r32_pfic_iprior59::R32_PFIC_IPRIOR59_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior59;
#[doc = "R32_PFIC_IPRIOR60 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR60_SPEC>`"]
pub type R32_PFIC_IPRIOR60 = crate::Reg<r32_pfic_iprior60::R32_PFIC_IPRIOR60_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior60;
#[doc = "R32_PFIC_IPRIOR61 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR61_SPEC>`"]
pub type R32_PFIC_IPRIOR61 = crate::Reg<r32_pfic_iprior61::R32_PFIC_IPRIOR61_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior61;
#[doc = "R32_PFIC_IPRIOR62 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR62_SPEC>`"]
pub type R32_PFIC_IPRIOR62 = crate::Reg<r32_pfic_iprior62::R32_PFIC_IPRIOR62_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior62;
#[doc = "R32_PFIC_IPRIOR63 (rw) register accessor: an alias for `Reg<R32_PFIC_IPRIOR63_SPEC>`"]
pub type R32_PFIC_IPRIOR63 = crate::Reg<r32_pfic_iprior63::R32_PFIC_IPRIOR63_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod r32_pfic_iprior63;
#[doc = "R32_PFIC_SCTLR (rw) register accessor: an alias for `Reg<R32_PFIC_SCTLR_SPEC>`"]
pub type R32_PFIC_SCTLR = crate::Reg<r32_pfic_sctlr::R32_PFIC_SCTLR_SPEC>;
#[doc = "System Control Register"]
pub mod r32_pfic_sctlr;
