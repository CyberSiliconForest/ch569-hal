use ch569_pac::SYS;

pub enum PllClockSource {
    Internal480MHz,
    External30MHz
}

pub enum GpioPort {
    PA,
    PB
}

pub struct System
{
    sys: SYS
}

impl System {
    pub fn new(sys: SYS) -> Self {
       
        return System {
            sys: sys
        }
    }

    pub fn safe_access(&self, enable: bool) {
        if enable == true {
            self.sys.r8_safe_access_sig.write(|w| { unsafe { w.bits(0x57) } });
            self.sys.r8_safe_access_sig.write(|w| { unsafe { w.bits(0xA8) } });
        } else {
            self.sys.r8_safe_access_sig.write(|w| { unsafe { w.bits(0) } });
        }
    }

    pub fn set_pll_div(&self, div: u8) {
        self.sys.r8_clk_pll_div.write(|w| {
            unsafe {
                w.bits(0x40 | div)
            }
        })
    }

    pub fn set_pll_source(&self, source: PllClockSource) {
        self.sys.r8_clk_cfg_ctrl.write(|w| {
            unsafe {w.bits(0x80)};

            match source {
                PllClockSource::Internal480MHz => w.rb_clk_sel_pll().set_bit(),
                PllClockSource::External30MHz => w.rb_clk_sel_pll().clear_bit()
            };

            w
        })
    }
}

impl System {
    pub fn port_set_dir(&self, port: GpioPort, pin: u32, output: bool) {

        let mut dir = match port {
            GpioPort::PA => self.sys.r32_pa_dir.read().bits(),
            GpioPort::PB => self.sys.r32_pb_dir.read().bits(),
        };

        if output == true {
            dir |= 1 << pin;
        } else {
            dir &= !(1 << pin);
        }

        match port {
            GpioPort::PA => self.sys.r32_pa_dir.write(|w| {unsafe{w.bits(dir)}}),
            GpioPort::PB => self.sys.r32_pb_dir.write(|w| {unsafe{w.bits(dir)}}),
        }
    }

    // Set slew rate on output, schmitt trigger on input
    pub fn port_set_smt(&self, port: GpioPort, pin: u32, enable: bool) {
        let mut smt = match port {
            GpioPort::PA => self.sys.r32_pa_smt.read().bits(),
            GpioPort::PB => self.sys.r32_pb_smt.read().bits()
        };

        if enable == true {
            smt |= 1 << pin;
        } else {
            smt &= !(1 << pin);
        }

        match port {
            GpioPort::PA => self.sys.r32_pa_smt.write(|w| {unsafe{w.bits(smt)}}),
            GpioPort::PB => self.sys.r32_pb_smt.write(|w| {unsafe{w.bits(smt)}}),
        }
    }

    pub fn port_out_set(&self, port: GpioPort, pin: u32) {
        let mut out = match port {
            GpioPort::PA => self.sys.r32_pa_out.read().bits(),
            GpioPort::PB => self.sys.r32_pb_out.read().bits()
        };

        out |= 1 << pin;

        match port {
            GpioPort::PA => self.sys.r32_pa_out.write(|w| {unsafe{w.bits(out)}}),
            GpioPort::PB => self.sys.r32_pb_out.write(|w| {unsafe{w.bits(out)}})
        }
    }

    pub fn port_out_clear(&self, port: GpioPort, pin: u32) {
        let out = 1 << pin;
        match port {
            GpioPort::PA => self.sys.r32_pa_clr.write(|w| {unsafe{w.bits(out)}}),
            GpioPort::PB => self.sys.r32_pb_clr.write(|w| {unsafe{w.bits(out)}})
        }
    }
}