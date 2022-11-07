// code modified from https://github.com/RusPiRo/ruspiro-i2c/blob/development/src/interface.rs

use core::result::Result;

use volatile::prelude::*;
use volatile::Volatile;

use crate::common::IO_BASE;
use crate::gpio::{Function, Gpio};
use crate::timer;
use crate::timer::spin_sleep_ms;

use stack_vec::StackVec;

/// The base address for the `BSC0` registers.
const BSC0_REG_BASE: usize = IO_BASE + 0x804000;

/// The `AUXENB` register from page 9 of the BCM2837 documentation.
const I2C_MAX_BYTES: usize = 16; // max FiFo size of the I²C peripheral
const I2C_DEFAULT_WAIT: u32 = 2000; // max cycles to wait for a device to acknowledge a request

#[repr(u32)]
enum I2C_REG_C {
    DataReady = 1,
    TxAvailable = 1 << 5,
}

#[repr(C)]
#[allow(non_snake_case)]
struct Registers {
    I2C_C_REG: Volatile<u32>,
    I2C_S_REG: Volatile<u32>,
    I2C_DLEN_REG: Volatile<u32>,
    I2C_A_REG: Volatile<u32>,
    I2C_FIFO_REG: Volatile<u32>,
    I2C_DIV_REG: Volatile<u32>,
    I2C_DEL_REG: Volatile<u32>,
    I2C_CLKT_REG: Volatile<u32>,
}

/// The Raspberry Pi's I2C controller
pub struct I2C {
    registers: &'static mut Registers,
}

impl I2C {
    /// Initializes the I2C controller
    pub fn new(core_speed: u32, fast_mode: bool) -> I2C {
        let registers = unsafe { &mut *(BSC0_REG_BASE as *mut Registers) };
        Gpio::new(2).into_alt(Function::Alt0);
        Gpio::new(3).into_alt(Function::Alt0);
        let mut reset = Gpio::new(24).into_output();
        reset.set();
        spin_sleep_ms(1);
        reset.clear();
        spin_sleep_ms(10);
        reset.set();

        let clock_divisor = if fast_mode {
            core_speed / 400_000
        } else {
            core_speed / 100_000
        };

        registers.I2C_DIV_REG.write(clock_divisor);

        I2C { registers }
    }

    /// Scan for I2C devices currently connected to the I2C bus. The scan will just try to get an acknowledge message
    /// from any slave address between 0x00 and 0x7F. If a device is connected this call succeeds and the corresponding
    /// address is written to the console
    pub fn scan_devices<'a>(&'a mut self, storage: &'a mut [u8; 512]) -> StackVec<u8> {
        let mut r = StackVec::new(storage);

        for addr in 0x00..0x80 {
            self.registers.I2C_A_REG.write(addr as u32);
            self.registers.I2C_DLEN_REG.write(1);

            self.registers.I2C_S_REG.write(0b1100000010);
            self.registers.I2C_C_REG.write(0b1000000010110001);

            if self.wait_i2c_done(100).is_ok() {
                r.push(addr as u8);
            };
        }

