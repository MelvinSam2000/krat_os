use uart_16550::MmioSerialPort;

pub struct Uart{
    uart_hndl: MmioSerialPort
}

use core::fmt::Error;

const UART_BASE_ADDR: usize = 0x1000_0000;

impl Uart {

    pub fn new() -> Self {
        let mut uart_hndl = unsafe { MmioSerialPort::new(UART_BASE_ADDR) };
        uart_hndl.init();
        Self { uart_hndl }
    }
    
    pub fn write_str(&mut self, msg: &str) -> Result<(), Error> { 
        for ch in msg.bytes() {
            self.uart_hndl.send(ch);
        }
        Ok(())
    }
}