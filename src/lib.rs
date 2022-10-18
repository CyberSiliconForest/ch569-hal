#![no_std]
#![feature(naked_functions)]

pub mod sys;
pub mod uart;

pub use ch569_pac as pac;
pub use sys::System;

use core::arch::asm;
use critical_section::{RawRestoreState};
critical_section::set_impl!(RiscvCriticalSection);

struct RiscvCriticalSection;

unsafe impl critical_section::Impl for RiscvCriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        let was_active = riscv::register::mstatus::read().mie();
        riscv::interrupt::disable();
        was_active
    }

    unsafe fn release(was_active: RawRestoreState) {
        if was_active {
            riscv::interrupt::enable()
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        unsafe {
            riscv::asm::wfi();
        }
    }
}

#[export_name = "_setup_interrupts"]
pub unsafe extern "Rust" fn ch569_setup_interrupts()
{
    riscv::register::mtvec::write(vector_table as usize, riscv::register::mtvec::TrapMode::Vectored);
}

#[export_name = "_nmi_handler"]
pub unsafe extern fn nmi_handler() -> !
{
    loop {

    }
}

#[export_name = "_hard_fault_handler"]
pub unsafe extern fn fault_handler() -> !
{
    loop {

    }
}


#[link_section = ".init"]
#[no_mangle]
#[naked]
pub unsafe extern fn vector_table() -> !
{
    asm!(
        "j _start",
        "j _start",
        "j _nmi_handler",
        "j _hard_fault_handler",
        options(noreturn)
    )
}
