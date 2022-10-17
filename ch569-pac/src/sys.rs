#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - safe accessing sign register"]
    pub r8_safe_access_sig: R8_SAFE_ACCESS_SIG,
    #[doc = "0x01 - chip ID register"]
    pub r8_chip_id: R8_CHIP_ID,
    #[doc = "0x02 - safe accessing ID register"]
    pub r8_safe_access_id: R8_SAFE_ACCESS_ID,
    #[doc = "0x03 - watch-dog count register"]
    pub r8_wdog_count: R8_WDOG_COUNT,
    #[doc = "0x04 - flash ROM configuration register"]
    pub r8_glob_rom_cfg: R8_GLOB_ROM_CFG,
    #[doc = "0x05 - reset status and boot/debug status"]
    pub r8_rst_boot_stat: R8_RST_BOOT_STAT,
    #[doc = "0x06 - reset and watch-dog control"]
    pub r8_rst_wdog_ctrl: R8_RST_WDOG_CTRL,
    #[doc = "0x07 - value keeper during global reset"]
    pub r8_glob_reset_keep: R8_GLOB_RESET_KEEP,
    #[doc = "0x08 - output clock divider from PLL"]
    pub r8_clk_pll_div: R8_CLK_PLL_DIV,
    _reserved9: [u8; 0x01],
    #[doc = "0x0a - clock control"]
    pub r8_clk_cfg_ctrl: R8_CLK_CFG_CTRL,
    #[doc = "0x0b - clock mode aux register"]
    pub r8_clk_mod_aux: R8_CLK_MOD_AUX,
    #[doc = "0x0c - sleep clock off control byte 0"]
    pub r8_slp_clk_off0: R8_SLP_CLK_OFF0,
    #[doc = "0x0d - sleep clock off control byte 1"]
    pub r8_slp_clk_off1: R8_SLP_CLK_OFF1,
    #[doc = "0x0e - wake control"]
    pub r8_slp_wake_ctrl: R8_SLP_WAKE_CTRL,
    #[doc = "0x0f - power control"]
    pub r8_slp_power_ctrl: R8_SLP_POWER_CTRL,
    _reserved15: [u8; 0x02],
    #[doc = "0x12 - alternate pin control"]
    pub r8_pin_alternate: R8_PIN_ALTERNATE,
    _reserved16: [u8; 0x09],
    #[doc = "0x1c - GPIO interrupt control"]
    pub r8_gpio_int_flag: R8_GPIO_INT_FLAG,
    #[doc = "0x1d - GPIO interrupt enable"]
    pub r8_gpio_int_enable: R8_GPIO_INT_ENABLE,
    #[doc = "0x1e - GPIO interrupt mode"]
    pub r8_gpio_int_mode: R8_GPIO_INT_MODE,
    #[doc = "0x1f - GPIO interrupt polarity"]
    pub r8_gpio_int_polar: R8_GPIO_INT_POLAR,
    #[doc = "0x20 - Serdes Analog parameter configuration1"]
    pub r16_serd_ana_cfg1: R16_SERD_ANA_CFG1,
    _reserved21: [u8; 0x02],
    #[doc = "0x24 - Serdes Analog parameter configuration2"]
    pub r32_serd_ana_cfg2: R32_SERD_ANA_CFG2,
    _reserved22: [u8; 0x18],
    #[doc = "0x40 - GPIO PA I/O direction"]
    pub r32_pa_dir: R32_PA_DIR,
    #[doc = "0x44 - GPIO PA input"]
    pub r32_pa_pin: R32_PA_PIN,
    #[doc = "0x48 - GPIO PA output"]
    pub r32_pa_out: R32_PA_OUT,
    #[doc = "0x4c - GPIO PA clear output"]
    pub r32_pa_clr: R32_PA_CLR,
    #[doc = "0x50 - GPIO PA pullup resistance enable"]
    pub r32_pa_pu: R32_PA_PU,
    #[doc = "0x54 - GPIO PA output open-drain and input pulldown resistance enable"]
    pub r32_pa_pd: R32_PA_PD,
    #[doc = "0x58 - GPIO PA driving capability"]
    pub r32_pa_drv: R32_PA_DRV,
    #[doc = "0x5c - GPIO PA output slew rate and input schmitt trigger"]
    pub r32_pa_smt: R32_PA_SMT,
    #[doc = "0x60 - GPIO PB I/O direction"]
    pub r32_pb_dir: R32_PB_DIR,
    #[doc = "0x64 - GPIO PB input"]
    pub r32_pb_pin: R32_PB_PIN,
    #[doc = "0x68 - GPIO PB output"]
    pub r32_pb_out: R32_PB_OUT,
    #[doc = "0x6c - GPIO PB clear output"]
    pub r32_pb_clr: R32_PB_CLR,
    #[doc = "0x70 - GPIO PB pullup resistance enable"]
    pub r32_pb_pu: R32_PB_PU,
    #[doc = "0x74 - GPIO PB output open-drain and input pulldown resistance enable"]
    pub r32_pb_pd: R32_PB_PD,
    #[doc = "0x78 - GPIO PB driving capability"]
    pub r32_pb_drv: R32_PB_DRV,
    #[doc = "0x7c - GPIO PB output slew rate and input schmitt trigger"]
    pub r32_pb_smt: R32_PB_SMT,
}
#[doc = "R8_SAFE_ACCESS_SIG (rw) register accessor: an alias for `Reg<R8_SAFE_ACCESS_SIG_SPEC>`"]
pub type R8_SAFE_ACCESS_SIG = crate::Reg<r8_safe_access_sig::R8_SAFE_ACCESS_SIG_SPEC>;
#[doc = "safe accessing sign register"]
pub mod r8_safe_access_sig;
#[doc = "R8_CHIP_ID (r) register accessor: an alias for `Reg<R8_CHIP_ID_SPEC>`"]
pub type R8_CHIP_ID = crate::Reg<r8_chip_id::R8_CHIP_ID_SPEC>;
#[doc = "chip ID register"]
pub mod r8_chip_id;
#[doc = "R8_SAFE_ACCESS_ID (r) register accessor: an alias for `Reg<R8_SAFE_ACCESS_ID_SPEC>`"]
pub type R8_SAFE_ACCESS_ID = crate::Reg<r8_safe_access_id::R8_SAFE_ACCESS_ID_SPEC>;
#[doc = "safe accessing ID register"]
pub mod r8_safe_access_id;
#[doc = "R8_WDOG_COUNT (rw) register accessor: an alias for `Reg<R8_WDOG_COUNT_SPEC>`"]
pub type R8_WDOG_COUNT = crate::Reg<r8_wdog_count::R8_WDOG_COUNT_SPEC>;
#[doc = "watch-dog count register"]
pub mod r8_wdog_count;
#[doc = "R8_GLOB_ROM_CFG (rw) register accessor: an alias for `Reg<R8_GLOB_ROM_CFG_SPEC>`"]
pub type R8_GLOB_ROM_CFG = crate::Reg<r8_glob_rom_cfg::R8_GLOB_ROM_CFG_SPEC>;
#[doc = "flash ROM configuration register"]
pub mod r8_glob_rom_cfg;
#[doc = "R8_RST_BOOT_STAT (r) register accessor: an alias for `Reg<R8_RST_BOOT_STAT_SPEC>`"]
pub type R8_RST_BOOT_STAT = crate::Reg<r8_rst_boot_stat::R8_RST_BOOT_STAT_SPEC>;
#[doc = "reset status and boot/debug status"]
pub mod r8_rst_boot_stat;
#[doc = "R8_RST_WDOG_CTRL (rw) register accessor: an alias for `Reg<R8_RST_WDOG_CTRL_SPEC>`"]
pub type R8_RST_WDOG_CTRL = crate::Reg<r8_rst_wdog_ctrl::R8_RST_WDOG_CTRL_SPEC>;
#[doc = "reset and watch-dog control"]
pub mod r8_rst_wdog_ctrl;
#[doc = "R8_GLOB_RESET_KEEP (rw) register accessor: an alias for `Reg<R8_GLOB_RESET_KEEP_SPEC>`"]
pub type R8_GLOB_RESET_KEEP = crate::Reg<r8_glob_reset_keep::R8_GLOB_RESET_KEEP_SPEC>;
#[doc = "value keeper during global reset"]
pub mod r8_glob_reset_keep;
#[doc = "R8_CLK_PLL_DIV (rw) register accessor: an alias for `Reg<R8_CLK_PLL_DIV_SPEC>`"]
pub type R8_CLK_PLL_DIV = crate::Reg<r8_clk_pll_div::R8_CLK_PLL_DIV_SPEC>;
#[doc = "output clock divider from PLL"]
pub mod r8_clk_pll_div;
#[doc = "R8_CLK_CFG_CTRL (rw) register accessor: an alias for `Reg<R8_CLK_CFG_CTRL_SPEC>`"]
pub type R8_CLK_CFG_CTRL = crate::Reg<r8_clk_cfg_ctrl::R8_CLK_CFG_CTRL_SPEC>;
#[doc = "clock control"]
pub mod r8_clk_cfg_ctrl;
#[doc = "R8_CLK_MOD_AUX (rw) register accessor: an alias for `Reg<R8_CLK_MOD_AUX_SPEC>`"]
pub type R8_CLK_MOD_AUX = crate::Reg<r8_clk_mod_aux::R8_CLK_MOD_AUX_SPEC>;
#[doc = "clock mode aux register"]
pub mod r8_clk_mod_aux;
#[doc = "R8_SLP_CLK_OFF0 (rw) register accessor: an alias for `Reg<R8_SLP_CLK_OFF0_SPEC>`"]
pub type R8_SLP_CLK_OFF0 = crate::Reg<r8_slp_clk_off0::R8_SLP_CLK_OFF0_SPEC>;
#[doc = "sleep clock off control byte 0"]
pub mod r8_slp_clk_off0;
#[doc = "R8_SLP_CLK_OFF1 (rw) register accessor: an alias for `Reg<R8_SLP_CLK_OFF1_SPEC>`"]
pub type R8_SLP_CLK_OFF1 = crate::Reg<r8_slp_clk_off1::R8_SLP_CLK_OFF1_SPEC>;
#[doc = "sleep clock off control byte 1"]
pub mod r8_slp_clk_off1;
#[doc = "R8_SLP_WAKE_CTRL (rw) register accessor: an alias for `Reg<R8_SLP_WAKE_CTRL_SPEC>`"]
pub type R8_SLP_WAKE_CTRL = crate::Reg<r8_slp_wake_ctrl::R8_SLP_WAKE_CTRL_SPEC>;
#[doc = "wake control"]
pub mod r8_slp_wake_ctrl;
#[doc = "R8_SLP_POWER_CTRL (rw) register accessor: an alias for `Reg<R8_SLP_POWER_CTRL_SPEC>`"]
pub type R8_SLP_POWER_CTRL = crate::Reg<r8_slp_power_ctrl::R8_SLP_POWER_CTRL_SPEC>;
#[doc = "power control"]
pub mod r8_slp_power_ctrl;
#[doc = "R16_SERD_ANA_CFG1 (rw) register accessor: an alias for `Reg<R16_SERD_ANA_CFG1_SPEC>`"]
pub type R16_SERD_ANA_CFG1 = crate::Reg<r16_serd_ana_cfg1::R16_SERD_ANA_CFG1_SPEC>;
#[doc = "Serdes Analog parameter configuration1"]
pub mod r16_serd_ana_cfg1;
#[doc = "R32_SERD_ANA_CFG2 (rw) register accessor: an alias for `Reg<R32_SERD_ANA_CFG2_SPEC>`"]
pub type R32_SERD_ANA_CFG2 = crate::Reg<r32_serd_ana_cfg2::R32_SERD_ANA_CFG2_SPEC>;
#[doc = "Serdes Analog parameter configuration2"]
pub mod r32_serd_ana_cfg2;
#[doc = "R8_GPIO_INT_FLAG (rw) register accessor: an alias for `Reg<R8_GPIO_INT_FLAG_SPEC>`"]
pub type R8_GPIO_INT_FLAG = crate::Reg<r8_gpio_int_flag::R8_GPIO_INT_FLAG_SPEC>;
#[doc = "GPIO interrupt control"]
pub mod r8_gpio_int_flag;
#[doc = "R8_GPIO_INT_ENABLE (rw) register accessor: an alias for `Reg<R8_GPIO_INT_ENABLE_SPEC>`"]
pub type R8_GPIO_INT_ENABLE = crate::Reg<r8_gpio_int_enable::R8_GPIO_INT_ENABLE_SPEC>;
#[doc = "GPIO interrupt enable"]
pub mod r8_gpio_int_enable;
#[doc = "R8_GPIO_INT_MODE (rw) register accessor: an alias for `Reg<R8_GPIO_INT_MODE_SPEC>`"]
pub type R8_GPIO_INT_MODE = crate::Reg<r8_gpio_int_mode::R8_GPIO_INT_MODE_SPEC>;
#[doc = "GPIO interrupt mode"]
pub mod r8_gpio_int_mode;
#[doc = "R8_GPIO_INT_POLAR (rw) register accessor: an alias for `Reg<R8_GPIO_INT_POLAR_SPEC>`"]
pub type R8_GPIO_INT_POLAR = crate::Reg<r8_gpio_int_polar::R8_GPIO_INT_POLAR_SPEC>;
#[doc = "GPIO interrupt polarity"]
pub mod r8_gpio_int_polar;
#[doc = "R32_PA_DIR (rw) register accessor: an alias for `Reg<R32_PA_DIR_SPEC>`"]
pub type R32_PA_DIR = crate::Reg<r32_pa_dir::R32_PA_DIR_SPEC>;
#[doc = "GPIO PA I/O direction"]
pub mod r32_pa_dir;
#[doc = "R32_PA_PIN (r) register accessor: an alias for `Reg<R32_PA_PIN_SPEC>`"]
pub type R32_PA_PIN = crate::Reg<r32_pa_pin::R32_PA_PIN_SPEC>;
#[doc = "GPIO PA input"]
pub mod r32_pa_pin;
#[doc = "R32_PA_OUT (rw) register accessor: an alias for `Reg<R32_PA_OUT_SPEC>`"]
pub type R32_PA_OUT = crate::Reg<r32_pa_out::R32_PA_OUT_SPEC>;
#[doc = "GPIO PA output"]
pub mod r32_pa_out;
#[doc = "R32_PA_CLR (w) register accessor: an alias for `Reg<R32_PA_CLR_SPEC>`"]
pub type R32_PA_CLR = crate::Reg<r32_pa_clr::R32_PA_CLR_SPEC>;
#[doc = "GPIO PA clear output"]
pub mod r32_pa_clr;
#[doc = "R32_PA_PU (rw) register accessor: an alias for `Reg<R32_PA_PU_SPEC>`"]
pub type R32_PA_PU = crate::Reg<r32_pa_pu::R32_PA_PU_SPEC>;
#[doc = "GPIO PA pullup resistance enable"]
pub mod r32_pa_pu;
#[doc = "R32_PA_PD (rw) register accessor: an alias for `Reg<R32_PA_PD_SPEC>`"]
pub type R32_PA_PD = crate::Reg<r32_pa_pd::R32_PA_PD_SPEC>;
#[doc = "GPIO PA output open-drain and input pulldown resistance enable"]
pub mod r32_pa_pd;
#[doc = "R32_PA_DRV (rw) register accessor: an alias for `Reg<R32_PA_DRV_SPEC>`"]
pub type R32_PA_DRV = crate::Reg<r32_pa_drv::R32_PA_DRV_SPEC>;
#[doc = "GPIO PA driving capability"]
pub mod r32_pa_drv;
#[doc = "R32_PA_SMT (rw) register accessor: an alias for `Reg<R32_PA_SMT_SPEC>`"]
pub type R32_PA_SMT = crate::Reg<r32_pa_smt::R32_PA_SMT_SPEC>;
#[doc = "GPIO PA output slew rate and input schmitt trigger"]
pub mod r32_pa_smt;
#[doc = "R32_PB_DIR (rw) register accessor: an alias for `Reg<R32_PB_DIR_SPEC>`"]
pub type R32_PB_DIR = crate::Reg<r32_pb_dir::R32_PB_DIR_SPEC>;
#[doc = "GPIO PB I/O direction"]
pub mod r32_pb_dir;
#[doc = "R32_PB_PIN (r) register accessor: an alias for `Reg<R32_PB_PIN_SPEC>`"]
pub type R32_PB_PIN = crate::Reg<r32_pb_pin::R32_PB_PIN_SPEC>;
#[doc = "GPIO PB input"]
pub mod r32_pb_pin;
#[doc = "R32_PB_OUT (rw) register accessor: an alias for `Reg<R32_PB_OUT_SPEC>`"]
pub type R32_PB_OUT = crate::Reg<r32_pb_out::R32_PB_OUT_SPEC>;
#[doc = "GPIO PB output"]
pub mod r32_pb_out;
#[doc = "R32_PB_CLR (w) register accessor: an alias for `Reg<R32_PB_CLR_SPEC>`"]
pub type R32_PB_CLR = crate::Reg<r32_pb_clr::R32_PB_CLR_SPEC>;
#[doc = "GPIO PB clear output"]
pub mod r32_pb_clr;
#[doc = "R32_PB_PU (rw) register accessor: an alias for `Reg<R32_PB_PU_SPEC>`"]
pub type R32_PB_PU = crate::Reg<r32_pb_pu::R32_PB_PU_SPEC>;
#[doc = "GPIO PB pullup resistance enable"]
pub mod r32_pb_pu;
#[doc = "R32_PB_PD (rw) register accessor: an alias for `Reg<R32_PB_PD_SPEC>`"]
pub type R32_PB_PD = crate::Reg<r32_pb_pd::R32_PB_PD_SPEC>;
#[doc = "GPIO PB output open-drain and input pulldown resistance enable"]
pub mod r32_pb_pd;
#[doc = "R32_PB_DRV (rw) register accessor: an alias for `Reg<R32_PB_DRV_SPEC>`"]
pub type R32_PB_DRV = crate::Reg<r32_pb_drv::R32_PB_DRV_SPEC>;
#[doc = "GPIO PB driving capability"]
pub mod r32_pb_drv;
#[doc = "R32_PB_SMT (rw) register accessor: an alias for `Reg<R32_PB_SMT_SPEC>`"]
pub type R32_PB_SMT = crate::Reg<r32_pb_smt::R32_PB_SMT_SPEC>;
#[doc = "GPIO PB output slew rate and input schmitt trigger"]
pub mod r32_pb_smt;
#[doc = "R8_PIN_ALTERNATE (rw) register accessor: an alias for `Reg<R8_PIN_ALTERNATE_SPEC>`"]
pub type R8_PIN_ALTERNATE = crate::Reg<r8_pin_alternate::R8_PIN_ALTERNATE_SPEC>;
#[doc = "alternate pin control"]
pub mod r8_pin_alternate;
