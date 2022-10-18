use ch569_pac::UART1;

use core::fmt::{Write};

static UART_FIFO_SIZE: u8 = 8;

pub struct Uart {
    uart: UART1
}

impl Uart {
    pub fn new(uart: UART1, baud: u32, sysclk: u32) -> Self {

        // Divisor latch
        //let dl: u32 = ((10 * sysclk * 2 / 16 / baud) + 5) / 10;
        // 131 for 115200 at 120MHz

        uart.r8_uart1_div.write(|w| {
            unsafe {w.bits(1)}
        });

        uart.r16_uart1_dl.write(|w| {
            unsafe {w.bits(131)}
        });

        uart.r8_uart1_fcr.write(|w| {
            unsafe {w.rb_fcr_fifo_trig().bits(0)};
            w.rb_fcr_tx_fifo_clr().set_bit();
            w.rb_fcr_rx_fifo_clr().set_bit();
            w.rb_fcr_fifo_en().set_bit()
        });

        uart.r8_uart1_lcr.write(|w| {

            // Setup no parity, 8 bits per byte, 1 stop bit
            unsafe {
                w.rb_lcr_par_en().clear_bit();
                w.rb_lcr_word_sz().bits(0x3);
                w.rb_lcr_stop_bit().clear_bit()
            }
        });

        uart.r8_uart1_ier.write(|w| {
            w.rb_ier_txd_en().set_bit()
        });

        Uart {
            uart: uart
        }
    }

    pub fn write_byte(&mut self, byte: u8)
    {
        while self.uart.r8_uart1_tfc.read().r8_uart1_tfc().bits() == UART_FIFO_SIZE {
            unsafe { riscv::asm::nop() };
        }
        self.uart.r8_uart1_rbr_r8_uart1_thr.write(|w| {
            unsafe {w.bits(byte)}
        });
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.bytes() {
            self.write_byte(c)
        }

        Ok(())
    }
}
