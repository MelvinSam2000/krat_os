pub struct Uart;

use core::fmt::Error;

const UART_BASE_ADDR: u32 = 0x1000_0000;

impl Uart {
    
    pub fn write_str(_: &str) -> Result<(), Error> { 
        todo!() 
    }
}