#![no_std]
#![no_main]

use ch569_hal as _;
use riscv_rt::entry;

use ch569_pac::Peripherals;
use core::fmt::{Write};

#[inline(never)]
fn delay() {
    for _ in 0..1_000_000 {
        unsafe { riscv::asm::nop() };
    }
}

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();


    // Enter "safe access mode"
    peripherals.SYS.r8_safe_access_sig.write(|w| {
        unsafe {
            w.bits(0x57)
        }
    });
 
    peripherals.SYS.r8_safe_access_sig.write(|w| {
        unsafe {
            w.bits(0xA8)
        }
    });

    // Set PLL to 120MHz
    peripherals.SYS.r8_clk_pll_div.write(|w| {
        unsafe {
            w.bits(0x40 | 0x04)
        }
    });

    peripherals.SYS.r8_clk_cfg_ctrl.write(|w| {
        unsafe {
            w.bits(0x80)
            .rb_clk_pll_sleep().clear_bit()
            .rb_clk_sel_pll().set_bit()
        }
    });

    // Exit safe access mode
    peripherals.SYS.r8_safe_access_sig.write(|w| {
        unsafe {
            w.bits(0x0)
        }
    });


    //R32_PA_SMT |= (1<<8) |(1<<7); // PA8 TXD1 & PA7 RXD1
    peripherals.SYS.r32_pa_smt.write(|w| {
        unsafe { w.bits(1<<8 | (1<<7)) }
    });
  
    // Make LED and UART1 TX outputs
    peripherals.SYS.r32_pa_dir.write(|w| {
        unsafe {
            w.bits(1<<15 | 1<<8)
        }
    });

    // Enable open drain
    /*   
    peripherals.SYS.r32_pa_pd.write(|w| {
        unsafe {
            w.bits(1<<15)
        }
    });
    */

    let mut uart = ch569_hal::uart::Uart::new(peripherals.UART1, 115200, 120_000_000);

    let mut count: u64 = 0;

    loop {
        write!(uart, "Hello Rust {}\r\n", count).unwrap();

        count += 1;

        delay();

        peripherals.SYS.r32_pa_out.write(|w| {
            unsafe {
                w.bits(1<<15)
            }
        });

        delay();
        
        peripherals.SYS.r32_pa_clr.write(|w| {
            unsafe {
                w.bits(1<<15)
            }
        });
    }
}
