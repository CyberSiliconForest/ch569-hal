#![no_std]

use core::arch::global_asm;

global_asm!(
r#"
    .section .init, "ax"
    .global _start_hal
    _start_hal:
"#
);