        r
    }

    pub fn check_device(&mut self, addr: u8) -> Result<(), &'static str> {
        self.registers.I2C_A_REG.write(addr as u32);
        self.registers.I2C_DLEN_REG.write(1);
        self.registers.I2C_S_REG.write(0b1100000010);
        self.registers.I2C_C_REG.write(0b1000000010110001);

        self.wait_i2c_done(100)
    }

    fn read_reg_u8(&mut self, addr: u8, reg: u8) -> Result<u8, &'static str> {
        // reading I²C device regiser data means:
        // 1. write the register address to the device and wait for acknowledge
        // 2. read from the device and wait for acknowledge
        // 3. data available in the fifo
        self.write_register(addr, reg)?;
        self.registers.I2C_DLEN_REG.write(1);
        self.registers.I2C_S_REG.write(0b1100000010);
        self.registers.I2C_C_REG.write(0b1000000010110001);

        self.wait_i2c_done(I2C_DEFAULT_WAIT)?;

        let mut buff: [u8; 1] = [0; 1];
        self.read_fifo(&mut buff);

        Ok(buff[0])
    }

    pub fn read_reg_u16(&mut self, addr: u8, reg: u8) -> Result<u16, &'static str> {
        let mut buff: [u8; 2] = [0; 2];
        self.read_reg_data(addr, reg, &mut buff)?;
        Ok((buff[0] as u16) << 8 | (buff[1] as u16))
    }

    pub fn read_reg_data(
        &mut self,
        addr: u8,
        reg: u8,
        buffer: &mut [u8],
    ) -> Result<usize, &'static str> {
        // reading I²C device regiser data means:
        // 1. write the register address to the device and wait for acknowledge
        // 2. read from the device and wait for acknowledge
        // 3. data available in the fifo
        self.write_register(addr, reg)?;

        self.registers.I2C_DLEN_REG.write(buffer.len() as u32);
        self.registers.I2C_S_REG.write(0b1100000010);
        self.registers.I2C_C_REG.write(0b1000000010110001);

        self.wait_i2c_done(I2C_DEFAULT_WAIT)?;

        //let mut data: Vec<u8> = Vec::with_capacity(count as usize);
        let chunks = buffer.len() / I2C_MAX_BYTES;
        let mut remainder = buffer.len();
        for c in 0..chunks + 1 {
            let start = c * I2C_MAX_BYTES;
            let size = if remainder > I2C_MAX_BYTES {
                I2C_MAX_BYTES
            } else {
                remainder
            };

            self.read_fifo(&mut buffer[start..start + size]);

            remainder -= I2C_MAX_BYTES;
        }

        Ok(buffer.len())
    }

    pub fn write_raw_u8(&mut self, addr: u8, data: u8) -> Result<(), &'static str> {
        // clear status flags
        self.registers.I2C_S_REG.write(0b1100000010);

        // clear FiFo data in case FiFo data has remained from previous calls
        self.registers.I2C_C_REG.write(0b110000);

        // set the slave address we would like to send data to and the register id
        self.registers.I2C_A_REG.write(addr as u32);
        self.registers.I2C_DLEN_REG.write(1);
        self.registers.I2C_FIFO_REG.write(data as u32);

        // transmit the data
        self.registers.I2C_C_REG.write(0b1000000010000000);

        self.wait_i2c_done(I2C_DEFAULT_WAIT)
    }

    pub fn write_reg_u8(&mut self, addr: u8, reg: u8, data: u8) -> Result<(), &'static str> {
        // clear status flags
        self.registers.I2C_S_REG.write(0b1100000010);

        // clear FiFo data in case FiFo data has remained from previous calls
        self.registers.I2C_C_REG.write(0b110000);

        // set the slave address we would like to send data to and the register id
        self.registers.I2C_A_REG.write(addr as u32);
        self.registers.I2C_DLEN_REG.write(2);
        self.registers.I2C_FIFO_REG.write(reg as u32);
        self.registers.I2C_FIFO_REG.write(data as u32);

        // transmit the data
        self.registers.I2C_C_REG.write(0b1000000010000000);

        self.wait_i2c_done(I2C_DEFAULT_WAIT)
    }

    pub fn write_reg_u16(&mut self, addr: u8, reg: u8, data: u16) -> Result<(), &'static str> {
        let buffer: [u8; 2] = [(data >> 8) as u8, (data & 0xFF) as u8];
        self.write_reg_data(addr, reg, &buffer)
    }

    pub fn write_reg_data(&mut self, addr: u8, reg: u8, data: &[u8]) -> Result<(), &'static str> {
        let mut data_len = data.len();

        // clear status flags
        self.registers.I2C_S_REG.write(0b1100000010);

        // clear FiFo data in case FiFo data has remained from previous calls
        self.registers.I2C_C_REG.write(0b110000);

        // set the slave address we would like to send data to and the register id
        self.registers.I2C_A_REG.write(addr as u32);
        self.registers.I2C_DLEN_REG.write(data_len as u32 + 1);
        self.registers.I2C_FIFO_REG.write(reg as u32);

        // transmit the data
        self.registers.I2C_C_REG.write(0b1000000010000000);

        let chunks = data_len / I2C_MAX_BYTES;
        for chunk in 0..chunks + 1 {
            let idx = chunk * data_len;
            let len = if data_len > I2C_MAX_BYTES {
                I2C_MAX_BYTES
            } else {
                data_len
            };
            self.write_fifo(&data[idx..len]);
            data_len -= I2C_MAX_BYTES;
        }

        self.wait_i2c_done(I2C_DEFAULT_WAIT)
    }

    /// Wait until the current I2C operation has been finished/acknowledged
    /// Returns an [Err] in case of a timeout or not beein acknowledged
    fn wait_i2c_done(&mut self, tries: u32) -> Result<(), &'static str> {
        for _ in 0..tries {
            if self.registers.I2C_S_REG.has_mask(0b10) {
                if !self.registers.I2C_S_REG.has_mask(0b100000000) {
                    return Ok(());
                } else {
                    return Err("I2C transmit not acknowledged");
                }
            }
            timer::spin_sleep_ms(1000);
        }
        Err("time out waiting for I2C transmit")
    }

    /// Write the register to the I2C device we would like to access next (e.g. write to)
    fn write_register(&mut self, addr: u8, reg: u8) -> Result<(), &'static str> {
        // set the slave address we would like to send data to and the register id
        self.registers.I2C_A_REG.write(addr as u32);
        self.registers.I2C_DLEN_REG.write(1);
        self.registers.I2C_FIFO_REG.write(reg as u32);

        // transmit the data
        self.registers.I2C_S_REG.write(0b1100000010);
        self.registers.I2C_C_REG.write(0b1000000010000000);

        self.wait_i2c_done(I2C_DEFAULT_WAIT)
    }

    /// Read the data from the I2C FIFO register
    fn read_fifo(&mut self, buffer: &mut [u8]) -> usize {
        //let mut data: Vec<u8> = Vec::with_capacity(count as usize);
        let num = if buffer.len() > I2C_MAX_BYTES {
            I2C_MAX_BYTES
        } else {
            buffer.len()
        };
        for i in 0..num {
            while !self.registers.I2C_S_REG.has_mask(0b100000) {}
            buffer[i] = (self.registers.I2C_FIFO_REG.read() & 0xFF) as u8;
        }
        num
    }

    /// Write a data buffer to the FIFO
    fn write_fifo(&mut self, data: &[u8]) {
        for i in 0..data.len() {
            while !self.registers.I2C_S_REG.has_mask(0b10000) {}
            self.registers.I2C_FIFO_REG.write(data[i] as u32);
        }
    }
}

impl embedded_hal::blocking::i2c::Write for I2C {
    type Error = &'static str;

    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        self.write_reg_data(address, 0x00, bytes)?;

        Ok(())
    }
}
