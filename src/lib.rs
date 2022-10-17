#![no_std]

use core::arch::global_asm;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        unsafe {
            riscv::asm::wfi();
        }
    }
}

global_asm!(
r#"
    .section .init, "ax"
    .global _start_hal
    _start_hal:
"#
);