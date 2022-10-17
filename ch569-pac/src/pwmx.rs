#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWM mode control"]
    pub r8_pwm_ctrl_mod: R8_PWM_CTRL_MOD,
    #[doc = "0x01 - PWM configuration control"]
    pub r8_pwm_ctrl_cfg: R8_PWM_CTRL_CFG,
    #[doc = "0x02 - PWM clock divisor"]
    pub r8_pwm_clock_div: R8_PWM_CLOCK_DIV,
    _reserved3: [u8; 0x01],
    #[doc = "0x04 - PWM data holding"]
    pub r8_pwm0_data: R8_PWM0_DATA,
    #[doc = "0x05 - PWM1 data holding"]
    pub r8_pwm1_data: R8_PWM1_DATA,
    #[doc = "0x06 - PWM2 data holding"]
    pub r8_pwm2_data: R8_PWM2_DATA,
    #[doc = "0x07 - PWM3 data holding"]
    pub r8_pwm3_data: R8_PWM3_DATA,
}
#[doc = "R8_PWM_CTRL_MOD (rw) register accessor: an alias for `Reg<R8_PWM_CTRL_MOD_SPEC>`"]
pub type R8_PWM_CTRL_MOD = crate::Reg<r8_pwm_ctrl_mod::R8_PWM_CTRL_MOD_SPEC>;
#[doc = "PWM mode control"]
pub mod r8_pwm_ctrl_mod;
#[doc = "R8_PWM_CTRL_CFG (rw) register accessor: an alias for `Reg<R8_PWM_CTRL_CFG_SPEC>`"]
pub type R8_PWM_CTRL_CFG = crate::Reg<r8_pwm_ctrl_cfg::R8_PWM_CTRL_CFG_SPEC>;
#[doc = "PWM configuration control"]
pub mod r8_pwm_ctrl_cfg;
#[doc = "R8_PWM_CLOCK_DIV (rw) register accessor: an alias for `Reg<R8_PWM_CLOCK_DIV_SPEC>`"]
pub type R8_PWM_CLOCK_DIV = crate::Reg<r8_pwm_clock_div::R8_PWM_CLOCK_DIV_SPEC>;
#[doc = "PWM clock divisor"]
pub mod r8_pwm_clock_div;
#[doc = "R8_PWM0_DATA (rw) register accessor: an alias for `Reg<R8_PWM0_DATA_SPEC>`"]
pub type R8_PWM0_DATA = crate::Reg<r8_pwm0_data::R8_PWM0_DATA_SPEC>;
#[doc = "PWM data holding"]
pub mod r8_pwm0_data;
#[doc = "R8_PWM1_DATA (rw) register accessor: an alias for `Reg<R8_PWM1_DATA_SPEC>`"]
pub type R8_PWM1_DATA = crate::Reg<r8_pwm1_data::R8_PWM1_DATA_SPEC>;
#[doc = "PWM1 data holding"]
pub mod r8_pwm1_data;
#[doc = "R8_PWM2_DATA (rw) register accessor: an alias for `Reg<R8_PWM2_DATA_SPEC>`"]
pub type R8_PWM2_DATA = crate::Reg<r8_pwm2_data::R8_PWM2_DATA_SPEC>;
#[doc = "PWM2 data holding"]
pub mod r8_pwm2_data;
#[doc = "R8_PWM3_DATA (rw) register accessor: an alias for `Reg<R8_PWM3_DATA_SPEC>`"]
pub type R8_PWM3_DATA = crate::Reg<r8_pwm3_data::R8_PWM3_DATA_SPEC>;
#[doc = "PWM3 data holding"]
pub mod r8_pwm3_data;
