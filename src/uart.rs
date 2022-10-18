use ch569_pac::UART1;

use core::fmt::{Write};

pub struct Uart {
    uart: UART1
}

impl Uart {
    pub fn new(uart: UART1, baud: u32, sysclk: u32) -> Self {

        // Divisor latch
        let dl: u32 = ((10 * sysclk / 8 / baud) + 5) / 10;

        uart.r8_uart1_div.write(|w| {
            unsafe {w.bits(1)}
        });

        uart.r16_uart1_dl.write(|w| {
            unsafe {w.bits(dl.try_into().unwrap())}
        });

        uart.r8_uart1_fcr.write(|w| {
            unsafe {w.rb_fcr_fifo_trig().bits(0x2)};
            w.rb_fcr_tx_fifo_clr().set_bit();
            w.rb_fcr_rx_fifo_clr().set_bit();
            w.rb_fcr_fifo_en().set_bit()
        });

        uart.r8_uart1_ier.write(|w| {
            unsafe { w.bits(0) }
            //w.rb_ier_txd_en().set_bit()
        });

        Uart {
            uart: uart
        }
    }

    pub fn write_byte(&mut self, byte: u8)
    {
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